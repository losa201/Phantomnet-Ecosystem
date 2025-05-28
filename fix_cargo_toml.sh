#!/bin/bash
set -e

echo "📦 Cleaning up duplicate [dependencies] in Cargo.toml files..."

find . -name "Cargo.toml" | while read -r file; do
  echo "🧽 Fixing $file"
  awk '
    BEGIN { in_block = ""; seen_deps = 0; skip = 0 }
    /^dependencies/ {
      seen_deps++;
      if (seen_deps > 1) { skip = 1; next }
    }
    /^.*/ && skip { skip = 0 }
    !skip { print }
  ' "$file" > "$file.fixed" && mv "$file.fixed" "$file"
done

echo "✅ All Cargo.toml files cleaned."
