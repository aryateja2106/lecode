//! `LeSearch` agent provider adapters.
//!
//! Each provider wraps an upstream CLI coding agent via stdio (MCP or ACP),
//! exposing a uniform `AgentProvider` trait to the daemon. New providers
//! ship as registered binaries without modifying daemon core.
//!
//! Phase A.0 stub — trait + first adapter (Claude Code) land Day 2.

#![doc(html_root_url = "https://docs.rs/lesearch-providers/0.0.1")]

/// Built-in provider identifiers.
pub const BUILTIN_PROVIDERS: &[&str] = &["claude", "codex", "opencode", "gemini", "generic-a2a"];
