#!/usr/bin/env bash
set -euo pipefail

if ! command -v cargo-deb >/dev/null 2>&1; then
  echo "Installing cargo-deb..."
  cargo install cargo-deb
fi

cargo deb

echo "Done. .deb package is in target/debian/."
