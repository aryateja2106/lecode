# LeSearch — Product Requirements Document (PRD)

**Version**: 0.1 (draft)
**Date**: 2026-04-14
**Owner**: Arya Teja Rudraraju
**Status**: Pre-implementation, for multi-agent review

Reads with: `PRODUCT_SPEC.md` (vision), `EPICS_AND_STORIES.md` (backlog), `SYSTEM_DESIGN.md` (architecture).

---

## 1. Executive Summary

LeSearch is a self-hosted, Apache-2.0, Rust-first agent control plane that gives developers one interface to spawn, observe, and steer any CLI coding agent from any of their devices over a zero-trust overlay. It provides per-agent filesystem isolation (AgentFS), signed session history with jsongrep-based search, a cryptographically audited policy engine (AVM), and a native A2A facade so every agent becomes interoperable with the broader agent ecosystem.

## 2. Goals

### 2.1 Primary goals (v1)

- **G1** — Replace direct use of Claude Code + Codex + OpenCode CLIs for the author's daily work within the Phase A dogfood window.
- **G2** — Ship a single-binary daemon that runs cleanly on macOS and Linux with no vendor cloud dependency.
- **G3** — Enable secure remote control of agents from a mobile device over LTE using OpenZiti identity-based transport.
- **G4** — Persist every agent session as signed, searchable JSONL; make "find every shell command that ran last month" a one-line query.
- **G5** — Enforce per-agent tool-call policy via an AVM sidecar; every decision cryptographically auditable.
- **G6** — Expose every lesearch-managed agent via the A2A protocol so external tools (OpenAI agents, Google A2A clients) can drive them.
- **G7** — Ship an Apache-2.0 codebase that permits commercial downstream use without license friction.

### 2.2 Non-goals (v1)

