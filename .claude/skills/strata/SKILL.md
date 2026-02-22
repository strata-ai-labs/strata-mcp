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

# Strata

Strata is a database built for AI agents. It provides persistent, structured state with zero configuration — no schemas, no migrations, no connection strings. You connect and immediately have a fully functional persistence layer through 8 intent-driven tools.

Traditional databases were designed for human developers writing SQL, managing schemas, and configuring infrastructure. Agents don't need that. Agents need to store structured state, find things by meaning, experiment safely, and never lose context. That's what Strata does.

## Tools

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

## What Makes This Different

**Everything is versioned.** Every write creates a new version. Nothing is ever truly lost. You can read any key as it was at any past point in time by passing `as_of` to `strata_recall`.

**Branching is built in.** Fork the entire database state before risky operations. If the experiment works, merge it back. If it doesn't, delete the branch. No data was harmed.

**Search by meaning, not just by key.** When you don't remember the exact key, describe what you're looking for in natural language. `strata_search` searches across all documents and events simultaneously using keyword matching and, when auto-embed is enabled, semantic similarity.

**Structured data with surgical updates.** Store any JSON value. Use JSONPath to read or update specific nested fields without overwriting the whole document.

**Immutable event log.** Record actions, decisions, observations, and errors as permanent, ordered, timestamped events that can never be modified or deleted.

## When to Use Each Tool

- **Persisting state across sessions** → `strata_store` — store structured JSON, get it back with `strata_recall`
- **Updating part of a document** → `strata_store` with `path` (e.g. `$.settings.theme`) to change one field
- **Finding data without knowing the key** → `strata_search` with natural language
- **Recording what happened** → `strata_log` for actions, decisions, errors — anything that should never be rewritten
- **Trying something risky** → `strata_branch` fork → experiment → merge if good, delete if bad
- **Understanding how state evolved** → `strata_history` for version history, `strata_recall` with `as_of` to read past state
- **Starting a session** → `strata_status` to see what branch you're on and what data exists

## Patterns

### Persist and recall structured state

```
strata_store(key="user:alice", value={"name": "Alice", "role": "admin", "prefs": {"theme": "dark"}})
strata_recall(key="user:alice")
strata_recall(key="user:alice", path="$.prefs.theme")
```

### Update a single field

```
strata_store(key="user:alice", path="$.prefs.theme", value="light")
```

### Find data by meaning

```
strata_search(query="admin users")
strata_search(query="recent errors in auth service", k=5)
```

### Experiment safely

```
strata_branch(action="fork", name="experiment")
strata_branch(action="switch", name="experiment")
# ... make changes, test things ...
strata_branch(action="diff", compare="default")
# Keep the results:
strata_branch(action="switch", name="default")
strata_branch(action="merge", source="experiment")
# Or discard:
strata_branch(action="switch", name="default")
strata_branch(action="delete", name="experiment")
```

### Record decisions and events

```
strata_log(event="decision", data={"choice": "PostgreSQL", "reason": "team familiarity"})
strata_log(event="error", data={"message": "connection timeout", "service": "auth"})
```

### Time-travel

```
strata_history(key="config")
strata_recall(key="config", as_of=1700000700000000)
```
