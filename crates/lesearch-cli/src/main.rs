//! `lesearch-cli` — command-line client for the `LeSearch` daemon.
//!
//! Connects to the daemon via WebSocket, spawns an agent, prints the assigned
//! `agent_id` to stderr, then streams all `agent.output` notification data
//! lines to stdout until the server closes the connection.
//!
//! # Usage
//!
//! ```text
//! lesearch-cli [--addr ws://127.0.0.1:6767] [--provider claude] [--worktree /path]
//! ```
//!
//! The address can also be supplied via the `LESEARCH_ADDR` environment variable.

use std::io::{self, Write as _};

use anyhow::{Context as _, bail};
use clap::Parser;
use futures::{SinkExt as _, StreamExt as _};
use tokio_tungstenite::connect_async;
use tokio_tungstenite::tungstenite::Message;

use lesearch_protocol::{RpcRequest, SpawnParams, SpawnResult};

/// Command-line arguments for the `lesearch-cli` client.
#[derive(Debug, Parser)]
#[command(
    name = "lesearch-cli",
    about = "LeSearch CLI — connect to a running daemon and stream agent output"
)]
struct Args {
    /// WebSocket address of the daemon (overrides `LESEARCH_ADDR` env var).
    ///
    /// Defaults to `ws://127.0.0.1:6767`.
    #[arg(long, env = "LESEARCH_ADDR", default_value = "ws://127.0.0.1:6767")]
    addr: String,

    /// Provider to use when spawning the agent (e.g. `claude`, `test`).
    #[arg(long, default_value = "claude")]
    provider: String,

    /// Optional working directory to pass to the agent.
    #[arg(long)]
    worktree: Option<String>,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .with_writer(std::io::stderr)
        .init();

    let args = Args::parse();
    run(args).await
}

/// Core logic extracted for testability.
async fn run(args: Args) -> anyhow::Result<()> {
    let url = &args.addr;

    tracing::debug!(%url, "connecting to daemon");

    let (mut ws, _) = connect_async(url)
        .await
        .with_context(|| format!("failed to connect to daemon at {url}"))?;

    // Build and send the agent.spawn request.
    let params = SpawnParams {
        provider: args.provider.clone(),
        worktree: args.worktree.clone(),
    };

    let request = RpcRequest {
        jsonrpc: "2.0".to_owned(),
        id: serde_json::Value::Number(1.into()),
        method: "agent.spawn".to_owned(),
        params: serde_json::to_value(&params).context("serialize SpawnParams")?,
    };

    let request_json = serde_json::to_string(&request).context("serialize agent.spawn request")?;

    ws.send(Message::Text(request_json.into()))
        .await
        .context("send agent.spawn request")?;

    // Read the spawn response (first text frame).
    let spawn_result = loop {
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
            Message::Close(_) => bail!("daemon closed connection before sending spawn response"),
            // Ignore ping/binary frames.
            _ => continue,
        }
    };

    // Print agent ID to stderr so it doesn't pollute stdout stream.
    eprintln!("agent: {}", spawn_result.agent_id);

    let stdout = io::stdout();

    // Stream subsequent agent.output notifications to stdout.
    while let Some(frame) = ws.next().await {
        let msg = frame.context("WebSocket error reading notification")?;

        let text = match msg {
            Message::Text(t) => t,
            Message::Close(_) => break,
            _ => continue,
        };

        let envelope: serde_json::Value = match serde_json::from_str::<serde_json::Value>(&text) {
            Ok(v) => v,
            Err(e) => {
                tracing::warn!("skipping non-JSON frame: {e}");
                continue;
            }
        };

        // Only handle agent.output notifications.
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
