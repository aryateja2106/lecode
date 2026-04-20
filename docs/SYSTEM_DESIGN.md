# LeSearch — System Design

**Version**: 0.1 (draft)
**Date**: 2026-04-14
**Status**: Pre-implementation, for multi-agent review

Reads with: `PRODUCT_SPEC.md` (vision), `PRD.md` (requirements), `EPICS_AND_STORIES.md` (backlog).

---

## 1. Architecture Overview

LeSearch is a six-layer system centered on a Rust daemon. Every layer has a single well-defined responsibility and a narrow interface.

```
┌─ L6 Clients ────────────────────────────────────────────────────────┐
│  Rust CLI  │  Tauri desktop  │  Web console  │  iOS / macOS native  │
└──────────────────────┬─────────────────────────────────────────────┘
                       │ WebSocket  JSON-RPC 2.0  + binary mux
                       │ (X25519 + ChaCha20-Poly1305 E2E)
┌──────────────────────▼────────────────────────────────────────────┐
│ L5 Transport                                                       │
│  direct (loopback)   │   ziti (default remote)   │   noise-ws       │
└──────────────────────┬────────────────────────────────────────────┘
                       │
┌──────────────────────▼────────────────────────────────────────────┐
│ L4 LeSearch Daemon  (Rust, tokio + axum, single binary)           │
│                                                                    │
│   ┌ Protocol router ──┐  ┌ Agent manager ──┐  ┌ MCP server ───┐   │
│   │ JSON-RPC dispatch │  │ state machine    │  │ daemon tools  │   │
│   └───────────────────┘  └──────────────────┘  └───────────────┘   │
│   ┌ Keyring ──────────┐  ┌ Session writer ──┐  ┌ FTS indexer ─┐   │
│   │ Ed25519 + X25519  │  │ CloudEvents JSONL│  │ SQLite FTS5   │   │
│   └───────────────────┘  └──────────────────┘  └───────────────┘   │
└──┬───────────┬────────────────┬──────────────────┬────────────────┘
   │           │                │                  │
   │           │                │                  │
 UDS         UDS            stdio                HTTP (localhost)
 MsgPack   MsgPack         MCP / ACP             to providers
   │           │                │                  │
┌──▼────┐  ┌──▼──────┐   ┌──────▼────────┐   ┌─────▼─────────┐
│L3 AVM │  │L2 Storage│   │L1 Providers  │   │L2' A2A Gateway│
│policy │  │AgentFS + │   │Claude / Codex│   │agent-card.json│
│engine │  │OpenFused │   │OpenCode /    │   │message/send   │
│       │  │keyring   │   │Gemini / A2A  │   │tasks/*        │
└───────┘  └──────────┘   └───────────────┘   └───────────────┘
```

## 2. Components

### 2.1 L4 — Daemon (Rust)

**Responsibility**: own agent state, route protocol messages, enforce policy via AVM, persist sessions, expose A2A.

**Crate layout** (Cargo workspace):

| Crate | Role |
|---|---|
| `lesearch-daemon` | Tokio runtime, axum server, top-level wiring |
| `lesearch-protocol` | JSON-RPC 2.0 types, message definitions, binary mux framing |
| `lesearch-agent-manager` | Agent lifecycle state machine, subscriber fan-out |
| `lesearch-pty` | `portable-pty` + `alacritty_terminal` headless buffer |
| `lesearch-storage` | AgentFS Rust SDK facade, session log writer, FTS5 indexer |
| `lesearch-keyring` | Ed25519 + age keypair management (OpenFused-compat format) |
| `lesearch-a2a` | HTTP surface for A2A + OpenFused-style inbox |
| `lesearch-avm-client` | UDS + MessagePack IPC to AVM sidecar |
| `lesearch-providers-core` | `AgentProvider` trait + shared utilities |
| `lesearch-providers-claude` | Wraps `claude` CLI via stdio MCP |
| `lesearch-providers-codex` | Wraps Codex CLI via stdio |
| `lesearch-providers-opencode` | ACP protocol |
| `lesearch-providers-gemini` | Wraps Gemini CLI |
| `lesearch-providers-generic-a2a` | Treats any remote A2A endpoint as a provider |
| `lesearch-search` | jsongrep integration + FTS5 mirror |
| `lesearch-transport-ziti` | OpenZiti Rust SDK integration |
| `lesearch-transport-noise` | Noise Protocol over WebSocket fallback |
| `lesearch-cli` | Commander-style `lesearch` CLI binary |
| `lesearch-web` | React + TanStack Router, served by daemon at `127.0.0.1:6767` |

