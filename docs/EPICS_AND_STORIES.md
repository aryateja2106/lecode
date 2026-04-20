# LeSearch — Epics & User Stories

**Version**: 0.1 (draft)
**Date**: 2026-04-14
**Status**: Backlog-ready; will be imported to Linear after team review.

Format: `As a [role], I want [capability], so that [outcome].` Acceptance criteria are BDD-style.

---

## E1 — Installation & Onboarding

### S-1.1 — One-command install
**As a** solo developer, **I want to** install LeSearch with one command, **so that** I can try it immediately without reading setup docs.

**Acceptance**
- GIVEN a macOS or Linux machine with Homebrew (or cargo) installed
- WHEN I run `brew install lesearch` (or `cargo install lesearch`)
- THEN a single `lesearch` binary is on my PATH within 60 seconds
- AND `lesearch --help` prints usage

### S-1.2 — First-run guided setup
**As a** new user, **I want** `lesearch` to guide me through first-run choices (transport mode, agent providers), **so that** I don't have to read config docs upfront.

**Acceptance**
- WHEN I run `lesearch daemon start` for the first time
- THEN I'm prompted to choose: `direct`, `ziti`, or `noise-ws` transport (default: direct)
- AND asked which providers to detect (scans PATH for `claude`, `codex`, `opencode`, `gemini`)
- AND a `~/.lesearch/config.toml` is created with explicit choices recorded

### S-1.3 — Clean uninstall
**As a** careful user, **I want to** uninstall LeSearch with one command, **so that** I have confidence it leaves no trace.

**Acceptance**
- WHEN I run `lesearch uninstall`
- THEN all daemon processes are stopped
- AND all AgentFS mounts are unmounted
- AND all LaunchAgents / systemd units are removed
- AND config, caches, session data, AgentFS files are deleted (with `--keep-sessions` opt-out)
- AND the binary removes itself last

### S-1.4 — Visible permission requests
**As a** security-conscious user, **I want** LeSearch to ask explicitly before requesting OS permissions (Full Disk Access, NFS mount, Accessibility), **so that** I understand what I'm granting.

**Acceptance**
- BEFORE any permission prompt appears
- THEN a LeSearch-native dialog explains: which permission, why, what happens if denied
- AND I can deny and still use the degraded feature set

---

## E2 — Spawn & Control Agents

### S-2.1 — Spawn any CLI agent
**As a** developer, **I want to** spawn Claude Code, Codex, OpenCode, or Gemini CLI through a single command, **so that** I don't context-switch between tools.

**Acceptance**
- GIVEN provider is installed on the host
- WHEN I run `lesearch run --provider claude "hello"`
- THEN an agent starts, emits output streamed to stdout
- AND an agent ID is printed for later reference

### S-2.2 — List active and recent agents
**As a** multi-project developer, **I want to** see my agents at a glance, **so that** I know what's running where.

**Acceptance**
- WHEN I run `lesearch ls`
- THEN a table shows: ID, provider, cwd (truncated), status, started, runtime
- AND `lesearch ls -a` adds recently-finished agents
- AND `lesearch ls --project <dir>` filters to agents in that cwd

### S-2.3 — Attach to live output
**As a** developer, **I want to** attach to a running agent's output stream, **so that** I can observe progress.

**Acceptance**
- WHEN I run `lesearch attach <id>`
- THEN I see a replay of the session buffer, then live output
- AND `Ctrl-c` detaches cleanly without killing the agent

### S-2.4 — Send follow-up prompt
**As a** developer, **I want to** send an additional message to a running or idle agent, **so that** I can steer it iteratively.

**Acceptance**
- WHEN I run `lesearch send <id> "also add tests"`
- THEN the message reaches the agent on its next turn boundary
- AND the new prompt appears in the session log

### S-2.5 — Stop cleanly
**As a** developer, **I want to** stop an agent and know no zombie processes remain, **so that** my machine stays tidy.

**Acceptance**
- WHEN I run `lesearch stop <id>`
- THEN the agent receives SIGTERM with 5s grace, then SIGKILL
- AND within 6s the agent appears as `status=stopped` in `lesearch ls`
- AND no child processes remain (`pgrep -P <pid>` is empty)

### S-2.6 — Concurrent agent ceiling
**As a** self-hoster, **I want to** limit how many agents run concurrently, **so that** my machine doesn't thrash.

