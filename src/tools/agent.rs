//! Agent-friendly tools.
//!
//! Provides 8 high-level, intent-driven tools designed for AI agent consumption.
//! These collapse Strata's granular developer operations into a simple cognitive interface:
//!
//! - `strata_store`   — Store data (JSON documents with optional path updates)
//! - `strata_recall`  — Retrieve data by key (with optional path and time-travel)
//! - `strata_search`  — Find relevant data via natural language
//! - `strata_forget`  — Delete data by key
//! - `strata_log`     — Append immutable events
//! - `strata_branch`  — Branching for safe experimentation
//! - `strata_history` — Time-travel and version history
//! - `strata_status`  — Database introspection
//!
//! All data operations are backed by the JSON document store, which gives agents
//! structured document access with optional JSONPath targeting.

use serde_json::{Map, Value as JsonValue};
use stratadb::{BranchId, Command, MergeStrategy, Output, SearchQuery};

use crate::convert::{
    get_optional_string, get_optional_u64, get_string_arg, get_value_arg, output_to_json,
};
use crate::error::{McpError, Result};
use crate::schema;
use crate::session::McpSession;
use crate::tools::ToolDef;

/// Get all agent tool definitions.
pub fn tools() -> Vec<ToolDef> {
    vec![
        // ── Core Data Tools ──────────────────────────────────────────────
        ToolDef::new(
            "strata_store",
            "Store data with a key. The value can be any type (string, number, boolean, object, \
             or array). Use path to update a specific field within an existing document (JSONPath \
             syntax, e.g. '$.settings.theme') — omit path to store the entire value. When \
             auto-embed is enabled, text content is automatically indexed for semantic search \
             via strata_search. Overwrites existing data at the given path. Returns the version \
             number.",
            schema!(object {
                required: { "key": string, "value": any },
                optional: { "path": string }
            }),
        ),
        ToolDef::new(
            "strata_recall",
            "Retrieve data by key. Returns the stored value with version info, or null if the \
             key doesn't exist. Use path to read a specific field (JSONPath syntax, e.g. \
             '$.settings.theme') — omit path to get the entire document. Pass as_of \
             (microsecond timestamp) to see what this key contained at a past point in time.",
            schema!(object {
                required: { "key": string },
                optional: { "path": string, "as_of": integer }
            }),
        ),
        ToolDef::new(
            "strata_search",
            "Search across all stored data using natural language. Returns ranked results with \
             relevance scores and text snippets. Uses keyword matching by default; adds semantic \
             similarity when auto-embed is enabled. Use this when you need to find data but don't \
             know the exact key.",
            schema!(object {
                required: { "query": string },
                optional: { "k": integer }
            }),
        ),
        ToolDef::new(
            "strata_forget",
            "Delete data by key. Returns true if the key existed, false otherwise.",
            schema!(object {
                required: { "key": string }
            }),
        ),
        ToolDef::new(
            "strata_log",
            "Append an immutable event to the log. Events are ordered, timestamped, and grouped \
             by type. Unlike strata_store, events cannot be overwritten — they form a permanent, \
             append-only record. Use for recording actions, observations, decisions, or any \
             sequential data.",
            schema!(object {
                required: { "event": string, "data": any }
            }),
        ),
        // ── Power Tools ──────────────────────────────────────────────────
        ToolDef::new(
            "strata_branch",
            "Manage branches for safe experimentation. Branches are instant, copy-on-write \
             snapshots of all data. Actions: 'create' a new empty branch, 'switch' to a branch, \
             'list' all branches, 'fork' the current branch (copies all data), 'merge' another \
             branch into current, 'diff' to compare current branch against another, 'delete' a \
             branch. The recommended workflow: fork → experiment → merge (or delete to discard).",
            serde_json::json!({
                "type": "object",
                "properties": {
                    "action": {
                        "type": "string",
                        "enum": ["create", "switch", "list", "fork", "merge", "diff", "delete"],
                        "description": "The branch operation to perform"
                    },
                    "name": {
                        "type": "string",
                        "description": "Branch name — used by create, switch, fork, delete"
                    },
                    "source": {
                        "type": "string",
                        "description": "Source branch to merge from — used by merge"
                    },
                    "compare": {
                        "type": "string",
                        "description": "Branch to compare against current — used by diff"
                    }
                },
                "required": ["action"]
            }),
        ),
        ToolDef::new(
            "strata_history",
            "View version history for a key, or get the time range available for time-travel \
             queries. With a key: returns all historical values with versions and timestamps \
             (useful for undo or audit). Without a key: returns the oldest and latest timestamps \
             for the current branch.",
            schema!(object {
                optional: { "key": string, "as_of": integer }
            }),
        ),
        ToolDef::new(
            "strata_status",
            "Get database status including current branch, version, auto-embed state, branch \
             count, and key count. Use this to orient yourself at the start of a session.",
            schema!(object {}),
        ),
    ]
}

