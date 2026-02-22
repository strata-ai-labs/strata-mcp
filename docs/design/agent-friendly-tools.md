# Design: Agent-Friendly MCP Tool Surface

## Problem

Strata's MCP server exposes 74 tools across 17 categories. Research shows:
- 10 tools = 100% tool selection accuracy
- 20 tools = 95% accuracy
- 30+ tools = degradation begins
- Cursor hard-limits at 40 MCP tools total across all servers

At 74 tools, Strata alone exceeds Cursor's limit and will cause tool selection confusion in every LLM. The current design mirrors the SDK — it's a developer interface, not an AI interface.

## Design Principles

1. **Intent over operation**: `store` not `kv_put`. The agent says what it wants, not how to do it.
2. **Hide the primitives**: The agent shouldn't know about KV vs JSON vs State vs Vector. These are implementation details.
3. **Built-in intelligence**: Auto-embed on store, hybrid search on recall. No vector concepts exposed.
4. **Expose differentiators**: Branches and time-travel are Strata's unique capabilities. These deserve first-class tools.
5. **Developer tools stay in SDKs**: Transactions, retention, durability, model config, bundles, compaction — these are for Python/Node SDKs, not AI agents.

## Proposed Tool Surface: 8 Tools

### Core Data Tools (5)

#### `strata_store`
**Intent**: "Remember this."

Store data with a key. Auto-embeds text content when `--auto-embed` is enabled, making it searchable via `strata_search` with zero extra work.

```
Parameters:
  key:       string (required) — identifier for the data
  value:     any    (required) — the data to store (string, number, object, array)
  tags:      array  (optional) — string tags for filtering during search
  namespace: string (optional) — logical grouping (defaults to "default")
```

Returns: `{ key, version, timestamp }`

**Behind the scenes**: Routes to KV store. If `--auto-embed` is on and value contains text, generates embedding and indexes it in the vector store for semantic search. Tags are stored as metadata for filtered retrieval.

#### `strata_recall`
**Intent**: "What did I store under this key?"

Retrieve data by exact key. Fast, direct lookup.

```
Parameters:
  key:       string  (required) — the key to look up
  namespace: string  (optional) — namespace to look in (defaults to "default")
  as_of:     string  (optional) — point-in-time query (ISO 8601 or relative like "1h ago")
```

Returns: `{ key, value, version, timestamp }` or `null` if not found.

**Time-travel**: Pass `as_of` to see what this key held at a past point in time. Use `strata_history` to discover available timestamps.

#### `strata_search`
**Intent**: "Find things related to this."

Hybrid search across all stored data. Uses keyword matching by default; adds semantic similarity when `--auto-embed` is enabled.

```
Parameters:
  query:     string  (required) — natural language search query
  k:         integer (optional) — max results to return (default: 10, max: 50)
  tags:      array   (optional) — filter results to entries with these tags
  namespace: string  (optional) — restrict search to namespace (default: search all)
```

Returns: `[{ key, value, score, snippet }]` — ranked by relevance, most relevant first.

**Why this works**: Strata's cross-primitive search already handles keyword + BM25 + vector similarity internally. This tool just exposes it with a clean interface. The agent never sees embeddings, vectors, or search modes.

#### `strata_forget`
**Intent**: "Delete this."

Remove data by key. Also removes associated embeddings and search index entries.

```
Parameters:
  key:       string (required) — the key to delete
  namespace: string (optional) — namespace (defaults to "default")
```

Returns: `{ deleted: true/false }`

#### `strata_log`
**Intent**: "This happened."

Append an event to the event log. Unlike `strata_store`, events are immutable and append-only — ideal for recording actions, observations, and state changes.

```
Parameters:
  event:   string (required) — event type label (e.g., "user_action", "observation")
  data:    any    (required) — event payload
```

Returns: `{ sequence, timestamp }`

**Why separate from store**: Events are fundamentally different — they're append-only, ordered, and typed. An agent logging its actions, recording observations, or tracking a workflow needs an event stream, not a key-value store.

### Power Tools (3)

#### `strata_branch`
**Intent**: "Work in isolation."

Branch operations for safe experimentation. Strata branches are copy-on-write — creating one is instant and free.

```
Parameters:
  action: string (required) — one of: "create", "switch", "list", "fork", "merge", "diff", "delete"

  # For create/fork/delete:
  name:     string (optional) — branch name

  # For switch:
  name:     string (required) — branch to switch to

  # For merge:
  source:   string (required) — branch to merge from

  # For diff:
  compare:  string (required) — branch to compare against current
```

Returns: Varies by action — branch info, list of branches, diff summary, or merge result.

**Why this matters**: Branches are Strata's killer feature for AI agents. An agent can fork the current state, try something risky, and either merge the result or discard it. No other embedded database offers this.

#### `strata_history`
**Intent**: "What changed over time?"

View version history for a key, or discover the time range available for time-travel queries.

```
Parameters:
  key:       string (optional) — get version history for this key
  namespace: string (optional) — namespace (defaults to "default")

  # If key is omitted, returns the branch's time range instead
```

Returns (with key): `[{ value, version, timestamp }]` — all historical values for the key.

Returns (without key): `{ oldest, latest, branch }` — the available time range for `as_of` queries.

#### `strata_status`
**Intent**: "What's the current state?"

