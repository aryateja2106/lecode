//! Agent lifecycle helpers.
//!
//! Utility functions for translating wire-level [`SpawnParams`] into the
//! provider-level [`AgentSpec`] used by [`AgentProvider::spawn`].

use lesearch_protocol::SpawnParams;
use lesearch_providers::AgentSpec;

/// Converts a [`SpawnParams`] into a resolved [`AgentSpec`].
///
/// Currently a simple field projection. Future revisions may add path
/// validation or provider-specific resolution here.
#[must_use]
pub fn build_spec(params: &SpawnParams) -> AgentSpec {
    AgentSpec {
        worktree: params.worktree.clone(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_params(worktree: Option<&str>) -> SpawnParams {
        SpawnParams {
            provider: "claude".to_owned(),
            worktree: worktree.map(str::to_owned),
        }
    }

    #[test]
    fn build_spec_threads_worktree_into_agent_spec() {
        let params = make_params(Some("/tmp/my-worktree"));
        let spec = build_spec(&params);
        assert_eq!(spec.worktree, Some("/tmp/my-worktree".to_owned()));
    }

    #[test]
    fn build_spec_none_worktree_stays_none() {
        let params = make_params(None);
        let spec = build_spec(&params);
        assert_eq!(spec.worktree, None);
    }
}
