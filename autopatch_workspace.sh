#!/data/data/com.termux/files/usr/bin/bash
set -e

echo "🔍 Scanning workspace crates..."
CRATES=$(find . -mindepth 2 -name Cargo.toml | grep -v target)

echo "🩺 Autopatching crates..."
for toml in $CRATES; do
  crate_dir=$(dirname "$toml")
  crate_name=$(basename "$crate_dir")

  if ! grep -q '^package' "$toml"; then
    echo "🛠️ Injecting [package] into $toml"
    {
      echo '[package]'
      echo "name = \"$crate_name\""
      echo 'version = "0.1.0"'
      echo 'edition = "2021"'
      echo
      cat "$toml"
    } > "$toml.fixed" && mv "$toml.fixed" "$toml"
  elif ! grep -q '^name\s*=' "$toml"; then
    echo "🛠️ Fixing missing 'name' in $toml"
    sed -i "/^package/a name = \"$crate_name\"\nversion = \"0.1.0\"\nedition = \"2021\"" "$toml"
  fi

  mkdir -p "$crate_dir/src"
  if [ ! -f "$crate_dir/src/lib.rs" ] && [ ! -f "$crate_dir/src/main.rs" ]; then
    echo "fn main() { println!(\"$crate_name placeholder\"); }" > "$crate_dir/src/main.rs"
  fi
done

echo "📦 Writing clean root Cargo.toml..."
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

echo "🧹 Cleaning build artifacts..."
cargo clean

echo "🚀 Building workspace..."
cargo build --workspace --all-targets --release
