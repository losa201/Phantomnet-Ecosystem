use once_cell::sync::Lazy;
use prometheus::{Encoder, IntCounter, IntGauge, TextEncoder, register_int_counter, register_int_gauge};
use axum::{Router, response::IntoResponse, routing::get};

pub static ZK_NODES_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!("phantomnet_zk_nodes_total", "Total zkNodes accepted").unwrap()
});

pub static ROLLUPS_TOTAL: Lazy<IntCounter> = Lazy::new(|| {
    register_int_counter!("phantomnet_rollups_total", "Total rollups generated").unwrap()
});

pub static MESH_PEERS: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!("phantomnet_mesh_peers", "Number of mesh peers connected").unwrap()
});

pub static LAST_ROLLUP_TIMESTAMP: Lazy<IntGauge> = Lazy::new(|| {
    register_int_gauge!("phantomnet_last_rollup_time", "Unix time of last rollup").unwrap()
});

async fn metrics_handler() -> impl IntoResponse {
    let encoder = TextEncoder::new();
    let mut buffer = Vec::new();
    let mf = prometheus::gather();
    encoder.encode(&mf, &mut buffer).unwrap();
    String::from_utf8(buffer).unwrap()
}

pub fn metrics_routes() -> Router {
    Router::new().route("/metrics", get(metrics_handler))
}
// Stub metrics to unblock compile
pub static ANCHORS_VERIFIED: prometheus::IntCounter = prometheus::IntCounter::new("anchors_verified", "Anchors Verified").unwrap();
pub static ANCHORS_REJECTED: prometheus::IntCounter = prometheus::IntCounter::new("anchors_rejected", "Anchors Rejected").unwrap();
