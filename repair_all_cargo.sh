#!/bin/bash
set -e

echo "🔧 Regenerating minimal Cargo.toml files across PhantomNet ecosystem..."

declare -A projects=(
  ["phantomnet-node"]="jsonrpc-core jsonrpc-http-server phantomnet-zk log env_logger"
  ["phantomnet-zk"]="ark-std ark-relations ark-r1cs-std ark-groth16 ark-bn254 ark-ff ark-snark rand"
  ["phantomnet-bridge"]="tokio serde serde_json reqwest"
  ["phantomnetctl"]="clap log env_logger"
  ["phantomnetd"]="tokio tracing anyhow"
  ["rollup-dag"]="uuid"
)

for dir in "${!projects[@]}"; do
  echo "🛠️ Repairing $dir/Cargo.toml"
  pkg_name="$dir"
  deps="${projects[$dir]}"

  mkdir -p "$dir"
  {
    echo "[package]"
    echo "name = \"$pkg_name\""
    echo "version = \"0.1.0\""
    echo "edition = \"2021\""
    echo ""
    echo "[dependencies]"
    for dep in $deps; do
      echo "$dep = \"*\""
    done
  } > "$dir/Cargo.toml"
done

echo "📦 Repairing workspace root Cargo.toml..."
cat << ROOT > Cargo.toml
[workspace]
members = [
  "phantomnet-node",
  "phantomnet-zk",
  "phantomnet-bridge",
  "phantomnetctl",
  "phantomnetd",
  "rollup-dag"
]
resolver = "2"
ROOT

echo "✅ All Cargo.toml files have been regenerated cleanly."
