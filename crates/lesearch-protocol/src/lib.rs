//! `LeSearch` wire protocol.
//!
//! JSON-RPC 2.0 request/response + notifications over a single `WebSocket`
//! connection. Binary frames are reserved for PTY streams; text frames carry
//! JSON-RPC envelopes.
//!
//! See `docs/protocol-v0.1.md` in the workspace root for the canonical spec.

#![doc(html_root_url = "https://docs.rs/lesearch-protocol/0.0.1")]

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn version_is_semver_prefix() {
        assert!(version().starts_with("0."));
    }
}
