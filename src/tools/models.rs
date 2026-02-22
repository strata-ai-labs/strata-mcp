//! Model management tools.
//!
//! Tools: strata_models_list, strata_models_pull, strata_models_local

use serde_json::{Map, Value as JsonValue};
use stratadb::Command;

use crate::convert::{get_string_arg, output_to_json};
use crate::error::{McpError, Result};
use crate::schema;
use crate::session::McpSession;
use crate::tools::ToolDef;

/// Get all model management tool definitions.
pub fn tools() -> Vec<ToolDef> {
    vec![
        ToolDef::new(
            "strata_models_list",
            "List all available models from the model registry. Returns an array of \
             model info objects with name, task, architecture, default_quant, \
             embedding_dim, is_local, and size_bytes.",
            schema!(object {}),
        ),
        ToolDef::new(
            "strata_models_pull",
            "Download a model by name from the registry. Returns a dict with the \
             model name and local file path.",
            schema!(object {
                required: { "name": string }
            }),
        ),
        ToolDef::new(
            "strata_models_local",
            "List models that have been downloaded locally. Same format as \
             strata_models_list but only includes models available on disk.",
            schema!(object {}),
        ),
    ]
}

/// Dispatch a model management tool call.
pub fn dispatch(
    session: &mut McpSession,
    name: &str,
    args: Map<String, JsonValue>,
) -> Result<JsonValue> {
    match name {
        "strata_models_list" => {
            let output = session.execute(Command::ModelsList)?;
            Ok(output_to_json(output))
        }

        "strata_models_pull" => {
            let name_arg = get_string_arg(&args, "name")?;
            let output = session.execute(Command::ModelsPull { name: name_arg })?;
            Ok(output_to_json(output))
        }

        "strata_models_local" => {
            let output = session.execute(Command::ModelsLocal)?;
            Ok(output_to_json(output))
        }

        _ => Err(McpError::UnknownTool(name.to_string())),
    }
}
