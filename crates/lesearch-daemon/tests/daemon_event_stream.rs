//! Integration test: verify that `AgentEvent::StreamChunk` events emitted by
//! a spawned provider are forwarded to the connected WebSocket client as
//! `agent.output` JSON-RPC 2.0 notifications.
//!
//! Uses [`TestProvider`] (via the `test-provider` feature of
//! `lesearch-providers`) so no real subprocess is spawned.

use std::net::SocketAddr;

use futures::{SinkExt, StreamExt};
use lesearch_daemon::{AgentManager, serve};
use lesearch_protocol::SpawnResult;
use lesearch_providers::TestProvider;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

/// Spin up the daemon with a `TestProvider` that emits two stdout chunks, then
/// verify the client receives the spawn response followed by two
/// `agent.output` notifications.
#[tokio::test]
async fn daemon_stdout_reaches_client() {
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();

    // Register a TestProvider that emits "hello\n" twice then closes.
    manager
        .add_provider("test", TestProvider::stdout_chunks("hello\n", 2))
        .await;

    let (bound, _server) = serve(addr, manager).await.expect("serve");

    let url = format!("ws://{bound}/ws");
    let (mut ws, _) = connect_async(&url).await.expect("connect ws");

    // ── Send agent.spawn ───────────────────────────────────────────────────
    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 42,
        "method": "agent.spawn",
        "params": { "provider": "test" }
    });

    ws.send(Message::Text(
        serde_json::to_string(&request)
            .expect("serialize request")
            .into(),
    ))
    .await
    .expect("send spawn request");

    // ── Receive spawn response ─────────────────────────────────────────────
    let spawn_reply = ws
        .next()
        .await
        .expect("no spawn reply")
        .expect("ws error on spawn reply");

    let spawn_text = match spawn_reply {
        Message::Text(t) => t,
        other => panic!("expected text frame for spawn response, got {other:?}"),
    };

    println!("Spawn response: {spawn_text}");

    let spawn_env: serde_json::Value =
        serde_json::from_str(&spawn_text).expect("parse spawn response JSON");

    assert_eq!(spawn_env["jsonrpc"], "2.0", "jsonrpc field must be \"2.0\"");
    assert_eq!(spawn_env["id"], 42, "id must be echoed");
    assert!(
        spawn_env.get("error").is_none(),
        "spawn response must not be an error: {spawn_text}"
    );

    let spawn_result: SpawnResult =
        serde_json::from_value(spawn_env["result"].clone()).expect("parse SpawnResult");
    let agent_id = spawn_result.agent_id.clone();
    assert!(!agent_id.is_empty(), "agent_id must not be empty");

    // ── Receive two agent.output notifications ─────────────────────────────
    for i in 0..2_u8 {
        // Allow a generous timeout so CI slow machines still pass.
        let notif_msg = tokio::time::timeout(std::time::Duration::from_secs(5), ws.next())
            .await
            .unwrap_or_else(|_| panic!("timed out waiting for agent.output notification #{i}"))
            .expect("stream ended before notification")
            .expect("ws error on notification");

        let notif_text = match notif_msg {
            Message::Text(t) => t,
            other => panic!("expected text frame for notification #{i}, got {other:?}"),
        };

        println!("Notification #{i}: {notif_text}");

        let notif: serde_json::Value =
            serde_json::from_str(&notif_text).expect("parse notification JSON");

        // Must be a valid JSON-RPC 2.0 notification envelope.
        assert_eq!(
            notif["jsonrpc"], "2.0",
            "notification jsonrpc must be \"2.0\""
        );
        assert_eq!(
            notif["method"], "agent.output",
            "notification method must be \"agent.output\""
        );
        assert!(
            notif.get("id").is_none(),
            "notifications must NOT have an id field: {notif_text}"
        );

        // Verify params.
        let params = &notif["params"];
        assert_eq!(
            params["agentId"], agent_id,
            "params.agentId must match spawn result agent_id"
        );
        assert_eq!(
            params["stream"], "stdout",
            "params.stream must be \"stdout\""
        );
        assert_eq!(
            params["data"], "hello\n",
            "params.data must be \"hello\\n\""
        );
    }

    ws.close(None).await.ok();
}

/// Verify that spawning an unknown provider still returns a valid spawn
/// response (no crash), just no events follow.
#[tokio::test]
async fn unknown_provider_returns_spawn_response() {
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    // No providers registered.
    let (bound, _server) = serve(addr, manager).await.expect("serve");

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
    assert!(env.get("error").is_none(), "should not be an error: {text}");
    assert!(
        !env["result"]["agentId"].as_str().unwrap_or("").is_empty(),
        "agentId must be present"
    );

    ws.close(None).await.ok();
}
