// Tipos TS de CorePort v0 — I7a (T-I7a-02).
// Fuente: contracts/json-schema/*.schema.json y crates/archmaker-{domain,rules,core}
// (serde camelCase estricto, deny_unknown_fields). Solo transporte y presentacion:
// ninguna regla de dominio vive aqui (ADR-0001, DEC-001).

import { invoke } from "@tauri-apps/api/core";

export type Sha256Digest = { algorithm: "sha256"; value: string; kind?: string };
export type CatalogRef = { namespace: string; id: string; version: string; digest: Sha256Digest };
export type TargetRef = { id: string; version: string };
export type Producer = { id: string; version: string };

export type SelectionValue =
  | { kind: "single"; optionId: string }
  | { kind: "multiple"; optionIds: string[] }
  | { kind: "boolean"; value: boolean }
  | { kind: "number"; value: number; unit?: string }
  | { kind: "text"; value: string }
  | { kind: "secretRef"; ref: string };

export type Selection = { stepId: string; value: SelectionValue };
export type ResolvedSelection = {
  stepId: string;
  optionId: string;
  value: SelectionValue;
  origin: "manual" | "derived";
  locked: boolean;
};

export type Draft = {
  id: string;
  documentVersion: string;
  schemaVersion: string;
  revision: number;
  contentDigest?: Sha256Digest;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  selections: Selection[];
};

export type CapabilityRef = { id: string; title?: string };
export type Conflict = {
  severity: "error" | "warning" | "info";
  capabilityId?: string;
  path: string;
  messageKey: string;
  suggestions: string[];
};

export type RuleDiagnostic = {
  code: string;
  severity: "error" | "warning" | "info";
  blocking: boolean;
  path: string;
  messageKey: string;
  source: string;
  ruleId: string;
  suggestions: string[];
};

export type ResolveResult = {
  resolutionDigest: Sha256Digest;
  effectiveSelections: ResolvedSelection[];
  providedCapabilities: CapabilityRef[];
  requiredCapabilities: CapabilityRef[];
  conflicts: Conflict[];
  diagnostics: RuleDiagnostic[];
};

export type ValidationResult = {
  diagnostics: RuleDiagnostic[];
  blocking: boolean;
  pipelineVersion: string;
};

export type Manifest = {
  documentVersion: string;
  schemaVersion: string;
  domainLabel: "archmaker:manifest:v1";
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  producer: Producer;
  resolutionDigest: Sha256Digest;
  effectiveSelections: ResolvedSelection[];
  providedCapabilities: CapabilityRef[];
  requiredCapabilities: CapabilityRef[];
  conflicts: Conflict[];
  contentDigest: Sha256Digest;
};

export type Artifact = {
  documentVersion: string;
  schemaVersion: string;
  artifactRef: { binaryDigest: Sha256Digest; mediaType: string };
  targetRef: TargetRef;
  producer: Producer;
  mediaType: string;
  binaryDigest: Sha256Digest;
  manifestRef: { contentDigest: Sha256Digest };
  experimental?: boolean;
};

export type CoreErrorFamily =
  | "AM-DOC" | "AM-SCHEMA" | "AM-CAT" | "AM-MIG" | "AM-RULE" | "AM-RES"
  | "AM-TGT" | "AM-IO" | "AM-PROTO" | "AM-RUN" | "AM-POL" | "AM-AUTH";

export type CoreError = {
  code: string;
  family: CoreErrorFamily;
  category: string;
  severity: "error" | "warning" | "info";
  retryable: boolean;
  messageKey: string;
  source: string;
  causeCode?: string;
  cause?: CoreError;
};

export type CoreResult<T> = { ok: true; value: T } | { ok: false; error: CoreError };

export type CoreCommand =
  | "create_draft" | "load_catalog" | "save_draft" | "resolve_draft"
  | "validate_draft" | "build_manifest" | "export_artifact";

export const CORE_COMMANDS: readonly CoreCommand[] = [
  "create_draft",
  "load_catalog",
  "save_draft",
  "resolve_draft",
  "validate_draft",
  "build_manifest",
  "export_artifact",
] as const;

export function isCoreError(value: unknown): value is CoreError {
  if (typeof value !== "object" || value === null) return false;
  const v = value as Record<string, unknown>;
  return (
    typeof v["code"] === "string" &&
    /^AM-[A-Z]+-[0-9]{3}$/.test(v["code"] as string) &&
    typeof v["family"] === "string" &&
    typeof v["messageKey"] === "string" &&
    typeof v["source"] === "string"
  );
}

export function typedError(code: string, source: string, messageKey: string): CoreError {
  const family = code.split("-").slice(0, 2).join("-") as CoreErrorFamily;
  return { code, family, category: "protocol", severity: "error", retryable: false, messageKey, source };
}

export async function invokeOp<T>(command: CoreCommand, input: unknown): Promise<CoreResult<T>> {
  try {
    const value = await invoke<T>(command, { input });
    return { ok: true, value };
  } catch (e) {
    if (isCoreError(e)) return { ok: false, error: e };
    const message = e instanceof Error ? e.message : String(e);
    if (/denied|forbidden|capability|permission|not allowed/i.test(message)) {
      return { ok: false, error: typedError("AM-PROTO-001", "archmaker-web", "error.protocol.invokeDenied") };
    }
    return { ok: false, error: typedError("AM-PROTO-002", "archmaker-web", "error.protocol.unsupportedOperation") };
  }
}
