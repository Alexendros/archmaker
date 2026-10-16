/**
 * Ambient types for wasm-bindgen output (alias `archmaker-wasm`).
 * Real files live under `crates/archmaker-wasm/pkg/` after `tools/build-wasm.sh`.
 */
declare module "archmaker-wasm" {
  const init: (input?: unknown) => Promise<unknown>;
  export default init;
  export function wasm_invoke(
    windowLabel: string,
    opName: string,
    inputJson: string,
  ): string;
}