**Acceptance**
- GIVEN `max_concurrent_agents = 3` in config
- WHEN I try to spawn a 4th agent
- THEN the command errors with a clear message + suggestion (stop one or raise the cap)

---

## E3 — Persistent Sessions

### S-3.1 — Session log auto-capture
**As a** developer, **I want** every agent session logged automatically, **so that** I can review anything later without opting in.

**Acceptance**
- WHEN any agent starts
- THEN a JSONL file is created at `$LESEARCH_HOME/agents/{id}/sessions/{session-id}.jsonl`
- AND every event (stream chunk, tool call, permission decision, exit) is appended as one Ed25519-signed CloudEvent line
- AND the file is never mutated after writing (append-only)

### S-3.2 — Search across all sessions
**As a** power user, **I want to** search across all past sessions for specific events or text, **so that** I can recall what an agent did.

**Acceptance**
- WHEN I run `lesearch sessions search "**.tool_call.name == \"Bash\""`
- THEN jsongrep DFA matches over all session JSONL return matching events with path + timestamp
- AND substring-style search `lesearch sessions search --grep "TODO"` uses FTS5 for < 100ms over 1 GB

### S-3.3 — Verify session integrity
**As a** compliance-conscious user, **I want to** verify a session file wasn't tampered with, **so that** I can rely on it as audit evidence.

**Acceptance**
- WHEN I run `lesearch sessions verify <id>`
- THEN every Ed25519 signature is checked against the daemon's recorded public key
- AND the command exits 0 iff all events are signed by the expected key and the log is append-only
- AND tampered files report the first tampered line number

### S-3.4 — Replay a session
**As a** developer, **I want to** replay a prior session, **so that** I can continue work with the same context without manually reconstructing.

**Acceptance**
- WHEN I run `lesearch sessions replay <id>`
- THEN a new agent is spawned with the original provider, prompt, cwd, environment, and last-known state
- AND the new agent receives the original system prompt PLUS a "replay context" message

### S-3.5 — Export a session
**As a** content creator, **I want to** export a session as markdown, **so that** I can turn it into a blog post or review.

**Acceptance**
- WHEN I run `lesearch sessions export <id> --format markdown`
- THEN a `.md` file is produced with event stream, tool calls formatted as code blocks, timestamps
- AND `--format jsonl` and `--format cbor` also work

### S-3.6 — Session retention and rotation
**As a** self-hoster, **I want** old sessions to be archived/pruned automatically, **so that** my disk doesn't fill up.

**Acceptance**
- GIVEN config `retention_days = 90`
- WHEN a session is older than 90 days
- THEN it's compressed to `.jsonl.zst` (retaining verifiability)
- AND after `retention_days + 180` it's deleted (configurable)

---

## E4 — Multi-Device Control

### S-4.1 — Pair a phone via QR code
**As a** developer, **I want to** pair my iPhone with my local daemon by scanning a QR code, **so that** I don't need to type secrets.

**Acceptance**
- WHEN I run `lesearch daemon pair` on the daemon host
- THEN a QR code appears (scannable in terminal as ANSI or browser)
- AND scanning from the iOS app enrolls the phone's identity in the daemon keyring
- AND the first reachability test from the phone succeeds

### S-4.2 — Phone sees agents in real time
**As a** mobile user, **I want** the iOS app to show my agents' live timelines, **so that** I can monitor work while away from my desk.

**Acceptance**
- WHEN an agent on the paired daemon updates its timeline
- THEN the iOS app receives the update within 500 ms p99 on good LTE
- AND the UI renders the new events without requiring refresh

### S-4.3 — Approve a tool permission from phone
**As a** mobile user, **I want** to get a push notification when an agent needs permission for a risky tool call, and approve/deny inline, **so that** I don't block agent progress while away from my desk.

**Acceptance**
- GIVEN an agent requests permission for a tool requiring consent
- WHEN the iOS app is backgrounded
- THEN a push notification arrives within 10 s
- AND I can tap `Approve` / `Deny` from the notification without opening the app
- AND the daemon receives the decision and resumes

### S-4.4 — Dispatch from phone
**As a** mobile user, **I want** to dispatch a task from my phone to any paired daemon, **so that** I can assign work from anywhere.

**Acceptance**
- WHEN I enter "write me a script to…" in the iOS composer
- AND select `claude-on-macbook` as target
- THEN a new agent is spawned on that daemon with my prompt
- AND I see streaming output in the iOS app

