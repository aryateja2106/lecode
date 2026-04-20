# Storage Model

**Status:** Draft. Updated 2026-04-14.

Companion to `protocol-v0.1.md` and `SYSTEM_DESIGN.md`. Owned by crate `lesearch-storage`.

## Goals

1. **Isolation by construction** — one agent cannot read another agent's files without explicit operator intent
2. **Durable session history** — every event append-only, signed, replayable, tamper-evident
3. **Fast search** — substring p99 ≤ 100ms over 1 GB; path-regex via jsongrep
4. **Portable** — the entire `$LESEARCH_HOME` directory can be copied between machines

## On-Disk Layout

```
$LESEARCH_HOME/                        default: ~/.lesearch/
├── config.toml                        per-install config (bind addr, limits, paths)
├── keyring/
│   ├── daemon.ed25519                 daemon signing key (never leaves disk)
│   ├── daemon.x25519                  daemon encryption key (E2E with clients)
│   └── clients/
│       └── <fingerprint>.pub          paired client pubkeys (trusted set)
├── agents/
│   └── <agent_id>/                    one dir per spawned agent (UUIDv7)
│       ├── profile.toml               provider, created_at, policy_profile
│       ├── card.json                  A2A agent card (auto-generated, served at /.well-known)
│       ├── fs.sqlite                  AgentFS-backed private filesystem namespace
│       ├── mounts/                    OS-level mountpoint (FUSE on Linux, NFS on macOS)
│       └── sessions/
│           ├── <session_id>.jsonl     append-only CloudEvents + Ed25519 signature per line
│           └── <session_id>.fts.db    SQLite FTS5 mirror (derivable from .jsonl)
├── workspaces/
│   └── <workspace_id>/                opt-in shared namespaces (multi-agent collab)
└── index/
    └── sessions.db                    global SQLite index of sessions for list/search
```

## AgentFS Integration (Layer L0)

Each agent receives a private filesystem namespace backed by [tursodatabase/agentfs](https://github.com/tursodatabase/agentfs). The namespace is:

- **SQLite-backed**: durable, portable, crash-safe
- **Mounted per-agent**: FUSE on Linux, NFS on macOS (macOS FUSE requires kext; NFS avoids that)
- **Sandbox by filesystem boundary**: agent's subprocess sees the mountpoint as `/`, cannot reach `$LESEARCH_HOME/agents/<other>/fs.sqlite`

Fallback path (R-9 mitigation): if AgentFS is unavailable or the mount fails (permissions, unsupported OS), the daemon falls back to a directory-backed sandbox (`agents/<id>/fs/`) with path-prefix enforcement in the provider adapter. AgentFS is the default; the fallback is a boolean flag in `config.toml`.

## Session Log Format

Each session is a single JSONL file. One CloudEvents-shaped JSON object per line, followed by a detached Ed25519 signature.

```json
{
  "specversion": "1.0",
  "id": "01HN7XQG8YJZ000000000001",
  "source": "lesearch:agent/9f8e...",
  "type": "com.lesearch.agent.tool_call.v1",
  "datacontenttype": "application/json",
  "time": "2026-04-14T22:18:03.512Z",
  "subject": "session/8b3c...",
  "data": {
    "tool": "bash",
    "arguments": { "command": "ls -la" },
    "decision": "allow",
    "policy_profile": "default"
  }
}
```

- `id` is a ULID (sortable, monotonic per session)
- `source` is the agent's A2A URI
- `type` follows CloudEvents reverse-DNS; versioned (`.v1`, `.v2`, ...)
- Signature is the concatenation of `id || time || sha256(data)` signed with the daemon's Ed25519 key, base64-encoded, appended after a single space

## Event Taxonomy (v1)

| Type | Meaning |
|---|---|
| `com.lesearch.agent.spawn.v1` | agent created |
| `com.lesearch.agent.prompt.v1` | user prompt delivered |
| `com.lesearch.agent.output.v1` | provider text output (chunked) |
| `com.lesearch.agent.tool_call.v1` | tool invocation + AVM decision |
| `com.lesearch.agent.tool_result.v1` | tool result |
| `com.lesearch.agent.status.v1` | state transition |
| `com.lesearch.agent.error.v1` | provider or daemon error |
| `com.lesearch.agent.done.v1` | session closed |
| `com.lesearch.a2a.inbox.v1` | inbound A2A message delivery |
| `com.lesearch.keyring.rotated.v1` | key rotation audit |

New event types are additive. Removing or renaming requires a MAJOR protocol bump.

## Verification

`lesearch sessions verify <session_id>`:

1. Read `.jsonl` line by line
2. For each line: parse JSON, extract signature, verify with daemon's public key
3. Assert `id` ordering (monotonic ULID)
4. Assert no missing lines (check `id` increments)
5. Report: pass / fail + line number of first tamper

## Search

Two query surfaces, both through `lesearch-search` crate:

**Substring** (FTS5):
```
lesearch sessions search "cargo check" --substring
```
Backed by per-session `fts.db`. Builds incrementally on log append. p99 < 100ms over 1 GB.

**Path-regex** (jsongrep):
```
lesearch sessions search '$..data.tool[?(@.command=~"^rm ")]'
```
Runs `jsongrep` directly over `.jsonl`. Compiled DFA; no index needed. Slower than FTS5 for long queries but more expressive.

## Retention

- Default: 90 days per session log, rotated at 100 MB
- Configurable per-agent via `profile.toml` (`retention_days`, `max_log_bytes`)
- `lesearch sessions prune` walks agents and drops logs older than retention; does not remove the session metadata in `index/sessions.db` (metadata kept forever, body eviction visible in `list`)

## Keyring

All signing keys live under `keyring/`. The daemon's Ed25519 key is generated on first run and never leaves disk. Paired client pubkeys accumulate in `keyring/clients/` during `lesearch daemon pair`. Rotation via `lesearch daemon rotate-key` is atomic: new key generated, old key marked in `keyring/archive/`, all existing session signatures remain verifiable.

## Open Questions (deferred)

- Combined vs separate FTS5 + jsongrep indexes (PRD Q-3) — currently separate
- Encrypted-at-rest session logs (v1 ships plaintext; opt-in age-encrypted variant in v0.2)
- Workspace access control (shared namespaces) — v0.1 ships manual opt-in; v0.2 adds ACL
