//! Integration test: verify that `agent.spawn` returns three distinct,
//! non-zero stream IDs via the WebSocket JSON-RPC API.
//!
//! The daemon is spun up in-process on an ephemeral port (127.0.0.1:0) and
//! torn down after each test.

use std::collections::HashSet;
use std::net::SocketAddr;

use futures::{SinkExt, StreamExt};
use lesearch_daemon::{AgentManager, serve};
use lesearch_protocol::SpawnResult;
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

/// Spin up the daemon, connect via WebSocket, send `agent.spawn`, and verify
/// the response contains three distinct non-zero stream IDs.
#[tokio::test]
async fn spawn_returns_nonzero_distinct_stream_ids() {
    // Bind to an ephemeral port so tests don't collide.
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    let (bound, _server) = serve(addr, manager).await.expect("serve");

    let url = format!("ws://{bound}/ws");
    let (mut ws, _) = connect_async(&url).await.expect("connect ws");

    let request = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "agent.spawn",
        "params": {
            "provider": "test"
        }
    });

    ws.send(Message::Text(
        serde_json::to_string(&request)
            .expect("serialize request")
            .into(),
    ))
    .await
    .expect("send request");

    let reply = ws.next().await.expect("no reply").expect("ws error");

    let text = match reply {
        Message::Text(t) => t,
        other => panic!("expected text frame, got {other:?}"),
    };

    println!("Response: {text}");

    // Parse the outer JSON-RPC envelope.
    let envelope: serde_json::Value = serde_json::from_str(&text).expect("parse response JSON");

    assert_eq!(envelope["jsonrpc"], "2.0", "jsonrpc field must be \"2.0\"");
    assert_eq!(envelope["id"], 1, "id must be echoed");
    assert!(
        envelope.get("error").is_none(),
        "response must not be an error: {text}"
    );

    let result = &envelope["result"];
    let spawn: SpawnResult = serde_json::from_value(result.clone()).expect("parse SpawnResult");

    // agent_id must be non-empty.
    assert!(!spawn.agent_id.is_empty(), "agent_id must not be empty");

    let ids = spawn.streams;

    // All three IDs must be non-zero.
    assert_ne!(ids.stdin, 0, "stdin stream ID must be non-zero");
    assert_ne!(ids.stdout, 0, "stdout stream ID must be non-zero");
    assert_ne!(ids.stderr, 0, "stderr stream ID must be non-zero");

    // All three IDs must be distinct.
    assert_ne!(ids.stdin, ids.stdout, "stdin and stdout IDs must differ");
    assert_ne!(ids.stdout, ids.stderr, "stdout and stderr IDs must differ");
    assert_ne!(ids.stdin, ids.stderr, "stdin and stderr IDs must differ");

    ws.close(None).await.ok();
}

/// Verify that IDs from two independent spawns are all distinct (monotonic
/// counter advances across calls).
#[tokio::test]
async fn two_spawns_produce_six_distinct_ids() {
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    let (bound, _server) = serve(addr, manager).await.expect("serve");

    let url = format!("ws://{bound}/ws");
    let (mut ws, _) = connect_async(&url).await.expect("connect ws");

    let spawn_req = serde_json::json!({
        "jsonrpc": "2.0",
        "id": serde_json::Value::Null,
        "method": "agent.spawn",
        "params": { "provider": "test" }
    });
    let spawn_text = serde_json::to_string(&spawn_req).expect("ser");

    // First spawn.
    ws.send(Message::Text(spawn_text.clone().into()))
        .await
        .expect("send 1");
    let r1 = ws.next().await.expect("recv 1").expect("ws err 1");
    let t1 = match r1 {
        Message::Text(t) => t,
        o => panic!("expected text, got {o:?}"),
    };
    let env1: serde_json::Value = serde_json::from_str(&t1).expect("json 1");
    let s1: SpawnResult = serde_json::from_value(env1["result"].clone()).expect("result 1");

    // Second spawn.
    ws.send(Message::Text(spawn_text.into()))
        .await
        .expect("send 2");
    let r2 = ws.next().await.expect("recv 2").expect("ws err 2");
    let t2 = match r2 {
        Message::Text(t) => t,
        o => panic!("expected text, got {o:?}"),
    };
    let env2: serde_json::Value = serde_json::from_str(&t2).expect("json 2");
    let s2: SpawnResult = serde_json::from_value(env2["result"].clone()).expect("result 2");

    // Collect all six IDs.
    let all_ids = [
        s1.streams.stdin,
        s1.streams.stdout,
        s1.streams.stderr,
        s2.streams.stdin,
        s2.streams.stdout,
        s2.streams.stderr,
    ];

    // None may be zero.
    for id in all_ids {
        assert_ne!(id, 0, "stream ID {id} must not be zero");
    }

    // All six must be distinct.
    let unique: HashSet<u16> = all_ids.iter().copied().collect();
    assert_eq!(
        unique.len(),
        6,
        "all six IDs across two spawns must be distinct, got {all_ids:?}"
    );

    ws.close(None).await.ok();
}