**Runtime**: single `tokio` runtime on daemon; all async. No threads besides blocking work (FS I/O on block-in-place or spawn-blocking).

**Entry points**: `lesearch daemon start` (foreground) / system service (launchd/systemd). Single binary. Default home `$LESEARCH_HOME = ~/.lesearch`.

### 2.2 L3 — AVM Policy Sidecar (Rust, karna + cmux heritage)

Separate process, separate repo (`lesearch-avm`). Listens on UDS `$LESEARCH_HOME/avm.sock`. Speaks MessagePack RPC.

**Request**: `{ agent_id, tool_call, args, cwd, user_context }` → **Response**: `allow | deny | audit_only`.

Why separate process:
- Policy engine can crash or upgrade without taking the daemon down.
- Future: run AVM as root (for kernel-level policy hooks on Linux) while daemon stays user-level.
- Clean license boundary — AVM may later include GPL-incompatible tooling without polluting the daemon.

### 2.3 L2 — Storage & Context

**AgentFS namespace per agent**:
- Path: `$LESEARCH_HOME/agents/{agent_id}/fs/`
- Backed by SQLite DB at `$LESEARCH_HOME/agents/{agent_id}/fs.sqlite`
- Mounted at runtime: FUSE on Linux, NFS on macOS, in-process SDK fallback if mounting refused
- Agent process `cwd` is the mount point → isolation boundary

**Session log**:
- Path: `$LESEARCH_HOME/agents/{agent_id}/sessions/{session_id}.jsonl`
- Append-only, one CloudEvent per line, each Ed25519-signed
- Event types: `session.started`, `prompt.submitted`, `stream.chunk`, `tool.call.requested`, `tool.call.decided`, `tool.call.result`, `permission.requested`, `permission.decided`, `session.stopped`
- Rotation: 100 MB per file by default; `sessions-{n}.jsonl.zst` after rotation

**FTS5 mirror**:
- Path: `$LESEARCH_HOME/index.sqlite`
- Incrementally updated by `lesearch-storage` on every append
- Used for substring queries (`lesearch sessions grep`); jsongrep operates on raw JSONL

**Keyring** (OpenFused-compat on-disk format):
- Path: `$LESEARCH_HOME/.keys/`
- Files: `signing.key` (Ed25519), `encryption.key` (age/X25519), `peers.json`
- Permissions: `chmod 600`
- Format: OpenFused-compatible — a user can point `openfuse` at the same dir

### 2.4 L2′ — A2A Gateway

Inside the daemon, handled by the `lesearch-a2a` crate. Exposes:

| Endpoint | Method | Auth | Purpose |
|---|---|---|---|
| `/.well-known/agent-card.json` | GET | None | A2A agent discovery (returns card for default agent or specified via `?agent=` query) |
| `/profile` | GET | None | `PROFILE.md` (OpenFused compat) |
| `/config` | GET | None | Public keys |
| `/message/send` | POST | Bearer | Create A2A task against an agent |
| `/message/stream` | POST | Bearer | Create task + SSE stream |
| `/tasks` | GET | Bearer | List tasks |
| `/tasks/{id}` | GET | Bearer | Get task |
| `/tasks/{id}/cancel` | POST | Bearer | Cancel task |
| `/inbox` | POST | Ed25519 sig | OpenFused-compat signed inbox |
| `/outbox/{name}` | GET | Ed25519 challenge | OpenFused-compat outbox pull |

Bearer tokens stored in OS keychain; body size capped at 1 MB; SSE timeout 30 min.

### 2.5 L1 — Providers

Each provider is an async Rust trait impl:

