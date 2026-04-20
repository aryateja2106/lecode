# LeSearch Wire Protocol — v0.1

**Status:** Draft. Subject to additive revisions until v1.0. Last updated 2026-04-14.

This document is the canonical specification of the LeSearch wire protocol. Implementers of clients (CLI, Tauri desktop, web, iOS, macOS, third-party A2A) MUST conform. The daemon implementation (`lesearch-daemon`) is authoritative in case of conflict, but MUST be corrected to match this spec.

## 1. Transport

All clients connect to the daemon over a single **WebSocket** (RFC 6455) connection. The default endpoint is `ws://127.0.0.1:6767/ws`. When OpenZiti is enabled, clients connect through a Ziti identity; when Noise-WS fallback is active, the WebSocket is wrapped in a Noise Protocol handshake before any protocol frames flow.

- **Text frames** carry JSON-RPC 2.0 envelopes (see §2).
- **Binary frames** carry PTY stream data multiplexed by `stream_id` (see §4).

Clients MUST NOT send PTY binary frames before completing the `server.handshake` request (§3).

## 2. JSON-RPC 2.0 Envelope

Every text frame is a single UTF-8 JSON-RPC 2.0 object, conforming to [JSON-RPC 2.0 Specification](https://www.jsonrpc.org/specification). Batched requests are permitted but not required.

```json
{
  "jsonrpc": "2.0",
  "id": "<string|number>",
  "method": "<dotted.name>",
  "params": { ... }
}
```

Responses carry either `result` or `error`. Notifications (method calls without an `id`) are used for server-to-client events only.

### 2.1 Versioning policy

- `PROTOCOL_VERSION` is surfaced via `server.handshake` and follows SemVer.
- MINOR bumps are additive-only: new methods, new optional params, new event types. Clients MUST ignore unknown fields.
- MAJOR bumps are breaking. A MAJOR bump will expose a versioned URL path (`/ws/v2`) so v1 clients keep working for at least 6 months after release.
- Never rename, never repurpose, never remove a method or field without a MAJOR bump.

## 3. Handshake

### `server.handshake`

Request (client → server):

```json
{
  "jsonrpc": "2.0", "id": "1",
  "method": "server.handshake",
  "params": {
    "client_name": "lesearch-cli",
    "client_version": "0.0.1",
    "client_capabilities": ["binary-mux", "sse"],
    "bearer_token": "<optional>"
  }
}
```

Response:

```json
{
  "jsonrpc": "2.0", "id": "1",
  "result": {
    "protocol_version": "0.1.0",
    "daemon_version": "0.0.1",
    "server_capabilities": ["binary-mux", "a2a", "sessions", "search"],
    "agent_id": "<daemon's own A2A agent id>"
  }
}
```

The server assigns the connection's trust level at this step. Loopback connections are `local`. Remote connections via Ziti or Noise-WS must present a valid `bearer_token` (daemon-issued, paired via QR).

## 4. Agent Lifecycle

### `agent.spawn`

Create a new agent instance.

Params:

- `provider` (string, required) — one of `claude`, `codex`, `opencode`, `gemini`, `generic-a2a`
- `prompt` (string, required) — initial prompt
- `cwd` (string, optional) — working directory (must resolve inside an AgentFS namespace)
- `worktree` (string, optional) — git worktree path to bind
- `model` (string, optional) — provider-specific model identifier
- `mode` (string, optional) — provider-specific mode (e.g., `plan`, `default`)
- `policy_profile` (string, optional) — named AVM profile

Result:

- `agent_id` (string) — UUIDv7
- `a2a_card_url` (string) — `http://127.0.0.1:6767/.well-known/agent-card.json?agent={agent_id}`
- `session_id` (string) — UUIDv7
- `streams` — object with `stdout`, `stdin`, `stderr` stream ids for binary-mux

### `agent.input`

Send text to an agent's stdin.

### `agent.stop`

Request graceful shutdown. Daemon SIGTERMs the provider subprocess and waits 10 seconds before SIGKILL. Clients receive `agent.status` notification when state reaches `closed`.

### Server-to-client notifications

- `agent.output` — stdout/stderr chunk (rare; prefer binary-mux for throughput)
- `agent.status` — state transitions (`initializing | idle | running | idle | closed | error`)
- `agent.tool_call` — emitted before each tool use; when AVM is active, includes policy decision
- `agent.done` — final notification with exit code + summary

## 5. Session Management

### `session.list`

Returns metadata for all sessions (active and historical) visible to the requesting connection. Supports cursor pagination.

### `session.search`

Params:

- `query` (string, required) — jsongrep path-regex expression, e.g., `$..messages[?(@.role=="tool")].name`
- `agent_filter` (array of agent_ids, optional)
- `time_range` (ISO8601 pair, optional)
- `limit` (integer, default 100, max 1000)

Returns matching events with signed attestation (Ed25519).

### `session.replay`

Rehydrates a past session's prompt and working environment into a new agent of the same provider. Result contains a new `agent_id`.

### `session.export`

Serializes a session as `markdown`, `jsonl`, or `cbor`. Ed25519 signature preserved in all formats.

## 6. A2A Surface (HTTP, not WebSocket)

Every agent exposes the [Agent-to-Agent protocol](https://github.com/a2a-ai/a2a-spec) at:

- `GET /.well-known/agent-card.json?agent={id}` — agent card (model, capabilities, auth scheme)
- `POST /message/send` — task submission (body: A2A Message)
- `POST /message/stream` — SSE streaming variant
- `GET /tasks` — list agent's task registry
- `GET /tasks/{task_id}` — task detail

All A2A endpoints require a bearer token configured in daemon `config.toml`. Rate limit: 60 requests/min per token (configurable).

## 7. Error Codes

| Code | Name | Meaning |
|---|---|---|
| -32700 | Parse error | Malformed JSON |
| -32600 | Invalid request | JSON-RPC violation |
| -32601 | Method not found | Unknown method |
| -32602 | Invalid params | Parameter validation failed |
| -32603 | Internal error | Unexpected daemon failure |
| -32000 | Unauthenticated | Missing or invalid bearer token |
| -32001 | Unauthorized | Valid token, disallowed method |
| -32002 | Provider missing | Provider binary not installed |
| -32003 | Agent not found | Unknown `agent_id` |
| -32004 | Agent state invalid | Method called in incompatible state |
| -32005 | Policy denied | AVM rejected tool call |
| -32006 | Storage error | AgentFS or session log I/O failure |
| -32007 | Resource limit | `max_concurrent_agents` exceeded |
| -32008 | Transport closed | Connection aborting |

## 8. Binary Multiplexing (informative)

Binary frames use the following layout:

```
u16 version (LE)  always 1
u16 stream_id   assigned during agent.spawn
u32 length      payload bytes
[payload]
```

Clients negotiate binary-mux support during handshake. Servers MUST fall back to `agent.output` JSON notifications for clients that did not advertise `binary-mux`.

## 9. Compatibility Commitments

- Clients built against v0.x continue to work with v0.y where y ≥ x until an explicit deprecation release.
- Every breaking change ships with a migration note in `CHANGELOG.md`.
- The daemon advertises `deprecated_methods` in `server.handshake.result` once deprecations begin.

## 10. Reference Implementation

`crates/lesearch-protocol` (this workspace) provides the canonical Rust types. A TypeScript port is planned in `lesearch-web/` and `lesearch-ios/` clients; both MUST regenerate from the Rust source of truth via `cargo run -p lesearch-protocol --bin emit-schema` (Day 4 deliverable).

## 11. Out of Scope (v0.1)

- Multi-daemon federation (Phase B.5 `dispatch`)
- Vector search (Phase B.7)
- Encrypted-at-rest session logs (storage layer optional extension)
- Full AVM policy enforcement (Phase B.2 — Phase A only logs, does not block)
