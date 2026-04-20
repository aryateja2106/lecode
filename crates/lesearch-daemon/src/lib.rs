//! `LeSearch` daemon core.
//!
//! Hosts the agent manager, protocol router, PTY pool, storage writer, and
//! A2A gateway. Exposes a `WebSocket` endpoint on `127.0.0.1:6767` by default.
//!
//! See `docs/PRD.md`, `docs/SYSTEM_DESIGN.md`, and `docs/protocol-v0.1.md`.

#![doc(html_root_url = "https://docs.rs/lesearch-daemon/0.0.1")]

pub mod agent_manager;

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use lesearch_protocol::{RpcError, RpcRequest, RpcResponse, SpawnParams, SpawnResult, StreamIds};

/// Returns the compiled protocol version this daemon speaks.
#[must_use]
pub const fn protocol_version() -> &'static str {
    lesearch_protocol::version()
}

// ── Stream ID allocator ────────────────────────────────────────────────────

/// Global monotonic counter for stream IDs.
///
/// Starts at 1 (0 is reserved for the control channel). Wraps back to 1 on
/// overflow, never returning 0.
static STREAM_ID_COUNTER: AtomicU16 = AtomicU16::new(1);

/// Allocate the next non-zero stream ID.
///
/// Wraps from `u16::MAX` back to 1 to ensure 0 is never returned.
fn next_stream_id() -> u16 {
    loop {
        let prev = STREAM_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        // fetch_add returns the value *before* the add.  If prev == 0 that
        // means the counter had wrapped to 0 on a previous call — skip it.
        // Otherwise prev is the ID we hand out.
        if prev != 0 {
            return prev;
        }
        // prev was 0: the counter is now 1, so the next iteration returns 1.
    }
}

/// Allocate three distinct, non-zero stream IDs for stdin/stdout/stderr.
fn alloc_stream_ids() -> StreamIds {
    StreamIds {
        stdin: next_stream_id(),
        stdout: next_stream_id(),
        stderr: next_stream_id(),
    }
}

// ── Agent registry ─────────────────────────────────────────────────────────

/// Metadata stored per registered agent.
///
/// Fields are retained for future slices (lookup, routing, teardown).
#[derive(Debug, Clone)]
struct AgentEntry {
    /// Assigned stream channel IDs (stable for the lifetime of the agent).
    #[allow(dead_code)]
    streams: StreamIds,
    /// Provider name used to spawn this agent.
    #[allow(dead_code)]
    provider: String,
}

/// Shared agent manager state, accessible from all WebSocket handlers.
#[derive(Debug, Default)]
pub struct AgentManager {
    agents: Mutex<HashMap<String, AgentEntry>>,
}

impl AgentManager {
    /// Create a new, empty agent manager.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Register a new agent and return its stable [`SpawnResult`].
    ///
    /// Allocates three distinct non-zero stream IDs for the agent's stdio
    /// channels.
    pub async fn register_agent(&self, params: SpawnParams) -> SpawnResult {
        let agent_id = uuid::Uuid::now_v7().to_string();
        let streams = alloc_stream_ids();
        let entry = AgentEntry {
            streams,
            provider: params.provider,
        };
        self.agents.lock().await.insert(agent_id.clone(), entry);
        SpawnResult { agent_id, streams }
    }
}

// ── Axum WebSocket server ──────────────────────────────────────────────────

/// Shared application state threaded through axum handlers.
#[derive(Clone)]
struct AppState {
    manager: Arc<AgentManager>,
}

/// Build an axum [`Router`] wired to the given [`AgentManager`].
pub fn build_router(manager: Arc<AgentManager>) -> Router {
    let state = AppState { manager };
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

/// HTTP upgrade handler — promotes the connection to a WebSocket.
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Drive a single WebSocket connection, dispatching JSON-RPC messages.
async fn handle_socket(mut socket: WebSocket, state: AppState) {
    while let Some(Ok(msg)) = socket.recv().await {
        let text = match msg {
            Message::Text(t) => t,
            Message::Close(_) => break,
            // Binary frames are reserved for PTY mux; ignore at this layer.
            _ => continue,
        };

        let reply = dispatch(&state, &text).await;
        if socket.send(Message::Text(reply.into())).await.is_err() {
            break;
        }
    }
}

/// Fallback serialization for error responses: a static JSON string.
const INTERNAL_ERROR_JSON: &str =
    r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32603,"message":"Internal error"}}"#;

/// Parse a JSON-RPC request and route it to the correct handler.
async fn dispatch(state: &AppState, raw: &str) -> String {
    let req: RpcRequest = if let Ok(r) = serde_json::from_str(raw) {
        r
    } else {
        let err = RpcError::new(serde_json::Value::Null, -32_700, "Parse error");
        return serde_json::to_string(&err).unwrap_or_else(|_| {
            r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":"Parse error"}}"#
                .to_owned()
        });
    };

    let id = req.id.clone();

    if req.method.as_str() == "agent.spawn" {
        handle_agent_spawn(state, id, req.params).await
    } else {
        let err = RpcError::new(id, -32_601, "Method not found");
        serde_json::to_string(&err).unwrap_or_else(|_| {
            r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32601,"message":"Method not found"}}"#
                .to_owned()
        })
    }
}

/// Handle `agent.spawn` — validate params, register agent, return stream IDs.
async fn handle_agent_spawn(
    state: &AppState,
    id: serde_json::Value,
    params: serde_json::Value,
) -> String {
    let spawn_params: SpawnParams = if let Ok(p) = serde_json::from_value(params) {
        p
    } else {
        let err = RpcError::new(id, -32_602, "Invalid params");
        return serde_json::to_string(&err).unwrap_or_else(|_| {
            r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32602,"message":"Invalid params"}}"#
                .to_owned()
        });
    };

    let result = state.manager.register_agent(spawn_params).await;

    let Ok(result_value) = serde_json::to_value(&result) else {
        let err = RpcError::new(id, -32_603, "Internal error");
        return serde_json::to_string(&err).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned());
    };

    let resp = RpcResponse::ok(id, result_value);
    serde_json::to_string(&resp).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned())
}

// ── Server entry point ─────────────────────────────────────────────────────

/// Bind to `addr`, serve the WebSocket API, and return the bound address plus
/// a [`tokio::task::JoinHandle`] for the server task.
///
/// Pass `127.0.0.1:0` to let the OS choose an ephemeral port (useful in
/// tests).
///
/// # Errors
///
/// Returns an error if binding the TCP listener fails.
pub async fn serve(
    addr: SocketAddr,
    manager: Arc<AgentManager>,
) -> Result<(SocketAddr, tokio::task::JoinHandle<()>), std::io::Error> {
    let listener = TcpListener::bind(addr).await?;
    let bound = listener.local_addr()?;
    let router = build_router(manager);
    let handle = tokio::spawn(async move {
        axum::serve(listener, router)
            .await
            .expect("axum server error");
    });
    Ok((bound, handle))
}
