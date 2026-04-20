//! Claude Code provider adapter.
//!
//! Wraps the `claude` CLI binary, launching it as a subprocess with stdio
//! connected to the daemon's streaming pipeline.

use std::path::PathBuf;

use futures::future::BoxFuture;
use tokio::process::{Child, Command};

use crate::{AgentProvider, AgentSpec, ProviderError, SpawnResult};

/// Binary name for the Claude Code CLI.
const CLAUDE_BIN: &str = "claude";

/// Provider adapter for Anthropic's Claude Code agent.
///
/// Spawns `claude` as a subprocess, optionally inside a specified worktree
/// directory. All stdio is kept open for the daemon to stream.
#[derive(Debug, Clone, Default)]
pub struct ClaudeProvider;

impl ClaudeProvider {
    /// Create a new [`ClaudeProvider`].
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

/// Build a [`Command`] for the Claude Code binary from the given [`AgentSpec`].
///
/// This is extracted as a free function so tests can inspect the command
/// configuration without needing to actually spawn a process.
///
/// The command is configured with:
/// - `current_dir` set to `spec.worktree` when `Some`, otherwise the
///   process inherits the daemon's working directory.
/// - `stdin`, `stdout`, `stderr` all set to `piped` so the daemon can
///   stream them over WebSocket.
/// - An optional `--print` flag with the prompt when `spec.prompt` is `Some`.
#[must_use]
pub fn build_command(spec: &AgentSpec) -> Command {
    build_command_with_bin(CLAUDE_BIN, spec)
}

/// Build a [`Command`] using the given binary name instead of `claude`.
///
/// Identical to [`build_command`] but accepts a custom binary path, which
/// makes it possible to test the command configuration without requiring the
/// real `claude` binary on `PATH`.
#[must_use]
pub fn build_command_with_bin(bin: &str, spec: &AgentSpec) -> Command {
    let mut cmd = Command::new(bin);

    // Set the working directory to the worktree when provided.
    if let Some(ref worktree) = spec.worktree {
        cmd.current_dir(worktree);
    }

    // Forward the prompt via `--print` for non-interactive mode.
    if let Some(ref prompt) = spec.prompt {
        cmd.arg("--print").arg(prompt);
    }

    // Keep all stdio open for the daemon to stream.
    cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    cmd
}

/// Spawn the configured command, returning the child handle.
fn do_spawn(cmd: &mut Command) -> Result<Child, ProviderError> {
    let child = cmd.spawn()?;
    Ok(child)
}

impl AgentProvider for ClaudeProvider {
    fn spawn<'a>(
        &'a self,
        spec: &'a AgentSpec,
    ) -> BoxFuture<'a, Result<SpawnResult, ProviderError>> {
        Box::pin(async move {
            // Validate that the worktree exists before attempting to spawn.
            if let Some(ref worktree) = spec.worktree {
                if !worktree.exists() {
                    return Err(ProviderError::WorktreeNotFound(worktree.clone()));
                }
            }

            let mut cmd = build_command(spec);
            let child = do_spawn(&mut cmd)?;
            Ok(SpawnResult { child })
        })
    }

    fn provider_id(&self) -> &'static str {
        "claude"
    }
}

/// Returns the default worktree path when none is provided in the spec.
///
/// Falls back to the process's current working directory.
///
/// # Errors
///
/// Returns an [`std::io::Error`] if the current directory cannot be determined.
pub fn default_worktree() -> std::io::Result<PathBuf> {
    std::env::current_dir()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::AgentSpec;

    fn make_spec(worktree: Option<PathBuf>) -> AgentSpec {
        AgentSpec {
            label: "test-session".to_owned(),
            provider: "claude".to_owned(),
            prompt: None,
            worktree,
        }
    }

    #[test]
    fn build_command_no_worktree_has_no_current_dir_override() {
        let provider = ClaudeProvider::new();
        assert_eq!(provider.provider_id(), "claude");

        let spec = make_spec(None);
        // Ensure build_command_with_bin returns without panic.
        let _cmd = build_command_with_bin("echo", &spec);
    }

    #[test]
    fn build_command_with_worktree_sets_current_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().to_path_buf();
        let spec = make_spec(Some(path.clone()));

        let cmd = build_command_with_bin("echo", &spec);
        let std_cmd = cmd.as_std();

        assert_eq!(
            std_cmd.get_current_dir(),
            Some(path.as_path()),
            "current_dir must be set to the worktree path"
        );
    }

    #[test]
    fn build_command_without_worktree_has_no_current_dir() {
        let spec = make_spec(None);
        let cmd = build_command_with_bin("echo", &spec);
        let std_cmd = cmd.as_std();

        assert_eq!(
            std_cmd.get_current_dir(),
            None,
            "current_dir must be None when no worktree is provided"
        );
    }
}
