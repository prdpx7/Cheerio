#!/bin/bash
set -e

echo "Building WASM..."
cargo build --target wasm32-unknown-unknown --release

echo "Preparing dist..."
mkdir -p dist

cp target/wasm32-unknown-unknown/release/cheerio.wasm dist/
cp web/index.html dist/

cp -r assets dist/ 2>/dev/null || true

echo "Build complete. Serve dist/ with a local HTTP server."
echo "  python3 -m http.server -d dist 8080"
