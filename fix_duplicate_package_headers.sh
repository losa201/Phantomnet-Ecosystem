#!/data/data/com.termux/files/usr/bin/bash
set -e

echo "🧼 Scanning for duplicate [package] headers..."
find . -name "Cargo.toml" | while read -r file; do
  if grep -q '^package' "$file"; then
    count=$(grep -c '^package' "$file")
    if [ "$count" -gt 1 ]; then
      echo "⚙️  Fixing $file ($count [package] headers)"
      awk '
        BEGIN {in_package=0; seen_package=0}
        /^package/ {
          if (seen_package) {
            in_package=1
            next
          } else {
            seen_package=1
          }
        }
        /^.*/ { in_package=0 }
        !in_package { print }
      ' "$file" > "$file.fixed" && mv "$file.fixed" "$file"
    fi
  fi
done

echo "✅ All duplicate [package] headers removed."
