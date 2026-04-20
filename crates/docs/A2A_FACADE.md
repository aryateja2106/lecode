# A2A Facade

**Status:** Draft. Updated 2026-04-14.

Companion to `protocol-v0.1.md` §6. Owned by `lesearch-daemon` and (eventually) extracted into `lesearch-a2a` crate per `SYSTEM_DESIGN.md`.

## Purpose

Every agent spawned through LeSearch exposes an [A2A (Agent-to-Agent)](https://github.com/a2a-ai/a2a-spec) endpoint out of the box. External A2A clients — OpenAI apps, Google agent frameworks, Anthropic's A2A-compatible tooling, third-party harnesses — can invoke a local LeSearch agent without knowing anything about the daemon's internals.

This is the primary differentiator from paseo: LeSearch agents are always **addressable**, not just controllable by our own clients.

## Routes

| Method | Path | Purpose |
|---|---|---|
| GET | `/.well-known/agent-card.json?agent={id}` | Model card (model, capabilities, auth, rate limit) |
| POST | `/message/send` | Submit A2A Message; returns task id |
| POST | `/message/stream` | Submit A2A Message; SSE stream of task events |
| GET | `/tasks` | List agent's tasks (supports filter + cursor) |
| GET | `/tasks/{task_id}` | Task detail (status, artifacts, history) |
| POST | `/inbox` | Deliver an OpenFused-compatible signed message |

All routes are served by the same `axum` server that hosts the WebSocket. Default bind: `127.0.0.1:6767`. When Ziti or Noise-WS is active, A2A traffic shares the same transport.

## Authentication

Bearer tokens are the only auth scheme in v0.1. Tokens are declared in `config.toml`:

```toml
[a2a]
bearer_tokens = [
  { name = "openai-app", token = "sk-ls-..." },
  { name = "ci-runner", token = "sk-ls-..." }
]
rate_limit_per_minute = 60
```

Every request MUST carry `Authorization: Bearer <token>`. The daemon rejects unauthenticated requests with `401` + an `X-LeSearch-Error: -32000` header.

## Rate Limits

Per-token token-bucket: 60 req/min default, configurable per-token. Exceeded requests get `429` + `Retry-After` header. Rate-limit state lives in-process (no Redis).

## OpenFused Compatibility (R-10)

LeSearch aims to be wire-compatible with [openfused](https://github.com/openfused/openfused) so that agents on either mesh can send messages to each other without a translation layer.

Compatibility commitments:

- **PROFILE.md** — each agent directory (`agents/<id>/profile.toml`) also emits a `PROFILE.md` at mount root matching OpenFused field names (`name`, `description`, `capabilities`, `pubkey`, `endpoint`)
- **Inbox/outbox** — signed messages land in `agents/<id>/inbox/` as individual `.msg` files per OpenFused format (age-encrypted envelope wrapping JSON)
- **Keyring** — daemon Ed25519 + age keys are stored in the OpenFused format under `keyring/`
- **CHARTER.md** — workspaces under `workspaces/<id>/` expose a `CHARTER.md` describing participating agents and shared policy

We re-implement the format in-house rather than hard-depending on the OpenFused Rust crate. Stay wire-compatible; break glass if needed.

## Agent Card Schema (informative)

```json
{
  "name": "claude-code-local",
  "version": "0.0.1",
  "description": "Claude Code CLI wrapped by LeSearch daemon",
  "provider": {
    "name": "anthropic",
    "model": "claude-opus-4-6"
  },
  "endpoints": {
    "send": "http://127.0.0.1:6767/message/send?agent=9f8e...",
    "stream": "http://127.0.0.1:6767/message/stream?agent=9f8e...",
    "inbox": "http://127.0.0.1:6767/inbox?agent=9f8e..."
  },
  "auth": {
    "type": "bearer",
    "scheme": "Authorization: Bearer <token>"
  },
  "capabilities": ["text", "tool_call", "streaming", "signed_events"],
  "rate_limit": { "per_minute": 60 },
  "pubkey": {
    "type": "ed25519",
    "value": "ba7816..."
  }
}
```

## Flow: External A2A Client → LeSearch Agent

```
1. Client GETs /.well-known/agent-card.json?agent=<id>
   ← receives model card + bearer scheme

2. Client POSTs /message/send
   body: { "role": "user", "parts": [{"type": "text", "text": "..."}] }
   auth: Bearer <token>
   ← receives { "task_id": "...", "status": "submitted" }

3. Client GETs /tasks/<task_id> (or subscribes via SSE)
   ← receives { "status": "running", "artifacts": [...] }
   ... until "status": "done"

4. Client reads final artifacts from task history
```

## Flow: LeSearch Agent → External A2A Provider

Using `generic-a2a` provider (mirror case):

```
1. lesearch run --provider generic-a2a --url https://external/card.json "task"
2. Daemon fetches card, negotiates auth, wraps remote endpoint as local agent
3. Session log + AgentFS behave identically
4. User sees local agent_id; remote state hidden
```

This closes the loop: LeSearch is both an A2A host and an A2A client.

## Deferred (v0.2+)

- mTLS auth scheme alongside bearer
- Per-agent rate limit overrides
- OAuth2 flow for hosted use cases
- Webhook delivery (vs SSE pull)

## Security Notes

- Bearer tokens MUST be ≥ 64 bytes entropy. Daemon `lesearch daemon issue-token` generates them.
- Tokens never land in logs. `tracing` redacts them at the span level.
- The A2A server binds loopback by default; remote exposure requires Ziti or Noise-WS.
