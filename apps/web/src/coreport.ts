export type CoreError = {
  code: string;
  family: string;
  category: string;
  severity: "error" | "warning" | "info";
  messageKey: string;
  source: string;
};

export type Result<T> = { ok: true; value: T } | { ok: false; error: CoreError };

export type CatalogRef = { namespace: string; id: string; version: string; digest: { algorithm: string; value: string } };
export type TargetRef = { id: string; version: string };
export type Selection = { step_id: string; option_id: string };
export type Draft = {
  id: string;
  schema_version: string;
  document_version: string;
  revision: number;
  catalog_ref: CatalogRef;
  target_ref: TargetRef;
  selections: Selection[];
};
export type Diagnostic = { code: string; path: string; messageKey: string; source: string };

export interface CorePort {
  createDraft(input: { draftId?: string; catalogRef: CatalogRef; targetRef: TargetRef }): Promise<Result<Draft>>;
  loadCatalog(input: { source: "embedded" | "file" }): Promise<Result<unknown>>;
  saveDraft(input: { draft: Draft; expectedRevision: number }): Promise<Result<Draft>>;
  resolveDraft(input: { draft: Draft; catalogRef: CatalogRef }): Promise<Result<{ resolutionDigest: unknown; effectiveSelections: Selection[] }>>;
  validateDraft(input: { draft: Draft; catalogRef: CatalogRef }): Promise<Result<{ blocking: boolean; diagnostics: Diagnostic[] }>>;
  buildManifest(input: { draft: Draft; catalogRef: CatalogRef; targetRef: TargetRef }): Promise<Result<unknown>>;
  exportArtifact(input: { manifestDigest: unknown; targetRef: TargetRef; destination: string }): Promise<Result<unknown>>;
}

export function typedError(code: string, source: string, messageKey: string): CoreError {
  const family = code.split("-").slice(0, 2).join("-");
  return { code, family, category: "Protocol", severity: "error", messageKey, source };
}
