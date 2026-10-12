import type {
  Artifact,
  BuildManifestInput,
  CorePort,
  CreateDraftInput,
  Draft,
  ExportArtifactInput,
  ImportDraftInput,
  LoadCatalogInput,
  Manifest,
  ResolveDraftInput,
  ResolveResult,
  Result,
  SaveDraftInput,
  ValidateDraftInput,
  ValidationResult,
} from "./coreport.js";
import { typedError } from "./coreport.js";

declare global {
  interface Window {
    __TAURI__?: {
      core: { invoke: (cmd: string, args?: unknown) => Promise<unknown> };
    };
  }
}

export class TauriCorePort implements CorePort {
  private async call<T>(cmd: string, args?: unknown): Promise<Result<T>> {
    const invoke = window.__TAURI__?.core.invoke;
    if (!invoke) {
      return {
        ok: false,
        error: typedError(
          "AM-PROTO-002",
          "archmaker-web",
          "error.proto.transportUnavailable",
        ),
      };
    }
    try {
      const value = (await invoke(cmd, args)) as T;
      return { ok: true, value };
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      if (/denied|forbidden|capability|permission/i.test(message)) {
        return {
          ok: false,
          error: typedError(
            "AM-PROTO-001",
            "archmaker-web",
            "error.proto.invokeDenied",
          ),
        };
      }
      return {
        ok: false,
        error: typedError(
          "AM-PROTO-002",
          "archmaker-web",
          "error.proto.invokeFailed",
        ),
      };
    }
  }

  createDraft(input: CreateDraftInput): Promise<Result<Draft>> {
    return this.call<Draft>("create_draft", { input });
  }
  loadCatalog(input: LoadCatalogInput): Promise<Result<unknown>> {
    return this.call<unknown>("load_catalog", { input });
  }
  saveDraft(input: SaveDraftInput): Promise<Result<Draft>> {
    return this.call<Draft>("save_draft", { input });
  }
  resolveDraft(input: ResolveDraftInput): Promise<Result<ResolveResult>> {
    return this.call<ResolveResult>("resolve_draft", { input });
  }
  validateDraft(input: ValidateDraftInput): Promise<Result<ValidationResult>> {
    return this.call<ValidationResult>("validate_draft", { input });
  }
  buildManifest(input: BuildManifestInput): Promise<Result<Manifest>> {
    return this.call<Manifest>("build_manifest", { input });
  }
  exportArtifact(input: ExportArtifactInput): Promise<Result<Artifact>> {
    return this.call<Artifact>("export_artifact", { input });
  }
  importDraft(input: ImportDraftInput): Promise<Result<unknown>> {
    return this.call<unknown>("import_draft", { input });
  }
}
