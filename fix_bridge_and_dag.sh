#!/data/data/com.termux/files/usr/bin/bash
set -e

echo "🔧 Fixing phantomnet-bridge dependencies..."

cat > phantomnet-bridge/Cargo.toml <<TOML
[package]
name = "phantomnet-bridge"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
blake3 = "1.5"
TOML

cat > phantomnet-bridge/src/lib.rs <<'RS'
use serde::{Serialize, Deserialize};
use serde_json;
use blake3;

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: u64,
    pub payload: String,
}

pub fn hash_message(msg: &Message) -> String {
    let serialized = serde_json::to_string(msg).unwrap();
    blake3::hash(serialized.as_bytes()).to_hex().to_string()
}
RS

echo "🔧 Fixing rollup-dag dependencies..."

cat > rollup-dag/Cargo.toml <<TOML
[package]
name = "rollup-dag"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
uuid = { version = "1.7", features = ["v4", "serde"] }
petgraph = "0.8"
TOML

cat > rollup-dag/src/lib.rs <<'RS'
use serde::{Serialize, Deserialize};
use uuid::Uuid;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Node {
    pub id: Uuid,
    pub name: String,
}

pub fn create_graph() -> Graph<Node, ()> {
    let mut graph = Graph::new();
    let a = graph.add_node(Node { id: Uuid::new_v4(), name: "A".into() });
    let b = graph.add_node(Node { id: Uuid::new_v4(), name: "B".into() });
    graph.add_edge(a, b, ());
    graph
}
RS

echo "🧹 Cleaning..."
cargo clean

echo "🚀 Rebuilding workspace..."
cargo build --workspace --all-targets --release
