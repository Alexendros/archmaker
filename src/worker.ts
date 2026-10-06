// Web Worker wrapper del adapter WASM — I7b (T-I7b-02).
// Expone la misma interfaz CorePort v0 que el adapter Tauri sin bloquear el hilo UI
// (docs/04-interfaces/tauri-wasm.md). El core sigue siendo Rust compilado a WASM;
// este fichero solo transporta mensajes. Sin red, sin FS arbitrario en runtime.

import type { CoreCommand, CoreError, CoreResult } from "./types/coreport.js";
import { typedError } from "./types/coreport.js";

export type WorkerRequest = { id: string; op: CoreCommand; input: unknown };
export type WorkerResponse =
  | { id: string; ok: true; data: unknown }
  | { id: string; ok: false; error: CoreError };

type WasmModule = {
  wasm_canonicalize(inputJson: string): string;
  wasm_digest(domain: string, canonical: string): string;
  wasm_invoke(windowLabel: string, opName: string, inputJson: string): string;
  wasm_create_draft(inputJson: string): string;
  wasm_load_catalog(inputJson: string): string;
  wasm_save_draft(inputJson: string): string;
  wasm_resolve_draft(inputJson: string): string;
  wasm_validate_draft(inputJson: string): string;
  wasm_build_manifest(inputJson: string): string;
  wasm_export_artifact(inputJson: string): string;
};

const OP_FN: Record<CoreCommand, keyof WasmModule> = {
  create_draft: "wasm_create_draft",
  load_catalog: "wasm_load_catalog",
  save_draft: "wasm_save_draft",
  resolve_draft: "wasm_resolve_draft",
  validate_draft: "wasm_validate_draft",
  build_manifest: "wasm_build_manifest",
  export_artifact: "wasm_export_artifact",
};

let wasm: WasmModule | null = null;

async function loadWasm(): Promise<WasmModule> {
  if (wasm) return wasm;
  // Artefacto generado por `wasm-pack build crates/archmaker-wasm --target web`
  // (pendiente hasta disponer de wasm-pack; ver reporte I7).
  const mod = (await import("../crates/archmaker-wasm/pkg/archmaker_wasm.js")) as WasmModule & {
    default?: () => Promise<void>;
  };
  if (typeof mod.default === "function") await mod.default();
  wasm = mod;
  return mod;
}

function parseCoreResult<T>(id: string, raw: string): WorkerResponse {
  try {
    return { id, ok: true, data: JSON.parse(raw) as T };
  } catch {
    return {
      id,
      ok: false,
      error: typedError("AM-SCHEMA-001", "archmaker-worker", "error.schema.invalidDocument"),
    };
  }
}

self.onmessage = async (event: MessageEvent<WorkerRequest>) => {
  const { id, op, input } = event.data;
  const respond = (res: WorkerResponse): void => self.postMessage(res);
  try {
    const mod = await loadWasm();
    const fn = mod[OP_FN[op]];
    const raw = (fn as (inputJson: string) => string).call(mod, JSON.stringify(input));
    respond(parseCoreResult(id, raw));
  } catch (e) {
    const message = e instanceof Error ? e.message : String(e);
    let error: CoreError;
    try {
      error = JSON.parse(message) as CoreError;
    } catch {
      error = /denied|forbidden|capability|permission/i.test(message)
        ? typedError("AM-PROTO-001", "archmaker-worker", "error.protocol.invokeDenied")
        : typedError("AM-PROTO-002", "archmaker-worker", "error.protocol.invokeFailed");
    }
    const res: CoreResult<never> = { ok: false, error };
    respond({ id, ok: false, error: res.error });
  }
};
