#!/usr/bin/env bash
# Tres invocaciones Playwright SEPARADAS (no --repeat-each=3).
# Cada intento usa --workers=1 --retries=0 y directorio de salida propio.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
cd "$ROOT"

BASE_ARGS=(test wasm-flow.playwright.test.ts --workers=1 --retries=0)

for attempt in 1 2 3; do
  echo "== F7-E2E-X3 intento ${attempt}/3 =="
  OUT="test-results/e2e-x3-${attempt}"
  rm -rf "${OUT}"
  pnpm exec playwright "${BASE_ARGS[@]}" --output="${OUT}"
done

echo "OK: tres invocaciones Playwright consecutivas (workers=1, retries=0)"
