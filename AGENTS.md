# AGENTS.md

Machine-readable integration reference for AI coding agents talking to lecode.

---

## If you are an agent spawned BY lecode (inside a worktree)

- Your cwd is the worktree path passed in the `SpawnParams.worktree` field (if provided).
- Your stdout is streamed back to the caller as `agent.output` JSON-RPC notifications.
- Your stderr is captured by the daemon but not yet forwarded (planned v0.1).
- You are a real subprocess — act normally. No special environment variables are injected.
- If you exit non-zero, the daemon closes the WebSocket stream for that agent.
- The daemon assigns you a stable `agent_id` (UUIDv7 string) for the session lifetime.

---

## If you are an agent that wants to CALL lecode

Connect to:

```
ws://127.0.0.1:6767/ws
```

Override with the `LESEARCH_ADDR` environment variable:

```bash
LESEARCH_ADDR=ws://192.168.1.10:6767 your-agent-binary
```

Send JSON-RPC 2.0 text frames. Receive JSON-RPC 2.0 text frames back.

### Protocol version

`0.1.0` — additive-only. New fields will always be optional. No field will ever be removed
or narrowed without a major version bump.

---

## RPC Methods

### `agent.spawn`

Spawn a new agent subprocess and begin streaming its output.

**Request params:**

```json
{
  "provider": "claude" | "codex" | "test",
  "worktree": "/absolute/path/to/worktree"
}
```

`worktree` is optional. `provider` is required.

**Success result:**

```json
{
  "agent_id": "019541ab-...",
  "streams": {
    "stdin":  3,
    "stdout": 4,
    "stderr": 5
  }
}
```

`streams` carries u16 channel IDs reserved for future binary-mux PTY framing. Currently
informational — output arrives as `agent.output` text notifications.

**Error example:**

```json
{
  "jsonrpc": "2.0",
  "id": 1,
  "error": { "code": -32602, "message": "Unknown provider" }
}
```

### `agent.list` _(planned v0.1)_

```json
{ "params": {} }
```

Result will be `{ "agents": [{ "agent_id", "provider", "started_at" }] }`.

### `agent.stop` _(planned v0.1)_

```json
{ "params": { "agent_id": "019541ab-..." } }
```

Result will be `{ "stopped": true }`.

### `server.handshake` _(planned v0.1)_

```json
{ "params": {} }
```

Result will be `{ "version": "0.0.1", "protocol": "0.1.0", "uptime_secs": 42 }`.

---

## Notifications emitted (server → client)

### `agent.output`

Sent for every line written to the agent's stdout.

```json
{
  "jsonrpc": "2.0",
  "method": "agent.output",
  "params": {
    "agent_id": "019541ab-...",
    "data": "line of agent output"
  }
}
```

Note: `stream` field (`"stdout"` | `"stderr"` discrimination) is defined in
`AgentOutputParams` but not yet populated in notifications — added in v0.1.

---

## CLI surface (`lesearch-cli`)

The binary is a flat command (no subcommand tree yet). All subcommands below are planned.

| Command / Flag | Status | Signature | Example |
|---|---|---|---|
| `--provider <name>` | ✅ live | Select provider | `--provider test` |
| `--addr <ws-url>` | ✅ live | Override daemon URL | `--addr ws://127.0.0.1:9999` |
| `--worktree <path>` | ✅ live | Set agent cwd | `--worktree /tmp/feat-auth` |
| `lesearch-cli run` | planned v0.1 | `run --provider <p> [--prompt <s>] [--worktree <path>]` | |
| `lesearch-cli ls` | planned v0.1 | `ls [--json]` | |
| `lesearch-cli stop <id>` | planned v0.1 | `stop <agent-id>` | |
| `lesearch-cli logs <id>` | planned v0.1 | `logs <agent-id> [--follow]` | |
| `lesearch-cli daemon status` | planned v0.1 | `daemon status` | |

**Current usage (v0.0.1):**

```bash
cd ~/Projects/lecode
cargo run -p lesearch-cli -- --provider test --addr ws://127.0.0.1:6767
```

---

## Common workflows for agents

### Verify the full flow

```bash
cd ~/Projects/lecode
cargo test --workspace
# The e2e test starts the daemon in-process, spawns a TestProvider agent, asserts output.
```

### Spawn in an isolated worktree

```bash
cd ~/Projects/lecode
# Provide a worktree so the agent starts in the right directory.
cargo run -p lesearch-cli -- --provider claude --worktree /absolute/path/to/worktree
```

### Orchestration pattern (once agent.list lands in v0.1)

```
# Spawn N agents in isolated worktrees, collect outputs, merge.
agent_1 = agent.spawn({ provider: "claude", worktree: "/worktrees/feat-a" })
agent_2 = agent.spawn({ provider: "codex",  worktree: "/worktrees/feat-b" })
# Stream agent.output notifications for both agent_ids concurrently.
# Read results, merge, call agent.stop for each.
```

---

## TypeScript SDK roadmap

Planned for lecode v0.2: a TypeScript client modeled on
[pi-ai](https://github.com/badlogic/pi-mono/tree/main/packages/ai) — unified agent
interface, streaming events, tool-call validation via TypeBox schemas, automatic context
serialization. The Rust core remains canonical; the TS SDK is a thin typed wrapper over
the WebSocket JSON-RPC protocol documented here.