/// Dispatch an agent tool call.
pub fn dispatch(
    session: &mut McpSession,
    name: &str,
    args: Map<String, JsonValue>,
) -> Result<JsonValue> {
    match name {
        "strata_store" => dispatch_store(session, args),
        "strata_recall" => dispatch_recall(session, args),
        "strata_search" => dispatch_search(session, args),
        "strata_forget" => dispatch_forget(session, args),
        "strata_log" => dispatch_log(session, args),
        "strata_branch" => dispatch_branch(session, args),
        "strata_history" => dispatch_history(session, args),
        "strata_status" => dispatch_status(session),
        _ => Err(McpError::UnknownTool(name.to_string())),
    }
}

// ── Store ────────────────────────────────────────────────────────────────

fn dispatch_store(session: &mut McpSession, args: Map<String, JsonValue>) -> Result<JsonValue> {
    let key = get_string_arg(&args, "key")?;
    let value = get_value_arg(&args, "value")?;
    let path = get_optional_string(&args, "path").unwrap_or_else(|| "$".to_string());

    let cmd = Command::JsonSet {
        branch: session.branch_id(),
        space: session.space_id(),
        key: key.clone(),
        path,
        value,
    };
    let output = session.execute(cmd)?;

    match output {
        Output::Version(v) => Ok(serde_json::json!({
            "key": key,
            "version": v,
            "stored": true,
        })),
        other => Ok(output_to_json(other)),
    }
}

// ── Recall ───────────────────────────────────────────────────────────────

fn dispatch_recall(session: &mut McpSession, args: Map<String, JsonValue>) -> Result<JsonValue> {
    let key = get_string_arg(&args, "key")?;
    let path = get_optional_string(&args, "path").unwrap_or_else(|| "$".to_string());
    let as_of = get_optional_u64(&args, "as_of");

    let cmd = Command::JsonGet {
        branch: session.branch_id(),
        space: session.space_id(),
        key,
        path,
        as_of,
    };
    let output = session.execute(cmd)?;
    Ok(output_to_json(output))
}

// ── Search ───────────────────────────────────────────────────────────────

fn dispatch_search(session: &mut McpSession, args: Map<String, JsonValue>) -> Result<JsonValue> {
    let query = get_string_arg(&args, "query")?;
    let k = get_optional_u64(&args, "k");

    let sq = SearchQuery {
        query,
        k,
        primitives: None, // search all primitives
        time_range: None,
        mode: None, // engine picks best available (hybrid when auto-embed is on)
        expand: None,
        rerank: None,
    };

    let cmd = Command::Search {
        branch: session.branch_id(),
        space: session.space_id(),
        search: sq,
    };
    let output = session.execute(cmd)?;

    // Simplify search results for agent consumption
    match output {
        Output::SearchResults(results) => {
            let arr: Vec<JsonValue> = results
                .into_iter()
                .map(|r| {
                    serde_json::json!({
                        "key": r.entity,
                        "score": r.score,
                        "snippet": r.snippet,
                    })
                })
                .collect();
            Ok(JsonValue::Array(arr))
        }
        other => Ok(output_to_json(other)),
    }
}

// ── Forget ───────────────────────────────────────────────────────────────

fn dispatch_forget(session: &mut McpSession, args: Map<String, JsonValue>) -> Result<JsonValue> {
    let key = get_string_arg(&args, "key")?;

    let cmd = Command::JsonDelete {
        branch: session.branch_id(),
        space: session.space_id(),
        key,
        path: "$".to_string(),
    };
    let output = session.execute(cmd)?;

    match output {
        Output::Uint(n) => Ok(serde_json::json!({ "deleted": n > 0 })),
        other => Ok(output_to_json(other)),
    }
}

// ── Log ──────────────────────────────────────────────────────────────────

fn dispatch_log(session: &mut McpSession, args: Map<String, JsonValue>) -> Result<JsonValue> {
    let event = get_string_arg(&args, "event")?;
    let data = get_value_arg(&args, "data")?;

    let cmd = Command::EventAppend {
        branch: session.branch_id(),
        space: session.space_id(),
        event_type: event,
        payload: data,
    };
    let output = session.execute(cmd)?;

    match output {
        Output::Version(v) => Ok(serde_json::json!({
            "sequence": v,
            "logged": true,
        })),
        other => Ok(output_to_json(other)),
    }
}

