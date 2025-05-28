use axum::{extract::Json, routing::post, Router};
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;

use std::fs::OpenOptions;
use std::io::Write;
use chrono::Utc;
use crate::metrics::{ANCHORS_VERIFIED, ANCHORS_REJECTED};


#[derive(Debug, Deserialize)]
pub struct AnchorPayload {
    pub chain_id: String,
    pub merkle_root: String,
    pub zk_proof: String, // base64-encoded zk proof string
}

pub async fn anchor_handler(Json(payload): Json<AnchorPayload>) -> Json<serde_json::Value> {
    // Placeholder: simulate successful proof verification
    let valid = payload.zk_proof.starts_with("zk");
    // Append to anchors.log
    let timestamp = Utc::now().to_rfc3339();
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("data/anchors.log")
        .expect("Cannot open log file");

    let log_entry = format!("{{\"ts\":\"{}\",\"chain\":\"{}\",\"root\":\"{}\",\"valid\":{}}}\n",
        timestamp, payload.chain_id, payload.merkle_root, valid);
    let _ = file.write_all(log_entry.as_bytes());
 // fake check

    if valid {
        Json(json!({
            "status": "verified",
            "chain": payload.chain_id,
            "root": payload.merkle_root
        }))
    } else {
        Json(json!({
            "status": "rejected",
            "reason": "invalid proof format"
        }))
    }
}

pub fn zk_anchor_routes() -> Router {
    Router::new().route("/zk/anchor", post(anchor_handler))
}