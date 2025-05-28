#!/data/data/com.termux/files/usr/bin/bash
set -e

echo "🔧 Building dashboard for GitHub Pages..."
cd "$(dirname "$0")/../dashboard"
npm install
npm run build

echo "📦 Copying build to gh-pages/"
rm -rf ../gh-pages
mkdir -p ../gh-pages
cp -r build/* ../gh-pages

echo "🚀 Deploying to gh-pages branch..."
cd ..
git checkout -B gh-pages
git add gh-pages
git commit -m "🌐 GitHub Pages dashboard deployment"
git subtree push --prefix gh-pages origin gh-pages

echo "✅ Dashboard deployed to GitHub Pages."
