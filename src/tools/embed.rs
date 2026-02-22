//! Embedding tools.
//!
//! Tools: strata_embed, strata_embed_batch, strata_embed_status

use serde_json::{Map, Value as JsonValue};
use stratadb::Command;

use crate::convert::{get_string_arg, output_to_json};
use crate::error::{McpError, Result};
use crate::schema;
use crate::session::McpSession;
use crate::tools::ToolDef;

/// Get all embedding tool definitions.
pub fn tools() -> Vec<ToolDef> {
    vec![
        ToolDef::new(
            "strata_embed",
            "Embed a single text string into a dense vector using the built-in \
             embedding model. Requires auto-embed to be enabled. Returns a float32 array.",
            schema!(object {
                required: { "text": string }
            }),
        ),
        ToolDef::new(
            "strata_embed_batch",
            "Embed multiple text strings into dense vectors. More efficient than \
             calling strata_embed in a loop. Returns an array of float32 arrays.",
            schema!(object {
                required: { "texts": array_string }
            }),
        ),
        ToolDef::new(
            "strata_embed_status",
            "Get the status of the embedding pipeline. Returns auto_embed (bool), \
             batch_size, pending items, total_queued, total_embedded, total_failed, \
             scheduler_queue_depth, scheduler_active_tasks, and is_idle.",
            schema!(object {}),
        ),
    ]
}

/// Dispatch an embedding tool call.
pub fn dispatch(
    session: &mut McpSession,
    name: &str,
    args: Map<String, JsonValue>,
) -> Result<JsonValue> {
    match name {
        "strata_embed" => {
            let text = get_string_arg(&args, "text")?;
            let output = session.execute(Command::Embed { text })?;
            Ok(output_to_json(output))
        }

        "strata_embed_batch" => {
            let texts_json = args
                .get("texts")
                .and_then(|v| v.as_array())
                .ok_or_else(|| McpError::MissingArg("texts".to_string()))?;

            let texts: Result<Vec<String>> = texts_json
                .iter()
                .map(|v| {
                    v.as_str()
                        .map(|s| s.to_string())
                        .ok_or_else(|| McpError::InvalidArg {
                            name: "texts".to_string(),
                            reason: "Expected array of strings".to_string(),
                        })
                })
                .collect();

            let output = session.execute(Command::EmbedBatch { texts: texts? })?;
            Ok(output_to_json(output))
        }

        "strata_embed_status" => {
            let output = session.execute(Command::EmbedStatus)?;
            Ok(output_to_json(output))
        }

        _ => Err(McpError::UnknownTool(name.to_string())),
    }
}
