#!/bin/bash

set -e

# Config
REPO_NAME="phantomnet-ecosystem"
GITHUB_USER="losa201"
REPO_URL="https://github.com/$GITHUB_USER/$REPO_NAME.git"
TAG="v1.0.0"
RELEASE_NAME="PhantomNet v1.0.0"
ZIP_PATH="../phantomnet-ecosystem.zip"

echo "✅ Assuming you're inside: $REPO_NAME"

# Verify Git initialized
if [ ! -d .git ]; then
  echo "🌱 Initializing Git..."
  git init
else
  echo "🧠 Git already initialized."
fi

# Add remote if not already added
if ! git remote | grep -q origin; then
  echo "🔗 Adding remote origin..."
  git remote add origin "$REPO_URL"
else
  echo "🔁 Remote origin already set."
fi

# Commit changes (if any)
echo "📦 Adding all files..."
git add .
git commit -m "Initial commit for PhantomNet v1.0.0" || echo "🔁 No changes to commit."

# Push to GitHub
git branch -M main
git push -u origin main

# Create tag (if it doesn't exist)
if git rev-parse "$TAG" >/dev/null 2>&1; then
  echo "🏷️ Tag $TAG already exists."
else
  git tag "$TAG"
  git push origin "$TAG"
fi

# Create GitHub Release
echo "🚀 Publishing GitHub Release..."
gh release create "$TAG" "$ZIP_PATH" \
  --title "$RELEASE_NAME" \
  --notes "PhantomNet v1.0.0 delivers:

- ✅ Zero-trust Kubernetes Operator (TypeScript)
- ✅ Real-time React Dashboard
- ✅ Full Helm chart for ecosystem deployment
- ✅ GitOps-compatible ArgoCD configs
- ✅ Grafana + Prometheus + Tempo observability
- ✅ Kyverno & Cosign policy enforcement
- ✅ Scripts and full documentation"

echo "🎉 Done! Your release is live at: https://github.com/$GITHUB_USER/$REPO_NAME/releases/tag/$TAG"
