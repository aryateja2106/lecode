//! `LeSearch` agent provider adapters.
//!
//! Each provider wraps an upstream CLI coding agent via stdio (MCP or ACP),
//! exposing a uniform `AgentProvider` trait to the daemon. New providers
//! ship as registered binaries without modifying daemon core.

#![doc(html_root_url = "https://docs.rs/lesearch-providers/0.0.1")]

pub mod claude;

use std::pin::Pin;

use tokio::sync::mpsc;

/// Built-in provider identifiers.
pub const BUILTIN_PROVIDERS: &[&str] = &["claude", "codex", "opencode", "gemini", "generic-a2a"];

// ── Stream discriminant ────────────────────────────────────────────────────

/// Which stdio stream an event originates from.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Stream {
    /// Agent's standard output.
    Stdout,
    /// Agent's standard error.
    Stderr,
}

impl Stream {
    /// Return the wire string: `"stdout"` or `"stderr"`.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Stdout => "stdout",
            Self::Stderr => "stderr",
        }
    }
}

// ── Agent events ───────────────────────────────────────────────────────────

/// An event emitted by a running agent process.
#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum AgentEvent {
    /// A UTF-8 chunk arrived on stdout or stderr.
    StreamChunk {
        /// Which stream the data came from.
        stream: Stream,
        /// The raw bytes as a UTF-8 string.
        data: String,
    },
    /// The agent process exited.
    Exited {
        /// OS exit code, if available.
        code: Option<i32>,
    },
}

// ── Agent handle ───────────────────────────────────────────────────────────

/// A live handle to a spawned agent process.
///
/// The `events` receiver yields [`AgentEvent`] values until the agent exits
/// and the channel closes.
pub struct AgentHandle {
    /// Receive end of the event channel. Closed when the agent terminates.
    pub events: mpsc::UnboundedReceiver<AgentEvent>,
}

// ── Agent spec ─────────────────────────────────────────────────────────────

/// Parameters describing how to spawn an agent.
#[derive(Debug, Clone, Default)]
pub struct AgentSpec {
    /// Optional working directory for the spawned agent process.
    pub worktree: Option<String>,
}

// ── Provider error ─────────────────────────────────────────────────────────

/// Errors that can occur when a provider attempts to spawn an agent.
#[derive(Debug, thiserror::Error)]
pub enum SpawnError {
    /// The provider binary was not found or could not be launched.
    #[error("failed to launch provider binary: {0}")]
    Launch(String),
    /// The provider name is not recognised by this implementation.
    #[error("unknown provider: {0}")]
    UnknownProvider(String),
    /// An I/O error occurred during spawn.
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
}

// ── Provider trait ─────────────────────────────────────────────────────────

/// Adapts a specific upstream agent CLI into the uniform `LeSearch` interface.
///
/// Implementors must be `Send + Sync + 'static` so they can be stored in the
/// shared `AgentManager` and called from async tasks.
///
/// The return type uses `Pin<Box<dyn Future>>` to remain dyn-compatible so the
/// daemon can hold `Box<dyn AgentProvider>` without knowing the concrete type.
pub trait AgentProvider: Send + Sync + 'static {
    /// Spawn a new agent instance described by `spec`.
    ///
    /// Returns an [`AgentHandle`] whose `events` channel will receive
    /// [`AgentEvent`] values until the agent exits.
    fn spawn(
        &self,
        spec: &AgentSpec,
    ) -> Pin<Box<dyn Future<Output = Result<AgentHandle, SpawnError>> + Send + '_>>;
}

/// Convenience re-export so callers don't need to import `std::future`.
pub use std::future::Future;

// ── TestProvider ───────────────────────────────────────────────────────────

/// A deterministic stub provider for integration tests.
///
/// Emits a configurable sequence of [`AgentEvent::StreamChunk`] events then
/// closes the channel. No real subprocess is spawned.
///
/// Gated behind the `test-provider` feature flag **or** `#[cfg(test)]`
/// compilations so it never ships in production binaries.
#[cfg(any(test, feature = "test-provider"))]
#[derive(Debug, Clone)]
pub struct TestProvider {
    /// Events to emit, in order, when `spawn` is called.
    pub events: Vec<AgentEvent>,
}

#[cfg(any(test, feature = "test-provider"))]
impl TestProvider {
    /// Create a provider that emits `count` identical stdout chunks of `data`.
    #[must_use]
    pub fn stdout_chunks(data: impl Into<String>, count: usize) -> Self {
        let data = data.into();
        Self {
            events: (0..count)
                .map(|_| AgentEvent::StreamChunk {
                    stream: Stream::Stdout,
                    data: data.clone(),
                })
                .collect(),
        }
    }
}

#[cfg(any(test, feature = "test-provider"))]
impl AgentProvider for TestProvider {
    fn spawn(
        &self,
        _spec: &AgentSpec,
    ) -> Pin<Box<dyn Future<Output = Result<AgentHandle, SpawnError>> + Send + '_>> {
        let events = self.events.clone();
        Box::pin(async move {
            let (tx, rx) = mpsc::unbounded_channel();
            for event in events {
                // Ignore send errors — receiver may have been dropped.
                let _ = tx.send(event);
            }
            // Drop tx so the receiver sees channel closed after all events.
            drop(tx);
            Ok(AgentHandle { events: rx })
        })
    }
}