### S-4.5 — Voice dictation
**As a** mobile user, **I want** to dictate follow-up prompts with my voice, **so that** I can use the app hands-free.

**Acceptance**
- WHEN I tap-and-hold the mic in the composer
- THEN speech is transcribed locally (Apple Speech framework on iOS) or optionally via a server model
- AND the transcript is editable before sending

---

## E5 — Zero-Trust Transport

### S-5.1 — Transport selectable in config
**As a** security-conscious user, **I want** to pick my transport mode (direct / Ziti / Noise-WS), **so that** I match my threat model.

**Acceptance**
- GIVEN `transport = "ziti"` in config
- WHEN the daemon starts
- THEN it does NOT bind on any TCP/UDP port
- AND only Ziti-enrolled clients can reach it
- AND `nmap` against the host shows zero relevant listening ports

### S-5.2 — Self-host Ziti controller
**As a** self-hoster, **I want** clear docs for running my own Ziti controller in Docker, **so that** I don't rely on a third-party controller.

**Acceptance**
- GIVEN `docs/ZITI_SETUP.md`
- WHEN I follow the steps
- THEN a Ziti controller runs locally in Docker within 15 minutes
- AND `lesearch daemon pair --issuer http://localhost:1280` produces a working enrollment JWT

### S-5.3 — Noise-WS fallback for zero-config
**As a** newcomer, **I want** a fallback transport that doesn't require standing up Ziti, **so that** I can get remote control working in 5 minutes.

