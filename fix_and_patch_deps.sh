#!/usr/bin/env bash
set -e

echo "🔍 Scanning local workspace members..."
CRATES=$(find . -name Cargo.toml | grep -v target | grep -v "backup" | grep -v "node_modules")

echo "🔧 Updating local path references..."
for crate_path in $CRATES; do
  crate_dir=$(dirname "$crate_path")
  crate_name=$(toml get "$crate_path" package.name | sed 's/"//g')

  if [[ -z "$crate_name" ]]; then
    echo "⚠️  Skipping empty crate name in $crate_path"
    continue
  fi

  for dep_toml in $CRATES; do
    if [ "$crate_path" = "$dep_toml" ]; then continue; fi
    if grep -q "$crate_name" "$dep_toml"; then
      echo "🔁 Patching $crate_name in $dep_toml"
      sed -i "/$crate_name *=.*/d" "$dep_toml"
      echo "$crate_name = { path = \"$(realpath --relative-to=$(dirname "$dep_toml") "$crate_dir")\" }" >> "$dep_toml"
    fi
  done
done

echo "🧠 Adding missing external dependencies (serde, petgraph)..."
for file in $(find . -name lib.rs); do
  dir=$(dirname "$file")
  toml="$dir/../Cargo.toml"

  grep -q 'use serde' "$file" && ! grep -q 'dependencies' "$toml" && echo '[dependencies]' >> "$toml"
  grep -q 'use serde' "$file" && ! grep -q 'serde' "$toml" && echo 'serde = { version = "1", features = ["derive"] }' >> "$toml"

  grep -q 'use petgraph' "$file" && ! grep -q 'petgraph' "$toml" && echo 'petgraph = "0.6"' >> "$toml"
done

echo "🧹 Cleaning build artifacts..."
cargo clean

echo "🚀 Rebuilding workspace..."
cargo build --workspace --all-targets --release
