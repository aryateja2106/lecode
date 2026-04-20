//! Integration test: provider spawn honours the worktree path as `current_dir`.
//!
//! We test the *command builder* (`build_command_with_bin`) rather than
//! actually spawning a process, so no real `claude` binary is required.

use std::path::PathBuf;

use lesearch_providers::{AgentSpec, claude::build_command_with_bin};

fn make_spec(worktree: Option<PathBuf>) -> AgentSpec {
    AgentSpec {
        label: "test".to_owned(),
        provider: "claude".to_owned(),
        prompt: None,
        worktree,
    }
}

/// Verifies that when `AgentSpec.worktree` is `Some(path)`, the built
/// `Command` has `current_dir` set to that path.
#[test]
fn provider_spawn_uses_worktree_cwd() {
    let dir = tempfile::tempdir().expect("should be able to create a temp directory");
    let worktree_path = dir.path().to_path_buf();

    let spec = make_spec(Some(worktree_path.clone()));
    let cmd = build_command_with_bin("echo", &spec);
    let std_cmd = cmd.as_std();

    assert_eq!(
        std_cmd.get_current_dir(),
        Some(worktree_path.as_path()),
        "Command::current_dir must be set to the worktree path when AgentSpec.worktree is Some"
    );
}

/// Verifies that when `AgentSpec.worktree` is `None`, the built `Command`
/// does *not* set `current_dir` (inherits daemon's cwd).
#[test]
fn provider_spawn_no_worktree_inherits_cwd() {
    let spec = make_spec(None);
    let cmd = build_command_with_bin("echo", &spec);
    let std_cmd = cmd.as_std();

    assert_eq!(
        std_cmd.get_current_dir(),
        None,
        "Command::current_dir must be None when AgentSpec.worktree is None"
    );
}
