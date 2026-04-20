//! Claude Code provider adapter.

use std::path::PathBuf;

use futures::future::BoxFuture;
use tokio::process::{Child, Command};

use crate::{AgentProvider, AgentSpec, ProviderError, SpawnResult};

const CLAUDE_BIN: &str = "claude";

/// Provider adapter for Anthropic's Claude Code agent.
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
#[must_use]
pub fn build_command(spec: &AgentSpec) -> Command {
    build_command_with_bin(CLAUDE_BIN, spec)
}

/// Build a [`Command`] using the given binary name instead of `claude`.
#[must_use]
pub fn build_command_with_bin(bin: &str, spec: &AgentSpec) -> Command {
    let mut cmd = Command::new(bin);

    if let Some(ref worktree) = spec.worktree {
        cmd.current_dir(worktree);
    }

    if let Some(ref prompt) = spec.prompt {
        cmd.arg("--print").arg(prompt);
    }

    cmd.stdin(std::process::Stdio::piped())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped());

    cmd
}

fn do_spawn(cmd: &mut Command) -> Result<Child, ProviderError> {
    Ok(cmd.spawn()?)
}

impl AgentProvider for ClaudeProvider {
    fn spawn<'a>(
        &'a self,
        spec: &'a AgentSpec,
    ) -> BoxFuture<'a, Result<SpawnResult, ProviderError>> {
        Box::pin(async move {
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

/// Returns the default worktree path (process cwd).
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
        let _cmd = build_command_with_bin("echo", &spec);
    }

    #[test]
    fn build_command_with_worktree_sets_current_dir() {
        let dir = tempfile::tempdir().expect("tempdir");
        let path = dir.path().to_path_buf();
        let spec = make_spec(Some(path.clone()));
        let cmd = build_command_with_bin("echo", &spec);
        let std_cmd = cmd.as_std();
        assert_eq!(std_cmd.get_current_dir(), Some(path.as_path()));
    }

    #[test]
    fn build_command_without_worktree_has_no_current_dir() {
        let spec = make_spec(None);
        let cmd = build_command_with_bin("echo", &spec);
        let std_cmd = cmd.as_std();
        assert_eq!(std_cmd.get_current_dir(), None);
    }
}
