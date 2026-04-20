# Issue: Scaffold A2A facade (/.well-known/agent-card.json + /message/send + /tasks)

**Labels:** `area/a2a` `type/feature` `priority/critical` `phase/A.4`
**Milestone:** v0.1.0 Daily Driver
**Story refs:** S-8.1, S-8.2, S-8.3
**Blocked by:** `02-daemon-scaffold`, `04-storage-scaffold`
**Blocks:** `AC-A-10`, `AC-A-11`

## Summary

Expose every spawned agent over the [A2A protocol](https://github.com/a2a-ai/a2a-spec) per `docs/A2A_FACADE.md`. External A2A clients (OpenAI apps, Google agent frameworks, etc.) authenticate with bearer tokens and drive local LeSearch agents without knowing about the WebSocket protocol.

## Acceptance criteria

- [ ] `GET /.well-known/agent-card.json?agent={id}` returns the card shape from `A2A_FACADE.md` §Agent Card Schema
- [ ] `POST /message/send` accepts an A2A Message, creates a Task, returns `{ task_id, status: "submitted" }`
- [ ] `POST /message/stream` streams task events as SSE (text/event-stream)
- [ ] `GET /tasks` lists tasks with cursor pagination
- [ ] `GET /tasks/{task_id}` returns task detail (status, artifacts, history)
- [ ] `POST /inbox` accepts OpenFused-compatible signed envelopes and writes them to `agents/{id}/inbox/`
- [ ] Bearer auth: requests without valid `Authorization: Bearer <token>` return `401` + `X-LeSearch-Error: -32000`
- [ ] Rate limit: token-bucket, 60/min default, returns `429` + `Retry-After` on exhaustion
- [ ] `lesearch daemon issue-token --name <n>` prints a new 64-byte token and writes it to `config.toml`
- [ ] Tokens are redacted in all logs (tracing filter)
- [ ] E2E test (AC-A-10): external `curl` hits `/.well-known/agent-card.json` + POSTs a task + polls until done

## Non-goals

- mTLS / OAuth2 (v0.2)
- Webhook delivery (v0.2)
- Multi-tenant token scoping (v0.2)

## Implementation notes

- Same axum server as `/ws`; A2A routes added under the same `Router`
- Route handlers: `get_agent_card`, `post_message_send`, `post_message_stream`, `list_tasks`, `get_task`, `post_inbox`
- Bearer tokens live in `config.toml` `[a2a]` section; hot-reload on SIGHUP
- Rate limiter: per-token token-bucket in-process (no Redis); `governor` crate
- OpenFused-compat envelope parse: use the same Ed25519 + age format as `openfused` crate (re-implement — don't hard-dep)

## References

- `docs/A2A_FACADE.md` (primary)
- `docs/protocol-v0.1.md` §6
- `docs/PRD.md` FR-29 through FR-31
- [A2A protocol spec](https://github.com/a2a-ai/a2a-spec)
- [OpenFused](https://github.com/openfused/openfused)
