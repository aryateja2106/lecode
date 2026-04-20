# ROADMAP

Concrete milestones. No fluff. Status reflects the state of the `main` branch.

---

## v0.0.1 — Rust foundation ✅ shipped

**Goal:** Working daemon + protocol + CLI. Prove the WebSocket JSON-RPC loop end-to-end.

- [x] `lesearch-protocol` — `RpcRequest`, `RpcResponse`, `RpcError`, `SpawnParams`, `SpawnResult`, `StreamIds`
- [x] `lesearch-daemon` — axum WebSocket at `127.0.0.1:6767/ws`; `AgentManager` registers agents and allocates stream IDs
- [x] `lesearch-providers` — `AgentProvider` trait; `ClaudeProvider` (subprocess); `TestProvider` (feature-gated)
- [x] `lesearch-cli` — connects to daemon, sends `agent.spawn`, streams `agent.output` to stdout
- [x] `lesearch-storage` — scaffold only (rusqlite dep wired, no active logic)
- [x] 13 passing tests including a full end-to-end spawn-and-stream integration test
- [x] CI (GitHub Actions), Apache-2.0 license, clean-room methodology documented

---

## v0.1 — Runnable daemon + richer CLI 🟡 in progress

**Goal:** `cargo run -p lesearch-daemon` actually starts the server. CLI gets subcommands.

- [ ] Wire `main.rs` to call `serve_with_providers` with the `ClaudeProvider` registered
- [ ] `agent.list` RPC method — returns all active agents with provider and start time
- [ ] `agent.stop` RPC method — kills the subprocess, removes from registry
- [ ] `server.handshake` RPC method — returns version, protocol, uptime
- [ ] `lesearch-cli run` subcommand — replaces flat flags, adds `--prompt`
- [ ] `lesearch-cli ls` subcommand — calls `agent.list`, prints table
- [ ] `lesearch-cli stop <id>` subcommand — calls `agent.stop`
- [ ] `lesearch-cli daemon status` subcommand — calls `server.handshake`
- [ ] `lesearch-cli logs <id> [--follow]` subcommand — streams stored output
- [ ] Session log persistence via `lesearch-storage` (SQLite; one row per agent output line)
- [ ] `stderr` forwarded as `agent.output` with `stream: "stderr"` discrimination
- [ ] VHS terminal demo GIF (`demos/lecode-quickstart.gif`)

---

## v0.2 — TypeScript SDK 🔴 planned

**Goal:** A typed TypeScript client over the WebSocket protocol.

Modeled on [pi-ai](https://github.com/badlogic/pi-mono/tree/main/packages/ai):
- Unified `AgentClient` interface with `spawn`, `list`, `stop`, `logs` methods
- Streaming events via async iterators
- TypeBox schemas for all RPC types (generated from Rust protocol definitions)
- Automatic context serialization helpers
- Published to npm as `@lecode/client`

---

## v0.3 — Desktop app 🔴 planned

**Goal:** Electron wrapper with parity to the daemon CLI.

- Agent list view with live output tailing
- Spawn dialog with provider and worktree picker
- Daemon start/stop/status from the menubar
- Auto-update via GitHub Releases

---

## v0.4 — Mobile app 🔴 planned

**Goal:** iOS and Android client for monitoring agents on the go.

- QR-code daemon pairing (local network)
- Live agent output stream with monospace rendering
- Push notifications on agent completion / error
- Built on the TypeScript SDK from v0.2

---

## v0.5 — Relay for remote access 🔴 planned

**Goal:** Reach your daemon from outside the local network without exposing ports.

- E2E encrypted relay (no plaintext at relay server)
- Short-lived pairing tokens (QR code flow)
- Self-hostable relay server
- Threat model documented in `SECURITY.md`

---

## Non-goals (permanently out of scope)

- Cloud-hosted agent execution (lecode is self-hosted only)
- Storing agent outputs off-device (your code stays on your machine)
- AGPL-licensed dependencies (clean-room methodology — see `NOTICE`)
