#!/usr/bin/env bash
set -euo pipefail

echo "→ Checking for cargo-deb..."
if ! command -v cargo-deb > /dev/null 2>&1; then
  cargo install cargo-deb
fi

echo "→ Building .deb package..."
cargo deb

echo "✓ .deb package is in target/debian/"
