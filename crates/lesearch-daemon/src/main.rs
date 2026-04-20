//! `LeSearch` daemon entry point.
//!
//! Parses CLI flags, initialises tracing, registers providers, binds the
//! WebSocket server, and blocks until Ctrl-C is received.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;

use anyhow::Context as _;
use clap::Parser;
use lesearch_daemon::{AgentManager, serve_with_providers};
use lesearch_providers::AgentProvider;
use lesearch_providers::claude::ClaudeProvider;
use tracing::info;

/// `lesearch-daemon` — local AI agent supervisor.
#[derive(Debug, Parser)]
#[command(
    name = "lesearch-daemon",
    about = "LeSearch daemon — WebSocket agent supervisor",
    version
)]
struct Args {
    /// TCP address to bind (host:port). Use `127.0.0.1:0` for ephemeral port.
    #[arg(long, default_value = "127.0.0.1:6767", env = "LESEARCH_BIND")]
    addr: SocketAddr,

    /// Log level filter (e.g. `info`, `debug`, `lesearch_daemon=trace`).
    #[arg(long, default_value = "info", env = "RUST_LOG")]
    log_level: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    // ── Tracing ───────────────────────────────────────────────────────────
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::try_new(&args.log_level)
                .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("info")),
        )
        .init();

    // ── Provider registry ─────────────────────────────────────────────────
    let mut providers: HashMap<String, Arc<dyn AgentProvider>> = HashMap::new();

    providers.insert("claude".to_owned(), Arc::new(ClaudeProvider::new()));

    #[cfg(feature = "test-provider")]
    {
        use lesearch_providers::test_provider::TestProvider;
        providers.insert("test".to_owned(), Arc::new(TestProvider::new()));
        info!("test-provider registered");
    }

    let providers = Arc::new(providers);

    // ── Serve ─────────────────────────────────────────────────────────────
    let manager = AgentManager::new();
    let (bound, server_handle) = serve_with_providers(args.addr, manager, providers)
        .await
        .with_context(|| format!("failed to bind {}", args.addr))?;

    info!(addr = %bound, "lesearch-daemon listening");

    // ── Graceful shutdown on Ctrl-C ───────────────────────────────────────
    tokio::signal::ctrl_c()
        .await
        .context("failed to listen for Ctrl-C")?;

    info!("received Ctrl-C, shutting down");
    server_handle.abort();
    let _ = server_handle.await;

    Ok(())
}
