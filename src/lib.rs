//! # strata-mcp
//!
//! MCP (Model Context Protocol) server for Strata database.
//!
//! This crate provides an MCP server that exposes Strata database operations as tools
//! for AI agents. It implements the MCP protocol over stdin/stdout using JSON-RPC 2.0.
//!
//! ## 8 Agent-Friendly Tools
//!
//! `store`, `recall`, `search`, `forget`, `log`, `branch`, `history`, `status`
//!
//! Backed by Strata's JSON document store with automatic semantic search indexing.
//!
//! ## Usage
//!
//! The server is typically run as an executable and configured in AI tools like Claude Desktop:
//!
//! ```json
//! {
//!   "mcpServers": {
//!     "strata": {
//!       "command": "/path/to/strata-mcp",
//!       "args": ["--db", "/path/to/data", "--auto-embed"]
//!     }
//!   }
//! }
//! ```
//!
//! ## Library Usage
//!
//! For testing or embedding, you can use the library API:
//!
//! ```no_run
//! use strata_mcp::{McpServer, McpSession};
//! use stratadb::Strata;
//!
//! let db = Strata::cache().expect("Failed to create database");
//! let session = McpSession::new(db);
//! let mut server = McpServer::new(session);
//!
//! // Run the server (reads from stdin, writes to stdout)
//! // server.run_sync().expect("Server error");
//! ```

#![warn(missing_docs)]

mod convert;
mod error;
mod server;
mod session;
mod tools;

pub use convert::{json_to_value, output_to_json, value_to_json};
pub use error::{McpError, Result};
pub use server::{JsonRpcRequest, JsonRpcResponse, McpServer};
pub use session::McpSession;
pub use tools::{ToolDef, ToolRegistry};