```rust
#[async_trait]
pub trait AgentProvider: Send + Sync + 'static {
    fn manifest(&self) -> &ProviderManifest;
    async fn spawn(&self, spec: AgentSpec) -> Result<AgentHandle>;
}

pub struct AgentHandle {
    pub id: AgentId,
    pub stdin:  Option<DynAsyncWrite>,
    pub events: mpsc::Receiver<AgentEvent>,
    pub wait:   JoinHandle<ExitStatus>,
}
```

`AgentEvent` enum: `Stream(bytes)`, `ToolCallRequested(ToolCall)`, `ToolCallResult(...)`, `PermissionRequested(...)`, `SessionEnded(ExitStatus)`.

### 2.6 L5 — Transport

Three concrete transports implementing one trait:

```rust
#[async_trait]
pub trait Transport: Send + Sync {
    async fn accept(&self) -> Result<Box<dyn ClientChannel>>;
}
```

- `direct` — binds `127.0.0.1:6767`; accepts any loopback connection (Docker-equivalent trust model)
- `ziti` — attaches to Ziti identity; accepts Ziti service binds
- `noise-ws` — accepts WebSocket upgrades; performs Noise handshake before delivering JSON-RPC frames

### 2.7 L6 — Clients

- **CLI** (`lesearch`): Rust, `clap`, Docker-style verbs. Connects to daemon over `direct` by default.
- **Tauri desktop** (Phase A): Rust core + React UI. Ships daemon as sidecar. macOS codesign + notary.
- **Web console**: React served from daemon at `/`. Works only with `direct` transport.
- **Native macOS app** (Phase B): SwiftUI, own LaunchAgent-wrapped daemon, menu bar, CGVirtualDisplay bridge.
- **Native iOS app** (Phase B): SwiftUI, Ziti iOS SDK, Secure Enclave identity, background refresh.

## 3. Wire Protocol (v0.1)

### 3.1 Framing

WebSocket text frames for JSON-RPC. Binary frames for mux:

```
+---------+----------+---------------+
| u8 chan | u8 flags | bytes payload |
+---------+----------+---------------+
```

- chan 0 = control (unused in binary; reserved)
- chan 1 = terminal data (agent → client stream, client → agent input)
- chan 2+ = reserved for multi-view future

### 3.2 JSON-RPC methods (subset; full list in `lesearch-wire-protocol.md`)

**Handshake**
- `hello` (client → server): `{ client_id, version, capabilities }`
- `welcome` (server → client): `{ server_id, version, session_id, capabilities }`

**Agent lifecycle**
- `agent.create` → `{ agent_id }`
- `agent.list` → `[ { agent_id, status, provider, cwd, started, … } ]`
- `agent.attach` → subscribes to events for an agent
- `agent.send` → sends a new prompt to an agent
- `agent.stop` → sends SIGTERM then SIGKILL

**Session**
- `sessions.list`
- `sessions.search` (returns iterator cursor)
- `sessions.get`
- `sessions.verify`
- `sessions.replay` → creates new agent
- `sessions.export`

**Policy**
- `policy.evaluate` (internal; daemon → AVM sidecar over UDS — not WebSocket)
- `policy.reload` (client → daemon)
- `policy.status`

**Notifications (server → client)**
- `agent.event` — `{ agent_id, event }`
- `permission.request` — `{ agent_id, tool_call }`

### 3.3 Backward compatibility rules

- Any new field MUST be `Optional` with a default
- Never remove a field — deprecate by stopping to emit it
- Never narrow a type (string → enum; nullable → non-null)
- Versions are monotone; server advertises capability set in `welcome`

## 4. Data Model

### 4.1 Session event (CloudEvent envelope)

```json
{
  "specversion": "1.0",
  "id": "01HXZ...",
  "source": "lesearch://daemon/{daemon_id}",
  "type": "lesearch.tool.call.requested.v1",
  "subject": "agent:{agent_id}",
  "time": "2026-04-14T20:15:00Z",
  "datacontenttype": "application/json",
  "data": { "tool": "Bash", "args": { "command": "ls -la" } },
  "signature": "ed25519:{base64}",
  "signed_over": ["id","source","type","subject","time","data"]
}
```

Tamper evident: any mutation breaks the signature. Append-only enforced at the file-system level through `O_APPEND` open flag.

### 4.2 Agent registry (SQLite at `$LESEARCH_HOME/registry.sqlite`)