**Acceptance**
- GIVEN `transport = "noise-ws"` selected
- WHEN I run `lesearch daemon expose` (with explicit confirmation)
- THEN a WebSocket endpoint is exposed through a configurable remote relay (paseo's, zrok's, or user's)
- AND the relay sees only ciphertext
- AND CLI clearly states: "Noise-WS is defense-in-depth, not zero-trust; prefer Ziti for production"

---

## E6 — Policy & Safety (AVM)

### S-6.1 — Baseline policy blocks dangerous commands
**As a** user, **I want** a sensible default policy blocking obvious footguns, **so that** an agent can't destroy my system on day one.

**Acceptance**
- GIVEN default policy loaded
- WHEN an agent attempts `rm -rf /`, `curl … | sh`, or write to `/etc/*`
- THEN the AVM returns `deny`
- AND the decision is logged as a signed event
- AND the agent sees an error explaining the deny + how to allowlist

### S-6.2 — Edit policy without restart
**As a** power user, **I want** to edit `policy.yaml` and reload without restarting the daemon, **so that** I iterate fast.

**Acceptance**
- WHEN I edit `~/.lesearch/policy.yaml`
- AND run `lesearch policy reload`
- THEN future tool calls use the new policy
- AND in-flight tool calls continue with the old policy (no mid-call swap)

### S-6.3 — Audit trail verifiable
**As a** auditor, **I want** to verify that the recorded policy decisions were signed by the daemon, **so that** I can trust them in compliance review.

**Acceptance**
- WHEN I run `lesearch audit verify <file.jsonl>`
- THEN every entry's Ed25519 signature is checked
- AND mismatch is reported with line numbers
- AND exit code reflects pass/fail

### S-6.4 — Per-agent policy
**As a** power user, **I want** different agents to run under different policies, **so that** a sandboxed task agent has tighter limits than a personal assistant.

**Acceptance**
- WHEN I spawn `lesearch run --provider claude --policy restricted "…"`
- THEN the `restricted` policy is loaded for this agent
- AND tool calls are evaluated against that policy

---

## E7 — Storage & Isolation (AgentFS)

### S-7.1 — Per-agent filesystem namespace
**As a** security-conscious user, **I want** each agent to only see its own files, **so that** a compromised agent cannot exfiltrate cross-agent data.

**Acceptance**
- WHEN two agents A and B are spawned
- THEN agent A's working directory is `/Users/$USER/lesearch/agents/A/fs/`
- AND `ls /Users/$USER/lesearch/agents/B/fs/` from inside A returns permission denied
- AND verified via integration test

### S-7.2 — Shared workspace
**As a** multi-agent orchestrator, **I want** agents to share a workspace when I opt in, **so that** they can collaborate on a single codebase.

**Acceptance**
- WHEN I run `lesearch workspace create my-project --agents claude-1,codex-2`
- THEN both agents mount the same AgentFS namespace
- AND writes from one are visible to the other within 100 ms
- AND the workspace auto-closes when the last agent exits (configurable)

### S-7.3 — Storage footprint visible
**As a** self-hoster, **I want** to see how much disk each agent is using, **so that** I can manage disk pressure.

**Acceptance**
- WHEN I run `lesearch storage status`
- THEN per-agent and shared-workspace disk usage are reported
- AND session log size is separated from AgentFS size
- AND pruning candidates (old sessions, finished agents) are listed

---

## E8 — A2A Interop

### S-8.1 — External A2A client consumes our agent
**As a** user of Google's A2A-compatible tools, **I want** to drive my local lesearch agent from those tools, **so that** I don't have to maintain two agent setups.

**Acceptance**
- GIVEN daemon running with A2A enabled + bearer token configured
- WHEN an external A2A client hits `/.well-known/agent-card.json` and `POST /message/send`
- THEN a task is created, SSE progress streams back, and the final result is available at `/tasks/{id}`

### S-8.2 — Import an external A2A agent as a provider
**As a** orchestrator, **I want** to dispatch tasks from lesearch to an external A2A agent, **so that** my dispatch layer reaches beyond my local CLI agents.

**Acceptance**
- WHEN I add an external A2A endpoint via `lesearch provider add a2a https://peer.example/agent-card.json`
- THEN the external agent appears as a provider in `lesearch providers ls`
- AND `lesearch run --provider peer.example "…"` dispatches via A2A

### S-8.3 — OpenFused interop
**As a** OpenFused user, **I want** to send signed messages to my lesearch agent from `openfuse send`, **so that** the two ecosystems overlap.

**Acceptance**
- GIVEN OpenFused CLI installed, lesearch daemon running with inbox enabled
- WHEN I run `openfuse send my-lesearch-agent "…"`
- THEN the message lands in the lesearch agent's inbox
- AND is tagged `[VERIFIED]` if the signing key is in the daemon keyring

---

## E9 — Native macOS & iOS (Phase B)

### S-9.1 — macOS app ships the daemon
**As a** casual macOS user, **I want** a one-click installer for a macOS app that runs the daemon in the background, **so that** I don't need to use the CLI.

**Acceptance**
- WHEN I install and open `LeSearchMac.app`
- THEN the daemon starts as a LaunchAgent signed with the app's Team ID
- AND the menu bar shows agent status
- AND closing the UI doesn't stop the daemon (keeps running in background)

### S-9.2 — Per-agent virtual display (macOS host)
**As a** power user on macOS, **I want** each GUI-capable agent to run in its own virtual display, **so that** I can observe multiple agents doing visual work simultaneously.

**Acceptance**
- GIVEN macOS 13+, app granted necessary permissions
- WHEN I spawn an agent with `--visual`
- THEN a virtual monitor is created via CGVirtualDisplay
- AND I can view it via the embedded VNC panel in the app
- AND pointer events are grounded with `[POINT:x,y]` CLI overlays

### S-9.3 — iOS background daemon awareness
**As a** iOS user, **I want** the app to receive push notifications when an agent needs attention, **so that** I don't miss important events.

**Acceptance**
- GIVEN app installed, push permission granted
- WHEN an agent requires permission or completes
- THEN a silent push wakes the app (or user push if permitted)
- AND an actionable notification appears

---

## E10 — Remote Desktop (Phase B)

### S-10.1 — HTML5 VNC for Linux hosts
**As a** Linux host user, **I want** to see a GUI agent's display from any browser, **so that** I don't need a VNC client.

**Acceptance**
- GIVEN KasmVNC running on the Linux host, wrapped as a Ziti service
- WHEN I open the web console or iOS app
- THEN the agent's display streams in a browser-native view (no extra client install)

---

## E11 — Observability

### S-11.1 — OpenTelemetry traces by default
**As a** operator, **I want** the daemon to emit OTEL spans to a local collector, **so that** I can diagnose issues.

**Acceptance**
- GIVEN `otel.endpoint = "http://localhost:4318"` in config
- WHEN any agent call travels through the stack
- THEN spans for `agent.create`, `tool.call`, `policy.decision`, `session.write` are emitted
- AND context propagates across provider and AVM sidecar

### S-11.2 — agents-observe integration
**As a** hook-based observer user, **I want** lesearch to emit the same hook events as Claude Code, **so that** agents-observe dashboard works out of the box.

**Acceptance**
- GIVEN agents-observe server running
- WHEN lesearch spawns an agent
- THEN PreToolUse / PostToolUse / SessionStart / Stop events are POSTed to agents-observe
- AND appear in the dashboard

### S-11.3 — `lesearch doctor`
**As a** self-hoster, **I want** a single diagnostic command, **so that** I can answer "is it healthy?" fast.

**Acceptance**
- WHEN I run `lesearch doctor`
- THEN RAM / CPU / open FDs / mount points / CI status / transport status / disk usage report in < 2 s
- AND problems are highlighted with remediation hints

---

## E12 — Resource Hygiene (user-flagged critical)

### S-12.1 — Memory ceiling
**As a** self-hoster, **I want** the daemon to respect a hard memory ceiling, **so that** it can't eat my entire RAM.

**Acceptance**
- GIVEN `max_daemon_memory_mb = 500` in config
- WHEN daemon usage hits the ceiling
- THEN new agent creation is refused with `resource_exhausted` error
- AND `lesearch doctor` flags the condition
- AND on macOS/Linux cgroups are used where available

### S-12.2 — No background polling when idle
**As a** power user, **I want** the daemon to sleep when no clients are connected, **so that** it doesn't cost CPU.

**Acceptance**
- GIVEN no clients connected for > 60 s
- THEN background timers are suspended or elongated to ≥ 30 s intervals
- AND CPU usage averages ≤ 0.05% over 5 minutes

### S-12.3 — Agents leave no orphans
**As a** careful user, **I want** every agent's subprocess tree to terminate when the agent terminates, **so that** I don't accumulate zombies.

**Acceptance**
- WHEN an agent exits or is killed
- THEN all child processes spawned by the agent (provider CLI + any children) are reaped within 2 s
- AND verified by integration test using process-tree inspection

### S-12.4 — Disk usage transparent
**As a** self-hoster, **I want** to see disk impact live, **so that** I don't wake up to a full disk.

**Acceptance**
- WHEN I run `lesearch storage status`
- THEN total disk used by lesearch is shown with top 10 offenders
- AND a warning fires if total exceeds `storage_warn_gb`

### S-12.5 — No phone-home
**As a** privacy-conscious user, **I want** zero outbound network calls by default, **so that** I can audit easily.

**Acceptance**
- GIVEN fresh install, no user action taken
- WHEN I inspect network traffic for 24 h
- THEN no outbound connection occurs except to targets I explicitly configured (pair peers, A2A endpoints, upstream provider APIs the agents themselves call)

### S-12.6 — Opt-in auto-update
**As a** cautious user, **I want** updates to require my explicit consent, **so that** a compromised update stream can't push code to my machine silently.

**Acceptance**
- GIVEN default install
- WHEN a new release exists on GitHub
- THEN the daemon does NOT auto-upgrade
- AND `lesearch update` prompts me before any download

---

## E13 — Developer Experience

### S-13.1 — Add a provider in ≤ 200 LOC
**As a** OSS contributor, **I want** to add a new agent provider with minimal code, **so that** community-driven integrations are viable.

**Acceptance**
- GIVEN `docs/AGENT_PROVIDER.md` reference
- WHEN I implement the `AgentProvider` trait
- THEN my provider integrates in < 200 lines (80/20) and < 2 hours on a typical CLI

### S-13.2 — SDK for third-party UI clients
**As a** third-party UI builder, **I want** a stable client SDK, **so that** I can build alternate front-ends without reverse-engineering.

**Acceptance**
- GIVEN published Rust + TS SDK
- WHEN I read `docs/CLIENT_SDK.md`
- THEN connecting, subscribing to timelines, and issuing commands takes ≤ 50 lines of code

---

## Prioritization (Phase A MVP scope)

**Must-have for v0.1.0** (dogfood-ready):
E1 (install+uninstall), E2 (spawn+control), E3 (session capture+search+replay), E7 (storage+isolation), E8 (A2A), E11 (observability), E12 (resource hygiene), E13.1 (provider API).

**Deferred to v0.2.0 / Phase B**:
E4 (multi-device), E5 (Ziti+Noise-WS), E6 (full AVM enforcement — Phase A gets hook points only), E9 (native Mac/iOS), E10 (remote desktop), E13.2 (public SDK polish).

---

## Revision History

| Rev | Date | Notes |
|---|---|---|
| 0.1 | 2026-04-14 | Initial backlog for review. |

