use super::prover::{compute_merkle_root, generate_dummy_rollup_proof};
use serde_json::json;

#[test]
fn test_merkle_root_generation() {
    let batch = vec![
        json!({ "id": "a", "data": "one" }),
        json!({ "id": "b", "data": "two" }),
        json!({ "id": "c", "data": "three" }),
    ];

    let root = compute_merkle_root(&batch);
    assert_eq!(root.len(), 64);
}

#[test]
fn test_rollup_proof_stub() {
    let root = "abc123";
    let proof = generate_dummy_rollup_proof(root);
    assert_eq!(proof, "rollup-proof-stub");
}