//! `LeSearch` daemon core.
//!
//! Hosts the agent manager, protocol router, PTY pool, storage writer, and
//! A2A gateway. Exposes a `WebSocket` endpoint on `127.0.0.1:6767` by default.
//!
//! See `docs/PRD.md`, `docs/SYSTEM_DESIGN.md`, and `docs/protocol-v0.1.md`.

#![doc(html_root_url = "https://docs.rs/lesearch-daemon/0.0.1")]

/// Returns the compiled protocol version this daemon speaks.
#[must_use]
pub const fn protocol_version() -> &'static str {
    lesearch_protocol::version()
}
