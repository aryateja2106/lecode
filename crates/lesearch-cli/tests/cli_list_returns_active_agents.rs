//! Integration test: `lesearch-cli ls` lists agents spawned via `run`.

use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::Arc;
use std::time::Duration;

use lesearch_daemon::{AgentManager, serve_with_providers};
use lesearch_providers::AgentProvider;
use lesearch_providers::test_provider::{TEST_PROVIDER_ID, TestProvider};
use tokio::io::{AsyncBufReadExt as _, BufReader};
use tokio::process::Command;
use tokio::time::{sleep, timeout};

fn test_providers() -> Arc<HashMap<String, Arc<dyn AgentProvider>>> {
    let mut map: HashMap<String, Arc<dyn AgentProvider>> = HashMap::new();
    map.insert(TEST_PROVIDER_ID.to_owned(), Arc::new(TestProvider::new()));
    Arc::new(map)
}

/// Helper: run `lesearch-cli run` in background, wait until it prints the
/// agent-id line on stderr, and return the child + `agent_id`.
async fn spawn_agent(cli_bin: &str, ws_addr: &str) -> (tokio::process::Child, String) {
    let mut child = Command::new(cli_bin)
        .args(["run", "--addr", ws_addr, "--provider", TEST_PROVIDER_ID])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lesearch-cli run");

    let stderr = child.stderr.take().expect("child stderr");
    let mut lines = BufReader::new(stderr).lines();

    // The CLI prints "agent: <id>" on stderr.
    let agent_id = timeout(Duration::from_secs(10), async {
        while let Ok(Some(line)) = lines.next_line().await {
            if let Some(id) = line.strip_prefix("agent: ") {
                return id.trim().to_owned();
            }
        }
        panic!("did not see 'agent: <id>' on stderr");
    })
    .await
    .expect("timed out waiting for agent id");

    (child, agent_id)
}

#[tokio::test]
async fn ls_shows_two_spawned_agents() {
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    let (bound, _server_handle) = serve_with_providers(addr, manager, test_providers())
        .await
        .expect("serve daemon");

    let ws_addr = format!("ws://{bound}/ws");
    let cli_bin = env!("CARGO_BIN_EXE_lesearch-cli");

    // Spawn two agents concurrently.
    let (mut child1, id1) = spawn_agent(cli_bin, &ws_addr).await;
    let (mut child2, id2) = spawn_agent(cli_bin, &ws_addr).await;

    // Small pause to let registrations settle.
    sleep(Duration::from_millis(100)).await;

    // Run `lesearch-cli ls`.
    let ls_output = Command::new(cli_bin)
        .args(["ls", "--addr", &ws_addr])
        .output()
        .await
        .expect("run lesearch-cli ls");

    let stdout = String::from_utf8_lossy(&ls_output.stdout);

    assert!(
        ls_output.status.success(),
        "lesearch-cli ls failed: {}",
        String::from_utf8_lossy(&ls_output.stderr)
    );
    assert!(
        stdout.contains(&id1),
        "ls output missing agent {id1}:\n{stdout}"
    );
    assert!(
        stdout.contains(&id2),
        "ls output missing agent {id2}:\n{stdout}"
    );

    let _ = child1.kill().await;
    let _ = child2.kill().await;
    let _ = child1.wait().await;
    let _ = child2.wait().await;
}
