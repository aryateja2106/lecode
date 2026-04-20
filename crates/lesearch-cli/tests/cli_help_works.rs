//! Integration test: `lesearch-cli --help` exits 0 and lists all subcommands.

use std::process::Command;

/// Run `lesearch-cli --help` and assert it exits 0 and mentions key subcommands.
#[test]
fn cli_help_shows_all_subcommands() {
    let cli_bin = env!("CARGO_BIN_EXE_lesearch-cli");

    let output = Command::new(cli_bin)
        .arg("--help")
        .output()
        .expect("failed to run lesearch-cli --help");

    assert!(
        output.status.success(),
        "lesearch-cli --help exited non-zero: {}",
        output.status
    );

    let stdout = String::from_utf8_lossy(&output.stdout);
    let combined = format!("{stdout}{}", String::from_utf8_lossy(&output.stderr));

    for keyword in ["run", "ls", "stop", "daemon"] {
        assert!(
            combined.contains(keyword),
            "--help output missing keyword '{keyword}'.\nFull output:\n{combined}"
        );
    }
}
