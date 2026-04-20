# LeSearch — Multi-Agent Review Synthesis

**Date**: 2026-04-14
**Reviewers**: Codex (gpt-5.4, engineering-heavy) + Gemini (strategy/UX)
**Subject**: PRODUCT_SPEC.md + PRD.md + EPICS_AND_STORIES.md + SYSTEM_DESIGN.md (rev 0.1)
**Artifacts**:
- `.omc/artifacts/ask/codex-*-2026-04-14T22-02-46-561Z.md`
- `.omc/artifacts/ask/gemini-*-2026-04-14T22-00-42-820Z.md`

---

## 1. Agreed Recommendations (both reviewers converge — high confidence, apply now)

### A1 — Drop AgentFS as hard dependency; make storage pluggable
Both agree: AgentFS is Turso BETA, FUSE on macOS is brittle, users hate system extensions. Codex cites PRD's own "High risk" label contradicting "MUST". Gemini calls it "a massive yak shave."
**Apply**: rewrite FR-10 to make AgentFS one of several `StorageBackend` impls. Default v0.1.0 = plain directories + OS permissions. AgentFS becomes opt-in when mature.

### A2 — Collapse to monorepo + 5–7 crates, not 19
Both slam the 6-repo / 19-crate split for a solo maintainer. Codex: "wrong time to optimize repo boundaries and license boundaries." Gemini: "will crush your velocity. See Zed or Roo Cline."
**Apply**: single monorepo `aryateja2106/lesearch` (Cargo workspace). Target 5–7 crates for v0.1.0. Separate repos only for spec, not code.

### A3 — Drop CGVirtualDisplay from v0.1.0 pitch
Both call it a ticking time bomb. Private Apple API, breaks on macOS updates, App Store hostile.
**Apply**: keep as research track (cmux stays salvage branch); NOT a v0.1.0 feature. Remove from PRODUCT_SPEC.md §7 differentiation list.

### A4 — AVM in-process for v0.1.0
Both: premature to split. Codex: "not UDS latency — lifecycle, version skew, debugging overhead." Gemini: "static lib in daemon, decouple when GPL/kernel-hook need materializes."
**Apply**: rewrite FR-32 as "in-process AVM trait, optional sidecar mode in Phase B." Kill separate `lesearch-avm` repo.

### A5 — Drop Ziti + Noise-WS choice from user onboarding
Both: transport selection is awful UX. Gemini: "developers want Tailscale-magic." Codex: loopback-trust is fine as v0.1.0 default, but don't gate users behind Ziti controller setup.
**Apply**: v0.1.0 ships `direct` (loopback only). Ziti arrives in Phase B with "Remote Access" toggle (no spec terminology in UI). Abstract to "Local Only / Remote Access."

### A6 — Phase A = loopback-only, 2 providers max, no remote
Codex: "v0.1.0 as loopback-only, single-host, CLI + basic desktop. Move all remote/mobile/A2A-import requirements OUT of the release gate."
Gemini: agrees scope too big.
**Apply**: redefine v0.1.0 = Claude Code + Codex, local CLI + thin Tauri, loopback only. Kill E4/E5/E6/E8-import from v0.1.0 gate.

### A7 — Replace subjective "80% usage for 2 weeks" with hard engineering gate
Codex proposes: 2 providers supported, 7-day soak without crash, bounded leak budget, cross-version protocol tests, deterministic crash/power-loss recovery.
**Apply**: keep dogfood metric as signal, NOT gate. Add the 5 hard engineering criteria as §8.1 of PRD.

### A8 — Session integrity is currently "tamper-evident in the narrowest sense"
Codex: O_APPEND + per-event Ed25519 doesn't prevent deletion, truncation, reordering, whole-file replacement after key compromise, or non-canonical JSON ambiguity.
**Apply**: add (a) canonical JSON serialization rule (JCS RFC 8785), (b) hash-chain across records (`prev_hash` field), (c) signed segment manifests at rotation boundaries, (d) explicit key-rotation verification protocol. Update SYSTEM_DESIGN.md §4.1.

