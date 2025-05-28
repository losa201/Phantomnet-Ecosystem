#!/data/data/com.termux/files/usr/bin/bash
set -e

echo "🧼 Resetting root Cargo.toml to pure workspace manifest..."

cat << TOML > ./Cargo.toml
[workspace]
members = [
    "phantomnet-bridge",
    "phantomnetctl",
    "phantomnetd",
    "phantomnet-node",
    "phantomnet-zk",
    "rollup-dag"
]
TOML

echo "🧹 Cleaning artifacts..."
cargo clean

echo "🚀 Rebuilding full workspace..."
cargo build --workspace --all-targets --release
