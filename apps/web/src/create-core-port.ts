import type { CorePort } from "./coreport.js";
import { MockCorePort } from "./mock-core-port.js";
import { TauriCorePort } from "./tauri-core-port.js";

export type PortKind = "tauri" | "wasm" | "mock";

export type CreateCorePortResult =
  { ok: true; port: CorePort; kind: PortKind } | { ok: false; error: string };

/**
 * Mock solo con opt-in explícito en desarrollo. Producción nunca admite mock.
 * Usado por F7 para demostrar fallo cerrado sin motor real.
 */
export function mockAllowed(input: {
  isProd: boolean;
  search: string;
  localStorageMock: string | null;
}): boolean {
  if (input.isProd) return false;
  const params = new URLSearchParams(
    input.search.startsWith("?") ? input.search : `?${input.search}`,
  );
  return params.get("mock") === "1" || input.localStorageMock === "1";
}

function allowMock(): boolean {
  if (typeof window === "undefined") return false;
  return mockAllowed({
    isProd: import.meta.env.PROD,
    search: window.location.search,
    localStorageMock: window.localStorage.getItem("archmaker.mock"),
  });
}

/**
 * Desktop → TauriCorePort; browser → WasmCorePort.
 * Mock only with explicit ?mock=1 (dev) or localStorage flag — never silently.
 * WasmCorePort is loaded dynamically so mock/a11y paths do not require pkg/.
 */
export async function createCorePort(): Promise<CreateCorePortResult> {
  if (allowMock()) {
    return { ok: true, port: new MockCorePort(), kind: "mock" };
  }

  try {
    const { isTauri } = await import("@tauri-apps/api/core");
    if (isTauri()) {
      return { ok: true, port: new TauriCorePort(), kind: "tauri" };
    }
  } catch {
    // Not a Tauri host; continue to WASM.
  }

  try {
    const { WasmCorePort } = await import("./wasm-core-port.js");
    const wasm = new WasmCorePort();
    await wasm.init();
    return { ok: true, port: wasm, kind: "wasm" };
  } catch (e) {
    const detail = e instanceof Error ? e.message : String(e);
    return {
      ok: false,
      error: `No se pudo cargar el motor Rust (WASM): ${detail}`,
    };
  }
}
