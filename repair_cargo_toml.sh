#!/bin/bash
set -e

echo "🔧 Cleaning duplicate [dependencies] headers..."

find . -name Cargo.toml | while read -r file; do
  echo "🛠️ Fixing $file"
  awk '
    BEGIN { in_deps = 0; found = 0 }
    /^dependencies/ {
      found++;
      if (found > 1) { in_deps = 1; next }
    }
    /^[^]]+/ {
      if (in_deps) { in_deps = 0 }
    }
    !in_deps { print }
  ' "$file" > "$file.tmp" && mv "$file.tmp" "$file"
done

echo "✅ Finished cleanup."