### A9 — Loopback ≠ trust; harden localhost
Codex: browser tabs, malicious extensions, supply-chain, same-user hostile processes.
**Apply**: require per-session bearer token on every WS + HTTP request, Origin/Host allowlist enforcement (deny by default), optional OS-auth (Touch ID / sudo) for sensitive ops (policy edit, key rotation). Update SYSTEM_DESIGN.md §8.1.

### A10 — A2A spec version is STALE
Codex: PRD says A2A 0.2.x; upstream is 1.0.0 with `A2A-Version` negotiation.
**Apply**: update constraint C-12, pin explicit A2A 1.0.0, design version-negotiation from day 1.

### A11 — Resource-budget NFRs are slogans without measurement plan
Codex: NFR-1 (50MB idle) vs config `max_daemon_memory_mb = 500` — inconsistent.
**Apply**: add NFR-39: "Benchmark harness CI job emits RSS + CPU + FD traces on every PR, fails if p95 over budget." Add profiling toolchain as explicit deps: `samply`, `cargo flamegraph`, `tokio-console`, `heaptrack`, `dhat`, `pprof-rs`, `cargo bloat`. Separate daemon-alone budgets from Tauri-shell budgets.

### A12 — Policy enforcement assumes interceptability some providers won't expose
Codex: opaque CLIs (Codex CLI, some OpenCode paths) don't reliably surface pre-execution tool events.
**Apply**: rewrite FR-32: "Strict enforcement only for providers with structured, interceptable tool events; others audit-only with loud warning." Add per-provider capability manifest field: `enforcement_mode: strict | audit-only`.

---

## 2. Disagreements Between Reviewers (decide consciously)

### D1 — Mobile story in Phase A
- **Gemini**: "Mobile deferral kills the hook. Build a PWA in Phase A served by axum." Argues the pitch DIES without phone access.
- **Codex**: "Defer iOS and native macOS apps from v0.1.0." Argues solo-maintainer scope focus beats feature completeness.
- **Resolution**: Split the difference. v0.1.0 serves a **minimal responsive web UI** from the daemon's axum port (same codebase as the Tauri inner webview, loopback only). No remote mobile access. No native apps. Phase B adds native Swift + remote transport for the actual "phone on LTE" pitch. This preserves the shape of the mobile story without the engineering cost.

### D2 — Dispatch terminology
- **Gemini**: "Dispatch" is reactive clone branding (Warp Dispatch, Claude Dispatch). Proposes "Control Plane" or "Hypervisor."
- **Codex**: doesn't address naming.
- **Resolution**: Gemini is right. Rebrand to **"Open Agent Control Plane"** in copy; keep `lesearch dispatch` as a CLI verb because it IS the action. One-liner becomes: *"The open control plane for your CLI agents — self-hosted, zero-trust, vendor-neutral."*

### D3 — IDE integration posture
- **Gemini**: blindspot; devs won't leave VSCode. Proposes `lesearch run --open-in-vscode`.
- **Codex**: silent.
- **Resolution**: Add one small affordance: `lesearch attach --open-in-$EDITOR` opens the attached terminal in the user's preferred editor (VS Code terminal pane / Zed / iTerm). No full IDE plugin. Updates EPICS_AND_STORIES.md E13.

### D4 — BDD vs JTBD user stories
- **Gemini**: BDD format feels contrived; prefer Jobs-To-Be-Done framing.
- **Codex**: silent.
- **Resolution**: Keep BDD for acceptance criteria (useful for test authors). Prefix each epic with a JTBD one-liner. Best of both.

### D5 — License strategy
- **Gemini**: "Fair-source / dual-license" is the 2026 reality. Apache-2.0 noble but commercial survival hard.
- **Codex**: silent; implicit agreement with Apache-2.0.
- **Resolution**: Keep Apache-2.0 for the core. Commercial offering (if ever) is a separate repo / hosted service. User already answered Gate A explicitly — no revisit.

