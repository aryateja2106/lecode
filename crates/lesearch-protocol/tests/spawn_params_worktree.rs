//! Protocol conformance tests for [`SpawnParams::worktree`].
//!
//! These tests guard against future regressions of worktree field
//! serialization. The TDD intent: the struct and its serde round-trip were
//! verified green on first run; the tests now act as a regression harness.

use lesearch_protocol::SpawnParams;

/// Serialize `SpawnParams` with `worktree: Some(...)` and deserialize back,
/// asserting the field round-trips without loss.
#[test]
fn spawn_params_with_worktree_roundtrips() {
    let original = SpawnParams {
        provider: "claude".to_owned(),
        worktree: Some("/tmp/wt".to_owned()),
    };

    let json = serde_json::to_string(&original).expect("serialization must succeed");

    // The field must be present in the JSON output.
    assert!(
        json.contains("\"worktree\""),
        "expected 'worktree' key in JSON, got: {json}"
    );
    assert!(
        json.contains("/tmp/wt"),
        "expected worktree path in JSON, got: {json}"
    );

    let round_tripped: SpawnParams =
        serde_json::from_str(&json).expect("deserialization must succeed");

    assert_eq!(round_tripped.provider, original.provider);
    assert_eq!(round_tripped.worktree, original.worktree);
    assert_eq!(round_tripped.worktree.as_deref(), Some("/tmp/wt"));
}

/// Serialize `SpawnParams` with `worktree: None` and deserialize back,
/// asserting the field is absent in the JSON (`skip_serializing_if` policy)
/// and deserializes to `None`.
#[test]
fn spawn_params_without_worktree_roundtrips() {
    let original = SpawnParams {
        provider: "codex".to_owned(),
        worktree: None,
    };

    let json = serde_json::to_string(&original).expect("serialization must succeed");

    // Per `#[serde(skip_serializing_if = "Option::is_none")]` the key must
    // be absent entirely (not present as `null`).
    assert!(
        !json.contains("\"worktree\""),
        "expected 'worktree' key to be absent when None, got: {json}"
    );

    let round_tripped: SpawnParams =
        serde_json::from_str(&json).expect("deserialization must succeed");

    assert_eq!(round_tripped.provider, original.provider);
    assert_eq!(round_tripped.worktree, original.worktree);
    assert_eq!(round_tripped.worktree, None);
}