Single introspection tool. Returns database info, current branch/space context, and health.

```
Parameters: none
```

Returns:
```json
{
  "version": "0.6.0",
  "branch": "default",
  "namespace": "default",
  "auto_embed": true,
  "branches": 3,
  "keys": 142,
  "events": 87
}
```

## What's Excluded (and Why)

| Current Tools | Why Excluded | Where It Lives |
|---|---|---|
| Transaction tools (5) | Agents don't need ACID transactions — branches provide safe experimentation | Python/Node SDK |
| Model configuration (1) | Infrastructure setup, not agent work | CLI flags, SDK |
| Retention policy (1) | Admin operation | SDK, cron job |
| Durability counters (1) | Monitoring/debugging | SDK, metrics |
| Bundle import/export (3) | Data migration | CLI, SDK |
| Embedding tools (3) | Auto-embed handles this transparently | SDK |
| Inference tools (4) | Text generation is outside Strata's scope for agents | SDK |
| Model management (3) | Download/manage models is admin work | CLI, SDK |
| DB admin (flush, compact) (2) | Infrastructure operations | SDK, auto-managed |
| Space management (5) | Renamed to "namespace", auto-managed, exposed as parameter | SDK for advanced use |
| Vector store (9) | Collapsed into store/search with auto-embed | SDK for direct access |
| JSON store (5) | Collapsed into store/recall | SDK for JSONPath access |
| State cells (7) | Collapsed into store/recall (CAS is SDK-only) | SDK |
| Batch operations (3) | Single tool handles multiple items via arrays | SDK for bulk operations |
| Time range query (1) | Folded into strata_history | — |
| KV history, JSON history, State history (3) | Unified into strata_history | — |

**Total excluded**: 56 tools → removed from MCP, remain accessible via Python/Node/Rust SDKs.

**Total collapsed**: 10 tools → functionality absorbed into the 8 new tools.

## Migration Strategy

### Phase 1: Build the Agent Layer (new module)
Add `src/tools/agent.rs` alongside existing tools. New `--agent-mode` flag (or make it the default) selects which tool set to expose.

### Phase 2: Dual Mode
- `strata-mcp --agent` (default): 8 agent-friendly tools
- `strata-mcp --developer`: 74 granular tools (existing behavior)

### Phase 3: Ship Agent Mode as Default
Make agent mode the default. Developer mode becomes `--developer` opt-in.

## Naming Comparison

| Operation | Current (primitive) | Proposed (intent-driven) |
|-----------|-------------------|------------------------|
| Store data | `strata_kv_put` | `strata_store` |
| Get data | `strata_kv_get` | `strata_recall` |
| Find data | `strata_search` | `strata_search` |
| Delete | `strata_kv_delete` | `strata_forget` |
| Log event | `strata_event_append` | `strata_log` |
| Create branch | `strata_branch_create` | `strata_branch` (action: "create") |
| View history | `strata_kv_history` | `strata_history` |
| Health check | `strata_db_ping` + `strata_db_info` | `strata_status` |

## Tool Annotation Hints (MCP Spec)

```json
{
  "strata_store":   { "readOnlyHint": false, "destructiveHint": false, "idempotentHint": true },
  "strata_recall":  { "readOnlyHint": true },
  "strata_search":  { "readOnlyHint": true },
  "strata_forget":  { "readOnlyHint": false, "destructiveHint": true, "idempotentHint": true },
  "strata_log":     { "readOnlyHint": false, "destructiveHint": false, "idempotentHint": false },
  "strata_branch":  { "readOnlyHint": false, "destructiveHint": false },
  "strata_history": { "readOnlyHint": true },
  "strata_status":  { "readOnlyHint": true }
}
```

## Prior Art

- **Qdrant** (2 tools): `store` + `find`. The gold standard for AI-friendliness. Our design follows this philosophy but adds branches and time-travel.
- **sylweriusz Neo4j** (4 tools): Collapsed 9 knowledge-graph tools to 4 using discriminator fields. Our `strata_branch` uses the same pattern.
- **WhenMoon claude-memory** (3 tools): `store`/`recall`/`forget` — cognitive metaphors. We adopt this naming.
- **Supabase** (38 tools): Three-tier safety model. Our `--read-only` flag and branch-based safety follow this thinking.
- **DuckDB** (4 tools): Output limiting (1024 rows / 50K chars). We should adopt result truncation for `strata_search` and `strata_history`.

## Open Questions

1. **Should `namespace` be a tool parameter or session state?** Current design uses it as a parameter (explicit > implicit). But session state (like current branch) reduces repetition.

2. **Should `strata_store` accept arrays for batch storage?** The Qdrant model doesn't distinguish single/batch. We could accept `value` as a single item OR `items` as an array, keeping one tool.

3. **Output truncation limits**: What's the right default? DuckDB uses 50K chars. We should pick a limit that fits in LLM context windows without overwhelming.

4. **Should `strata_log` events be searchable?** If yes, auto-embed event payloads too. This makes the event log a searchable activity stream.

5. **Human-friendly timestamps**: `as_of` currently takes microsecond timestamps. Should the agent layer accept "1 hour ago", "yesterday", ISO 8601? Internal coercion would be more AI-friendly (Arcade.dev recommends parameter coercion).
