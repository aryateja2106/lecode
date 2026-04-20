//! `LeSearch` storage layer.
//!
//! Each agent gets a private `AgentFS` namespace mounted at
//! `$LESEARCH_HOME/agents/{id}/fs/`. Session events are appended to
//! `$LESEARCH_HOME/agents/{id}/sessions/{session-id}.jsonl` as Ed25519-signed
//! `CloudEvents`. An `SQLite` FTS5 mirror provides substring search; `jsongrep`
//! runs path-regex queries directly over JSONL.
//!
//! See `docs/STORAGE_MODEL.md` for the full design.

#![doc(html_root_url = "https://docs.rs/lesearch-storage/0.0.1")]

/// Default filesystem layout root under `$HOME`.
pub const DEFAULT_HOME: &str = ".lesearch";
