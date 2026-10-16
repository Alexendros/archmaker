/** Frontier types for CorePort v0 + importDraft. Rust owns semantics (ADR-0001). */

export type CoreError = {
  code: string;
  family: string;
  category: string;
  severity: "error" | "warning" | "info";
  messageKey: string;
  source: string;
};

export type Result<T> =
  { ok: true; value: T } | { ok: false; error: CoreError };

export type ContentDigest = {
  algorithm: "sha256";
  value: string;
  kind?: "content";
};

export type BinaryDigest = {
  algorithm: "sha256";
  value: string;
  kind?: "binary";
};

export type CatalogRef = {
  namespace: string;
  id: string;
  version: string;
  digest: ContentDigest;
};

export type TargetRef = { id: string; version: string };

export type SelectionValue =
  | { kind: "single"; optionId: string }
  | { kind: "multiple"; optionIds: string[] }
  | { kind: "boolean"; value: boolean }
  | { kind: "number"; value: number; unit?: string }
  | { kind: "text"; value: string }
  | { kind: "secretRef"; ref: string };

export type Selection = { stepId: string; value: SelectionValue };

export type Draft = {
  id: string;
  schemaVersion: string;
  documentVersion: string;
  revision: number;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  selections: Selection[];
  contentDigest?: ContentDigest;
};

export type Diagnostic = {
  code: string;
  path: string;
  messageKey: string;
  source: string;
  blocking?: boolean;
  severity?: "error" | "warning" | "info";
};

export type ResolvedSelection = {
  stepId: string;
  optionId: string;
  value: SelectionValue;
  origin: "manual" | "derived";
  locked: boolean;
};

export type ResolveResult = {
  resolutionDigest: ContentDigest;
  effectiveSelections: ResolvedSelection[];
  providedCapabilities: { id: string }[];
  requiredCapabilities: { id: string }[];
  conflicts: unknown[];
  diagnostics: Diagnostic[];
};

export type ValidationResult = {
  diagnostics: Diagnostic[];
  blocking: boolean;
  pipelineVersion: string;
};

export type Manifest = {
  documentVersion: string;
  schemaVersion: string;
  domainLabel: string;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  producer: { id: string; version: string };
  resolutionDigest: ContentDigest;
  effectiveSelections: ResolvedSelection[];
  providedCapabilities: { id: string }[];
  requiredCapabilities: { id: string }[];
  conflicts: unknown[];
  contentDigest: ContentDigest;
};

export type Artifact = {
  documentVersion: string;
  schemaVersion: string;
  artifactRef: { binaryDigest: BinaryDigest; mediaType: string };
  targetRef: TargetRef;
  producer: { id: string; version: string };
  mediaType: string;
  binaryDigest: BinaryDigest;
  manifestRef: { contentDigest: ContentDigest };
  experimental?: boolean;
  content?: unknown;
};

export type CreateDraftInput = {
  draftId?: string;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  requestId?: string;
};

export type LoadCatalogInput = {
  source: "embedded" | "file";
  catalogRef?: CatalogRef;
  requestId?: string;
};

export type SaveDraftInput = {
  draft: Draft;
  expectedRevision: number;
  requestId?: string;
};

export type OpenDraftInput = {
  draftId: string;
  seed?: Draft;
  requestId?: string;
};

export type ResolveDraftInput = {
  draft: Draft;
  catalogRef: CatalogRef;
  requestId?: string;
};

export type ValidateDraftInput = {
  draft: Draft;
  catalogRef: CatalogRef;
  targetRef?: TargetRef;
  requestId?: string;
};

export type BuildManifestInput = {
  draft: Draft;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  expectedResolutionDigest?: ContentDigest;
  requestId?: string;
};

export type ExportArtifactInput = {
  manifest: Manifest;
  targetRef: TargetRef;
  /** Opaque destination handle token; never an arbitrary filesystem path from React. */
  destination: { token: string };
  requestId?: string;
};

export type ImportDraftInput = {
  sourceBytes: number[];
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  draftId?: string;
  confirmBlockingLosses: boolean;
  requestId?: string;
};

export interface CorePort {
  createDraft(input: CreateDraftInput): Promise<Result<Draft>>;
  loadCatalog(input: LoadCatalogInput): Promise<Result<unknown>>;
  saveDraft(input: SaveDraftInput): Promise<Result<Draft>>;
  openDraft(input: OpenDraftInput): Promise<Result<Draft>>;
  resolveDraft(input: ResolveDraftInput): Promise<Result<ResolveResult>>;
  validateDraft(input: ValidateDraftInput): Promise<Result<ValidationResult>>;
  buildManifest(input: BuildManifestInput): Promise<Result<Manifest>>;
  exportArtifact(input: ExportArtifactInput): Promise<Result<Artifact>>;
  importDraft(input: ImportDraftInput): Promise<Result<unknown>>;
}

export function typedError(
  code: string,
  source: string,
  messageKey: string,
): CoreError {
  return {
    code,
    family: code.split("-")[1] ?? "DOC",
    category: "contract",
    severity: "error",
    messageKey,
    source,
  };
}

/** Product catalog identity (local-catalog-mvp1). Digest is content digest from Rust. */
export const DESKTOP_CATALOG_REF: CatalogRef = {
  namespace: "archmaker.core",
  id: "desktop",
  version: "1.0.0",
  digest: {
    algorithm: "sha256",
    value: "e3b7eda1106b270ffe04866b3d838ea9201070da6a79c64e9914c6d081ab8cca",
  },
};

/** @deprecated Use DESKTOP_CATALOG_REF */
export const FIXED_CATALOG_REF = DESKTOP_CATALOG_REF;

export const DRAFT_TARGET: TargetRef = {
  id: "arch-x86_64",
  version: "2026.10.0",
};

export function exportTarget(id: string): TargetRef {
  return { id, version: "0.1.0" };
}

export function singleSelection(stepId: string, optionId: string): Selection {
  return { stepId, value: { kind: "single", optionId } };
}

export function withSelection(draft: Draft, sel: Selection): Draft {
  const rest = draft.selections.filter((s) => s.stepId !== sel.stepId);
  return { ...draft, selections: [...rest, sel] };
}
