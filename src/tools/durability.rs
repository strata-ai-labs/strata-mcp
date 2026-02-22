//! Durability tools.
//!
//! Tools: strata_durability_counters

use serde_json::{Map, Value as JsonValue};
use stratadb::Command;

use crate::convert::output_to_json;
use crate::error::{McpError, Result};
use crate::schema;
use crate::session::McpSession;
use crate::tools::ToolDef;

/// Get all durability tool definitions.
pub fn tools() -> Vec<ToolDef> {
    vec![ToolDef::new(
        "strata_durability_counters",
        "Get WAL (Write-Ahead Log) durability counters for monitoring write \
         performance. Returns wal_appends, sync_calls, bytes_written, and sync_nanos.",
        schema!(object {}),
    )]
}

/// Dispatch a durability tool call.
pub fn dispatch(
    session: &mut McpSession,
    name: &str,
    _args: Map<String, JsonValue>,
) -> Result<JsonValue> {
    match name {
        "strata_durability_counters" => {
            let output = session.execute(Command::DurabilityCounters)?;
            Ok(output_to_json(output))
        }

        _ => Err(McpError::UnknownTool(name.to_string())),
    }
}
