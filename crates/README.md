# LeSearch

**Self-hosted, Rust-first agent control plane.** One daemon. One protocol. Any CLI coding agent.

[![License](https://img.shields.io/badge/license-Apache--2.0-blue)](LICENSE)
[![MSRV](https://img.shields.io/badge/rust-1.85+-orange)](https://www.rust-lang.org)

Spawn, observe, and steer Claude Code, Codex, OpenCode, Gemini CLI (and any A2A-compatible agent) from a single interface. Per-agent filesystem isolation. Signed session history. Cryptographically-audited policy engine. Zero-trust remote access.

> Status: **Phase A.0** — scaffolding. Not yet runnable. See [`docs/PRD.md`](docs/PRD.md) and [`docs/EPICS_AND_STORIES.md`](docs/EPICS_AND_STORIES.md).

## Why

Every week brings a new CLI coding agent. Each ships its own UX, transport, session model, observability story, and security posture. Your terminal becomes a zoo.

LeSearch is the adapter layer. One daemon speaks to all of them. You get:

- **Isolation** — each agent runs in its own AgentFS namespace; one agent can't read another's files
- **Replay** — every session is a signed JSONL CloudEvents log; `lesearch sessions replay <id>` rehydrates state
- **Search** — `lesearch sessions search '**.(error|warn)'` via jsongrep over all history
- **Interop** — every agent exposes an A2A `/message/send` endpoint; external A2A clients (OpenAI, Google) drive your local agents
- **Remote** — zero-trust access over OpenZiti from phone or laptop; works on LTE
- **Policy** — AVM sidecar blocks `rm -rf`, curl|sh, writes to `/etc`, and anything you declare; every decision is signed-audited

## Design Principles

1. **Self-hosted only.** No cloud, no phone-home, no telemetry default.
2. **Apache-2.0.** No copyleft. Commercial downstream friction = zero.
3. **Single binary.** `brew install lesearch && lesearch run "hello"` in under 60 seconds.
4. **Protocol-first.** Wire format versioned, additive-only, clients ≥ 6 months back still work.
5. **Rust daemon.** MSRV 1.85, edition 2024, `#![forbid(unsafe_code)]`.

## Architecture (6 layers)

```
L6  Clients               Tauri desktop + web console (Phase A)
                          Native SwiftUI macOS + iOS (Phase B)
         ↓ WebSocket + JSON-RPC 2.0 + E2E crypto (X25519 + ChaCha20-Poly1305)
L5  Transport             OpenZiti overlay + Noise-WS fallback + zrok
         ↓
L4  LeSearch Daemon       Rust — protocol router, agent manager, PTY pool, MCP server
         ├─ L3  AVM       policy + sandbox (Rust, karna + cmux lift)
         ├─ L2  A2A       /.well-known/agent-card.json, /message/send, /tasks/*
         └─ L1  Providers Claude Code, Codex, OpenCode, Gemini, generic-A2A
                ↓
L0  Storage & Context    AgentFS per-agent namespace + JSONL CloudEvents session log
                         + jsongrep index + Ed25519-signed keyring
```

## Roadmap

- **Phase A** (in progress): dogfoodable daily driver. Daemon + providers + web console + Tauri. Exit when Arya uses lesearch ≥80% of Claude Code work for 2 weeks.
- **Phase B**: zero-trust remote + native SwiftUI macOS (CGVirtualDisplay per-agent virtual displays) + iOS + full AVM enforcement.

Full plan: [`docs/PRD.md`](docs/PRD.md), [`docs/EPICS_AND_STORIES.md`](docs/EPICS_AND_STORIES.md).

## Relationship to paseo

[paseo](https://github.com/paseo-ai/paseo) is the incumbent multi-agent control plane (3.3k★, AGPL-3.0). LeSearch takes a **hybrid posture**:

- Contribute observability (OTEL tracing), guardrails, and input-arbiter upstream to paseo
- Keep differentiators (AgentFS isolation, A2A facade, signed sessions, AVM, Rust daemon) in a separately-licensed (Apache-2.0) project
- Interop: paseo sessions can be launched from lesearch's dispatch layer; lesearch agents expose A2A so paseo (and anything else) can drive them

## Build

```bash
# scaffold-only for now
cargo check --workspace
```

Real daemon comes online Day 2. See [`docs/protocol-v0.1.md`](docs/protocol-v0.1.md) for the wire protocol.

## License

Apache License 2.0. See [`LICENSE`](LICENSE) and [`NOTICE`](NOTICE).
