//! `LeSearch` daemon core.
//!
//! Hosts the agent manager, protocol router, PTY pool, storage writer, and
//! A2A gateway. Exposes a `WebSocket` endpoint on `127.0.0.1:6767` by default.
//!
//! See `docs/PRD.md`, `docs/SYSTEM_DESIGN.md`, and `docs/protocol-v0.1.md`.

#![doc(html_root_url = "https://docs.rs/lesearch-daemon/0.0.1")]

pub mod agent_manager;

use std::collections::HashMap;
use std::hash::BuildHasher;
use std::net::SocketAddr;
use std::sync::Arc;
use std::sync::atomic::{AtomicU16, Ordering};

use axum::Router;
use axum::extract::State;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::response::IntoResponse;
use axum::routing::get;
use chrono::{DateTime, Utc};
use futures::{SinkExt as _, StreamExt as _};
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tokio::sync::mpsc;

use lesearch_protocol::{RpcError, RpcRequest, RpcResponse, SpawnParams, SpawnResult, StreamIds};
use lesearch_providers::AgentProvider;

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
    provider: String,
    /// UTC timestamp when the agent was spawned.
    started_at: DateTime<Utc>,
    /// Live child process handle (used for `agent.stop`).
    child: Option<tokio::process::Child>,
}

/// Public summary of a running agent, returned by `agent.list`.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct AgentSummary {
    /// Stable agent identifier.
    pub agent_id: String,
    /// Provider that was used to spawn this agent.
    pub provider: String,
    /// ISO-8601 UTC timestamp of when the agent was started.
    pub started_at: String,
}

/// Shared agent manager state, accessible from all WebSocket handlers.
#[derive(Debug, Default)]
pub struct AgentManager {
    agents: Mutex<HashMap<String, AgentEntry>>,
    /// Timestamp when the daemon started (used for uptime calculation).
    started_at: std::sync::OnceLock<DateTime<Utc>>,
}

impl AgentManager {
    /// Create a new, empty agent manager.
    #[must_use]
    pub fn new() -> Arc<Self> {
        let mgr = Arc::new(Self::default());
        // Record startup time immediately.
        let _ = mgr.started_at.set(Utc::now());
        mgr
    }

    /// How many seconds since this manager was created.
    #[must_use]
    pub fn uptime_secs(&self) -> u64 {
        self.started_at.get().map_or(0, |t| {
            (Utc::now() - t)
                .num_seconds()
                .try_into()
                .unwrap_or_default()
        })
    }

    /// Register a new agent and return its stable [`SpawnResult`].
    ///
    /// Allocates three distinct non-zero stream IDs for the agent's stdio
    /// channels.
    pub async fn register_agent(
        &self,
        params: &SpawnParams,
        child: tokio::process::Child,
    ) -> SpawnResult {
        let agent_id = uuid::Uuid::now_v7().to_string();
        let streams = alloc_stream_ids();
        let entry = AgentEntry {
            streams,
            provider: params.provider.clone(),
            started_at: Utc::now(),
            child: Some(child),
        };
        self.agents.lock().await.insert(agent_id.clone(), entry);
        SpawnResult { agent_id, streams }
    }

    /// List all currently-registered agents.
    pub async fn list_agents(&self) -> Vec<AgentSummary> {
        self.agents
            .lock()
            .await
            .iter()
            .map(|(id, entry)| AgentSummary {
                agent_id: id.clone(),
                provider: entry.provider.clone(),
                started_at: entry.started_at.to_rfc3339(),
            })
            .collect()
    }

    /// Stop a specific agent by ID. Returns `true` if the agent existed.
    pub async fn stop_agent(&self, agent_id: &str) -> bool {
        let mut agents = self.agents.lock().await;
        if let Some(entry) = agents.get_mut(agent_id) {
            if let Some(mut child) = entry.child.take() {
                // Best-effort kill; ignore errors.
                let _ = child.kill().await;
                let _ = child.wait().await;
            }
            agents.remove(agent_id);
            true
        } else {
            false
        }
    }
}

// ── Axum WebSocket server ──────────────────────────────────────────────────

/// Shared application state threaded through axum handlers.
#[derive(Clone)]
struct AppState {
    manager: Arc<AgentManager>,
    /// Provider registry: maps provider id to arc provider.
    providers: Arc<HashMap<String, Arc<dyn AgentProvider>>>,
}

