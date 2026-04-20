//! In-process test provider.
//!
//! Emits a fixed sequence of lines to stdout so integration tests can verify
//! the full CLI -> daemon -> provider -> client pipeline without requiring a
//! real agent binary.

use futures::future::BoxFuture;
use tokio::process::Command;

use crate::{AgentProvider, AgentSpec, ProviderError, SpawnResult};

/// Provider identifier for the test provider.
pub const TEST_PROVIDER_ID: &str = "test";

/// Lines emitted by the test provider on stdout.
pub const TEST_OUTPUT_LINES: &[&str] = &["hello", "hello"];

/// In-process test provider that spawns a shell one-liner subprocess.
#[derive(Debug, Clone, Default)]
pub struct TestProvider;

impl TestProvider {
    /// Create a new [`TestProvider`].
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl AgentProvider for TestProvider {
    fn spawn<'a>(
        &'a self,
        _spec: &'a AgentSpec,
    ) -> BoxFuture<'a, Result<SpawnResult, ProviderError>> {
        Box::pin(async move {
            let mut cmd = Command::new("sh");
            cmd.args(["-c", "printf 'hello\nhello\n'"])
                .stdin(std::process::Stdio::null())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped());
            let child = cmd.spawn()?;
            Ok(SpawnResult { child })
        })
    }

    fn provider_id(&self) -> &'static str {
        TEST_PROVIDER_ID
    }
}
