#!/bin/bash
set -e

echo "🔧 Cleaning up corrupted Cargo.toml files..."

find . -name Cargo.toml | while read -r file; do
  echo "🧹 Fixing $file"
  # Remove all invalid lines like '= { path = ".." }'
  sed -i '/^ *= *{ *path *=/d' "$file"

  # Ensure there's a [package] and a valid name line
  if ! grep -q "^package" "$file"; then
    sed -i '1i [package]\nname = "'$(basename "$(dirname "$file")")'"\nversion = "0.1.0"\nedition = "2021"\n' "$file"
  elif ! grep -q "^name *= *" "$file"; then
    sed -i '/^package/a name = "'$(basename "$(dirname "$file")")'"' "$file"
  fi
done

echo "✅ Cargo.toml structure repaired."

echo "🔧 Re-linking local workspace crates..."

find . -name Cargo.toml | grep -v "target" | while read -r consumer; do
  consumer_dir=$(dirname "$consumer")
  for provider in $(find . -name Cargo.toml | grep -v "target"); do
    provider_dir=$(dirname "$provider")
    provider_name=$(grep '^name *= *' "$provider" | cut -d'"' -f2)
    [ "$provider_name" == "" ] && continue
    [ "$consumer" == "$provider" ] && continue

    if grep -q "$provider_name" "$consumer"; then
      echo "🔁 Linking $provider_name -> $consumer"
      sed -i "/$provider_name *=.*/d" "$consumer"
      echo "$provider_name = { path = \"$(realpath --relative-to="$consumer_dir" "$provider_dir")\" }" >> "$consumer"
    fi
  done
done

echo "🧼 Cleaning build artifacts..."
cargo clean

echo "🚀 Rebuilding workspace..."
cargo build --workspace --all-targets --release
