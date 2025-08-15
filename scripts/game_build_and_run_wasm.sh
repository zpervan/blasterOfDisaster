#!/usr/bin/env bash
cd ../game

set -euo pipefail

BIN_NAME="game"

# 1) Build
cargo build --release --target wasm32-unknown-unknown

# 2) Move the built artifact to directory root
if [ -f "target/wasm32-unknown-unknown/release/${BIN_NAME}.wasm" ]; then
  cp "target/wasm32-unknown-unknown/release/${BIN_NAME}.wasm" "${BIN_NAME}.wasm"
fi

# 3) Start basic-http-server from project root, serving ./game
# Requires: cargo install basic-http-server
basic-http-server .