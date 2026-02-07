//! Retention tools.
//!
//! Tools: strata_retention_apply

use serde_json::{Map, Value as JsonValue};
use stratadb::Command;

use crate::convert::output_to_json;
use crate::error::{McpError, Result};
use crate::schema;
use crate::session::McpSession;
use crate::tools::ToolDef;

/// Get all retention tool definitions.
pub fn tools() -> Vec<ToolDef> {
    vec![ToolDef::new(
        "strata_retention_apply",
        "Apply the retention policy to the current branch, trimming old versions \
         and expired data according to configured rules. Returns null on success.",
        schema!(object {}),
    )]
}

/// Dispatch a retention tool call.
pub fn dispatch(
    session: &mut McpSession,
    name: &str,
    _args: Map<String, JsonValue>,
) -> Result<JsonValue> {
    match name {
        "strata_retention_apply" => {
            let cmd = Command::RetentionApply {
                branch: session.branch_id(),
            };
            let output = session.execute(cmd)?;
            Ok(output_to_json(output))
        }

        _ => Err(McpError::UnknownTool(name.to_string())),
    }
}
