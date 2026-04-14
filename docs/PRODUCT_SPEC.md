# LeSearch — Product Spec

**Version**: 0.1 (draft)
**Date**: 2026-04-14
**Owner**: Arya Teja Rudraraju
**Status**: Pre-implementation, for multi-agent review

---

## 1. Product One-Liner

**LeSearch** is an open-source, self-hosted agent control plane. One download, one protocol, your hardware — control any CLI coding agent (Claude Code, Codex, OpenCode, Gemini CLI, custom) from your phone, desktop, or terminal, over a zero-trust overlay.

> "Claude Dispatch for any CLI agent — open, self-hosted, zero-trust, agent-FS-sandboxed, session-searchable, A2A-interoperable."

## 2. The Problem

Developers running AI coding agents today face four trapped-in-silo problems:

1. **Agent silos** — Claude Code, Codex, OpenCode, Gemini CLI each live in separate UIs, terminals, and state directories. Switching is friction. There is no unified control surface.
2. **No secure remote control** — Starting a long-running agent on your desk and checking it from your phone either means (a) SSH + tmux (painful on mobile), (b) closed SaaS (Warp, Cursor Cloud) locking your code in, or (c) exposing your dev machine to the internet.
3. **No session persistence across agents** — Each agent writes logs in its own format in its own directory. There is no single searchable history of "everything my agents did this month."
4. **No policy or isolation** — Agents can read any file, run any command, make any network call. Enterprises and security-minded users have no way to say "this agent can do X but not Y."

Paseo solves #1 and #2 partially (mobile control, multi-provider) but is AGPL-3.0, Node-based, Cloudflare-relay-centric, and has no policy/isolation/interop layer. LeSearch is the Apache-2.0 clean-room answer built on open standards.

## 3. Who It's For

### Primary persona — Solo Power-User Developer (Arya)

- Runs Claude Code + Codex + OpenCode daily across multiple projects
- Owns Mac, iPhone, sometimes Linux server, sometimes Raspberry Pi
- Wants to start an agent at desk, check progress from phone, send follow-up tasks from anywhere
- Has zero patience for cloud-dependent tools that touch his code
- Values: open source, privacy, terminal-native, high leverage

### Secondary persona — Security-Conscious Developer

- Works at a company with strict data-egress rules
- Cannot use cloud-hosted AI IDE tooling
- Needs: agent runs locally, identity-based auth, audit trail, tool-call policy enforcement
- Values: zero-trust model, cryptographic audit, no surprise network calls

### Secondary persona — Multi-Provider Hobbyist

- Experiments across LLM providers weekly
- Wants one UI for all CLI agents they try
- Needs: easy provider install, unified timeline, bring-your-own-API-key

### Tertiary persona — OSS Ecosystem Contributor

- Wants to add a new CLI agent to the platform
- Needs: clear AgentProvider spec, sample implementation, CI harness

### Tertiary persona — Small Team Lead (Phase B)

- Hosts one lesearch instance on a server for a 3–10 person team
- Wants per-user identity, per-agent policy, shared workspaces
- Needs: multi-user support (Phase B), audit export to SIEM

### Out of scope (v1)

- Enterprise SSO / LDAP integration (Phase B+)
- Multi-tenant hosted cloud offering (never — self-hosted is the product)
- Windows host support (Linux + macOS first; Windows daemon: Phase B)

## 4. What It Does (user-facing)

### From the CLI

```bash
# Install once
brew install lesearch          # or cargo install / curl installer
lesearch daemon start

# Spawn any CLI agent
lesearch run --provider claude "summarize this directory"
lesearch run --provider codex --worktree feature-x "add auth middleware"
lesearch run --provider opencode "refactor the billing module"

# Monitor / control
lesearch ls                    # list running + recent agents
lesearch attach <id>           # stream live output
lesearch send <id> "also add tests"
lesearch stop <id>

# Search session history
lesearch sessions search '**.tool_call.name == "Bash"'
lesearch sessions replay <id>  # rehydrate into new agent run

# Dispatch from anywhere
lesearch dispatch "fix this test" --to claude-on-macbook
```

### From the Tauri desktop app (Phase A)

- Agent list + live timelines
- Terminal attach (xterm.js embedded)
- Permission prompts surface as notifications
- Session browser + jsongrep search bar

### From the native iOS app (Phase B)

- QR-code pair with a daemon
- See live agents, approve/deny permission requests
- Voice-dictate follow-up tasks
- Background notifications when an agent completes or asks for permission
- Dispatch a task to any paired daemon

### From the native macOS app (Phase B)

- Menu bar lives with daemon running in background
- Permission sheets for Full Disk Access, Accessibility, NFS mount (all one-time)
- Per-agent virtual display (CGVirtualDisplay) for GUI tasks
- Integration with Raycast, Alfred, Shortcuts for dispatch shortcuts

### From any A2A-speaking tool

- Point any A2A client (OpenAI agents ecosystem, Google A2A tools) at `https://your-daemon/.well-known/agent-card.json`
- Drive your local lesearch agent as if it were a remote OpenAI assistant

## 5. How It Works (from the user's point of view)