- **NG1** — Hosting a SaaS version of LeSearch.
- **NG2** — Supporting Windows as a host OS (Phase B consideration only).
- **NG3** — Building a proprietary cloud relay. (We rely on user-run Ziti controllers or OpenFused's hosted mailbox for people who opt in.)
- **NG4** — Integrating SSO / SAML / enterprise IAM.
- **NG5** — Replacing IDEs. LeSearch is an agent control plane, not a code editor.

## 3. Target Users & Primary Use Cases

Canonical personas (details in `PRODUCT_SPEC.md §3`):

1. Solo Power-User Developer (primary)
2. Security-Conscious Developer (secondary)
3. Multi-Provider Hobbyist (secondary)
4. OSS Ecosystem Contributor (tertiary)
5. Small Team Lead (Phase B)

### Top-level use cases

| UC | Description |
|---|---|
| UC-1 | Spawn a coding agent from the CLI and watch it stream output |
| UC-2 | Attach to an already-running agent and send a follow-up prompt |
| UC-3 | List all recent agents across providers, filtered by project or status |
| UC-4 | Search across all past sessions for a specific tool call or error |
| UC-5 | Replay a prior session to start a new agent with the same context |
| UC-6 | Pair a phone and send a task to a remote daemon from LTE |
| UC-7 | Approve or deny an in-flight tool permission request from phone |
| UC-8 | Configure a policy that blocks `rm -rf`, `curl | sh`, and a list of domains |
| UC-9 | Point an external A2A client at a lesearch daemon and drive an agent via REST |
| UC-10 | Export a session as markdown for a blog post or review |
| UC-11 | Rotate the daemon's signing key and re-issue identities |
| UC-12 | Uninstall lesearch and have zero leftover files, processes, or mounts |

## 4. Functional Requirements (FR)

### 4.1 Agent lifecycle

- **FR-1** — The daemon MUST spawn an agent given: provider, prompt, working directory, optional worktree, optional model, optional mode.
- **FR-2** — The daemon MUST maintain agent state machine: `initializing → idle → running → idle → closed` with error-state transitions.
- **FR-3** — The daemon MUST support concurrent agents bounded by a configurable ceiling (`max_concurrent_agents`, default 10).
- **FR-4** — The daemon MUST reparent or kill all child processes on shutdown; zero zombies after SIGTERM + 10s grace.
- **FR-5** — `lesearch stop <id>` MUST send SIGTERM, then SIGKILL after a 5s grace window.

### 4.2 Providers

- **FR-6** — The daemon MUST support Claude Code, Codex, OpenCode, Gemini CLI, and a generic A2A import adapter at GA.
- **FR-7** — Provider adapters MUST be thin subprocesses communicating via stdio (MCP / ACP) and MUST NOT hold agent state beyond their own CLI.
- **FR-8** — Adding a new provider MUST be possible without modifying the daemon core — drop a new crate/binary, register in config.
- **FR-9** — Each provider MUST declare its capabilities in a static manifest (languages, tools, modes, known auth method).

### 4.3 Storage & isolation

- **FR-10** — Each agent MUST be given a private AgentFS namespace at `$LESEARCH_HOME/agents/{id}/fs/` (SQLite-backed) mounted at a well-known path.
- **FR-11** — The agent process MUST be unable to read another agent's namespace (enforced by sandbox or mount permissions).
- **FR-12** — A shared workspace MAY be mounted by multiple agents if the user explicitly creates it with `lesearch workspace create`.
- **FR-13** — Each agent MUST write a session log to `$LESEARCH_HOME/agents/{id}/sessions/{session-id}.jsonl`; each event MUST be Ed25519-signed with the daemon's key.

### 4.4 Protocol & clients

- **FR-14** — The daemon MUST expose a WebSocket endpoint at `127.0.0.1:6767/ws` speaking JSON-RPC 2.0.
- **FR-15** — The WebSocket protocol MUST support binary multiplexing of terminal data alongside JSON control messages.
- **FR-16** — The daemon MUST expose an HTTP surface for A2A at `/.well-known/agent-card.json`, `POST /message/send`, `GET /tasks/{id}`.
- **FR-17** — Protocol changes MUST be backward-compatible with clients ≥ 6 months old (additive fields only; no removed or narrowed types).
- **FR-18** — All clients (CLI, Tauri desktop, native iOS/macOS, web) MUST speak the same wire protocol.

### 4.5 Transport & pairing

- **FR-19** — The daemon MUST support three transports selectable by config: `direct` (loopback only), `ziti` (OpenZiti overlay), `noise-ws` (Noise-Protocol-over-WebSocket fallback for zero-config).
- **FR-20** — `lesearch daemon pair` MUST produce a QR code containing daemon public key, optional Ziti enrollment JWT, and a pairing session ID.
- **FR-21** — On successful pairing, the client's public key MUST be added to the daemon's keyring with default "trusted" classification.
- **FR-22** — The daemon MUST NOT bind to any non-loopback interface without an explicit transport that requires it (e.g., `noise-ws` in public mode).

### 4.6 Session search

- **FR-23** — `lesearch sessions list` MUST list all sessions across agents with metadata (agent, provider, cwd, started, duration, status).
- **FR-24** — `lesearch sessions search <query>` MUST accept jsongrep path-regex syntax and return matching events.
- **FR-25** — The daemon MUST maintain a SQLite FTS5 mirror of session content for substring search latency < 100 ms p99 over 1 GB of history.
- **FR-26** — `lesearch sessions verify <id>` MUST verify all Ed25519 signatures and report tamper detection.
- **FR-27** — `lesearch sessions replay <id>` MUST rehydrate a new agent with the original prompt + environment + last-known cwd.
- **FR-28** — `lesearch sessions export <id> --format markdown|jsonl|cbor` MUST be implemented.

### 4.7 A2A interop

- **FR-29** — Every agent MUST have an auto-generated A2A agent card at `/.well-known/agent-card.json?agent={id}`.
- **FR-30** — External A2A clients authenticated with a bearer token MUST be able to send tasks via `POST /message/send` and receive SSE updates.
- **FR-31** — The `generic-a2a` provider MUST be able to consume any external A2A agent and treat it as a local provider for dispatch targeting.

### 4.8 Policy engine (AVM) (Phase B in full; hooks in Phase A)

- **FR-32** — The daemon MUST call the AVM sidecar (UDS + MessagePack) before every tool call; AVM returns `allow | deny | audit-only`.
- **FR-33** — AVM policy MUST be editable via YAML, reloadable without daemon restart (`lesearch policy reload`).
- **FR-34** — Every AVM decision MUST be emitted as a signed CloudEvent into the session log.
- **FR-35** — Default baseline policy MUST block: `rm -rf /`, `rm -rf ~`, `curl | sh`, `wget | sh`, writes to `/etc`, access to `~/.ssh/**` (unless allow-listed).

### 4.9 Configuration

- **FR-36** — All config MUST live in `~/.lesearch/config.toml` (per-user) and `$LESEARCH_HOME/config.toml` (per-install).
- **FR-37** — No config values MUST be required for the "Hello world" path — `lesearch run` MUST work with zero config files on first use.
- **FR-38** — Secrets (API keys, Ziti certs) MUST never be stored in config — read from env vars or OS keychain.

## 5. Non-Functional Requirements (NFR)

### 5.1 Resource budget (self-host hygiene — user-flagged critical)

- **NFR-1** — Daemon idle RAM (no agents running) MUST be ≤ 50 MB resident after 60 s stabilization.
- **NFR-2** — Daemon idle CPU MUST be ≤ 0.1% averaged over 60 s.
- **NFR-3** — Per agent idle RAM overhead (agent process excluded) MUST be ≤ 20 MB in the daemon.
- **NFR-4** — Disk footprint for binary + web console + FTS indexes (excluding user session data) MUST be ≤ 500 MB.
- **NFR-5** — Session log files MUST be rotated at a default 100 MB per agent with a config-tunable retention window (default 90 days).
- **NFR-6** — No background task MUST poll more often than every 10 s unless actively servicing a client request.
- **NFR-7** — The daemon MUST ship with a `lesearch doctor` command reporting resource health (RAM/CPU/FDs/disk/open mounts) in < 2 s.
- **NFR-8** — `lesearch uninstall` MUST remove all binaries, config, caches, AgentFS mounts, systemd/launchd unit files, and leave no orphan processes.

### 5.2 Performance

- **NFR-9** — Agent spawn time MUST be ≤ 2 s p99 from `lesearch run` to first output byte.
- **NFR-10** — Local WebSocket round-trip MUST be ≤ 5 ms p99 over loopback.
- **NFR-11** — Terminal input-to-echo latency MUST be ≤ 10 ms p99 over loopback, ≤ 150 ms p99 over Ziti on LTE.
- **NFR-12** — Session search p99 MUST be ≤ 100 ms over 1 GB history for substring; ≤ 500 ms for jsongrep path queries.
- **NFR-13** — Tauri desktop cold start MUST be ≤ 3 s on 2023+ Apple Silicon.

### 5.3 Reliability

- **NFR-14** — Graceful shutdown on SIGTERM MUST cleanup all child processes within 10 s.
- **NFR-15** — On crash, last-known agent state MUST be recoverable from session log + agent registry on next start.
- **NFR-16** — Concurrent writes to session logs MUST NOT corrupt files (fcntl locking).
- **NFR-17** — Daemon MUST survive ≥ 30 days of continuous uptime without restart in a dogfood environment.
- **NFR-18** — On daemon restart, previously running agents MUST either be reattached (if supervised by tmux mode) OR cleanly marked as `terminated` in the registry.

### 5.4 Security

- **NFR-19** — Default bind MUST be `127.0.0.1` only.
- **NFR-20** — No outbound network call MUST be initiated without an explicit user action (no phone-home, no telemetry by default).
- **NFR-21** — All audit events MUST be Ed25519-signed.
- **NFR-22** — Secrets (API keys, bearer tokens) MUST never be written to logs, stdout, or session events.
- **NFR-23** — Binaries MUST be reproducibly built and signed (macOS codesign + notary; Linux: sigstore signing).
- **NFR-24** — Dependency supply chain: CI MUST run `cargo audit` and block on known CVEs ≥ high severity.
- **NFR-25** — TLS / Noise crypto MUST be constant-time primitives from audited libraries (ring, libsodium, CryptoKit).

### 5.5 Privacy

- **NFR-26** — Crash reporting MUST be opt-in.
- **NFR-27** — Telemetry (if ever added) MUST be opt-in and source-visible.
- **NFR-28** — Session data MUST never leave the host unless the user explicitly shares via `lesearch sessions export` or a paired peer.
- **NFR-29** — No auto-update MUST happen without user consent; the daemon MUST NOT contact any update server by default.

### 5.6 Usability

- **NFR-30** — First-run experience: `brew install lesearch && lesearch run --provider claude "hello"` MUST produce agent output within 60 s on a fresh machine with Claude CLI already installed.
- **NFR-31** — Every CLI command MUST have `--help` with examples.
- **NFR-32** — Every error message MUST include a stable error code and a pointer to a doc URL.
- **NFR-33** — Every permission prompt MUST clearly state: who is asking, what they want to do, what happens if denied.

### 5.7 Maintainability & Operability

- **NFR-34** — Code coverage MUST be ≥ 70% on daemon core crates.
- **NFR-35** — Protocol schema changes MUST be reviewed and land with a CHANGELOG entry.
- **NFR-36** — Release cycle MUST be traceable to git tags with signed release artifacts.
- **NFR-37** — Logs MUST be structured (JSON) at `info` level by default; `trace` available for debugging.
- **NFR-38** — An ADR (Architecture Decision Record) MUST be written for each significant decision and stored in `docs/adrs/`.

## 6. Constraints

### 6.1 Technical constraints

- **C-1** — Language for daemon core: Rust (user-locked).
- **C-2** — Phase A UI: Tauri + web (user-locked).
- **C-3** — Phase B native apps: Swift / SwiftUI on macOS + iOS (user-locked).
- **C-4** — License: Apache-2.0 (user-locked).
- **C-5** — No dependency may be AGPL or GPLv3 (incompatible with Apache-2.0 downstream friendliness).
- **C-6** — Target OSes v1: macOS 13+ (Apple Silicon + Intel), Linux (x86_64 + aarch64, glibc ≥ 2.31). Windows: Phase B.
- **C-7** — Minimum Rust MSRV: 1.85 (per user CLAUDE.md).

### 6.2 Operational constraints

- **C-8** — The product is self-hosted. There is no LeSearch cloud.
- **C-9** — The user owns their data end to end. Any telemetry or crash report must be opt-in, source-visible, and signed.
- **C-10** — Updates are opt-in; auto-update is never enabled by default.

### 6.3 Ecosystem constraints

- **C-11** — MCP (Anthropic) and ACP (Zed) specs are external and may evolve; we pin versions and maintain a compat matrix.
- **C-12** — A2A spec is at v0.2.x at time of writing; we commit to tracking but not blocking on its evolution.
- **C-13** — Upstream CLI agents (Claude Code, Codex) may change their stdio protocols; provider adapters absorb these with thin wrappers.

### 6.4 Team constraints

- **C-14** — Solo maintainer at kickoff. Scope must reflect one-person throughput.
- **C-15** — No external funding assumed. All infrastructure runs on the maintainer's hardware.
- **C-16** — Phase A has no deadline pressure; Phase B kicks off only after dogfood exit criteria met.

## 7. Success Metrics & KPIs

### 7.1 Phase A (dogfood)

| Metric | Target |
|---|---|
| M-A-1 Author daily-usage ratio (lesearch / direct CLI) | ≥ 80% for 2 consecutive weeks |
| M-A-2 Agents spawned via lesearch per week | ≥ 50 |
| M-A-3 Session search queries per week | ≥ 10 |
| M-A-4 Daemon uptime between restarts | ≥ 7 days median |
| M-A-5 Memory footprint stability | ≤ 10% RSS growth over 7 days idle |
| M-A-6 GitHub stars at 30 days post-launch | ≥ 100 |
| M-A-7 External contributor provider adapters | ≥ 1 within 60 days |

### 7.2 Phase B

| Metric | Target |
|---|---|
| M-B-1 Author remote-from-phone uses per week | ≥ 5 |
| M-B-2 Native iOS app crash-free sessions | ≥ 99% |
| M-B-3 Native macOS app daemon uptime | ≥ 30 days |
| M-B-4 First external enterprise / security team evaluation | ≥ 1 within 6 months of Phase B launch |
| M-B-5 Audit trail verification failures | 0 in production use |

## 8. Release Criteria

### 8.1 Phase A v0.1.0 "Daily Driver" release criteria

All MUST be met:

- [ ] All FR-1 through FR-31 pass integration tests
- [ ] NFR-1 through NFR-13 pass benchmarks
- [ ] NFR-14 through NFR-18 pass chaos tests
- [ ] NFR-19 through NFR-33 pass security review
- [ ] Documentation complete: README, QUICKSTART, ARCHITECTURE, SECURITY, CONTRIBUTING
- [ ] CI green on macOS + Linux for 14 consecutive days
- [ ] Dogfood exit criteria met (§ Phase A target M-A-1)
- [ ] Signed release artifacts published on GitHub

### 8.2 Phase B v1.0.0 "Native + Remote" release criteria

All MUST be met:

- [ ] All Phase A criteria still hold
- [ ] FR-32 through FR-38 (AVM full integration) pass
- [ ] Native macOS app notarized and published
- [ ] Native iOS app on TestFlight + App Store
- [ ] Ziti remote pairing E2E test passes on LTE
- [ ] External penetration test on transport + A2A surface passes
- [ ] Audit log verifiability documented and CLI-verifiable

## 9. Open Questions (for team review to resolve)

- Q-1: Should AgentFS be a hard dependency or a pluggable storage backend? (Risk: maturity vs. elegance)
- Q-2: Should the daemon embed an HTTP server (axum) for A2A or use a separate sidecar? (Memory vs. complexity)
- Q-3: Should session search use SQLite FTS5 + jsongrep as a combined index, or keep them separate layers? (Simplicity vs. performance)
- Q-4: Should Phase A include AVM hooks (no-op) so Phase B can flip to enforced mode, or leave AVM for Phase B entirely? (Risk of architectural retrofit)
- Q-5: For macOS mount points, do we use AgentFS's NFS mode or contribute FUSE support? (Operational UX)
- Q-6: Should `lesearch dispatch` use A2A internally for local dispatch too (uniformity) or a faster WebSocket path for same-host? (Latency vs. consistency)
- Q-7: Key management — do we integrate with OS keychain (Keychain/Secret Service), or keep keys in `$LESEARCH_HOME/.keys/` per OpenFused?
- Q-8: Multi-user v1 — single user per daemon or do we already design for multi-identity on one daemon?

## 10. Assumptions

- A-1: User CLI agents (Claude Code, Codex, OpenCode, Gemini) remain stdio-driven and MCP/ACP-capable.
- A-2: OpenZiti iOS SDK is production-ready by Phase B.
- A-3: AgentFS Rust SDK reaches production stability within Phase A window; fallback plan (§ Q-1) mitigates otherwise.
- A-4: Apple's CGVirtualDisplay private API remains functional on macOS 14/15.
- A-5: Apache-2.0 remains compatible with all listed deps (verified as of 2026-04-14).
- A-6: A2A spec remains file-compatible with OpenFused's implementation for the interop goal.

## 11. Dependencies (external)

| Dep | License | Purpose | Risk tier |
|---|---|---|---|
| `tokio`, `axum`, `serde`, `serde_json` | MIT/Apache-2.0 | Rust core | Low |
| `node-pty` / `portable-pty` | MIT | PTY | Low |
| `alacritty_terminal` | Apache-2.0 | Terminal emulation | Low |
| `ring`, `libsodium` | ISC / MIT | Crypto | Low |
| OpenZiti Rust SDK | Apache-2.0 | Zero-trust overlay | Medium (evolving) |
| AgentFS Rust SDK | MIT | Storage | **High** (new project) |
| jsongrep crate | MIT | Session search | Low |
| OpenFused patterns (Rust crate optional) | MIT | Context mesh | Medium |
| Tauri 2 | Apache-2.0/MIT | Desktop shell | Low |
| MCP Rust SDK (`rmcp`) | Apache-2.0 | Provider protocol | Low |
| ACP Rust SDK | Apache-2.0 | Provider protocol | Low |

## 12. Risks

See `PLAN` §6 for the plan-level risk register. Product-level additions:

- **PR-1** — Solo-maintainer bottleneck.
- **PR-2** — Scope creep from the rich feature-matrix (mitigated by dogfood gate).
- **PR-3** — Mismatch between "open & self-hosted" ethos and any future commercial side-offering (mitigated: keep all core code Apache-2.0; paid offerings are separate repos/services).

## 13. Appendix

- Reference implementations studied: paseo (getpaseo/paseo), OpenFused (openfused/openfused), AgentFS (tursodatabase/agentfs), jsongrep (micahkepe/jsongrep), OpenZiti (openziti/ziti), zrok (openziti/zrok).
- Relevant prior work inside this repo family: arya-cmux (CGVirtualDisplay + RoyalVNCKit + Rust AVM), karna (13 crates, security primitives), CloudAGI (x402 payments).

## 14. Revision History

| Rev | Date | Notes |
|---|---|---|
| 0.1 | 2026-04-14 | Initial draft for multi-agent review. |

