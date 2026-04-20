//! `lesearch-cli` — command-line client for the `LeSearch` daemon.
//!
//! Connects to a running daemon via WebSocket and provides subcommands for
//! spawning agents, listing them, stopping them, and checking daemon status.
//!
//! # Usage
//!
//! ```text
//! lesearch-cli run   --provider <name> [--prompt <text>] [--worktree <path>] [--addr <url>]
//! lesearch-cli ls    [--addr <url>]
//! lesearch-cli stop  <agent-id> [--addr <url>]
//! lesearch-cli daemon status [--addr <url>]
//! ```

use std::io::{self, Write as _};

use anyhow::{Context as _, bail};
use clap::{Parser, Subcommand};
use futures::{SinkExt as _, StreamExt as _};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use lesearch_protocol::{RpcRequest, SpawnParams, SpawnResult};

// ── Top-level CLI ──────────────────────────────────────────────────────────

/// `lesearch-cli` — interact with a running `lesearch-daemon`.
#[derive(Debug, Parser)]
#[command(
    name = "lesearch-cli",
    about = "LeSearch CLI — manage AI coding agents through a running daemon",
    version
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// Spawn an agent and stream its output to stdout.
    Run(RunArgs),
    /// List currently-running agents.
    Ls(CommonArgs),
    /// Stop a running agent by ID.
    Stop(StopArgs),
    /// Daemon management subcommands.
    Daemon(DaemonArgs),
}

// ── Shared addr arg ────────────────────────────────────────────────────────

/// Arguments shared by every command that talks to the daemon.
#[derive(Debug, clap::Args)]
struct CommonArgs {
    /// WebSocket URL of the daemon.
    #[arg(long, env = "LESEARCH_ADDR", default_value = "ws://127.0.0.1:6767/ws")]
    addr: String,
}

// ── run ────────────────────────────────────────────────────────────────────

/// Arguments for `lesearch-cli run`.
#[derive(Debug, clap::Args)]
struct RunArgs {
    /// Provider to use (e.g. `claude`, `test`).
    #[arg(long, default_value = "claude")]
    provider: String,

    /// Optional prompt/task text forwarded to the agent on startup.
    #[arg(long)]
    prompt: Option<String>,

    /// Optional working directory passed to the agent.
    #[arg(long)]
    worktree: Option<String>,

    /// WebSocket URL of the daemon.
    #[arg(long, env = "LESEARCH_ADDR", default_value = "ws://127.0.0.1:6767/ws")]
    addr: String,
}

// ── stop ───────────────────────────────────────────────────────────────────

/// Arguments for `lesearch-cli stop`.
#[derive(Debug, clap::Args)]
struct StopArgs {
    /// ID of the agent to stop.
    agent_id: String,

    /// WebSocket URL of the daemon.
    #[arg(long, env = "LESEARCH_ADDR", default_value = "ws://127.0.0.1:6767/ws")]
    addr: String,
}

// ── daemon ─────────────────────────────────────────────────────────────────

/// Daemon management commands.
#[derive(Debug, clap::Args)]
struct DaemonArgs {
    #[command(subcommand)]
    command: DaemonCommands,
}

#[derive(Debug, Subcommand)]
enum DaemonCommands {
    /// Print daemon version and uptime.
    Status(CommonArgs),
}

// ── entry point ────────────────────────────────────────────────────────────

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let cli = Cli::parse();

    match cli.command {
        Commands::Run(args) => cmd_run(args).await,
        Commands::Ls(args) => cmd_ls(args).await,
        Commands::Stop(args) => cmd_stop(args).await,
        Commands::Daemon(DaemonArgs {
            command: DaemonCommands::Status(args),
        }) => cmd_daemon_status(args).await,
    }
}

// ── helpers ────────────────────────────────────────────────────────────────

/// Open a WebSocket connection to `url` and return the split sink/stream.
async fn ws_connect(
    url: &str,
) -> anyhow::Result<
    tokio_tungstenite::WebSocketStream<tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>>,
> {
    let (ws, _) = connect_async(url)
        .await
        .with_context(|| format!("failed to connect to daemon at {url}"))?;
    Ok(ws)
}

/// Send a JSON-RPC request and return the raw response envelope as a
/// `serde_json::Value`.
async fn rpc_call(
    ws: &mut tokio_tungstenite::WebSocketStream<
        tokio_tungstenite::MaybeTlsStream<tokio::net::TcpStream>,
    >,
    method: &str,
    params: serde_json::Value,
) -> anyhow::Result<serde_json::Value> {
    let req = RpcRequest {
        jsonrpc: "2.0".to_owned(),
        id: serde_json::Value::Number(1.into()),
        method: method.to_owned(),
        params,
    };
    let json = serde_json::to_string(&req).context("serialize RPC request")?;
    ws.send(Message::Text(json.into()))
        .await
        .context("send RPC request")?;

    // Read until we get a text frame (skip pings etc.).
    loop {
        let msg = ws
            .next()
            .await
            .context("connection closed before response")?
            .context("WebSocket error")?;
        match msg {
            Message::Text(t) => {
                let envelope: serde_json::Value =
                    serde_json::from_str(&t).context("parse RPC response JSON")?;
                if let Some(err) = envelope.get("error") {
                    bail!("daemon returned error: {err}");
                }
                return Ok(envelope);
            }
            Message::Close(_) => bail!("daemon closed connection before response"),
            _ => {}
        }
    }
}

