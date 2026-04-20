//! `LeSearch` daemon entry point.
//!
//! Phase A.0 stub — does not yet bind the protocol server. Real implementation
//! arrives with `lesearch-protocol` wire types and the agent manager state machine.

fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    tracing::info!(
        version = lesearch_daemon::protocol_version(),
        "lesearch-daemon scaffold — protocol binding arrives in Day 2-3"
    );
}
