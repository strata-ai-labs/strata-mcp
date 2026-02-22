# Database MCP Server Survey

Surveyed 16 database MCP servers to understand how production databases design their AI agent interfaces.

## Tool Count Spectrum

| Database | Tools | Approach |
|----------|-------|----------|
| Qdrant | 2 | Minimalist memory layer |
| Weaviate (official) | 2 | Minimalist |
| DuckDB/MotherDuck | 4 | Focused analytics |
| Cloudflare D1 | 4 | Platform-embedded |
| Elasticsearch | 5 | Read-only search |
| SQLite (Anthropic) | 6 | Reference implementation |
| Turso | 9 | Granular CRUD |
| Pinecone | 9 | Inference-integrated |
| Weaviate (community) | 11 | Full search suite |
| ChromaDB | 12 | Collection + document CRUD |
| Milvus | 12 | Full vector DB operations |
| PlanetScale | 12 | Safety-first SQL |
| Upstash/Redis | 15 | Management + commands |
| Neon | 28 | Full lifecycle + branching |
| Supabase | 38 | Full platform |
| MongoDB | 40 | Largest surface area |

## Systems

### Qdrant — 2 tools (most minimalist)

| Tool | Description |
|------|-------------|
| `qdrant-store` | Store information with optional metadata |
| `qdrant-find` | Retrieve relevant information using semantic queries |

The purest "memory layer" design. Built-in embeddings (FastEmbed/MiniLM-L6-v2) — the AI never sees vectors. Auto-creates collections. Customizable tool descriptions via environment variables. An AI agent using this has no idea it's talking to a vector database.

### Weaviate — 2 tools (official) / 11 tools (community)

Official: `insert_one`, `query`. Community adds: `list_collections`, `get_schema`, `search` (hybrid default), `semantic_search`, `keyword_search`, `hybrid_search`, multi-tenancy tools.

Four search modalities in the community version (semantic, keyword, hybrid, and a simplified `search` that defaults to hybrid).

### SQLite (Anthropic Reference) — 6 tools

| Tool | Description |
|------|-------------|
| `read_query` | Execute SELECT queries |
| `write_query` | Execute INSERT, UPDATE, DELETE |
| `list_tables` | List all tables |
| `describe-table` | View schema for a table |
| `create_table` | Create new tables |
| `append_insight` | Add business insights to memo resource |

The canonical MCP database example. Clean read/write separation at the tool level. Uses MCP resources (the insight memo). Archived/superseded by community implementations.

### Elasticsearch — 5 tools (read-only)

| Tool | Description |
|------|-------------|
| `list_indices` | List all indices |
| `get_mappings` | Get field mappings for an index |
| `search` | Execute search with Query DSL (auto-enabled highlights) |
| `esql` | Perform an ES|QL query |
| `get_shards` | Get shard information |

Deliberately no write tools. Auto-highlighting for text fields helps LLMs identify relevant parts. Now deprecated in favor of built-in MCP in Elasticsearch 9.2+.

### DuckDB/MotherDuck — 4 tools

| Tool | Description |
|------|-------------|
| `execute_query` | Execute SQL query (DuckDB dialect) |
| `list_databases` | List all databases |
| `list_tables` | List tables and views |
| `switch_database_connection` | Switch to different database |

Read-only by default; `--read-write` flag enables writes. Output limited to 1024 rows / 50K chars by default — prevents context window overflow. Also has a DuckDB extension that turns any instance into an MCP resource provider.

### Turso (libSQL) — 9 tools

| Tool | Description |
|------|-------------|
| `open_database` | Open a database |
| `current_database` | Describe current database |
| `list_tables` | List all tables |
| `describe_table` | Get structure of a table |
| `execute_query` | Execute read-only SELECT queries |
| `insert_data` | Insert new data |
| `update_data` | Update existing data |
| `delete_data` | Delete data |
| `schema_change` | Execute CREATE/ALTER/DROP TABLE |

Most fine-grained read/write separation: one tool per operation type. Native `--mcp` flag — `tursodb --mcp` turns any Turso DB into an MCP server with zero setup.

### Pinecone — 9 tools

| Tool | Description |
|------|-------------|
| `search-docs` | Search Pinecone documentation |
| `list-indexes` | List all indexes |
| `describe-index` | Describe index configuration |
| `describe-index-stats` | Statistics about index data |
| `create-index-for-model` | Create index with integrated inference |
| `upsert-records` | Insert/update records with integrated inference |
| `search-records` | Search via text query with inference + filtering + reranking |
| `cascading-search` | Search across multiple indexes with dedup and reranking |
| `rerank-documents` | Rerank records/documents using specialized model |

Inference-integrated — embedding generation is built into index creation and search. `cascading-search` across multiple indexes with automatic reranking is purpose-built for agentic RAG.

### ChromaDB — 12 tools

Collection management (7): `list_collections`, `create_collection`, `peek_collection`, `get_collection_info`, `get_collection_count`, `modify_collection`, `delete_collection`. Document operations (5): `add_documents`, `query_documents`, `get_documents`, `update_documents`, `delete_documents`.

Full CRUD on both collections and documents. Configurable embedding providers (Cohere, OpenAI, Jina, Voyage AI, Roboflow). `peek_collection` samples documents without a query — helps LLMs understand what data a collection contains.

### Upstash/Redis — 15 tools

13 management/ops tools + 2 data tools (`run_single_redis_command`, `run_multiple_redis_commands`). Heavily infrastructure-focused. The data tools accept raw Redis commands (GET, SET, DEL, etc.) without distinction.

