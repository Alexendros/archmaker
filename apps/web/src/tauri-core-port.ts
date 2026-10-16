import { invoke } from "@tauri-apps/api/core";
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

function asCoreError(value: unknown): CoreError | null {
  if (!value || typeof value !== "object") return null;
  const o = value as Record<string, unknown>;
  if (typeof o.code !== "string" || typeof o.messageKey !== "string")
    return null;
  return {
    code: o.code,
    family: typeof o.family === "string" ? o.family : "PROTO",
    category: typeof o.category === "string" ? o.category : "protocol",
    severity:
      o.severity === "warning" || o.severity === "info" ? o.severity : "error",
    messageKey: o.messageKey,
    source: typeof o.source === "string" ? o.source : "archmaker-web",
  };
}

export class TauriCorePort implements CorePort {
  private async call<T>(cmd: string, input: unknown): Promise<Result<T>> {
    try {
      const value = await invoke<T>(cmd, { input });
      return { ok: true, value };
    } catch (e) {
      const structured =
        asCoreError(e) ?? asCoreError((e as { message?: unknown })?.message);
      if (structured) {
        return { ok: false, error: structured };
      }
      const message = e instanceof Error ? e.message : String(e);
      if (/denied|forbidden|capability|permission|acl/i.test(message)) {
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