---

## 3. Realities Both Reviewers Surfaced (new constraints to add to PRD)

1. **APNs = cloud dependency** for iOS push notifications; "no cloud" and "phone gets background approvals" are in tension. Phase B must choose: local-only (no push), or explicit APNs disclosure in privacy docs.
2. **Session replay cannot reliably recreate** opaque CLI auth state, cached context, model versions, shell init files, provider-side memory. Rewrite FR-27 as "replay RE-PROMPTS with original prompt + cwd; does not guarantee bit-identical context."
3. **PID-scoped egress control is hard on macOS** (`pf` is not per-process). Drop from SYSTEM_DESIGN.md §8.3 for v0.1.0.
4. **FUSE/NFS blocked in enterprise** — another reason AgentFS can't be a hard dep.
5. **Localhost web console is a real attack surface** (browser tab exploits, DNS rebinding). Apply A9.
6. **Reproducible builds + notarization + sigstore + Linux packaging = separate project** requiring dedicated release-engineering time. Add NFR-40 for CI release pipeline.
7. **A2A / MCP / CLI providers evolve on different cadences** — compat matrix + test fixtures are ongoing maintenance, not one-off work. Add to operational constraints.
8. **Localhost-daemon debugging IS distributed-systems debugging** once you add terminal + FTS + protocol + web UI + remote. Plan structured logging + trace IDs from day 1.
9. **BYOK exhaustion** (Gemini) — users are tired of managing API keys. Document "we never manage keys" as a feature, not a gap.
10. **IDE-native agents won the context war** (Cursor, Roo, Aider). Positioning should accept this and claim the orthogonal space: "when you aren't in your IDE."

---

## 4. Ruthless v0.1.0 Scope (synthesized from both)

### KEEP
- Rust daemon, tokio + axum, single binary
- Loopback-only transport (`direct`)
- CLI first; thin Tauri desktop second
- **2 providers max**: Claude Code + Codex
- Agent lifecycle: create / list / attach / send / stop
- Session capture: append-only JSONL + Ed25519 signing + hash chain + canonical JSON
- **Simple** session search: SQLite FTS5 substring only (jsongrep comes later)
- `lesearch doctor` — RSS / CPU / FDs / disk / process tree
- `lesearch uninstall` — verifiable clean teardown
- Per-agent directory isolation (plain dirs + OS perms) — NOT AgentFS FUSE
- Responsive web UI served from axum on loopback (no mobile native)
- Basic A2A agent-card endpoint (READ ONLY — no POST /message/send in v0.1.0)
- Monorepo with 5–7 crates

### KILL or DEFER
- iOS + native macOS apps (Phase B)
- Ziti + Noise-WS (Phase B)
- Generic A2A import provider (Phase B)
- OpenFused inbox/outbox compatibility (Phase B)
- AgentFS as hard dep (behind feature flag)
- Full AVM enforcement + separate AVM repo (in-process trait only in v0.1.0)
- Per-agent CGVirtualDisplay (research track only)
- Push approvals, voice dictation, remote desktop
- Full environment/state replay (v0.1.0 = prompt + cwd only)
- jsongrep session search (FTS5 substring only in v0.1.0)
- OTEL-by-default (opt-in feature flag)
- CBOR session export (markdown + jsonl only)
- A2A POST /message/send inbound (read-only card first)
- `lesearch dispatch` to external A2A endpoints
- Six-repo split

### ADD (explicitly flagged missing by reviewers)
- CI benchmark harness that FAILS PR if RSS/CPU regress past budget
- Per-request bearer token + Origin/Host allowlist on localhost
- Canonical JSON (JCS / RFC 8785) for session events
- Hash-chain across session records
- Explicit A2A 1.0.0 compliance with version negotiation
- "Life of a Command" section in SYSTEM_DESIGN.md (Gemini)
- Benchmark harness crate + profiling toolchain as dev-deps
- Structured logs + trace IDs from day 1

