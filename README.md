# lecode

> One terminal. Every agent. Every machine.

An open-source, self-hosted platform for running AI coding agents on your own hardware. Connect from any device — desktop, phone, terminal — to agents spawned in isolated git worktrees. Apache-2.0.

**Status: v0.0.1 (pre-alpha).** Core daemon + protocol + CLI land-and-iterating. Mobile client and desktop app come next.

## What it is

- **Daemon** (`crates/lesearch-daemon`) — Rust axum WebSocket server at `127.0.0.1:6767/ws`. Spawns agents, allocates stream IDs, routes agent output as JSON-RPC notifications.
- **Protocol** (`crates/lesearch-protocol`) — JSON-RPC 2.0 envelope, `SpawnParams`, `SpawnResult`, `StreamIds`, `AgentOutputParams`. Versioned `0.1.0`, additive-only.
- **Providers** (`crates/lesearch-providers`) — `AgentProvider` trait; `ClaudeProvider` (subprocess), `TestProvider` (test-gated).
- **CLI** (`crates/lesearch-cli`) — `lesearch run`, connects to daemon, streams agent stdout.
- **Storage** (`crates/lesearch-storage`) — session log scaffold.
- **Website** (`web/`) — Next.js 16 landing page, Apache-2.0 content.

## Quick start

```bash
# rust
cargo test --workspace
cargo run -p lesearch-daemon --bin lesearch-daemon

# web
cd web && npm run dev
```

## Clean-room methodology

lecode draws architectural inspiration from several open-source agent-management projects. Per clean-room practice, no code from AGPL-licensed projects is incorporated. Concept extraction lives in `SPEC/concepts/`. See `NOTICE` and `SPEC/README.md`.

## License

Apache-2.0. Contributions licensed inbound=outbound.

## Repo layout

```
lecode/
├── crates/            Rust workspace
│   ├── lesearch-protocol/
│   ├── lesearch-daemon/
│   ├── lesearch-cli/
│   ├── lesearch-providers/
│   └── lesearch-storage/
├── web/               Next.js landing page
├── SPEC/              Clean-room design docs
├── docs/              Architecture + product docs
└── .github/workflows/ CI
```

## Links

- [LICENSE](LICENSE) — Apache-2.0
- [NOTICE](NOTICE) — attribution + clean-room statement
- [SECURITY.md](SECURITY.md) — responsible disclosure
- [CONTRIBUTING.md](CONTRIBUTING.md) — DCO, tests required
