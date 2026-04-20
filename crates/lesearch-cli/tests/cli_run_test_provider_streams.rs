//! Integration test: `lesearch-cli run --provider test` streams "hello" lines.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use lesearch_daemon::{AgentManager, serve_with_providers};
use lesearch_providers::AgentProvider;
use lesearch_providers::test_provider::{TEST_PROVIDER_ID, TestProvider};
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::process::Command;
use tokio::time::timeout;

fn test_providers() -> Arc<HashMap<String, Arc<dyn AgentProvider>>> {
    let mut map: HashMap<String, Arc<dyn AgentProvider>> = HashMap::new();
    map.insert(TEST_PROVIDER_ID.to_owned(), Arc::new(TestProvider::new()));
    Arc::new(map)
}

#[tokio::test]
async fn run_test_provider_streams_hello() {
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    let (bound, _server_handle) = serve_with_providers(addr, manager, test_providers())
        .await
        .expect("serve daemon");

    let ws_addr = format!("ws://{bound}/ws");
    let cli_bin = env!("CARGO_BIN_EXE_lesearch-cli");

    let mut child = Command::new(cli_bin)
        .args(["run", "--addr", &ws_addr, "--provider", TEST_PROVIDER_ID])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lesearch-cli");

    let stdout = child.stdout.take().expect("child stdout");
    let mut lines = BufReader::new(stdout).lines();

    let mut found_hello = false;
    let result = timeout(Duration::from_secs(10), async {
        while let Ok(Some(line)) = lines.next_line().await {
            if line.contains("hello") {
                found_hello = true;
                break;
            }
        }
    })
    .await;

    assert!(
        result.is_ok(),
        "timed out waiting for 'hello' in CLI output"
    );
    assert!(found_hello, "CLI output did not contain 'hello'");

    let _ = child.kill().await;
    let _ = child.wait().await;
}
