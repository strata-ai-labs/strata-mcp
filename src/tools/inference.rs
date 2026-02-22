//! Inference tools (text generation, tokenization).
//!
//! Tools: strata_generate, strata_tokenize, strata_detokenize, strata_generate_unload

use serde_json::{Map, Value as JsonValue};
use stratadb::Command;

use crate::convert::{
    get_optional_bool, get_optional_u64, get_string_arg, output_to_json,
};
use crate::error::{McpError, Result};
use crate::schema;
use crate::session::McpSession;
use crate::tools::ToolDef;

/// Get all inference tool definitions.
pub fn tools() -> Vec<ToolDef> {
    vec![
        ToolDef::new(
            "strata_generate",
            "Generate text using a locally loaded model. Requires a model to be pulled \
             first with strata_models_pull. Returns text, stop_reason, prompt_tokens, \
             completion_tokens, and model name.",
            schema!(object {
                required: { "model": string, "prompt": string },
                optional: {
                    "max_tokens": integer,
                    "temperature": number,
                    "top_k": integer,
                    "top_p": number,
                    "seed": integer,
                    "stop_tokens": array_number
                }
            }),
        ),
        ToolDef::new(
            "strata_tokenize",
            "Tokenize text into token IDs using a model's tokenizer. \
             Returns ids (array of ints), count, and model name.",
            schema!(object {
                required: { "model": string, "text": string },
                optional: { "add_special_tokens": boolean }
            }),
        ),
        ToolDef::new(
            "strata_detokenize",
            "Convert token IDs back to text using a model's tokenizer. \
             Returns the decoded text string.",
            schema!(object {
                required: { "model": string, "ids": array_number }
            }),
        ),
        ToolDef::new(
            "strata_generate_unload",
            "Unload a model from memory, freeing GPU/CPU resources. \
             Returns true if the model was loaded and is now unloaded.",
            schema!(object {
                required: { "model": string }
            }),
        ),
    ]
}

/// Dispatch an inference tool call.
pub fn dispatch(
    session: &mut McpSession,
    name: &str,
    args: Map<String, JsonValue>,
) -> Result<JsonValue> {
    match name {
        "strata_generate" => {
            let model = get_string_arg(&args, "model")?;
            let prompt = get_string_arg(&args, "prompt")?;
            let max_tokens = get_optional_u64(&args, "max_tokens").map(|v| v as usize);
            let temperature = get_optional_f32(&args, "temperature");
            let top_k = get_optional_u64(&args, "top_k").map(|v| v as usize);
            let top_p = get_optional_f32(&args, "top_p");
            let seed = get_optional_u64(&args, "seed");
            let stop_tokens = get_optional_u32_array(&args, "stop_tokens");

            let output = session.execute(Command::Generate {
                model,
                prompt,
                max_tokens,
                temperature,
                top_k,
                top_p,
                seed,
                stop_tokens,
            })?;
            Ok(output_to_json(output))
        }

        "strata_tokenize" => {
            let model = get_string_arg(&args, "model")?;
            let text = get_string_arg(&args, "text")?;
            let add_special_tokens = get_optional_bool(&args, "add_special_tokens");

            let output = session.execute(Command::Tokenize {
                model,
                text,
                add_special_tokens,
            })?;
            Ok(output_to_json(output))
        }

        "strata_detokenize" => {
            let model = get_string_arg(&args, "model")?;
            let ids = get_u32_array(&args, "ids")?;

            let output = session.execute(Command::Detokenize { model, ids })?;
            Ok(output_to_json(output))
        }

        "strata_generate_unload" => {
            let model = get_string_arg(&args, "model")?;
            let output = session.execute(Command::GenerateUnload { model })?;
            Ok(output_to_json(output))
        }

        _ => Err(McpError::UnknownTool(name.to_string())),
    }
}

/// Helper to get an optional f32 argument.
fn get_optional_f32(args: &Map<String, JsonValue>, name: &str) -> Option<f32> {
    args.get(name).and_then(|v| v.as_f64()).map(|f| f as f32)
}

/// Helper to get a required u32 array argument.
fn get_u32_array(args: &Map<String, JsonValue>, name: &str) -> Result<Vec<u32>> {
    let arr = args
        .get(name)
        .and_then(|v| v.as_array())
        .ok_or_else(|| McpError::MissingArg(name.to_string()))?;

    arr.iter()
        .map(|v| {
            v.as_u64().map(|n| n as u32).ok_or_else(|| McpError::InvalidArg {
                name: name.to_string(),
                reason: "Expected array of integers".to_string(),
            })
        })
        .collect()
}

/// Helper to get an optional u32 array argument.
fn get_optional_u32_array(args: &Map<String, JsonValue>, name: &str) -> Option<Vec<u32>> {
    args.get(name)
        .and_then(|v| v.as_array())
        .map(|arr| arr.iter().filter_map(|v| v.as_u64().map(|n| n as u32)).collect())
}