/// Build an axum [`Router`] wired to the given [`AgentManager`] and provider map.
///
/// The `providers` map is generalized over its hasher so callers may pass any
/// [`HashMap`] variant (e.g. `AHashMap`, `FxHashMap`).
pub fn build_router<S>(
    manager: Arc<AgentManager>,
    providers: &Arc<HashMap<String, Arc<dyn AgentProvider>, S>>,
) -> Router
where
    S: BuildHasher + Send + Sync + 'static,
{
    let providers: Arc<HashMap<String, Arc<dyn AgentProvider>>> = Arc::new(
        providers
            .iter()
            .map(|(k, v)| (k.clone(), Arc::clone(v)))
            .collect(),
    );
    let state = AppState { manager, providers };
    Router::new()
        .route("/ws", get(ws_handler))
        .with_state(state)
}

/// HTTP upgrade handler — promotes the connection to a WebSocket.
async fn ws_handler(ws: WebSocketUpgrade, State(state): State<AppState>) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

/// Drive a single WebSocket connection, dispatching JSON-RPC messages.
async fn handle_socket(socket: WebSocket, state: AppState) {
    let (mut ws_tx, mut ws_rx) = socket.split();
    let (out_tx, mut out_rx) = mpsc::channel::<String>(64);

    let forward_handle = tokio::spawn(async move {
        while let Some(json) = out_rx.recv().await {
            if ws_tx.send(Message::Text(json.into())).await.is_err() {
                break;
            }
        }
        let _ = ws_tx.close().await;
    });

    while let Some(Ok(msg)) = ws_rx.next().await {
        let text = match msg {
            Message::Text(t) => t,
            Message::Close(_) => break,
            _ => continue,
        };

        let reply = dispatch(&state, &text, out_tx.clone()).await;
        if out_tx.send(reply).await.is_err() {
            break;
        }
    }

    drop(out_tx);
    let _ = forward_handle.await;
}

/// Fallback serialization for error responses.
const INTERNAL_ERROR_JSON: &str =
    r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32603,"message":"Internal error"}}"#;

/// Parse a JSON-RPC request and route it to the correct handler.
async fn dispatch(state: &AppState, raw: &str, out_tx: mpsc::Sender<String>) -> String {
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

    match req.method.as_str() {
        "agent.spawn" => handle_agent_spawn(state, id, req.params, out_tx).await,
        "agent.list" => handle_agent_list(state, id).await,
        "agent.stop" => handle_agent_stop(state, id, req.params).await,
        "server.handshake" => handle_server_handshake(state, id),
        _ => {
            let err = RpcError::new(id, -32_601, "Method not found");
            serde_json::to_string(&err).unwrap_or_else(|_| {
                r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32601,"message":"Method not found"}}"#
                    .to_owned()
            })
        }
    }
}

/// Notification frame sent for each chunk of agent output.
#[derive(serde::Serialize)]
struct OutputNotification<'a> {
    jsonrpc: &'a str,
    method: &'a str,
    params: OutputParams<'a>,
}

/// Parameters of an `agent.output` notification.
#[derive(serde::Serialize)]
struct OutputParams<'a> {
    agent_id: &'a str,
    data: &'a str,
}

/// Handle `agent.spawn` — validate params, register agent, spawn provider,
/// then stream `agent.output` notifications back to the client.
async fn handle_agent_spawn(
    state: &AppState,
    id: serde_json::Value,
    params: serde_json::Value,
    out_tx: mpsc::Sender<String>,
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

    let Some(provider) = state.providers.get(&spawn_params.provider).map(Arc::clone) else {
        let err = RpcError::new(id, -32_602, "Unknown provider");
        return serde_json::to_string(&err).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned());
    };

    let provider_params = lesearch_providers::SpawnParams {
        label: uuid::Uuid::now_v7().to_string(),
        provider: spawn_params.provider.clone(),
        prompt: None,
        worktree: spawn_params.worktree.clone(),
    };

    let spec = match agent_manager::build_spec(&provider_params) {
        Ok(s) => s,
        Err(e) => {
            let err = RpcError::new(id, -32_603, &e.to_string());
            return serde_json::to_string(&err).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned());
        }
    };

    let mut provider_child = match provider.spawn(&spec).await {
        Ok(r) => r.child,
        Err(e) => {
            let err = RpcError::new(id, -32_603, &e.to_string());
            return serde_json::to_string(&err).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned());
        }
    };

    // Take stdout *before* moving child into the manager.
    let stdout = provider_child.stdout.take();

    // Register with the manager (takes ownership of child for lifecycle management).
    let result = state
        .manager
        .register_agent(&spawn_params, provider_child)
        .await;
    let agent_id = result.agent_id.clone();

    if let Some(stdout) = stdout {
        let agent_id_owned = agent_id;
        let tx = out_tx.clone();
        tokio::spawn(async move {
            let reader = BufReader::new(stdout);
            let mut lines = reader.lines();
            while let Ok(Some(line)) = lines.next_line().await {
                let notif = OutputNotification {
                    jsonrpc: "2.0",
                    method: "agent.output",
                    params: OutputParams {
                        agent_id: &agent_id_owned,
                        data: &line,
                    },
                };
                let Ok(json) = serde_json::to_string(&notif) else {
                    continue;
                };
                if tx.send(json).await.is_err() {
                    break;
                }
            }
        });
    }

    let Ok(result_value) = serde_json::to_value(&result) else {
        let err = RpcError::new(id, -32_603, "Internal error");
        return serde_json::to_string(&err).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned());
    };

    let resp = RpcResponse::ok(id, result_value);
    serde_json::to_string(&resp).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned())
}