```
Table agents
  id             TEXT PRIMARY KEY
  provider       TEXT
  status         TEXT CHECK (status IN ('initializing','idle','running','stopped','error'))
  cwd            TEXT
  created_at     INTEGER
  last_active_at INTEGER
  config_json    TEXT
  owner_pubkey   TEXT
  policy_name    TEXT

Table sessions
  id          TEXT PRIMARY KEY
  agent_id    TEXT REFERENCES agents(id)
  started_at  INTEGER
  stopped_at  INTEGER
  status      TEXT
  file_path   TEXT
```

### 4.3 Config schema (`~/.lesearch/config.toml`)

```toml
[daemon]
home = "~/.lesearch"
log_level = "info"

[transport]
mode = "direct"           # "direct" | "ziti" | "noise-ws"

[limits]
max_concurrent_agents = 10
max_daemon_memory_mb  = 500
max_agent_memory_mb   = 2048
session_retention_days = 90
storage_warn_gb = 20

[providers]
claude   = { path = "claude", enabled = true }
codex    = { path = "codex",  enabled = true }
opencode = { path = "opencode", enabled = false }
gemini   = { path = "gemini", enabled = false }

[a2a]
enabled = false
bearer_token_env = "LESEARCH_A2A_TOKEN"

[otel]
endpoint = ""             # set to enable OpenTelemetry export
```

## 5. Sequence Diagrams (text form)

### 5.1 Spawn an agent

```
User           CLI               Daemon              AVM            Provider      AgentFS
 |  lesearch run claude "..."     |                   |               |             |
 |------------>|                  |                   |               |             |
 |             |  hello           |                   |               |             |
 |             |----------------->|                   |               |             |
 |             |  welcome         |                   |               |             |
 |             |<-----------------|                   |               |             |
 |             |  agent.create    |                   |               |             |
 |             |----------------->|                   |               |             |
 |             |                  |  create namespace |               |             |
 |             |                  |-------------------------------------------------->|
 |             |                  |                   |               |   ok        |
 |             |                  |<--------------------------------------------------|
 |             |                  |  policy.evaluate  |               |             |
 |             |                  |------------------>|               |             |
 |             |                  |       allow       |               |             |
 |             |                  |<------------------|               |             |
 |             |                  |  spawn provider   |               |             |
 |             |                  |---------------------------------->|             |
 |             |                  |  session.started  |               |             |
 |             |                  |  (write to JSONL) |               |             |
 |             |  { agent_id }    |                   |               |             |
 |             |<-----------------|                   |               |             |
 |  output ... |                  |                   |               |             |
```

### 5.2 Remote pair (Phase B)

```
User           iOS App            Daemon             Ziti Controller    Keyring
 |  lesearch daemon pair          |                        |               |
 |------------------------------->|                        |               |
 |                   QR code      |                        |               |
 |<-------------------------------|                        |               |
 |  scan QR         |             |                        |               |
 |----------------->|             |                        |               |
 |                  |  enroll(JWT)|                        |               |
 |                  |-------------------------------------->|               |
 |                  |       identity cert                   |               |
 |                  |<--------------------------------------|               |
 |                  |  add pubkey(phone)                    |               |
 |                  |--------------------------------------------------->   |
 |                  |  open Ziti dial                       |               |
 |                  |<--------------------------------------|               |
 |                  |  hello+E2E    |                       |               |
 |                  |--------------->                       |               |
 |                  |  welcome      |                       |               |
 |                  |<---------------                       |               |
```

### 5.3 Tool call with policy

```
Provider         Daemon              AVM             Session JSONL
   | tool.call.requested              |                     |
   |--------------->|                 |                     |
   |                |  policy.evaluate|                     |
   |                |---------------->|                     |
   |                |   decision      |                     |
   |                |<----------------|                     |
   |                |  append "tool.call.decided"           |
   |                |-------------------------------------->|
   |  allow/deny    |                 |                     |
   |<---------------|                 |                     |
```

## 6. Concurrency Model