### Neon (Postgres) — 28 tools

Across 8 categories: project management (6), branch management (6), SQL execution (5), migrations (2), performance (4), auth/APIs (2), discovery (2), documentation (2).

Exposes raw SQL via `run_sql` and `run_sql_transaction`. Branch-based safety: `prepare_database_migration` creates a temporary branch to test changes before `complete_database_migration` applies them. Neon published research showing they improved tool selection accuracy from 60% to 100% through iterative prompt engineering of tool descriptions alone.

### Supabase — 38 tools

Across 8 feature groups: account (9), knowledge base (1), database (5), debugging (2), development (3), edge functions (3), branching (6), storage (3).

Three-tier safety model using PostgreSQL's parser (`pglast`): safe (always allowed), write (requires write mode), destructive (requires write mode + 2-step confirmation). Starts in read-only mode by default, auto-reverts after writes. Automatically generates migration scripts for all write/destructive SQL.

### MongoDB — 40 tools (largest)

Atlas Cloud (~13), Atlas Local (4), Database Core (~24: `find`, `aggregate`, `count`, `insert-many`, `update-many`, `delete-many`, `list-databases`, `list-collections`, `create-collection`, etc.).

**No raw query strings** — everything is structured tools. Read-only by default via `--readOnly`. Auto-generates embeddings on insert for fields with vector search indexes (Winter 2026). The most strongly opinionated anti-raw-query design.

### PlanetScale — 12 tools

Strongest SQL write safety model: separate `execute_read_query`/`execute_write_query` tools, read queries auto-routed to replicas, ephemeral credentials per query, blocks UPDATE/DELETE without WHERE, blocks TRUNCATE, requires human confirmation for DDL. SQL comments injected for query tracking. CLI-first setup: `pscale mcp install --target <claude|cursor|zed>`.

### Cloudflare D1 — 4 tools

`d1_list_databases`, `d1_create_database`, `d1_delete_database`, `d1_query`. Part of broader Cloudflare Workers Bindings MCP server (not standalone). OAuth authentication.

## Cross-Cutting Analysis

### Raw SQL vs Structured Operations

**Raw SQL**: Neon, Supabase, SQLite, DuckDB, Cloudflare D1, PlanetScale, Elasticsearch (Query DSL/ES|QL), Turso

**Structured only (no raw queries)**: MongoDB, Qdrant, Weaviate, Pinecone, ChromaDB, Milvus

**Hybrid**: Upstash/Redis (management API + raw commands)

SQL databases tend to expose raw SQL. Document/vector databases wrap everything in typed operations. MongoDB is the notable exception — despite supporting MQL, it chose structured tools over raw queries.

### Read/Write Safety Patterns (ranked by sophistication)

1. **PlanetScale**: Separate read/write tools + replica routing + ephemeral credentials + destructive query blocking + DDL human confirmation
2. **Supabase**: Three-tier (safe/write/destructive) with SQL parser + auto-revert to read-only + auto migration generation
3. **Neon**: Branch-based safety — migrations and tuning on temporary branches before applying
4. **MongoDB**: Read-only by default via `--readOnly` flag
5. **DuckDB**: Read-only by default via `--read-write` flag
6. **SQLite**: Separate `read_query` and `write_query` tools
7. **Turso**: One tool per operation type (5 distinct tools)
8. **Elasticsearch**: No write tools at all

### Vector DBs vs Traditional DBs

**Traditional databases** (10-40 tools): schema inspection, query execution, infrastructure management. The AI must understand SQL or the database's query language.

**Vector databases** (2-12 tools): collection/index management, document CRUD, multiple search modalities, embedding configuration. No query language required.

**Memory layer pattern** (2 tools): only store and find. Abstracts away all database concepts. Built-in embedding generation. The AI never handles vectors. Qdrant is the canonical example.

### Design Patterns

**Embedded documentation**: Neon, Pinecone, PlanetScale, Supabase include doc search within the MCP server, helping the LLM self-correct.

**Two-step confirmations**: Supabase (destructive ops), Neon (migrations), PlanetScale (DDL) use prepare/confirm workflows.

**Output limiting**: DuckDB limits results to 1024 rows / 50K chars to prevent context window overflow. Most others ignore this.

**Feature group toggling**: Supabase lets operators enable/disable entire feature groups to control what the AI can access.

**Customizable tool descriptions**: Qdrant allows env var overrides for tool descriptions, letting operators tune LLM interpretation.

**Workflow-based tools**: Neon's `prepare_database_migration`/`complete_database_migration` encapsulate multi-step processes rather than exposing atomic operations.

## Key Takeaways for Strata

1. **The Qdrant pattern is the gold standard for AI-friendliness**: 2 tools, automatic embeddings, zero database concepts exposed. This is where Strata's MCP server should aim.

2. **Strata has unique capabilities none of these have**: branches with time-travel, six data primitives, hybrid search, built-in inference. The challenge is exposing that power without exposing the complexity.

3. **Read/write safety matters**: At minimum, read-only by default with explicit opt-in. Branch-based safety (like Neon) maps naturally to Strata's branching model.

4. **Embedded search is the universal pattern**: Every database MCP server includes search. For Strata, hybrid search (keyword + semantic) is the differentiator.

5. **Output limiting is practical**: Truncating large results prevents context window issues. Strata should consider this.

6. **MongoDB's anti-raw-query approach is relevant**: Strata isn't SQL — it shouldn't expose a query language. Structured tools make more sense.