/// Handle `agent.list` — return all currently-running agents.
async fn handle_agent_list(state: &AppState, id: serde_json::Value) -> String {
    let agents = state.manager.list_agents().await;
    let payload = serde_json::json!({ "agents": agents });
    let resp = RpcResponse::ok(id, payload);
    serde_json::to_string(&resp).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned())
}

/// Parameters for `agent.stop`.
#[derive(serde::Deserialize)]
struct StopParams {
    agent_id: String,
}

/// Handle `agent.stop` — kill the named agent's subprocess.
async fn handle_agent_stop(
    state: &AppState,
    id: serde_json::Value,
    params: serde_json::Value,
) -> String {
    let stop_params: StopParams = if let Ok(p) = serde_json::from_value(params) {
        p
    } else {
        let err = RpcError::new(id, -32_602, "Invalid params: expected {agent_id}");
        return serde_json::to_string(&err).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned());
    };

    let stopped = state.manager.stop_agent(&stop_params.agent_id).await;
    if stopped {
        let resp = RpcResponse::ok(id, serde_json::json!({ "stopped": true }));
        serde_json::to_string(&resp).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned())
    } else {
        let err = RpcError::new(id, -32_602, "Agent not found");
        serde_json::to_string(&err).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned())
    }
}

/// Handle `server.handshake` — return daemon version and uptime.
fn handle_server_handshake(state: &AppState, id: serde_json::Value) -> String {
    let uptime_secs = state.manager.uptime_secs();
    let payload = serde_json::json!({
        "version": env!("CARGO_PKG_VERSION"),
        "protocol": lesearch_protocol::version(),
        "uptime_secs": uptime_secs,
    });
    let resp = RpcResponse::ok(id, payload);
    serde_json::to_string(&resp).unwrap_or_else(|_| INTERNAL_ERROR_JSON.to_owned())
}

// ── Server entry points ────────────────────────────────────────────────────

/// Bind to `addr`, serve the WebSocket API with an empty provider map.
///
/// Pass `127.0.0.1:0` to let the OS choose an ephemeral port.
///
/// # Errors
///
/// Returns an error if binding the TCP listener fails.
pub async fn serve(
    addr: SocketAddr,
    manager: Arc<AgentManager>,
) -> Result<(SocketAddr, tokio::task::JoinHandle<()>), std::io::Error> {
    let providers: Arc<HashMap<String, Arc<dyn AgentProvider>>> = Arc::new(HashMap::new());
    serve_with_providers(addr, manager, providers).await
}

/// Bind to `addr` and serve with a pre-populated provider registry.
///
/// Use this in tests or custom launchers where specific providers must be
/// available. Pass `127.0.0.1:0` for an ephemeral port.
///
/// # Errors
///
/// Returns an error if binding the TCP listener fails.
pub async fn serve_with_providers<S>(
    addr: SocketAddr,
    manager: Arc<AgentManager>,
    providers: Arc<HashMap<String, Arc<dyn AgentProvider>, S>>,
) -> Result<(SocketAddr, tokio::task::JoinHandle<()>), std::io::Error>
where
    S: BuildHasher + Send + Sync + 'static,
{
    let listener = TcpListener::bind(addr).await?;
    let bound = listener.local_addr()?;
    let router = build_router(manager, &providers);
    let handle = tokio::spawn(async move {
        axum::serve(listener, router)
            .await
            .expect("axum server error");
    });
    Ok((bound, handle))
}
