//! Smoke test: start daemon on ephemeral port, TCP-connect, assert success.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use lesearch_daemon::{AgentManager, serve_with_providers};
use lesearch_providers::AgentProvider;
use tokio::net::TcpStream;
use tokio::time::timeout;

/// Spin up the daemon on an ephemeral port and verify a raw TCP connection
/// succeeds within a short timeout.
#[tokio::test]
async fn daemon_accepts_tcp_connection() {
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    let providers: Arc<HashMap<String, Arc<dyn AgentProvider>>> = Arc::new(HashMap::new());

    let (bound, _handle) = serve_with_providers(addr, manager, providers)
        .await
        .expect("serve daemon");

    let connect_result = timeout(Duration::from_secs(5), TcpStream::connect(bound)).await;

    assert!(
        connect_result.is_ok(),
        "timed out waiting for TCP connection to {bound}"
    );
    assert!(
        connect_result.unwrap().is_ok(),
        "TCP connection to {bound} failed"
    );
}
