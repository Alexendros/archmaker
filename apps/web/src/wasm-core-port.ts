import type {
  Artifact,
  BuildManifestInput,
  CoreError,
  CorePort,
  CreateDraftInput,
  Draft,
  ExportArtifactInput,
  ImportDraftInput,
  LoadCatalogInput,
  Manifest,
  OpenDraftInput,
  ResolveDraftInput,
  ResolveResult,
  Result,
  SaveDraftInput,
  ValidateDraftInput,
  ValidationResult,
} from "./coreport.js";
import { typedError } from "./coreport.js";

type WasmBindings = {
  default?: (input?: unknown) => Promise<unknown>;
  wasm_invoke: (
    windowLabel: string,
    opName: string,
    inputJson: string,
  ) => string;
};

function parseCoreError(raw: string): CoreError | null {
  try {
    const value = JSON.parse(raw) as Record<string, unknown>;
    if (
      typeof value.code !== "string" ||
      typeof value.messageKey !== "string"
    ) {
      return null;
    }
    return {
      code: value.code,
      family: typeof value.family === "string" ? value.family : "PROTO",
      category:
        typeof value.category === "string" ? value.category : "protocol",
      severity:
        value.severity === "warning" || value.severity === "info"
          ? value.severity
          : "error",
      messageKey: value.messageKey,
      source:
        typeof value.source === "string" ? value.source : "archmaker-wasm",
    };
  } catch {
    return null;
  }
}

function parseOk<T>(raw: string): Result<T> {
  try {
    return { ok: true, value: JSON.parse(raw) as T };
  } catch {
    return {
      ok: false,
      error: typedError(
        "AM-PROTO-002",
        "archmaker-wasm",
        "error.proto.invalidResponse",
      ),
    };
  }
}

export class WasmCorePort implements CorePort {
  private bindings: WasmBindings | null = null;
  private initPromise: Promise<void> | null = null;

  async init(): Promise<void> {
    if (this.bindings) return;
    if (!this.initPromise) {
      this.initPromise = (async () => {
        const mod =
          (await import("../../../crates/archmaker-wasm/pkg/archmaker_wasm.js")) as WasmBindings;
        if (typeof mod.default === "function") {
          await mod.default();
        }
        if (typeof mod.wasm_invoke !== "function") {
          throw new Error("wasm_invoke missing from archmaker_wasm bindings");
        }
        this.bindings = mod;
      })();
    }
    await this.initPromise;
  }

  private async call<T>(opName: string, input: unknown): Promise<Result<T>> {
    try {
      await this.init();
    } catch {
      return {
        ok: false,
        error: typedError(
          "AM-PROTO-002",
          "archmaker-wasm",
          "error.proto.transportUnavailable",
        ),
      };
    }
    const bindings = this.bindings;
    if (!bindings) {
      return {
        ok: false,
        error: typedError(
          "AM-PROTO-002",
          "archmaker-wasm",
          "error.proto.transportUnavailable",
        ),
      };
    }
    try {
      const raw = bindings.wasm_invoke("main", opName, JSON.stringify(input));
      return parseOk<T>(raw);
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      const structured = parseCoreError(message);
      if (structured) return { ok: false, error: structured };
      return {
        ok: false,
        error: typedError(
          "AM-PROTO-002",
          "archmaker-wasm",
          "error.proto.invokeFailed",
        ),
      };
    }
  }

  createDraft(input: CreateDraftInput): Promise<Result<Draft>> {
    return this.call<Draft>("create_draft", input);
  }
  loadCatalog(input: LoadCatalogInput): Promise<Result<unknown>> {
    return this.call<unknown>("load_catalog", input);
  }
  saveDraft(input: SaveDraftInput): Promise<Result<Draft>> {
    return this.call<Draft>("save_draft", input);
  }
  openDraft(input: OpenDraftInput): Promise<Result<Draft>> {
    return this.call<Draft>("open_draft", input);
  }
  resolveDraft(input: ResolveDraftInput): Promise<Result<ResolveResult>> {
    return this.call<ResolveResult>("resolve_draft", input);
  }
  validateDraft(input: ValidateDraftInput): Promise<Result<ValidationResult>> {
    return this.call<ValidationResult>("validate_draft", input);
  }
  buildManifest(input: BuildManifestInput): Promise<Result<Manifest>> {
    return this.call<Manifest>("build_manifest", input);
  }
  exportArtifact(input: ExportArtifactInput): Promise<Result<Artifact>> {
    return this.call<Artifact>("export_artifact", input);
  }
  importDraft(input: ImportDraftInput): Promise<Result<unknown>> {
    return this.call<unknown>("import_draft", input);
  }
}
