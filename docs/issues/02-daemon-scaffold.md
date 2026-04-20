# Issue: Scaffold lesearch-daemon with axum + tokio + WebSocket bind

**Labels:** `area/daemon` `type/feature` `priority/critical` `phase/A.1`
**Milestone:** v0.1.0 Daily Driver
**Story refs:** S-1.1, S-1.2, S-2.1
**Blocks:** `03-pty-crate`, `04-storage-scaffold`, `05-a2a-scaffold`

## Summary

Stand up the `lesearch-daemon` binary: tokio runtime, axum server, WebSocket endpoint on `127.0.0.1:6767/ws`, graceful shutdown, structured logging. No agent spawning yet — just the protocol handshake and a loopback echo for integration testing.

## Acceptance criteria

- [ ] `cargo run -p lesearch-daemon` binds `127.0.0.1:6767`
- [ ] `GET /healthz` returns `200 OK` with `{ "status": "ok", "version": "0.0.1" }`
- [ ] `WS /ws` accepts connections and responds to `server.handshake` with the correct shape from `protocol-v0.1.md` §3
- [ ] `SIGTERM` triggers graceful shutdown: all connections closed with 1001, server exits in ≤ 5 seconds
- [ ] `tracing-subscriber` configured with `RUST_LOG=info` default, JSON output in `--json` mode
- [ ] Integration test: spawn daemon, connect via `tokio-tungstenite`, send handshake, verify response, tear down
- [ ] Zero clippy warnings under workspace lint config (pedantic + nursery)

## Non-goals

- Agent spawning (issue `03`)
- Storage integration (issue `04`)
- A2A HTTP routes (issue `05`)
- Authentication (loopback only for this iteration)

## Implementation notes

- Use `axum::Router` with a single `/ws` handler and `/healthz` handler
- WebSocket handler: `axum::extract::ws::WebSocketUpgrade`
- Handshake router: dispatch on `method` string in a `match`; unknown methods return `-32601`
- Graceful shutdown: `tokio::signal::ctrl_c()` + `axum::serve::with_graceful_shutdown`

## References

- `docs/SYSTEM_DESIGN.md` §2.1
- `docs/protocol-v0.1.md` §2-3
- `crates/lesearch-daemon/src/main.rs` (current stub)
