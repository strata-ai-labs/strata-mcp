# strata-mcp

MCP (Model Context Protocol) server for [Strata](https://github.com/strata-ai-labs/strata-core) database.

Designed for AI agents. 8 intent-driven tools by default — store, recall, search, forget, log, branch, history, status. No database concepts exposed.

## Installation

### From Source

```bash
git clone https://github.com/strata-ai-labs/strata-mcp.git
cd strata-mcp
cargo build --release
```

The binary will be at `target/release/strata-mcp`.

## Quick Start

### With Claude Desktop

Add to your `claude_desktop_config.json`:

```json
{
  "mcpServers": {
    "strata": {
      "command": "/path/to/strata-mcp",
      "args": ["--db", "/path/to/your/data", "--auto-embed"]
    }
  }
}
```

This gives the AI agent 8 tools with automatic semantic search. That's it.

### With Claude Code

Add to your `.mcp.json`:

```json
{
  "mcpServers": {
    "strata": {
      "command": "/path/to/strata-mcp",
      "args": ["--db", ".strata", "--auto-embed"]
    }
  }
}
```

### In-Memory (Ephemeral)

For testing without persistence:

```json
{
  "mcpServers": {
    "strata": {
      "command": "/path/to/strata-mcp",
      "args": ["--cache", "--auto-embed"]
    }
  }
}
```

## Command Line Options

```
strata-mcp [OPTIONS]

Options:
  --db <PATH>       Path to the database directory
  --cache           Use an in-memory database (no persistence)
  --read-only       Open database in read-only mode
  --auto-embed      Enable automatic text embedding for semantic search
-v, --verbose     Enable debug logging to stderr
  -h, --help        Print help
  -V, --version     Print version
```

## Agent Tools (default — 8 tools)

These are the tools AI agents see. Intent-driven naming, no database internals exposed.

| Tool | Intent | Description |
|------|--------|-------------|
| `strata_store` | "Remember this" | Store data with a key. Auto-embeds text for semantic search. |
| `strata_recall` | "What's stored here?" | Retrieve data by key. Supports time-travel via `as_of`. |
| `strata_search` | "Find relevant things" | Natural language search across all data. Hybrid keyword + semantic. |
| `strata_forget` | "Delete this" | Delete data by key. |
| `strata_log` | "This happened" | Append an immutable event. Ordered, timestamped, grouped by type. |
| `strata_branch` | "Work in isolation" | Create, switch, fork, merge, diff, delete branches. |
| `strata_history` | "What changed?" | Version history for a key, or time range for the branch. |
| `strata_status` | "What's going on?" | Database info, current branch, auto-embed state. |

### Example Conversation

```
User: Store my preferences
Agent: [calls strata_store with key="preferences", value={"theme": "dark", "language": "en"}]

User: Find anything about themes
Agent: [calls strata_search with query="theme settings"]
→ Returns: [{key: "preferences", score: 0.92, snippet: "theme: dark"}]

User: Let me experiment with a new setup
Agent: [calls strata_branch with action="fork", name="experiment"]
Agent: [calls strata_branch with action="switch", name="experiment"]
Agent: [calls strata_store with key="preferences", value={"theme": "light", "language": "fr"}]

User: Actually, go back to the original
Agent: [calls strata_branch with action="switch", name="default"]
→ Original preferences are intact

User: What did preferences look like over time?
Agent: [calls strata_history with key="preferences"]
→ Returns all versions with timestamps
```

### Why 8 Tools?

Research across 26 MCP servers (Qdrant, Neon, Supabase, MongoDB, Mem0, etc.) shows:
- **10 tools** = 100% tool selection accuracy
- **30+ tools** = accuracy degrades
- **Cursor** hard-limits at 40 MCP tools total across all servers

Strata's 8 tools are modeled after Qdrant (2 tools, the gold standard for AI-friendliness), extended with branches and time-travel — Strata's unique differentiators.

## Claude Code Skill

The `/strata` skill teaches Claude Code what Strata is and when to use each tool. It triggers automatically when you mention storing, searching, branching, or persisting data.

### Installation

Copy the skill into your project or global Claude Code config:

```bash
# Per-project (recommended)
mkdir -p .claude/skills/strata
cp /path/to/strata-mcp/.claude/skills/strata/SKILL.md .claude/skills/strata/

# Global (available in all projects)
mkdir -p ~/.claude/skills/strata
cp /path/to/strata-mcp/.claude/skills/strata/SKILL.md ~/.claude/skills/strata/
```

Once installed, Claude Code will use `/strata` automatically when relevant, or you can invoke it directly with `/strata [action or question]`.

## Read-Only Mode

When `--read-only` is used, all write operations are rejected with an `ACCESS_DENIED` error.
This is useful for sharing a database safely with AI agents that should only read data.

## Time-Travel

Most read operations support an optional `as_of` parameter (microseconds since epoch)
for querying historical data. Use `strata_history` to discover the available time range.

## Protocol

The server implements [MCP](https://modelcontextprotocol.io/) over JSON-RPC 2.0 on stdin/stdout.

Supported methods:
- `initialize` — Initialize the server
- `tools/list` — List available tools
- `tools/call` — Execute a tool
- `ping` — Health check

## Development

```bash
# Run tests
cargo test

# Run with in-memory database
./target/release/strata-mcp --cache -v

# Test with a JSON-RPC request
echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{}}' | ./target/release/strata-mcp --cache
```

## License

MIT
