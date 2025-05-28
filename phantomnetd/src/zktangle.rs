use axum::{Router, routing::{post, get}, Json};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fs::{OpenOptions, File};
use std::io::{BufWriter, BufReader, BufRead, Write};
use chrono::Utc;
use crate::zkmesh::verify::verify_proof_groth16;
use phantomnetd::rollup_dag::RollupDag;
use phantomnetd::phantomnet_bridge::BridgeProof;
use crate::metrics::{ZK_NODES_TOTAL, ROLLUPS_TOTAL, LAST_ROLLUP_TIMESTAMP};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
pub struct ZkNode {
    pub id: String,
    pub parents: Vec<String>,
    pub ts: String,
    pub kind: String,
    pub payload: Value,
    pub proof: Option<String>,
}

#[derive(Deserialize)]
pub struct TanglePayload {
    pub parents: Vec<String>,
    pub kind: String,
    pub payload: Value,
    pub proof: Option<String>,
}

pub async fn submit_node(Json(payload): Json<TanglePayload>) -> Json<ZkNode> {
    let node = ZkNode {
        id: Uuid::new_v4().to_string(),
        parents: payload.parents,
        ts: Utc::now().to_rfc3339(),
        kind: payload.kind,
        payload: payload.payload,
        proof: payload.proof,
    };

    let file = OpenOptions::new().create(true).append(true).open("data/zktangle.log").unwrap();
    let mut writer = BufWriter::new(file);
    writeln!(writer, "{}", serde_json::to_string(&node).unwrap()).unwrap();

    
    let node_json = serde_json::to_value(&node).unwrap();
    if !verify_proof_groth16(&node_json) {
        println!("❌ zkProof rejected");
        return ZK_NODES_TOTAL.inc();
    Json(node); // You may return error here instead
    }
    ZK_NODES_TOTAL.inc();
    Json(node)
}

pub async fn read_tangle() -> Json<Value> {
    let file = File::open("data/zktangle.log").ok();
    let mut nodes = vec![];
    if let Some(f) = file {
        for line in BufReader::new(f).lines().flatten().rev().take(100) {
            if let Ok(val) = serde_json::from_str::<Value>(&line) {
                nodes.push(val);
            }
        }
    }
    Json(Value::Array(nodes))
}

pub fn tangle_routes() -> Router {
    Router::new()
        .route("/zk/tangle/submit", post(submit_node))
        .route("/api/tangle", get(read_tangle))
}