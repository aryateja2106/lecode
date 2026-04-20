//! End-to-end integration test: CLI → daemon → `TestProvider` → stdout.
//!
//! Spawns the daemon in-process on an ephemeral port with the `TestProvider`
//! registered, then launches the `lesearch-cli` binary as a subprocess,
//! captures its stdout, and asserts that all expected lines emitted by
//! `TestProvider` appear.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use lesearch_daemon::{AgentManager, serve_with_providers};
use lesearch_providers::AgentProvider;
use lesearch_providers::test_provider::{TEST_OUTPUT_LINES, TEST_PROVIDER_ID, TestProvider};
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::process::Command;
use tokio::time::timeout;

/// Build a provider map containing only the [`TestProvider`].
fn test_providers() -> Arc<HashMap<String, Arc<dyn AgentProvider>>> {
    let mut map: HashMap<String, Arc<dyn AgentProvider>> = HashMap::new();
    map.insert(TEST_PROVIDER_ID.to_owned(), Arc::new(TestProvider::new()));
    Arc::new(map)
}

/// Spawn the daemon in-process on an ephemeral port, launch the CLI binary,
/// collect its stdout, and assert expected lines are present.
#[tokio::test]
async fn cli_spawns_agent_and_streams_stdout() {
    // ── Daemon ────────────────────────────────────────────────────────────
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    let (bound, _server_handle) = serve_with_providers(addr, manager, test_providers())
        .await
        .expect("serve daemon");

    let ws_addr = format!("ws://{bound}/ws");

    // ── CLI subprocess ────────────────────────────────────────────────────
    // `env!("CARGO_BIN_EXE_lesearch-cli")` is set by Cargo at test-compile
    // time to the path of the built binary.
    let cli_bin = env!("CARGO_BIN_EXE_lesearch-cli");

    let mut child = Command::new(cli_bin)
        .args(["run", "--addr", &ws_addr, "--provider", TEST_PROVIDER_ID])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lesearch-cli");

    let stdout = child.stdout.take().expect("child stdout");
    let mut lines = BufReader::new(stdout).lines();

    // ── Collect expected lines ────────────────────────────────────────────
    let mut collected: Vec<String> = Vec::new();

    // Give the pipeline up to 10 seconds to deliver all expected lines.
    let collect_result = timeout(Duration::from_secs(10), async {
        while collected.len() < TEST_OUTPUT_LINES.len() {
            match lines.next_line().await {
                Ok(Some(line)) => collected.push(line),
                Ok(None) => break, // EOF
                Err(e) => panic!("error reading CLI stdout: {e}"),
            }
        }
    })
    .await;

    assert!(
        collect_result.is_ok(),
        "timed out waiting for CLI output; collected so far: {collected:?}"
    );

    // ── Assert output matches TestProvider's sequence ─────────────────────
    assert_eq!(
        collected.len(),
        TEST_OUTPUT_LINES.len(),
        "expected {} lines from TestProvider, got {}: {collected:?}",
        TEST_OUTPUT_LINES.len(),
        collected.len()
    );

    for (i, (got, expected)) in collected.iter().zip(TEST_OUTPUT_LINES.iter()).enumerate() {
        assert_eq!(
            got.as_str(),
            *expected,
            "line {i} mismatch: got {got:?}, expected {expected:?}"
        );
    }

    // ── Clean up ──────────────────────────────────────────────────────────
    // The CLI should exit on its own once the daemon closes the connection,
    // but kill it defensively to avoid test hangs.
    let _ = child.kill().await;
    let _ = child.wait().await;
}