1. **Install** a single binary. Run `lesearch daemon start`. The daemon listens on loopback by default.
2. **Spawn** an agent with `lesearch run`. The daemon creates an isolated AgentFS namespace for it, loads the appropriate provider adapter (Claude Code / Codex / etc.), and streams output back.
3. **Pair** a phone by running `lesearch daemon pair` — a QR code appears. Scan it in the iOS app. Your phone now has a Ziti identity bound to this daemon.
4. **Control** from anywhere — your phone on LTE can now reach the daemon through the Ziti overlay, no open ports, no cloud relay required.
5. **Enforce policy** by editing `~/.lesearch/policy.yaml` — the AVM sidecar blocks dangerous tool calls per agent. Every decision is a signed audit event.
6. **Search history** — every session is a signed JSONL file. Query with jsongrep syntax. Replay or export any session.
7. **Interoperate** — every agent exposes an A2A endpoint. Point any A2A client at your daemon and consume your agents as a standard tool.

## 6. Competitive Landscape

| Product | Open | Self-Host | Multi-Agent | Mobile | Zero-Trust | Policy | Session Search | A2A | License |
|---|---|---|---|---|---|---|---|---|---|
| **LeSearch** (this) | ✅ | ✅ | ✅ (all CLI) | ✅ native + web | ✅ Ziti | ✅ AVM | ✅ jsongrep | ✅ native | Apache-2.0 |
| Paseo | ✅ | ✅ | ✅ (3 agents) | ✅ Expo | ⚠️ ECDH relay | ❌ | ⚠️ basic | ❌ | AGPL-3.0 |
| Warp Dispatch | ❌ | ❌ | ⚠️ own only | ✅ | ❌ | ❌ | ⚠️ | ❌ | Proprietary |
| Cursor / Composer | ❌ | ❌ | ⚠️ own only | ⚠️ | ❌ | ❌ | ⚠️ | ❌ | Proprietary |
| Claude Dispatch | ❌ | ❌ | ❌ Claude only | ⚠️ | ❌ | ❌ | ⚠️ | ❌ | Proprietary |
| OpenFused | ✅ | ✅ | n/a (context only) | ❌ | ⚠️ | ❌ | ⚠️ | ✅ | MIT |
| AgentFS | ✅ | ✅ | n/a (storage only) | ❌ | ❌ | ❌ | n/a | ❌ | MIT |
| Tmux + SSH | ✅ | ✅ | any | ⚠️ painful | ❌ | ❌ | ❌ | ❌ | BSD |
| Claude Code + tmux | ✅ | ✅ | any CLI | ⚠️ painful | ❌ | ❌ | ❌ | ❌ | mixed |

## 7. Differentiation

LeSearch is unique because it is the **only** product that combines:

1. **Open-source + self-hosted** (not SaaS)
2. **Apache-2.0** (not AGPL — commercially safe for downstream)
3. **Multi-provider** (any CLI agent)
4. **Zero-trust transport** (OpenZiti identity-based, not shared-secret relay)
5. **Agent-FS-sandboxed** (each agent in its own SQLite-backed filesystem)
6. **Session search built-in** (jsongrep DFA queries over signed JSONL)
7. **A2A facade out of the box** (every lesearch agent is consumable by any A2A client)
8. **Policy engine** (AVM enforces per-agent tool-call allowlists)
9. **Native Mac + iOS end-state** (not Expo — Phase B delivers Swift-native)
10. **Per-agent virtual display on macOS** (CGVirtualDisplay isolation — unique)

None of the existing products combine more than 3 of these. This is the moat.

## 8. Positioning Statement

> For developers who run AI coding agents and demand open, self-hosted, privacy-first infrastructure, **LeSearch** is the open-source agent control plane that combines multi-provider agent orchestration, zero-trust remote access, per-agent filesystem sandboxing, searchable session history, and native A2A interop — without cloud dependencies or vendor lock-in. Unlike Paseo (AGPL, Expo, no policy, no search), Cursor Cloud (closed, SaaS), or raw tmux+SSH, LeSearch is Apache-2.0, speaks the open A2A standard natively, and isolates every agent in its own AgentFS namespace with cryptographically audited tool calls.

## 9. Success Indicators (qualitative)

Phase A (dogfood):
- Arya voluntarily uses `lesearch` for 80% of Claude Code + Codex work for 2 consecutive weeks
- Paseo users + OpenFused users on Discord notice and try the Apache-2.0 alternative
- At least one external contributor adds an agent provider within 60 days of public launch

Phase B (native apps + remote):
- Arya controls agents from phone on LTE for real work at least 5 times per week
- First enterprise / security team tries it as a self-hosted alternative to SaaS dev-AI tools

## 10. Out of Scope (v1)

- Cloud-hosted multi-tenant LeSearch
- Windows daemon (Phase B if demand)
- Proprietary agent providers (closed-source CLI wrapping)
- SSO / SAML / LDAP (Phase B)
- Code editor embed (IDE plugin is a later adapter, not the product)

## 11. Open Positioning Question (needs user decision)

Two candidate public names:

- `lesearch` — short, unique, consistent with LeSearch AI brand
- `agentctl` or `agents` or `mconnect` — generic, searchable

Recommendation: stay with `lesearch` — it's part of the personal brand, and the product rides the LeSearch AI umbrella.

Also TBD: whether to publicly acknowledge the "former LeCoder MConnect" heritage or present LeSearch as a fresh product.

