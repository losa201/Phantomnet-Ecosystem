#!/data/data/com.termux/files/usr/bin/bash
set -e

echo "🔧 Fixing phantomnet-bridge crate..."

cat > phantomnet-bridge/Cargo.toml <<TOML
[package]
name = "phantomnet-bridge"
version = "0.1.0"
edition = "2021"

[dependencies]
serde = { version = "1.0", features = ["derive"] }
blake3 = "1.5"
TOML

mkdir -p phantomnet-bridge/src

cat > phantomnet-bridge/src/lib.rs <<'RS'
use serde::{Serialize, Deserialize};
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

echo "🧹 Cleaning previous build..."
cargo clean

echo "🚀 Rebuilding workspace..."
cargo build --workspace --all-targets --release