// ── cmd_run ────────────────────────────────────────────────────────────────

async fn cmd_run(args: RunArgs) -> anyhow::Result<()> {
    let url = &args.addr;
    tracing::debug!(%url, "connecting for run");

    let mut ws = ws_connect(url).await?;

    let params = SpawnParams {
        provider: args.provider.clone(),
        worktree: args.worktree.clone(),
    };

    let req = RpcRequest {
        jsonrpc: "2.0".to_owned(),
        id: serde_json::Value::Number(1.into()),
        method: "agent.spawn".to_owned(),
        params: serde_json::to_value(&params).context("serialize SpawnParams")?,
    };

    let req_json = serde_json::to_string(&req).context("serialize agent.spawn")?;
    ws.send(Message::Text(req_json.into()))
        .await
        .context("send agent.spawn")?;

    // Read the spawn response.
    let spawn_result: SpawnResult = loop {
        let msg = ws
            .next()
            .await
            .context("connection closed before spawn response")?
            .context("WebSocket error reading spawn response")?;

        match msg {
            Message::Text(t) => {
                let envelope: serde_json::Value =
                    serde_json::from_str(&t).context("parse spawn response JSON")?;
                if let Some(err) = envelope.get("error") {
                    bail!("daemon returned error: {err}");
                }
                let result = envelope
                    .get("result")
                    .context("spawn response missing 'result' field")?;
                let spawn: SpawnResult =
                    serde_json::from_value(result.clone()).context("parse SpawnResult")?;
                break spawn;
            }
            Message::Close(_) => bail!("daemon closed connection before spawn response"),
            _ => {}
        }
    };

    eprintln!("agent: {}", spawn_result.agent_id);

    let stdout = io::stdout();

    // Stream agent.output notifications to stdout.
    while let Some(frame) = ws.next().await {
        let msg = frame.context("WebSocket error reading notification")?;
        let text = match msg {
            Message::Text(t) => t,
            Message::Close(_) => break,
            _ => continue,
        };

        let envelope: serde_json::Value = match serde_json::from_str(&text) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("skipping non-JSON frame: {e}");
                continue;
            }
        };

        if envelope.get("method").and_then(|m| m.as_str()) != Some("agent.output") {
            continue;
        }

        if let Some(data) = envelope
            .get("params")
            .and_then(|p| p.get("data"))
            .and_then(|d| d.as_str())
        {
            let mut out = stdout.lock();
            writeln!(out, "{data}").context("write to stdout")?;
        }
    }

    Ok(())
}

// ── cmd_ls ─────────────────────────────────────────────────────────────────

async fn cmd_ls(args: CommonArgs) -> anyhow::Result<()> {
    let mut ws = ws_connect(&args.addr).await?;
    let envelope = rpc_call(&mut ws, "agent.list", serde_json::Value::Null).await?;

    let agents = envelope
        .get("result")
        .and_then(|r| r.get("agents"))
        .and_then(|a| a.as_array())
        .context("agent.list response missing agents array")?;

    if agents.is_empty() {
        println!("No agents running.");
    } else {
        println!("{:<38}  {:<12}  STARTED_AT", "AGENT_ID", "PROVIDER");
        for agent in agents {
            let id = agent
                .get("agent_id")
                .and_then(|v| v.as_str())
                .unwrap_or("-");
            let provider = agent
                .get("provider")
                .and_then(|v| v.as_str())
                .unwrap_or("-");
            let started = agent
                .get("started_at")
                .and_then(|v| v.as_str())
                .unwrap_or("-");
            println!("{id:<38}  {provider:<12}  {started}");
        }
    }

    let _ = ws.close(None).await;
    Ok(())
}

// ── cmd_stop ───────────────────────────────────────────────────────────────

async fn cmd_stop(args: StopArgs) -> anyhow::Result<()> {
    let mut ws = ws_connect(&args.addr).await?;
    let params = serde_json::json!({ "agent_id": args.agent_id });
    let envelope = rpc_call(&mut ws, "agent.stop", params).await?;

    let stopped = envelope
        .get("result")
        .and_then(|r| r.get("stopped"))
        .and_then(serde_json::Value::as_bool)
        .unwrap_or(false);

    if stopped {
        println!("Agent {} stopped.", args.agent_id);
    } else {
        bail!("unexpected response from daemon: {envelope}");
    }

    let _ = ws.close(None).await;
    Ok(())
}

// ── cmd_daemon_status ──────────────────────────────────────────────────────

async fn cmd_daemon_status(args: CommonArgs) -> anyhow::Result<()> {
    let mut ws = ws_connect(&args.addr).await?;
    let envelope = rpc_call(&mut ws, "server.handshake", serde_json::Value::Null).await?;

    let result = envelope
        .get("result")
        .context("server.handshake response missing result")?;

    let version = result
        .get("version")
        .and_then(|v| v.as_str())
        .unwrap_or("?");
    let protocol = result
        .get("protocol")
        .and_then(|v| v.as_str())
        .unwrap_or("?");
    let uptime = result
        .get("uptime_secs")
        .and_then(serde_json::Value::as_u64)
        .unwrap_or(0);

    println!("daemon version : {version}");
    println!("protocol       : {protocol}");
    println!("uptime         : {uptime}s");

    let _ = ws.close(None).await;
    Ok(())
}
