# Agent Memory MCP Server Survey

Surveyed 10+ agent memory MCP servers to understand design patterns for AI-friendly tool surfaces.

## Tool Count Distribution

| Range | Systems | Examples |
|-------|---------|---------|
| **3 tools** | 3 | pinkpixel mem0, coleam00 mem0, WhenMoon claude-memory |
| **4 tools** | 2 | Mem0 OpenMemory, sylweriusz neo4j |
| **5 tools** | 1 | Letta/MemGPT |
| **9 tools** | 4 | Anthropic official, Mem0 official, Neo4j official, Graphiti/Zep |
| **12-16+ tools** | 1 | doobidoo mcp-memory-service |

The ecosystem clusters around two modes: minimalist (3-5 tools) or comprehensive (9 tools). The 9-tool servers are typically knowledge-graph-based with full CRUD on entities, relations, and observations. Flat memory stores get away with 3-5 tools.

## Systems

### Anthropic Official Knowledge Graph Memory Server
**Source:** [modelcontextprotocol/servers](https://github.com/modelcontextprotocol/servers/tree/main/src/memory) — **9 tools**

| Tool | Naming Style |
|------|-------------|
| `create_entities` | Primitive (CRUD) |
| `create_relations` | Primitive (CRUD) |
| `add_observations` | Primitive (CRUD) |
| `delete_entities` | Primitive (CRUD) |
| `delete_observations` | Primitive (CRUD) |
| `delete_relations` | Primitive (CRUD) |
| `read_graph` | Primitive (CRUD) |
| `search_nodes` | Primitive |
| `open_nodes` | Primitive |

Fine-grained CRUD on a knowledge graph. Purely primitive naming. The agent must understand the graph model.

### Mem0 Official MCP Server
**Source:** [mem0ai/mem0-mcp](https://github.com/mem0ai/mem0-mcp) — **9 tools**

| Tool | Description |
|------|-------------|
| `add_memory` | Save text or conversation history for a user/agent |
| `search_memories` | Semantic search across existing memories |
| `get_memories` | List memories with structured filters and pagination |
| `get_memory` | Retrieve one memory by its memory_id |
| `update_memory` | Overwrite a memory's text by memory_id |
| `delete_memory` | Delete a single memory by memory_id |
| `delete_all_memories` | Bulk delete all memories in scope |
| `delete_entities` | Delete a user/agent/app/run entity and its memories |
| `list_entities` | Enumerate users/agents/apps/runs stored in Mem0 |

Flat memory store with CRUD. Slightly intent-leaning naming. Multi-tenant scoping. Semantic search is first-class.

### Mem0 OpenMemory MCP
**Source:** [mem0.ai/openmemory](https://mem0.ai/openmemory) — **4 tools**

| Tool | Description |
|------|-------------|
| `add_memories` | Store new memory objects |
| `search_memory` | Retrieve relevant memories based on queries |
| `list_memories` | View all stored memory entries |
| `delete_all_memories` | Clear the entire memory store |

Radically minimal. The most opinionated "less is more" approach. Semantic search is the primary retrieval mechanism.

### pinkpixel mem0-mcp (Community)
**Source:** [pinkpixel-dev/mem0-mcp](https://github.com/pinkpixel-dev/mem0-mcp) — **3 tools**

| Tool | Description |
|------|-------------|
| `add_memory` | Store text content as a memory |
| `search_memory` | Search stored memories via natural language |
| `delete_memory` | Delete a specific memory by ID |

Absolute minimum viable surface. Store, search, delete.

### coleam00 mcp-mem0 (Community)
**Source:** [coleam00/mcp-mem0](https://github.com/coleam00/mcp-mem0) — **3 tools**

| Tool | Description |
|------|-------------|
| `save_memory` | Store information in long-term memory with semantic indexing |
| `get_all_memories` | Retrieve all stored memories for comprehensive context |
| `search_memories` | Find relevant memories using semantic search |

Notable: uses `save_memory` (intent-leaning). No delete tool at all.

### WhenMoon claude-memory-mcp
**Source:** [WhenMoon-afk/claude-memory-mcp](https://github.com/WhenMoon-afk/claude-memory-mcp) — **3 tools**

| Tool | Description |
|------|-------------|
| `memory_store` | Store a memory with auto-summarization and entity extraction |
| `memory_recall` | Search memories with token-aware loading |
| `memory_forget` | Soft-delete a memory (preserves audit trail) |

The most intent-driven naming in the ecosystem. `store`/`recall`/`forget` are human-cognitive metaphors. Soft-delete is a notable safety choice.

### Neo4j Official Memory Server
**Source:** [neo4j-contrib/mcp-neo4j](https://github.com/neo4j-contrib/mcp-neo4j) — **9 tools**

Nearly identical to Anthropic official (same knowledge graph model). `create_entities`, `create_relations`, `add_observations`, `delete_entities`, `delete_relations`, `delete_observations`, `read_graph`, `search_nodes`, `find_nodes`. Purely primitive CRUD naming.

### sylweriusz mcp-neo4j-memory-server (Consolidated)
**Source:** [sylweriusz/mcp-neo4j-memory-server](https://github.com/sylweriusz/mcp-neo4j-memory-server) — **4 tools**

| Tool | Description |
|------|-------------|
| `memory_store` | Create memories with observations and immediate relations in ONE operation |
| `memory_find` | Unified search/retrieval (semantic search, ID lookup, date filtering, graph traversal) |
| `memory_modify` | Comprehensive modification (update, delete, observations, relations) |
| `database_switch` | Switch database context for isolated environments |

Collapsed the 9-tool Anthropic pattern to 4 tools using discriminator fields internally. Uses `memory_` prefix namespace. Preserves graph power while reducing tool surface.

### Graphiti/Zep MCP Server
**Source:** [getzep/graphiti](https://github.com/getzep/graphiti) — **9 tools**

| Tool | Description |
|------|-------------|
| `add_episode` | Add an episode to the knowledge graph (text, JSON, message formats) |
| `search_nodes` | Search for relevant node summaries |
| `search_facts` | Search for relevant facts (edges between entities) |
| `get_episodes` | Get the most recent episodes for a specific group |
| `get_entity_edge` | Get an entity edge by its UUID |
| `delete_entity_edge` | Delete an entity edge |
| `delete_episode` | Delete an episode |
| `clear_graph` | Clear all data and rebuild indices |
| `get_status` | Get status of the server and Neo4j connection |

Temporal knowledge graph. Uses "episodes" as the write primitive. The only system that models time as first-class. Split search into nodes (entity summaries) and facts (relationships).

### Letta/MemGPT Memory MCP
**Source:** [Smithery](https://smithery.ai/server/letta-ai/memory-mcp) — **3-5 tools**

| Tool | Description |
|------|-------------|
| `get_user_memory` | Get a summary of memories about the current user |
| `add_episodic` | Add episodic memories into vector store for later search |
| `search_vector_store` | Search vector store for relevant episodic memories |

Two-tier memory model from the MemGPT research paper: "core memory" (user profile, always in context) and "archival/episodic memory" (vector DB, retrieved on demand).

### doobidoo mcp-memory-service
**Source:** [doobidoo/mcp-memory-service](https://github.com/doobidoo/mcp-memory-service) — **16+ tools**

Includes `store_memory`, `retrieve_memory`, `recall_memory`, `search_by_tag`, `exact_match_retrieve`, `debug_retrieve`, `delete_memory`, `delete_by_tag`, `cleanup_duplicates`, `create_backup`, `optimize_db`, `get_stats`, `check_database_health`, `check_embedding_model`, `ingest_document`, `ingest_directory`.

The most bloated. Went through a "64% tool reduction" effort, confirming that mixing admin/operational tools with agent-facing tools is a mistake.

## Key Findings

### 1. The 3-5 tool sweet spot
The most successful community-adopted memory servers expose 3-5 tools. The minimal viable surface is: **store**, **search**, and optionally **delete**.

### 2. Semantic search is universal
Every single memory MCP server includes semantic/vector search as the primary retrieval mechanism.

### 3. Intent-driven naming is emerging
Older/official servers use primitive CRUD naming (`create_entities`). Newer community servers trend toward intent-driven (`memory_store`, `memory_recall`, `save_memory`).

### 4. Tool count directly impacts accuracy
- 10 tools = 100% accuracy
- 20 tools = 95% accuracy
- 30+ tools = degradation starts
- 100+ tools = collapse
- Cursor hard-limits at 40 MCP tools total across all servers

(Source: [Speakeasy research](https://www.speakeasy.com/mcp/tool-design/less-is-more))

### 5. Admin tools must be separated
The doobidoo project learned this the hard way. Tools like `create_backup`, `optimize_db`, and `check_database_health` should be in a separate admin server.

### 6. The consolidation/discriminator pattern works
sylweriusz collapsed 9 knowledge graph tools to 4 by using discriminator fields internally. Preserves power while reducing surface.

### 7. Two-tier memory (core + archival) is research-backed
Letta's MemGPT separates "core memory" (always in context) from "archival memory" (vector-searchable). Maps to how human memory works.

### 8. A self-documentation tool is a novel pattern
knowall-ai includes `get_guidance` which helps the agent understand how to use the memory tools. Interesting for complex tool surfaces.

## Best Practices

### From Speakeasy
- 30 tools is the critical threshold where tool descriptions begin to overlap
- Performance collapses above 100 tools
- Cursor enforces 40 MCP tools total across all servers

### From Philipp Schmid
- 5-15 tools per server. One server, one job.
- **Outcomes over operations**: design around agent goals, not REST endpoints
- Flatten arguments: use top-level primitives, not nested dicts
- Service-prefixed naming: `{service}_{action}_{resource}`

### From Arcade.dev
- Design for the LLM, not the human
- Parameter coercion: accept flexible inputs ("yesterday", "2024-01-15") and normalize internally
- Error-guided recovery: return actionable error messages
- Start atomic, watch how agents use tools, then consolidate

### From MCP Spec
- Tool annotations: `readOnlyHint`, `destructiveHint`, `idempotentHint`, `openWorldHint`
