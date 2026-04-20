//! `LeSearch` agent provider adapters.
//!
//! Each provider wraps an upstream CLI coding agent via stdio (MCP or ACP),
//! exposing a uniform `AgentProvider` trait to the daemon. New providers
//! ship as registered binaries without modifying daemon core.

#![doc(html_root_url = "https://docs.rs/lesearch-providers/0.0.1")]

pub mod claude;

use std::path::PathBuf;

use futures::future::BoxFuture;
use thiserror::Error;
use tokio::process::Child;

/// Built-in provider identifiers.
pub const BUILTIN_PROVIDERS: &[&str] = &["claude", "codex", "opencode", "gemini", "generic-a2a"];

/// Parameters supplied by the client when requesting a new agent session.
///
/// Mirrors the `agents/spawn` JSON-RPC request body; all fields that the
/// protocol marks as optional are `Option<_>` here.
#[derive(Debug, Clone)]
pub struct SpawnParams {
    /// Human-readable label for the session (e.g. `"my-feature"`).
    pub label: String,
    /// Provider identifier (e.g. `"claude"`, `"codex"`).
    pub provider: String,
    /// Prompt/task text forwarded to the agent on stdin at startup.
    pub prompt: Option<String>,
    /// Filesystem path the agent should treat as its working directory.
    ///
    /// When `Some`, the spawned subprocess's `current_dir` is set to this
    /// path. When `None` the daemon's own `cwd` is inherited.
    pub worktree: Option<String>,
}

/// Fully resolved specification used internally to launch an agent process.
///
/// Built from [`SpawnParams`] after validation and path resolution. Passed to
/// [`AgentProvider::spawn`] by the agent manager.
#[derive(Debug, Clone)]
pub struct AgentSpec {
    /// Human-readable session label.
    pub label: String,
    /// Resolved provider identifier.
    pub provider: String,
    /// Optional startup prompt forwarded to the agent on stdin.
    pub prompt: Option<String>,
    /// Resolved working directory for the agent subprocess.
    ///
    /// `Some(path)` → subprocess is started with `current_dir = path`.
    /// `None` → subprocess inherits the daemon's `cwd`.
    pub worktree: Option<PathBuf>,
}

/// Result of a successful [`AgentProvider::spawn`] call.
#[derive(Debug)]
pub struct SpawnResult {
    /// The live child process handle.
    pub child: Child,
}

/// Errors that can occur within a provider adapter.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// The worktree path supplied in [`AgentSpec`] does not exist.
    #[error("worktree path does not exist: {0}")]
    WorktreeNotFound(PathBuf),

    /// I/O error while spawning the subprocess.
    #[error("failed to spawn agent process: {0}")]
    Spawn(#[from] std::io::Error),

    /// The provider is not supported or not installed.
    #[error("unsupported provider: {0}")]
    Unsupported(String),
}

/// Uniform interface implemented by every provider adapter.
///
/// The daemon calls [`AgentProvider::spawn`] and holds the returned
/// [`SpawnResult`], streaming its stdio to connected WebSocket clients.
///
/// The return type uses [`BoxFuture`] to remain dyn-compatible so the daemon
/// can hold `Box<dyn AgentProvider>` without knowing the concrete type.
pub trait AgentProvider: Send + Sync {
    /// Launch the agent subprocess described by `spec`.
    ///
    /// Implementors must honour `spec.worktree` by calling
    /// [`std::process::Command::current_dir`] when it is `Some`.
    ///
    /// # Errors
    ///
    /// Returns [`ProviderError`] if the subprocess cannot be started.
    fn spawn<'a>(
        &'a self,
        spec: &'a AgentSpec,
    ) -> BoxFuture<'a, Result<SpawnResult, ProviderError>>;

    /// Returns the provider identifier string (e.g. `"claude"`).
    fn provider_id(&self) -> &'static str;
}
