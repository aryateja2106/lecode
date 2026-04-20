//! Claude Code provider adapter.
//!
//! Wraps the `claude` CLI binary, launching it as a subprocess with stdio
//! connected to the daemon's streaming pipeline.

use std::path::Path;
use std::pin::Pin;

use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, Command};
use tokio::sync::mpsc;

use crate::{AgentEvent, AgentHandle, AgentProvider, AgentSpec, Future, SpawnError, Stream};

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

    // Keep all stdio open for the daemon to stream.
    cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    cmd
}

/// Validate that `worktree` exists on disk, if provided.
fn validate_worktree(worktree: Option<&str>) -> Result<(), SpawnError> {
    if let Some(path) = worktree {
        if !Path::new(path).exists() {
            return Err(SpawnError::Launch(format!(
                "worktree path does not exist: {path}"
            )));
        }
    }
    Ok(())
}

/// Attach stdio readers to a live child and forward lines as [`AgentEvent`]s
/// over `tx`.
fn attach_stdio_forwarder(child: &mut Child, tx: mpsc::UnboundedSender<AgentEvent>) {
    // Take stdout.
    if let Some(stdout) = child.stdout.take() {
        let tx_out = tx.clone();
        tokio::spawn(async move {
            let mut lines = BufReader::new(stdout).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = tx_out.send(AgentEvent::StreamChunk {
                    stream: Stream::Stdout,
                    data: line + "\n",
                });
            }
        });
    }

    // Take stderr.
    if let Some(stderr) = child.stderr.take() {
        tokio::spawn(async move {
            let mut lines = BufReader::new(stderr).lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let _ = tx.send(AgentEvent::StreamChunk {
                    stream: Stream::Stderr,
                    data: line + "\n",
                });
            }
        });
    }
}

impl AgentProvider for ClaudeProvider {
    fn spawn(
        &self,
        spec: &AgentSpec,
    ) -> Pin<Box<dyn Future<Output = Result<AgentHandle, SpawnError>> + Send + '_>> {
        let worktree = spec.worktree.clone();
        Box::pin(async move {
            validate_worktree(worktree.as_deref())?;

            let spec_local = AgentSpec { worktree };
            let mut cmd = build_command(&spec_local);
            let mut child = cmd.spawn()?;

            let (tx, rx) = mpsc::unbounded_channel();
            attach_stdio_forwarder(&mut child, tx.clone());

            // Wait for the child to exit and send an Exited event.
            tokio::spawn(async move {
                let code = child.wait().await.ok().and_then(|s| s.code());
                let _ = tx.send(AgentEvent::Exited { code });
            });

            Ok(AgentHandle { events: rx })
        })
    }
}

/// Returns the default worktree path when none is provided in the spec.
///
/// Falls back to the process's current working directory.
///
/// # Errors
///
/// Returns an [`std::io::Error`] if the current directory cannot be determined.
pub fn default_worktree() -> std::io::Result<std::path::PathBuf> {
    std::env::current_dir()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_spec(worktree: Option<String>) -> AgentSpec {
        AgentSpec { worktree }
    }

    #[test]
    fn build_command_no_worktree_has_no_current_dir_override() {
        let spec = make_spec(None);
        // Ensure build_command_with_bin returns without panic.
        let _cmd = build_command_with_bin("echo", &spec);
    }

    #[test]
    fn build_command_with_worktree_sets_current_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().to_path_buf();
        let spec = make_spec(Some(path.to_string_lossy().into_owned()));

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
