//! `LeSearch` wire protocol.
//!
//! JSON-RPC 2.0 request/response + notifications over a single `WebSocket`
//! connection. Binary frames are reserved for PTY streams; text frames carry
//! JSON-RPC envelopes.
//!
//! See `docs/protocol-v0.1.md` in the workspace root for the canonical spec.

#![doc(html_root_url = "https://docs.rs/lesearch-protocol/0.0.1")]

use serde::{Deserialize, Serialize};

/// Protocol version exposed via `server.handshake`. Clients ≥ 6 months old
/// must keep working; bump the minor version for additive changes only.
pub const PROTOCOL_VERSION: &str = "0.1.0";

/// Default loopback bind address for the daemon.
pub const DEFAULT_BIND: &str = "127.0.0.1:6767";

/// Returns the protocol version string.
#[must_use]
pub const fn version() -> &'static str {
    PROTOCOL_VERSION
}

// ── JSON-RPC 2.0 envelope ──────────────────────────────────────────────────

/// A JSON-RPC 2.0 request sent by the client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcRequest {
    /// Must be `"2.0"`.
    pub jsonrpc: String,
    /// Caller-chosen ID; echoed in the response.
    pub id: serde_json::Value,
    /// Method name, e.g. `"agent.spawn"`.
    pub method: String,
    /// Optional method parameters.
    #[serde(default)]
    pub params: serde_json::Value,
}

/// A successful JSON-RPC 2.0 response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcResponse {
    /// Always `"2.0"`.
    pub jsonrpc: String,
    /// Echoed from the request.
    pub id: serde_json::Value,
    /// Method result payload.
    pub result: serde_json::Value,
}

impl RpcResponse {
    /// Construct a successful response.
    #[must_use]
    pub fn ok(id: serde_json::Value, result: serde_json::Value) -> Self {
        Self {
            jsonrpc: "2.0".to_owned(),
            id,
            result,
        }
    }
}

/// A JSON-RPC 2.0 error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcError {
    /// Always `"2.0"`.
    pub jsonrpc: String,
    /// Echoed from the request (may be `null` for parse errors).
    pub id: serde_json::Value,
    /// Error object.
    pub error: RpcErrorObject,
}

impl RpcError {
    /// Construct an error response with the given code and message.
    #[must_use]
    pub fn new(id: serde_json::Value, code: i32, message: &str) -> Self {
        Self {
            jsonrpc: "2.0".to_owned(),
            id,
            error: RpcErrorObject {
                code,
                message: message.to_owned(),
            },
        }
    }
}

/// The inner error object of a JSON-RPC 2.0 error response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RpcErrorObject {
    /// Standard or application-defined error code.
    pub code: i32,
    /// Human-readable error description.
    pub message: String,
}

// ── agent.spawn ────────────────────────────────────────────────────────────

/// Parameters for the `agent.spawn` RPC method.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnParams {
    /// Provider identifier, e.g. `"claude"`, `"codex"`, or `"test"`.
    pub provider: String,
    /// Optional working directory for the spawned agent.
    #[serde(default)]
    pub worktree: Option<String>,
}

/// Three non-zero u16 channel IDs assigned to an agent's stdio streams.
///
/// `0` is reserved for the control channel and is never issued here.
/// All three IDs are guaranteed to be distinct within a session.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct StreamIds {
    /// Channel ID for the agent's stdin feed.
    pub stdin: u16,
    /// Channel ID for the agent's stdout feed.
    pub stdout: u16,
    /// Channel ID for the agent's stderr feed.
    pub stderr: u16,
}

/// Result returned by a successful `agent.spawn` call.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SpawnResult {
    /// Stable unique identifier for the spawned agent within this session.
    pub agent_id: String,
    /// Stream channel IDs for binary-mux routing (slice 4).
    pub streams: StreamIds,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_semver_prefix() {
        assert!(version().starts_with("0."));
    }

    #[test]
    fn spawn_result_roundtrips() {
        let r = SpawnResult {
            agent_id: "abc-123".to_owned(),
            streams: StreamIds {
                stdin: 1,
                stdout: 2,
                stderr: 3,
            },
        };
        let s = serde_json::to_string(&r).expect("serialize");
        let r2: SpawnResult = serde_json::from_str(&s).expect("deserialize");
        assert_eq!(r2.agent_id, "abc-123");
        assert_eq!(r2.streams.stdin, 1);
        assert_eq!(r2.streams.stdout, 2);
        assert_eq!(r2.streams.stderr, 3);
    }
}