---

## 5. Positioning Changes

### Old
> "Claude Dispatch for any CLI agent — open, self-hosted, zero-trust, agent-FS-sandboxed, session-searchable, A2A-interoperable."

### New (synthesized from Gemini's rebrand)
> "**LeSearch — the open control plane for your CLI agents.**
> Self-hosted. Vendor-neutral. Works with Claude Code, Codex, OpenCode, and anything with a CLI. Search every session you've ever run. Eventually: control from your phone over a zero-trust overlay."

Reasons:
- "Control plane" positions us as orthogonal to IDE agents, not competing
- Drops "Claude Dispatch" clone framing
- Keeps the expansion path (zero-trust, mobile) as future promise
- Fronts session search as a day-1 feature
- "Vendor-neutral" answers BYOK exhaustion (Gemini's market-reality note)

---

## 6. Research Before Coding (merged list)

| Topic | Source | Purpose |
|---|---|---|
| A2A 1.0.0 spec + versioning | a2a-protocol.org/latest/specification/ | Pin correct version from day 1 |
| MCP spec + versioning | modelcontextprotocol.io/specification/ | Provider adapters |
| Apple App Review constraints | developer.apple.com/app-store/review/guidelines/ | Phase B planning |
| AgentFS status + mount model | docs.turso.tech/agentfs/introduction | Decide feature-flag timing |
| OpenZiti iOS + SDK docs | openziti.io/docs/reference/developer/sdk/ | Phase B transport |
| JSON Canonicalization Scheme (JCS) | RFC 8785 | Session signing canonicalization |
| CloudEvents core + SQL | cloudevents.io/ | Queryable event log shape |
| Rust profiling toolchain | samply, cargo-flamegraph, tokio-console, heaptrack, dhat, pprof-rs, cargo-bloat | NFR measurement |
| Ollama docs UX | ollama.com/docs | Self-hosted install UX gold standard |
| Coolify (self-host strategy) | coolify.io | OSS self-host UX |
| Tailscale zero-trust framing | tailscale.com | How to explain Ziti without jargon |
| GitButler (Rust + Tauri monorepo) | gitbutler.com | Reference repo structure |
| Roo Cline (multi-provider IDE) | github.com/RooCodeInc/Roo-Code | Multi-provider UX |
| Zed repo structure | github.com/zed-industries/zed | Rust monorepo at scale |

---

## 7. Updated Go/No-Go Gate Before Phase A Code

Replaces rev3 plan §11 with synthesis outcomes:

- [ ] Apache-2.0 committed
- [ ] Single monorepo `aryateja2106/lesearch` reserved (no 6-repo split)
- [ ] Protocol spec v0.1 draft (≥500 words, pins A2A 1.0.0)
- [ ] Canonical JSON + hash-chain spec in SYSTEM_DESIGN.md
- [ ] "Life of a Command" section in SYSTEM_DESIGN.md
- [ ] Per-request bearer token + Origin allowlist spec
- [ ] Storage-backend trait designed (plain dirs default; AgentFS optional)
- [ ] AVM as in-process trait (no separate repo)
- [ ] Benchmark harness plan documented
- [ ] 5–7 crate plan (NOT 19)
- [ ] 2 providers on the v0.1.0 list (Claude Code + Codex)
- [ ] README + LICENSE + NOTICE + SECURITY.md
- [ ] First 5 issues: protocol spec, daemon skeleton, pty crate, storage trait, FTS5 substring search

---

## 8. Changelog

- 2026-04-14 — Synthesis from Codex (gpt-5.4) + Gemini critique on rev-0.1 docs. 12 agreed recommendations, 5 disagreements resolved, ruthless v0.1.0 scope cut, positioning rebrand to "open control plane."