// ── Branch ───────────────────────────────────────────────────────────────

fn dispatch_branch(session: &mut McpSession, args: Map<String, JsonValue>) -> Result<JsonValue> {
    let action = get_string_arg(&args, "action")?;

    match action.as_str() {
        "create" => {
            let name = get_optional_string(&args, "name");
            let cmd = Command::BranchCreate {
                branch_id: name,
                metadata: None,
            };
            let output = session.execute(cmd)?;
            Ok(output_to_json(output))
        }

        "switch" => {
            let name = get_string_arg(&args, "name")?;
            session.switch_branch(&name)?;
            Ok(serde_json::json!({
                "switched": true,
                "branch": name,
            }))
        }

        "list" => {
            let cmd = Command::BranchList {
                state: None,
                limit: None,
                offset: None,
            };
            let output = session.execute(cmd)?;
            Ok(output_to_json(output))
        }

        "fork" => {
            let name = get_string_arg(&args, "name")?;
            let info = session.fork_branch(&name)?;
            Ok(serde_json::json!({
                "forked": true,
                "source": info.source,
                "destination": info.destination,
                "keys_copied": info.keys_copied,
            }))
        }

        "merge" => {
            let source = get_string_arg(&args, "source")?;
            let info = session.merge_branch(&source, MergeStrategy::LastWriterWins)?;

            let conflicts: Vec<JsonValue> = info
                .conflicts
                .into_iter()
                .map(|c| {
                    serde_json::json!({
                        "key": c.key,
                        "space": c.space,
                    })
                })
                .collect();

            Ok(serde_json::json!({
                "merged": true,
                "keys_applied": info.keys_applied,
                "spaces_merged": info.spaces_merged,
                "conflicts": conflicts,
            }))
        }

        "diff" => {
            let compare = get_string_arg(&args, "compare")?;
            let current = session.branch().to_string();
            let diff = session.diff_branches(&current, &compare)?;

            Ok(serde_json::json!({
                "current_branch": diff.branch_a,
                "compare_branch": diff.branch_b,
                "added": diff.summary.total_added,
                "removed": diff.summary.total_removed,
                "modified": diff.summary.total_modified,
            }))
        }

        "delete" => {
            let name = get_string_arg(&args, "name")?;
            let cmd = Command::BranchDelete {
                branch: BranchId::from(name),
            };
            let output = session.execute(cmd)?;
            Ok(output_to_json(output))
        }

        other => Err(McpError::InvalidArg {
            name: "action".to_string(),
            reason: format!(
                "Unknown action '{}'. Use: create, switch, list, fork, merge, diff, or delete.",
                other
            ),
        }),
    }
}

// ── History ──────────────────────────────────────────────────────────────

fn dispatch_history(session: &mut McpSession, args: Map<String, JsonValue>) -> Result<JsonValue> {
    let key = get_optional_string(&args, "key");
    let as_of = get_optional_u64(&args, "as_of");

    match key {
        Some(key) => {
            // Version history for a specific key (JSON document store)
            let cmd = Command::JsonGetv {
                branch: session.branch_id(),
                space: session.space_id(),
                key,
                as_of,
            };
            let output = session.execute(cmd)?;
            Ok(output_to_json(output))
        }
        None => {
            // Branch time range
            let cmd = Command::TimeRange {
                branch: session.branch_id(),
            };
            let output = session.execute(cmd)?;

            match output {
                Output::TimeRange {
                    oldest_ts,
                    latest_ts,
                } => Ok(serde_json::json!({
                    "branch": session.branch(),
                    "oldest": oldest_ts,
                    "latest": latest_ts,
                })),
                other => Ok(output_to_json(other)),
            }
        }
    }
}

// ── Status ───────────────────────────────────────────────────────────────

fn dispatch_status(session: &mut McpSession) -> Result<JsonValue> {
    let info_output = session.execute(Command::Info)?;

    let mut result = match info_output {
        Output::DatabaseInfo(info) => serde_json::json!({
            "version": info.version,
            "branch": session.branch(),
            "namespace": session.space(),
            "branches": info.branch_count,
            "keys": info.total_keys,
            "uptime_secs": info.uptime_secs,
        }),
        _ => serde_json::json!({
            "branch": session.branch(),
            "namespace": session.space(),
        }),
    };

    // Include auto-embed status if available
    if let Ok(Output::EmbedStatus(embed)) = session.execute(Command::EmbedStatus) {
        if let Some(obj) = result.as_object_mut() {
            obj.insert("auto_embed".to_string(), JsonValue::Bool(embed.auto_embed));
        }
    }

    Ok(result)
}
