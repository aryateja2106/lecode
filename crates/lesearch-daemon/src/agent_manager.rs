//! Agent lifecycle manager.
//!
//! Responsible for translating [`SpawnParams`] (the wire-level request) into
//! a fully resolved [`AgentSpec`] and dispatching to the appropriate
//! [`AgentProvider`] adapter.

use std::path::PathBuf;

use lesearch_providers::{AgentProvider, AgentSpec, ProviderError, SpawnParams, SpawnResult};

/// Converts a [`SpawnParams`] into a resolved [`AgentSpec`].
///
/// This is where the daemon performs any validation or path resolution before
/// handing off to the provider layer.
///
/// # Errors
///
/// Currently infallible; returns `Ok` in all cases. Future revisions may add
/// path-existence validation here.
pub fn build_spec(params: &SpawnParams) -> Result<AgentSpec, ProviderError> {
    let worktree: Option<PathBuf> = params.worktree.as_deref().map(PathBuf::from);

    Ok(AgentSpec {
        label: params.label.clone(),
        provider: params.provider.clone(),
        prompt: params.prompt.clone(),
        worktree,
    })
}

/// Spawn an agent using the given provider and spawn parameters.
///
/// Builds an [`AgentSpec`] from `params`, delegates to `provider.spawn`, and
/// returns the [`SpawnResult`].
///
/// # Errors
///
/// Propagates any [`ProviderError`] returned by the provider adapter.
pub async fn spawn_agent(
    provider: &dyn AgentProvider,
    params: &SpawnParams,
) -> Result<SpawnResult, ProviderError> {
    let spec = build_spec(params)?;
    provider.spawn(&spec).await
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_params(worktree: Option<&str>) -> SpawnParams {
        SpawnParams {
            label: "test".to_owned(),
            provider: "claude".to_owned(),
            prompt: None,
            worktree: worktree.map(str::to_owned),
        }
    }

    #[test]
    fn build_spec_threads_worktree_into_agent_spec() {
        let params = make_params(Some("/tmp/my-worktree"));
        let spec = build_spec(&params).expect("build_spec should succeed");
        assert_eq!(spec.worktree, Some(PathBuf::from("/tmp/my-worktree")));
    }

    #[test]
    fn build_spec_none_worktree_stays_none() {
        let params = make_params(None);
        let spec = build_spec(&params).expect("build_spec should succeed");
        assert_eq!(spec.worktree, None);
    }
}
