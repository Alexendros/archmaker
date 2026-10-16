#!/usr/bin/env bash
# Build archmaker-wasm for the browser (wasm32 + wasm-bindgen --target web).
# Clears host RUSTFLAGS (e.g. -C target-cpu=znver5) that break wasm32.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"
unset RUSTFLAGS CARGO_ENCODED_RUSTFLAGS || true
export RUSTFLAGS=""
cargo build -p archmaker-wasm --release --target wasm32-unknown-unknown --locked
mkdir -p crates/archmaker-wasm/pkg
wasm-bindgen \
  "${CARGO_TARGET_DIR:-target}/wasm32-unknown-unknown/release/archmaker_wasm.wasm" \
  --out-dir crates/archmaker-wasm/pkg \
  --target web \
  --typescript
# Drop wasm-pack style ignore if present so Vite can serve the module in CI artifacts.
rm -f crates/archmaker-wasm/pkg/.gitignore
echo "WASM bindings written to crates/archmaker-wasm/pkg/"
