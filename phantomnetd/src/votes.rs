use axum::{Router, routing::get, response::Json};
use serde_json::Value;
use std::fs::File;
use std::io::{BufReader, BufRead};

pub async fn votes_history() -> Json<Value> {
    let file = File::open("data/votes.log").ok();
    let mut votes = vec![];
    if let Some(f) = file {
        for line in BufReader::new(f).lines().flatten().rev().take(100) {
            if let Ok(val) = serde_json::from_str::<Value>(&line) {
                votes.push(val);
            }
        }
    }
    Json(Value::Array(votes))
}

pub fn votes_routes() -> Router {
    Router::new().route("/api/votes", get(votes_history))
}