#!/data/data/com.termux/files/usr/bin/bash
set -e

echo "🚀 Switching to PhantomNet v1.1.0-dev branch"
cd "$(dirname "$0")/.."

# Ensure repo initialized
git init
git checkout -B v1.1.0-dev

# Setup remote if not exists
if ! git remote | grep origin > /dev/null; then
  git remote add origin https://github.com/losa201/phantomnet-ecosystem.git
fi

echo "📦 Adding and committing changes..."
git add .
git commit -m "🚀 PhantomNet v1.1.0-dev: Autopilot, Prometheus, Dashboard Enhancements"

echo "🏷️ Tagging v1.1.0-dev"
git tag v1.1.0-dev || true

echo "📤 Pushing branch and tag..."
git push origin v1.1.0-dev --force
git push origin v1.1.0-dev --tags

echo "✅ Done: https://github.com/losa201/phantomnet-ecosystem/tree/v1.1.0-dev"
