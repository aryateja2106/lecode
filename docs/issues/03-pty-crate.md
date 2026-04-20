# Issue: Add lesearch-pty crate (portable-pty + alacritty headless buffer)

**Labels:** `area/pty` `type/feature` `priority/high` `phase/A.1`
**Milestone:** v0.1.0 Daily Driver
**Story refs:** S-2.1, S-2.2, S-2.3
**Blocked by:** `02-daemon-scaffold`
**Blocks:** `agent.spawn` implementation

## Summary

Introduce `crates/lesearch-pty` to own PTY allocation, stream capture, and terminal-state reconstruction. The daemon spawns provider subprocesses (claude, codex, opencode, gemini) through this crate; clients receive live terminal output via binary-mux frames on the existing WebSocket.

## Acceptance criteria

- [ ] New crate `lesearch-pty` with `portable-pty` + `alacritty_terminal` dependencies
- [ ] `PtyHandle::spawn(cmd, args, env, cwd)` returns a handle with `AsyncRead` (stdout/stderr) and `AsyncWrite` (stdin)
- [ ] Headless terminal buffer snapshot: `handle.snapshot()` returns framebuffer state serializable as JSON (rows, cols, cells, cursor)
- [ ] Resize: `handle.resize(rows, cols)` plumbs SIGWINCH to the child
- [ ] Graceful close: 10s SIGTERM grace, then SIGKILL; zero zombies on shutdown
- [ ] fcntl advisory locking on session-log writes (for R-8 durability)
- [ ] Unit tests covering spawn/read/write/resize/close on macOS + Linux
- [ ] Integration test: spawn `/bin/echo hello`, read output, assert "hello\n"

## Non-goals

- Windows ConPTY (deferred; v0.1 is macOS + Linux only per C-6)
- Scrollback beyond 10,000 lines (make configurable, default 10k)
- Reattach-to-existing-PTY (considered for tmux mode in NFR-18)

## Implementation notes

- `portable-pty` for cross-platform PTY allocation
- `alacritty_terminal` for headless framebuffer (battle-tested, used by Warp)
- Wrap child stdin/stdout/stderr as `tokio::io::AsyncRead` / `AsyncWrite`
- Session logging: bridge output bytes → `lesearch-storage` event writer

## References

- `docs/SYSTEM_DESIGN.md` §2.1 (crate layout)
- `docs/PRD.md` FR-1 through FR-5 (agent lifecycle)
- [portable-pty](https://crates.io/crates/portable-pty)
- [alacritty_terminal](https://crates.io/crates/alacritty_terminal)
