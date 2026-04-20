//! Integration test: verify that output emitted by a spawned provider is
//! forwarded to the connected WebSocket client as `agent.output` JSON-RPC 2.0
//! notifications.
//!
//! Uses [`TestProvider`] (via the `test-provider` feature of
//! `lesearch-providers`) so no real subprocess is spawned.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use futures::{SinkExt, StreamExt};
use lesearch_daemon::{AgentManager, serve_with_providers};
use lesearch_protocol::SpawnResult;
use lesearch_providers::AgentProvider;
use lesearch_providers::test_provider::{TEST_OUTPUT_LINES, TEST_PROVIDER_ID, TestProvider};
use tokio::time::timeout;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

fn test_providers() -> Arc<HashMap<String, Arc<dyn AgentProvider>>> {
    let mut map: HashMap<String, Arc<dyn AgentProvider>> = HashMap::new();
    map.insert(TEST_PROVIDER_ID.to_owned(), Arc::new(TestProvider::new()));
    Arc::new(map)
}

/// Spin up the daemon with a `TestProvider`, verify the client receives the
/// spawn response followed by `agent.output` notifications for each line.
#[tokio::test]
async fn daemon_stdout_reaches_client() {
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    let (bound, _server) = serve_with_providers(addr, manager, test_providers())
        .await
        .expect("serve");

    let url = format!("ws://{bound}/ws");
    let (mut ws, _) = connect_async(&url).await.expect("connect ws");

    // Send agent.spawn.
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 42,
        "method": "agent.spawn",
        "params": { "provider": TEST_PROVIDER_ID }
    });
    ws.send(Message::Text(
        serde_json::to_string(&request).expect("serialize").into(),
    ))
    .await
    .expect("send spawn");

    // Receive spawn response.
    let spawn_reply = ws.next().await.expect("no reply").expect("ws error");
    let spawn_text = match spawn_reply {
        Message::Text(t) => t,
        other => panic!("expected text, got {other:?}"),
    };

    let spawn_env: serde_json::Value = serde_json::from_str(&spawn_text).expect("parse spawn JSON");
    assert_eq!(spawn_env["jsonrpc"], "2.0");
    assert_eq!(spawn_env["id"], 42);
    assert!(
        spawn_env.get("error").is_none(),
        "spawn must not be an error: {spawn_text}"
    );

    let spawn_result: SpawnResult =
        serde_json::from_value(spawn_env["result"].clone()).expect("parse SpawnResult");
    let agent_id = spawn_result.agent_id.clone();
    assert!(!agent_id.is_empty(), "agent_id must not be empty");

    // Receive one agent.output notification per TEST_OUTPUT_LINES entry.
    for (i, expected_line) in TEST_OUTPUT_LINES.iter().enumerate() {
        let notif_msg = timeout(Duration::from_secs(5), ws.next())
            .await
            .unwrap_or_else(|_| panic!("timed out waiting for notification #{i}"))
            .expect("stream ended")
            .expect("ws error");

        let notif_text = match notif_msg {
            Message::Text(t) => t,
            other => panic!("expected text for notification #{i}, got {other:?}"),
        };

        let notif: serde_json::Value =
            serde_json::from_str(&notif_text).expect("parse notification JSON");

        assert_eq!(notif["jsonrpc"], "2.0");
        assert_eq!(notif["method"], "agent.output");
        assert!(notif.get("id").is_none(), "notifications must not have id");

        let params = &notif["params"];
        assert_eq!(params["agent_id"], agent_id);
        assert_eq!(
            params["data"].as_str().unwrap_or(""),
            *expected_line,
            "notification #{i} data mismatch"
        );
    }

    ws.close(None).await.ok();
}

/// Verify that spawning an unknown provider returns an error response.
#[tokio::test]
async fn unknown_provider_returns_error() {
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    // No providers registered.
    let providers: Arc<HashMap<String, Arc<dyn AgentProvider>>> = Arc::new(HashMap::new());
    let (bound, _server) = serve_with_providers(addr, manager, providers)
        .await
        .expect("serve");

    let url = format!("ws://{bound}/ws");
    let (mut ws, _) = connect_async(&url).await.expect("connect ws");

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "agent.spawn",
        "params": { "provider": "nonexistent" }
    });
    ws.send(Message::Text(
        serde_json::to_string(&request).expect("serialize").into(),
    ))
    .await
    .expect("send");

    let reply = ws.next().await.expect("no reply").expect("ws error");
    let text = match reply {
        Message::Text(t) => t,
        other => panic!("expected text, got {other:?}"),
    };

    let env: serde_json::Value = serde_json::from_str(&text).expect("parse JSON");
    assert_eq!(env["jsonrpc"], "2.0");
    assert_eq!(env["id"], 1);
    assert!(
        env.get("error").is_some(),
        "unknown provider must return an error response: {text}"
    );

    ws.close(None).await.ok();
}
