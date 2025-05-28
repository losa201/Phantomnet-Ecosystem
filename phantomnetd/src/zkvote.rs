use axum::{Router, routing::post, Json};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;

#[derive(Deserialize)]
pub struct VotePayload {
    commitment: String,
    proof: String,
}

#[derive(Serialize)]
pub struct VoteResponse {
    success: bool,
    timestamp: String,
    commitment: String,
}

pub async fn cast_vote(Json(payload): Json<VotePayload>) -> Json<VoteResponse> {
    // Placeholder validation logic — in prod validate Groth16 proof here
    let valid = payload.proof.starts_with("0x");

    let ts = Utc::now().to_rfc3339();
    let entry = json!({
        "ts": ts,
        "commitment": payload.commitment,
        "valid": valid
    });

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data/votes.log")
        .unwrap();
    writeln!(file, "{}", entry.to_string()).unwrap();

    Json(VoteResponse {
        success: valid,
        timestamp: ts,
        commitment: payload.commitment,
    })
}

pub fn vote_routes() -> Router {
    Router::new().route("/zk/vote/cast", post(cast_vote))
}use crate::zkrollup::{batch::collect_valid_batch, prover::{compute_merkle_root, generate_dummy_rollup_proof}};
use serde_json::json;
use axum::{Router, routing::post, Json};
use serde::Serialize;

#[derive(Serialize)]
pub struct RollupResponse {
    pub rollup_id: String,
    pub merkle_root: String,
    pub proof: String,
    pub nodes: Vec<String>,
}

pub async fn create_rollup(Json(payload): Json<Value>) -> Json<RollupResponse> {
    let batch_size = payload.get("batch_size").unwrap_or(&json!(10)).as_u64().unwrap_or(10) as usize;
    let batch = collect_valid_batch(batch_size);
    let merkle_root = compute_merkle_root(&batch);
    let proof = generate_dummy_rollup_proof(&merkle_root);

    let rollup_id = format!("rollup-{}", uuid::Uuid::new_v4());

    // Create and save RollupNode to tangle
    let rollup_node = json!({
        "type": "rollup",
        "id": rollup_id,
        "batch_root": merkle_root,
        "compressed": blake3::hash(batch.to_string().as_bytes()).to_hex().to_string(),
        "proof": proof,
        "nodes": batch.iter().map(|node| node.get("id").unwrap().as_str().unwrap()).collect::<Vec<_>>()
    });

    let file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open("data/zktangle.log")
        .unwrap();

    let _ = writeln!(file, "{}", rollup_node.to_string());

    Json(RollupResponse {
        rollup_id,
        merkle_root,
        proof,
        nodes: rollup_node["nodes"].as_array().unwrap().iter()
            .map(|node| node.as_str().unwrap().to_string())
            .collect(),
    })
}

pub fn rollup_routes() -> Router {
    Router::new().route("/zk/rollup/create", post(create_rollup))
}