- One tokio runtime in the daemon (`#[tokio::main(flavor = "multi_thread")]`).
- Per-agent actor: each agent lives in its own `tokio::spawn` task holding a `mpsc::Receiver` of commands. Messages are the only way in.
- Fan-out for subscribers: `tokio::sync::broadcast` per agent for `AgentEvent` subscribers (clients).
- Blocking work: session log writes use `spawn_blocking`; AgentFS I/O through its async SDK.
- Backpressure: event channels have bounded capacity; if a slow client can't keep up, oldest events are dropped with a `stream.gap` marker recorded in the session log.

## 7. Error Handling

- All errors typed via `thiserror` in crates, flattened to `anyhow::Error` at crate boundary for CLI only.
- Every user-visible error carries a stable error code (`ERR_1001`…) and a doc URL pointer.
- The daemon never exits on recoverable errors; agent-level failures stay scoped to that agent.
- Unrecoverable errors (FS corruption, crypto failure) fail the daemon fast with a clear message + where state is recovered from.

## 8. Security Design

### 8.1 Trust boundaries

| Boundary | Control |
|---|---|
| User ↔ Daemon (local) | Loopback, OS user == trust |
| Daemon ↔ AVM sidecar | UDS with `chmod 600` socket path |
| Daemon ↔ Provider | stdio to user-owned subprocess; provider's own auth handles its API |
| Daemon ↔ Client (remote) | Ziti mTLS OR Noise-WS E2E + bearer; A2A bearer token |
| Client ↔ Client | Never direct; always through daemon |

### 8.2 Key management

- Daemon key: generated on first run, Ed25519 + X25519; stored `chmod 600` in `$LESEARCH_HOME/.keys/`
- Optional integration with macOS Keychain / Linux Secret Service / Windows DPAPI (Phase B)
- Rotation: `lesearch keys rotate` writes a transition-signed manifest; old key stays valid for 24 h for in-flight sessions
- Revocation: `lesearch keys revoke <fingerprint>` writes a revocation record to keyring + pushes to paired peers

### 8.3 Sandboxing

- Agent `cwd` is the AgentFS mount → file access isolation
- Rust AVM policy engine blocks commands by pattern before execution
- Network egress policy (Phase B): pf (macOS) / iptables (Linux) rules scoped to agent PID cgroup

## 9. Observability

- OpenTelemetry traces with `opentelemetry-rust` exporter
- Metrics: agent counts, tool-call rate, policy-decision rate, memory RSS, open FDs, mount count
- Structured logs in JSON at `info` level by default
- `lesearch doctor` aggregates health in one command
- agents-observe hook events emitted on agent/tool events for dashboard integration

## 10. Resource Hygiene (PRD-NFR-1..8)

Concrete implementation plan for the user-flagged critical requirements:

| NFR | Implementation |
|---|---|
| Idle RAM ≤ 50 MB | Lazy-init of heavy modules; tokio runtime with `worker_threads = 2` when idle; no in-memory caches for FTS (SQLite handles it) |
| Idle CPU ≤ 0.1% | Background tasks use tokio timers with ≥ 30s intervals when no clients connected; suspend timers entirely after 5 min idle |
| Per-agent RAM overhead ≤ 20 MB | Bounded event-buffer channels; drop-oldest backpressure; xterm-headless buffer capped at N rows |
| Disk ≤ 500 MB (excl user data) | Rust binary + web console < 80 MB typical; index fragmentation handled by periodic `VACUUM` |
| Session log rotation | `O_APPEND` + file size check on every N events |
| `lesearch doctor` < 2 s | All metrics precomputed; `doctor` reads them from shared state |
| Zero zombies on shutdown | Tokio graceful shutdown guard kills all `tokio::process::Child` handles |
| Clean uninstall | `lesearch uninstall` walks LaunchAgent/systemd unit paths, AgentFS mount points, removes all |

## 11. Trade-offs & Decisions

### Why Rust for the daemon (vs Node/paseo)
+ Single static binary for end users → no runtime install friction
+ Memory safety for a long-running background service
+ First-class Ziti Rust SDK
+ karna + cmux AVM already Rust — no cross-language IPC headaches later
– Smaller pool of contributors than Node
– Slower to iterate early vs TypeScript

