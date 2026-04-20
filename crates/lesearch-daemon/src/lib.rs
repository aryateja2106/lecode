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
use tokio::sync::mpsc;

use lesearch_protocol::{
    AgentOutputParams, RpcError, RpcNotification, RpcRequest, RpcResponse, SpawnParams,
    SpawnResult, StreamIds,
};
use lesearch_providers::{AgentEvent, AgentProvider, AgentSpec};

/// Returns the compiled protocol version this daemon speaks.
#[must_use]
pub const fn protocol_version() -> &'static str {
    lesearch_protocol::version()
}

// ── Stream ID allocator ────────────────────────────────────────────────────

/// Global monotonic counter for stream IDs.
static STREAM_ID_COUNTER: AtomicU16 = AtomicU16::new(1);

/// Allocate the next non-zero stream ID.
fn next_stream_id() -> u16 {
    loop {
        let prev = STREAM_ID_COUNTER.fetch_add(1, Ordering::Relaxed);
        if prev != 0 {
            return prev;
        }
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
#[derive(Debug)]
struct AgentEntry {
    /// Assigned stream channel IDs (stable for the lifetime of the agent).
    #[allow(dead_code)]
    streams: StreamIds,
    /// Provider name used to spawn this agent.
    #[allow(dead_code)]
    provider: String,
}

// ── Agent manager ──────────────────────────────────────────────────────────

/// Shared agent manager state, accessible from all WebSocket handlers.
pub struct AgentManager {
    agents: Mutex<HashMap<String, AgentEntry>>,
    /// Registered provider implementations keyed by provider name.
    providers: Mutex<HashMap<String, Box<dyn AgentProvider>>>,
}

impl std::fmt::Debug for AgentManager {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("AgentManager").finish_non_exhaustive()
    }
}

impl Default for AgentManager {
    fn default() -> Self {
        Self {
            agents: Mutex::new(HashMap::new()),
            providers: Mutex::new(HashMap::new()),
        }
    }
}

impl AgentManager {
    /// Create a new, empty agent manager.
    #[must_use]
    pub fn new() -> Arc<Self> {
        Arc::new(Self::default())
    }

    /// Register a provider implementation under the given name.
    pub async fn add_provider(&self, name: impl Into<String>, provider: impl AgentProvider) {
        self.providers
            .lock()
            .await
            .insert(name.into(), Box::new(provider));
    }

    /// Register a new agent, spawn it via the named provider, and stream its
    /// events back over `ws_tx` as serialised JSON-RPC notifications.
    pub async fn register_agent(
        &self,
        params: SpawnParams,
        ws_tx: mpsc::UnboundedSender<String>,
    ) -> SpawnResult {
        let agent_id = uuid::Uuid::now_v7().to_string();
        let streams = alloc_stream_ids();
        let entry = AgentEntry {
            streams,
            provider: params.provider.clone(),
        };
        self.agents.lock().await.insert(agent_id.clone(), entry);

        self.try_spawn_event_task(agent_id.clone(), params, ws_tx)
            .await;

        SpawnResult { agent_id, streams }
    }

    /// Attempt to find the named provider, spawn the agent, and start an
    /// event-routing task that forwards output to `ws_tx`.
    async fn try_spawn_event_task(
        &self,
        agent_id: String,
        params: SpawnParams,
        ws_tx: mpsc::UnboundedSender<String>,
    ) {
        // Build the spec before locking the provider map.
        let spec = AgentSpec {
            worktree: params.worktree,
        };

        // Lock, call spawn() (which returns a future), await it, unlock.
        // We hold the lock only for the async call to spawn(); real providers
        // return quickly (they just fork a process).
        let handle_result = {
            let providers = self.providers.lock().await;
            match providers.get(&params.provider) {
                None => {
                    tracing::warn!(
                        provider = %params.provider,
                        "unknown provider — no events will be streamed"
                    );
                    return;
                }
                Some(provider) => provider.spawn(&spec).await,
            }
        };

        match handle_result {
            Err(e) => {
                tracing::error!(
                    provider = %params.provider,
                    error = %e,
                    "provider spawn failed"
                );
            }
            Ok(mut handle) => {
                tokio::spawn(async move {
                    while let Some(event) = handle.events.recv().await {
                        let notif = event_to_notification(&agent_id, event);
                        match serde_json::to_string(&notif) {
                            Ok(json) => {
                                if ws_tx.send(json).is_err() {
                                    // Client disconnected.
                                    break;
                                }
                            }
                            Err(e) => {
                                tracing::error!(error = %e, "failed to serialise agent event");
                            }
                        }
                    }
                });
            }
        }
    }
}

/// Serialise an [`AgentEvent`] into a JSON-RPC [`RpcNotification`].
fn event_to_notification(agent_id: &str, event: AgentEvent) -> RpcNotification {
    match event {
        AgentEvent::StreamChunk { stream, data } => {
            let params = AgentOutputParams {
                agent_id: agent_id.to_owned(),
                stream: stream.as_str().to_owned(),
                data,
            };
            RpcNotification::new(
                "agent.output",
                serde_json::to_value(params).unwrap_or(serde_json::Value::Null),
            )
        }
        AgentEvent::Exited { code } => RpcNotification::new(
            "agent.exited",
            serde_json::json!({ "agentId": agent_id, "code": code }),
        ),
        // Non-exhaustive enum: forward-compatible catch-all.
        _ => RpcNotification::new("agent.event", serde_json::json!({ "agentId": agent_id })),
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

/// Drive a single WebSocket connection.
///
/// Creates a per-connection unbounded mpsc channel. A `tokio::select!` loop
/// either forwards an outbound JSON string from the channel as a text frame,
/// or receives an inbound client message and dispatches it. This keeps a
/// single task without requiring `WebSocket::split`.
async fn handle_socket(mut socket: WebSocket, state: AppState) {
    let (ws_tx, mut ws_rx) = mpsc::unbounded_channel::<String>();

    loop {
        tokio::select! {
            // Inbound: client sent a frame.
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        let reply = dispatch(&state, &text, ws_tx.clone()).await;
                        if socket.send(Message::Text(reply.into())).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_)) | Err(_)) | None => break,
                    Some(Ok(_)) => {} // ignore ping/pong/binary
                }
            }
            // Outbound: an agent event notification is ready.
            Some(json) = ws_rx.recv() => {
                if socket.send(Message::Text(json.into())).await.is_err() {
                    break;
                }
            }
        }
    }
}

/// Fallback serialization for error responses.
const INTERNAL_ERROR_JSON: &str =
    r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32603,"message":"Internal error"}}"#;

/// Parse a JSON-RPC request and route it to the correct handler.
async fn dispatch(state: &AppState, raw: &str, ws_tx: mpsc::UnboundedSender<String>) -> String {
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
        handle_agent_spawn(state, id, req.params, ws_tx).await
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
    ws_tx: mpsc::UnboundedSender<String>,
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

    let result = state.manager.register_agent(spawn_params, ws_tx).await;

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
