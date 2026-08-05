# strata-mcp — the agent on-ramp for Strata

> **Status: charter.** This repo was wiped on 2026-08-05 (founder decision) and is
> being rebuilt from scratch as Strata's npm-distributed agent-integration package.
> The pre-V1 standalone MCP server that lived here is retired; its history remains
> in git.

## What this package is

One command that makes Strata usable by every AI agent in a project:

```bash
npx strata init
```

*(Command name pending an npm name dispute for the abandoned 2013 `strata` package —
see "Naming" below. Fallback: `npx strata-db init`.)*

`init`:

1. **Verifies or installs the `strata` CLI** (Homebrew tap or the curl installer;
   never a bundled duplicate of the engine).
2. **Registers the Strata MCP server with every agent surface it finds** —
   `.vscode/mcp.json`, `.cursor/mcp.json`, `.mcp.json` (Claude Code),
   `claude_desktop_config.json` — workspace-scoped, idempotent, reversible
   (`npx strata init --remove`), with `--yes` for agent-driven runs.
3. **Installs agent skills** (SKILL.md format) once the skills repo exists —
   generated from strata-core's IDL docs pipeline, never hand-written drift.
4. **Prints the handoff line**: *tell your agent "get started with Strata".*

Also provided:

```bash
npx strata mcp   # resolves the installed strata binary and execs `strata mcp serve`
```

so agent configs can reference this package without knowing install paths.

## What this package is NOT

**It is not an MCP server.** The Strata MCP server is `strata mcp serve`, inside the
`strata` CLI binary — one canonical path. It runs locally over stdio, opens the
database as an IPC host (so the VS Code extension and other readers observe the same
database the agent is using, live), and needs no hosted endpoint, no OAuth, and no
network: the database is local files, air-gapped by construction. This package is
distribution and onboarding for that server, never a second implementation.

## Design constraints (carry into the build)

- **MCP access is on by default.** Registration is workspace-scoped — one `strata`
  entry per workspace, never a per-database chore. Database selection is the MCP
  server's job at runtime; `strata mcp serve` grows workspace discovery upstream to
  support this (until then, entries pin the workspace's primary database).
- **Byte-identical entries** with the VS Code extension's F6 registration
  (`stratalab/strata-vscode`, requirements §F6) — two writers, one format.
- TypeScript, zero runtime dependencies where feasible, Node 18+.
- The pre-V1 server's "intent tools" idea (store / recall / search / forget — no
  database concepts exposed) is worth preserving **as tool-naming input to
  `strata mcp serve`'s curated toolset**, not as a separate server.

## Naming

npm reality (checked 2026-08-05):

| Name | Status |
|---|---|
| `strata` | Taken by an HTTP framework abandoned since 2013 — **file an npm name dispute** |
| `stratadb` | Taken by an unrelated active project (2026) — not available |
| `strata-mcp` | Taken by an unrelated project (2026) — not available |
| `strata-db` | **Available — secure immediately** as the fallback |
| `@stratadb/*` | Org scope in use by strata-nodesdk (`@stratadb/core`) |

Reference pattern: Neon's `npx neon@latest init`
(https://neon.com/docs/get-started/with-an-agent).
