# Issue: Scaffold lesearch-storage (AgentFS facade + CloudEvents JSONL + Ed25519)

**Labels:** `area/storage` `type/feature` `priority/critical` `phase/A.3`
**Milestone:** v0.1.0 Daily Driver
**Story refs:** S-3.1, S-3.2, S-7.1, S-7.2, S-7.3, S-11.1
**Blocked by:** `02-daemon-scaffold`
**Blocks:** `agent.spawn`, `session.search`, `session.replay`

## Summary

Implement the storage layer per `docs/STORAGE_MODEL.md`: per-agent AgentFS namespace, append-only JSONL CloudEvents session log, Ed25519 signing, SQLite FTS5 mirror for substring search, and the `lesearch sessions verify/list` commands. R-9 fallback (plain SQLite+dir sandbox) is required behind a config flag.

## Acceptance criteria

- [ ] `lesearch-storage::AgentStorage::new(home, agent_id)` creates `$LESEARCH_HOME/agents/{id}/` layout from `STORAGE_MODEL.md`
- [ ] AgentFS SDK integration (tursodatabase/agentfs) — namespace mounted via FUSE (Linux) or NFS (macOS)
- [ ] Fallback: plain `agents/{id}/fs/` directory with path-prefix enforcement when `config.storage.fallback = true`
- [ ] `SessionLog::append(event: CloudEvent)` signs with Ed25519 and appends one JSON line + signature
- [ ] `SessionLog::verify()` reads line-by-line, verifies signatures, asserts ULID monotonicity, reports tamper position
- [ ] FTS5 mirror: `SessionLog::index_incremental()` builds `sessions/{id}.fts.db` on append
- [ ] `lesearch sessions list` shows agents + sessions
- [ ] `lesearch sessions verify <id>` prints pass/fail + first tamper line
- [ ] Isolation test: agent A cannot read agent B's fs.sqlite (AC-A-7)
- [ ] Benchmark: substring search p99 ≤ 100ms over 1 GB of JSONL

## Non-goals

- Vector search (Phase B.7)
- Encrypted-at-rest logs (v0.2 optional)
- Workspace ACLs (v0.2)

## Implementation notes

- Key generation: `ed25519-dalek::SigningKey::generate()` on first daemon start, persisted to `keyring/daemon.ed25519` with `0600` perms
- ULID: use `uuid::v7` (sortable, monotonic-enough) for event ids
- FTS5: `rusqlite` with `features = ["bundled"]` so we don't depend on system sqlite
- JSONL format: one CloudEvent per line followed by ` <base64 signature>\n`

## References

- `docs/STORAGE_MODEL.md` (primary)
- `docs/PRD.md` FR-10 through FR-13, FR-23 through FR-28
- `docs/SYSTEM_DESIGN.md` §2.1, §2.4
- [CloudEvents 1.0 spec](https://github.com/cloudevents/spec)
- [tursodatabase/agentfs](https://github.com/tursodatabase/agentfs)
- [micahkepe/jsongrep](https://github.com/micahkepe/jsongrep)