### Why Tauri for Phase A (vs Electron)
+ Uses system WebView (tiny binary, ~10 MB vs Electron's 100 MB+)
+ Rust core → shares state with daemon naturally
+ macOS + Linux + Windows from one build
– Slightly less mature for complex UI, but acceptable for dashboards

### Why AgentFS (vs plain directories)
+ SQLite-backed → portable, durable, snapshot-friendly
+ SDK abstracts FUSE/NFS differences
+ Trivial sandbox boundary (just a different mount per agent)
– New project — maturity risk mitigated by thin facade + fallback

### Why node-pty-equivalent + alacritty_terminal vs tmux
+ No subprocess dep for end users
+ Cross-platform (including Windows via ConPTY)
+ Agent events already modeled in-memory; tmux would be a second source of truth
– Lose "daemon restart, session survives" unless we add tmux mode as opt-in (planned in DP2)

### Why JSONL + jsongrep + FTS5 (vs Postgres or Elasticsearch)
+ Zero install — SQLite is everywhere
+ JSONL is append-only and stream-friendly — no index corruption on crash
+ jsongrep gives regex-over-paths which jq / Elastic don't natively
– Limits us to single-host search; multi-daemon federated search is Phase B

## 12. Performance Budget

| Hot path | Budget (p99) | Why |
|---|---|---|
| PTY byte → WebSocket byte (loopback) | ≤ 1 ms | User feels it |
| PTY byte → WebSocket byte (Ziti LTE) | ≤ 150 ms | LTE RTT floor |
| Tool-call policy decision (UDS round-trip) | ≤ 5 ms | Inline in tool-call path |
| Session event write (fsync, batch) | ≤ 2 ms average | Amortized by batch |
| Session FTS query (1 GB history) | ≤ 100 ms | SQLite FTS5 |
| Session jsongrep query (1 GB history) | ≤ 500 ms | DFA linear scan |
| `agent.create` end-to-end | ≤ 2 s | PTY spawn + MCP handshake + storage init |

## 13. Capacity Planning

- 10 concurrent agents per daemon default ceiling
- 1 session log per agent, rotated at 100 MB → typical session log ≤ 5 MB
- FTS5 index grows at ~25% of raw JSONL size → plan for ~500 MB index per 2 GB history
- AgentFS per agent typically < 100 MB for code+build artifacts; user should monitor

## 14. Failure Modes & Recovery

| Failure | Detection | Recovery |
|---|---|---|
| Daemon OOM | OS kill; config `max_daemon_memory_mb` preemptively refuses new agents | systemd/launchd restarts; registry replay restores agent states; sessions marked `recovered` |
| Agent provider crash | stdio EOF | Agent transitions `running → error`; event emitted; user notified |
| Agent hang | timeout on `PermissionRequested` > 1 h (default) | Auto-deny with audit event; admin override possible |
| AVM sidecar crash | UDS EOF | Daemon falls back to `deny-all` for sensitive tools; logs loudly |
| Disk full | Write fails | New sessions refused; `doctor` flags; existing agents marked `paused` with retry policy |
| Clock skew breaking audit sigs | Sig verification fails | `lesearch doctor` flags; events still stored but flagged |
| Network loss (remote client) | WebSocket close | Client reconnects exponential backoff up to 30 s; local daemon + agent unaffected |

## 15. Open Design Questions (for team review)

Q-D1 — Should the daemon expose its HTTP surface (A2A + web console) on a different port from WebSocket? Separating isolates DoS surfaces but complicates config.
Q-D2 — Is the `Transport` trait too coarse? Consider splitting `listen` from `accept_client` to allow multi-transport daemon (Ziti AND loopback both active).
Q-D3 — Should session events use JSON Web Signatures (JWS) instead of raw Ed25519 over CloudEvents? JWS is more standardized but adds deps.
Q-D4 — Should the FTS5 mirror be rebuildable from JSONL (true) or authoritative (false)? Rebuildable is simpler but slower on boot.
Q-D5 — Do we bake `openfused` as a hard Rust crate dependency, or reimplement the on-disk format ourselves for control?
Q-D6 — Should we support a MCP proxy mode where the daemon is itself an MCP server other agents connect to?

## 16. Revision History

| Rev | Date | Notes |
|---|---|---|
| 0.1 | 2026-04-14 | Initial system design for multi-agent review. |

