# Strata Agent On-Ramp (`npx … init`) — V1 Requirements

**Status:** Draft for review
**Date:** 2026-08-05
**Repo:** `stratalab/strata-mcp` (rebuilt from scratch; pre-V1 server retired 2026-08-05)
**Reference pattern:** Neon's `npx neon@latest init`
(https://neon.com/docs/get-started/with-an-agent)

> **Naming is provisional throughout.** A product-wide rename is under
> consideration, and the npm names are contested (`strata` requires a dispute;
> `strata-db` is the available fallback). This document uses `strata` as a
> placeholder in package names, bin names, and config keys; every occurrence is a
> find-and-replace site, and nothing in the requirements depends on the final name.

---

## 1. Context

Installing a database is not the finish line anymore — the finish line is the
user's AI agent being able to *use* it. This package is that last step, compressed
into one command a human (or an agent mid-task) runs from a project root:

```bash
npx strata init
```

After it runs, every AI agent surface in the project — VS Code agent mode, Cursor,
Claude Code, Claude Desktop — can reach the project's Strata database through MCP,
and the user is told the handoff line: *tell your agent "get started with Strata."*

Three commitments anchor everything below:

1. **This package is not an MCP server.** The Strata MCP server is
   `strata mcp serve`, inside the CLI binary — one canonical path. It runs locally
   over stdio, opens the database as an IPC host (other readers, including the
   VS Code extension, observe the agent's database live), and needs no hosted
   endpoint, no OAuth, and no network. This package is distribution and
   onboarding for that server; it implements zero MCP tools.
2. **MCP access is on by default, workspace-scoped, never a per-database chore**
   (founder decision, 2026-08-05). One registration per workspace; database
   selection is the server's job at runtime.
3. **npx is the front door because it predates the product.** The user may have
   nothing installed and may never have heard the product name; `npx` is the one
   runner both humans and agents already have. `init` therefore must work from a
   bare machine — including installing the CLI itself.

## 2. Product scope

### In scope (V1)

| # | Feature | One-liner |
|---|---------|-----------|
| F1 | `init` | Verify/install the CLI, register the MCP server with every detected agent surface, print the handoff line |
| F2 | `init --remove` | Cleanly unregister everything `init` wrote |
| F3 | `init --check` | Report detected agent surfaces and current registration state, changing nothing |
| F4 | `mcp` passthrough | `npx strata mcp` resolves the installed CLI and execs `strata mcp serve` |
| F5 | Skills install | Install/refresh agent skills as part of `init` — **blocked** on the agent-skills repo decision; ships behind a flag until then |

### Out of scope (V1)

- **Implementing MCP tools** — permanently out; the CLI's server is canonical.
- **Hosted/remote MCP, OAuth, API keys** — there is no cloud in this path; the
  database is local files. (This is a positioning fact, not a limitation.)
- **Database creation** — `init` wires agents to Strata; the agent (or the user)
  creates databases through the tools it just gained (open question Q3).
- **Windows** — follows the upstream transport (strata-core G10); `init` on
  Windows exits with a clear not-yet message rather than half-registering.
- **Telemetry** — none, matching every other Strata surface.

## 3. System context

### 3.1 What registration means per agent surface

| Surface | Mechanism | Scope |
|---|---|---|
| VS Code (Copilot agent mode) | `.vscode/mcp.json` | Workspace |
| Cursor | `.cursor/mcp.json` | Workspace |
| Claude Code | `.mcp.json` | Workspace |
| Claude Desktop | `claude_desktop_config.json` | **User-global** (Q2) |

The entry written is the standard stdio shape, identical across surfaces except
for each file's envelope:

```json
{ "command": "<resolved strata binary>", "args": ["mcp", "serve", "--db", "<db>"] }
```

### 3.2 Two writers, one format

The VS Code extension (`stratalab/strata-vscode`, requirements §F6) registers the
same entries from inside the editor. The two implementations MUST write
byte-identical entries, verified by a **shared fixture set** both repos test
against (the fixtures live here; the extension vendors them). Either writer must
recognize, update, and remove the other's entries as its own.

### 3.3 Relationship to the canonical server

`strata mcp serve` opens the database with `IpcMode::Host`: while an agent works,
its session is visible in `ipc_status.clients`, and its writes stream to any
attached observer via version ticks. Registration is wiring; the live-observation
payoff belongs to the extension and CLI.

## 4. Architecture requirements

- **AR-1 — Exec, never embed.** The package contains no engine, no wire client,
  and no MCP implementation. Everything database-shaped happens by exec'ing the
  CLI. The package stays small enough to be an incidental `npx` download.
- **AR-2 — Registration discipline.** Idempotent (re-runs converge, never
  duplicate); reversible (F2 removes exactly what was written, preserving
  everything else in each file); explicit (every write is printed as a diff
  before/as it happens); JSON-preserving (unknown keys and formatting of existing
  config files survive edits).
- **AR-3 — CLI discovery and install.** Resolve the `strata` binary: explicit
  `--binary` flag → `PATH` → known install locations. If absent, offer install
  via Homebrew tap or the curl installer (tier-1 distribution channels) with the
  chosen command printed before running; `--yes` consents. Never a bundled or
  vendored engine. After resolve/install, verify with `strata --version`.
- **AR-4 — Workspace scoping.** One entry per workspace, named `strata`. Database
  path resolution for the transitional shape (until the server grows workspace
  discovery): a single database in the workspace → pin it; multiple → prompt
  (`--db` to preselect); none → register with the workspace root as the intended
  home and tell the user their agent can create one (Q3).
- **AR-5 — Agent-operable.** `--yes` answers every prompt with the safe default;
  exit codes are meaningful; output is line-oriented and parseable; `--json`
  emits a machine-readable result. An agent running `npx strata init --yes`
  mid-task must succeed or fail atomically and legibly.
- **AR-6 — Rename tolerance.** The package name, bin name, and entry name
  (`strata`) are constants in exactly one module; the naming decision lands as a
  one-module change plus republish.
- **AR-7 — Toolchain.** TypeScript, Node 18+, zero runtime dependencies (dev
  dependencies unrestricted); single-purpose bin; no postinstall scripts.

## 5. Functional requirements

- **F1.1** `init` runs: CLI resolve/install (AR-3) → surface detection (§3.1) →
  per-surface registration (AR-2, AR-4) → skills hook (F5, when unblocked) →
  summary + handoff line. Interactive by default; `--yes` for unattended.
- **F1.2** Detection is presence-based: a surface is offered when its config file
  or directory convention exists in the workspace (or, for Claude Desktop, on the
  machine). `--all` registers every supported surface unconditionally.
- **F1.3** Partial failure is loud: each surface registers independently; the
  summary names every success and every failure with its reason. Exit code is
  non-zero if any requested surface failed.
- **F2.1** `--remove` deletes exactly the entries this tool (or the extension —
  §3.2) wrote, across all surfaces, and reports what it removed.
- **F3.1** `--check` prints the detection/registration matrix and the resolved
  CLI version, changing nothing. This is also the support/debug artifact.
- **F4.1** `mcp` resolves the CLI (AR-3, no install offer — fail fast with the
  install hint) and execs `strata mcp serve` with all remaining args passed
  through verbatim. Used by configs that prefer an npx command to an absolute
  path; the default written entry uses the absolute path (faster, offline), with
  `npx strata mcp` documented as the alternative for unstable-PATH environments.
- **F5.1** Skills install (blocked): once the skills repo exists, `init` installs
  or refreshes the Strata skills for detected Claude Code / compatible surfaces;
  `--no-skills` opts out.

## 6. Non-functional requirements

- **N1 — Footprint.** Zero runtime deps; package small enough that `npx` cold
  start is dominated by the registry fetch, not the payload.
- **N2 — Security.** Writes only the four known config files, shown before
  writing; entries reference the resolved absolute binary path, never a
  workspace-relative one; network access happens only in the explicit install
  step (AR-3) and the registry fetch npx itself performs; no postinstall.
- **N3 — Error discipline.** CLI-originated failures surface the registry code
  verbatim (`class.area.detail`); package-originated failures use a small, stable
  set of its own codes. Tests assert on codes, not prose.
- **N4 — Testing.** The shared fixture set (§3.2) is the contract test. CI on
  macOS + Linux: fixture round-trips over pre-seeded config files (fresh, existing
  with foreign entries, malformed), idempotence (`init` twice = once), remove
  restores byte-identical originals, `--check` accuracy, and an end-to-end lane
  with a real CLI binary: `init --yes` then a real MCP handshake through the
  registered entry.
- **N5 — Release.** npm with provenance; semver independent of the CLI; a
  compatibility floor for the CLI enforced at runtime via `strata --version`
  (mirroring the extension's Q3 policy once decided).

## 7. Upstream dependencies

- **Workspace-discovery mode for `strata mcp serve`** (serve without a pinned
  `--db`; list/select databases at runtime) — removes AR-4's transitional shape.
  File in strata-core when F1 lands.
- **Agent-skills repo** — blocks F5; founding decision pending.
- **The curl installer** (tier-1 distribution) — AR-3's non-Homebrew install path.
- **The naming decision** — gates the npm publish name and dispute strategy
  (owner: founder).

## 8. Open questions

- **Q1** Should the VS Code extension delegate its F6 file-based registrations to
  this package (`npx strata init --yes --surface cursor,claude-code`) instead of
  reimplementing the writes? (Draft assumes shared fixtures, separate
  implementations; delegation would collapse them to one.)
- **Q2** Claude Desktop is user-global, not per-workspace — register by default,
  or only with `--all`/explicit consent, since it affects every conversation on
  the machine? (Draft assumes explicit only.)
- **Q3** When no database exists, should `init` create one (Neon's init creates a
  project), or is that the agent's first act through its new tools? (Draft: the
  agent's — creating state belongs to the surface that will own it.)
- **Q4** Should `init` also register the MCP server with agent surfaces' *global*
  configs (`~/.cursor/mcp.json` etc.) behind a flag, for users who want Strata
  everywhere?
