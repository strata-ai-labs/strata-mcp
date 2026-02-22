---
name: strata
description: >
  Use when the user wants to persist, query, search, or manage structured data.
  Triggers: any mention of "store", "save", "remember", "look up", "search for",
  "find", "database", "persist", "branch", "history", "time travel", "strata",
  or when the user needs data that survives across sessions. Also use when the
  user asks to compare states, experiment safely, or undo changes. Do NOT use for
  ephemeral computation, file I/O, or git operations.
argument-hint: [action or question]
---

# Strata Database

Strata is a multi-primitive embedded database with built-in versioning, branching, time-travel, cross-primitive search, and local inference. One process, one data directory, zero configuration. It replaces the fragmented stack of Redis + SQLite + vector DBs with a single unified interface.

## Tools at a Glance

| Intent | Tool | What it does |
|--------|------|--------------|
| Save data | `strata_store` | Store a JSON document by key, with optional JSONPath for nested updates |
| Read data | `strata_recall` | Retrieve a document by key, with optional time-travel via `as_of` |
| Find data | `strata_search` | Natural language search across all documents and events |
| Delete data | `strata_forget` | Delete a document by key (history is preserved) |
| Record an event | `strata_log` | Append an immutable, timestamped event to the log |
| Safe experimentation | `strata_branch` | Fork, merge, diff, switch branches (like git for data) |
| See what changed | `strata_history` | View all versions of a key, or discover the time range |
| Orient yourself | `strata_status` | Get current branch, key count, auto-embed state |

## Key Capabilities

- **Versioning**: Every write is versioned. Nothing is ever truly lost.
- **Time-travel**: Read any key as it was at any past timestamp via the `as_of` parameter on `strata_recall`.
- **Branching**: Fork the entire database state, experiment freely, merge back or discard. Branches are instant copy-on-write snapshots.
- **Cross-data search**: One `strata_search` query searches across all documents and events simultaneously.
- **JSONPath**: Surgical updates to nested fields without overwriting entire documents. Use `path` on `strata_store` and `strata_recall`.
- **Auto-embed**: When enabled, all text content is automatically indexed for semantic search — no manual embedding required.

## When to Reach for Strata

- **Persisting structured data across sessions** → `strata_store` — versioned, searchable, branchable
- **Storing nested/hierarchical documents** → `strata_store` with `path` for targeted reads/writes
- **Finding data without knowing the exact key** → `strata_search` with natural language
- **Recording actions, decisions, or audit trails** → `strata_log` — immutable, ordered, typed events
- **Experimenting safely with data** → `strata_branch` fork/merge workflow
- **Understanding how data evolved** → `strata_history` + time-travel via `as_of`
- **Orienting at session start** → `strata_status` to see current branch and data state

## Common Patterns

### Store and recall structured data

```
strata_store(key="user:alice", value={"name": "Alice", "role": "admin", "settings": {"theme": "dark"}})
strata_recall(key="user:alice")
strata_recall(key="user:alice", path="$.settings.theme")
```

### Update a nested field without overwriting

```
strata_store(key="user:alice", path="$.settings.theme", value="light")
```

### Search with natural language

```
strata_search(query="admin users with dark theme")
strata_search(query="error logs from today", k=5)
```

### Safe experimentation with branches

```
strata_branch(action="fork", name="experiment")
strata_branch(action="switch", name="experiment")
# ... make changes ...
strata_branch(action="diff", compare="default")
# If good:
strata_branch(action="switch", name="default")
strata_branch(action="merge", source="experiment")
# If bad:
strata_branch(action="switch", name="default")
strata_branch(action="delete", name="experiment")
```

### Track decisions with the event log

```
strata_log(event="decision", data={"choice": "use PostgreSQL", "reason": "team familiarity", "alternatives": ["MySQL", "SQLite"]})
strata_log(event="error", data={"message": "connection timeout", "service": "auth"})
```

### Time-travel to see past state

```
strata_history(key="config")
# Returns all versions with timestamps
strata_recall(key="config", as_of=1700000700000000)
# Returns the value as it was at that timestamp
```
