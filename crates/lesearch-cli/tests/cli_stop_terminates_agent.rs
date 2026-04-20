//! Integration test: `lesearch-cli stop <id>` removes agent from `ls`.

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

async fn spawn_agent(cli_bin: &str, ws_addr: &str) -> (tokio::process::Child, String) {
    let mut child = Command::new(cli_bin)
        .args(["run", "--addr", ws_addr, "--provider", TEST_PROVIDER_ID])
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("spawn lesearch-cli run");

    let stderr = child.stderr.take().expect("child stderr");
    let mut lines = BufReader::new(stderr).lines();

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
async fn stop_removes_agent_from_ls() {
    let addr: SocketAddr = "127.0.0.1:0".parse().expect("parse addr");
    let manager = AgentManager::new();
    let (bound, _server_handle) = serve_with_providers(addr, manager, test_providers())
        .await
        .expect("serve daemon");

    let ws_addr = format!("ws://{bound}/ws");
    let cli_bin = env!("CARGO_BIN_EXE_lesearch-cli");

    // Spawn agent.
    let (mut child, agent_id) = spawn_agent(cli_bin, &ws_addr).await;
    sleep(Duration::from_millis(100)).await;

    // Verify it shows in `ls`.
    let ls_before = Command::new(cli_bin)
        .args(["ls", "--addr", &ws_addr])
        .output()
        .await
        .expect("run ls before stop");
    let out_before = String::from_utf8_lossy(&ls_before.stdout);
    assert!(
        out_before.contains(&agent_id),
        "agent {agent_id} not in ls before stop:\n{out_before}"
    );

    // Stop it.
    let stop_out = Command::new(cli_bin)
        .args(["stop", &agent_id, "--addr", &ws_addr])
        .output()
        .await
        .expect("run stop");
    assert!(
        stop_out.status.success(),
        "lesearch-cli stop failed: {}",
        String::from_utf8_lossy(&stop_out.stderr)
    );

    // Brief pause for state to update.
    sleep(Duration::from_millis(100)).await;

    // Verify it no longer appears in `ls`.
    let ls_after = Command::new(cli_bin)
        .args(["ls", "--addr", &ws_addr])
        .output()
        .await
        .expect("run ls after stop");
    let out_after = String::from_utf8_lossy(&ls_after.stdout);
    assert!(
        !out_after.contains(&agent_id),
        "agent {agent_id} still in ls after stop:\n{out_after}"
    );

    let _ = child.kill().await;
    let _ = child.wait().await;
}
