/** Importación consentida: tamaño → análisis → informe → confirmación → aplicación. */

import type {
  CatalogRef,
  CoreError,
  CorePort,
  Draft,
  Result,
  TargetRef,
} from "./coreport.js";

/** Límite F0 (C-04): 2_000_000 bytes; se comprueba con `file.size` antes de leer. */
export const IMPORT_MAX_BYTES = 2_000_000;

export type MigrationEntry = {
  path: string;
  fate: string;
  messageKey: string;
  detail?: string;
  blocking: boolean;
};

export type MigrationReport = {
  schemaVersion?: string;
  sourceFormat?: string;
  entries: MigrationEntry[];
  blocking: boolean;
};

export type ImportDraftPayload = {
  draft: Draft;
  report: MigrationReport;
  originalDigest?: string;
};

export type SizeCheckResult =
  | { ok: true }
  | { ok: false; code: "too-large"; size: number; max: number };

export type AnalyzeResult =
  | {
      kind: "ready";
      payload: ImportDraftPayload;
      sourceBytes: number[];
      needsBlockingConfirm: boolean;
    }
  | {
      kind: "needs_blocking_confirm";
      error: CoreError;
      sourceBytes: number[];
    }
  | { kind: "rejected"; error: CoreError }
  | { kind: "too-large"; size: number; max: number };

export type ApplyImportResult =
  | { ok: true; draft: Draft; report: MigrationReport }
  | {
      ok: false;
      reason: "stale-edit" | "core-error" | "missing-draft" | "cancelled";
      error?: CoreError;
      message: string;
    };

export function checkImportFileSize(file: { size: number }): SizeCheckResult {
  if (file.size > IMPORT_MAX_BYTES) {
    return { ok: false, code: "too-large", size: file.size, max: IMPORT_MAX_BYTES };
  }
  return { ok: true };
}

export async function readImportBytes(file: Blob): Promise<Uint8Array> {
  return new Uint8Array(await file.arrayBuffer());
}

function asRecord(value: unknown): Record<string, unknown> | null {
  if (value && typeof value === "object" && !Array.isArray(value)) {
    return value as Record<string, unknown>;
  }
  return null;
}

function parseEntry(raw: unknown): MigrationEntry | null {
  const o = asRecord(raw);
  if (!o || typeof o.path !== "string" || typeof o.messageKey !== "string") {
    return null;
  }
  return {
    path: o.path,
    fate: typeof o.fate === "string" ? o.fate : "unknown",
    messageKey: o.messageKey,
    detail: typeof o.detail === "string" ? o.detail : undefined,
    blocking: Boolean(o.blocking),
  };
}

export function parseImportPayload(value: unknown): ImportDraftPayload | null {
  const root = asRecord(value);
  if (!root) return null;
  const draft = root.draft as Draft | undefined;
  if (!draft || typeof draft.id !== "string" || !Array.isArray(draft.selections)) {
    return null;
  }
  const reportRaw = asRecord(root.report) ?? {};
  const entriesRaw = Array.isArray(reportRaw.entries) ? reportRaw.entries : [];
  const entries = entriesRaw
    .map(parseEntry)
    .filter((e): e is MigrationEntry => e !== null);
  const blocking =
    typeof reportRaw.blocking === "boolean"
      ? reportRaw.blocking
      : typeof reportRaw.blockingLosses === "boolean"
        ? reportRaw.blockingLosses
        : entries.some((e) => e.blocking);
  return {
    draft,
    report: {
      schemaVersion:
        typeof reportRaw.schemaVersion === "string"
          ? reportRaw.schemaVersion
          : undefined,
      sourceFormat:
        typeof reportRaw.sourceFormat === "string"
          ? reportRaw.sourceFormat
          : undefined,
      entries,
      blocking,
    },
    originalDigest:
      typeof root.originalDigest === "string" ? root.originalDigest : undefined,
  };
}

export function summarizeReport(report: MigrationReport): string {
  const total = report.entries.length;
  const blocked = report.entries.filter((e) => e.blocking).length;
  const dropped = report.entries.filter((e) => e.fate === "dropped").length;
  if (total === 0) {
    return report.blocking
      ? "La migración tiene pérdidas bloqueantes."
      : "Migración sin entradas de informe.";
  }
  const parts = [`${total} entrada${total === 1 ? "" : "s"}`];
  if (blocked > 0) parts.push(`${blocked} bloqueante${blocked === 1 ? "" : "s"}`);
  if (dropped > 0) parts.push(`${dropped} descartada${dropped === 1 ? "" : "s"}`);
  return parts.join(" · ");
}

export async function analyzeImportFile(args: {
  file: { size: number } & Blob;
  port: CorePort;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
}): Promise<AnalyzeResult> {
  const sizeCheck = checkImportFileSize(args.file);
  if (!sizeCheck.ok) {
    return {
      kind: "too-large",
      size: sizeCheck.size,
      max: sizeCheck.max,
    };
  }

  const bytes = await readImportBytes(args.file);
  const sourceBytes = Array.from(bytes);

  const res: Result<unknown> = await args.port.importDraft({
    sourceBytes,
    catalogRef: args.catalogRef,
    targetRef: args.targetRef,
    confirmBlockingLosses: false,
  });

  if (!res.ok) {
    if (res.error.code === "AM-MIG-002") {
      return {
        kind: "needs_blocking_confirm",
        error: res.error,
        sourceBytes,
      };
    }
    return { kind: "rejected", error: res.error };
  }

  const payload = parseImportPayload(res.value);
  if (!payload) {
    return {
      kind: "rejected",
      error: {
        code: "AM-DOC-001",
        family: "DOC",
        category: "contract",
        severity: "error",
        messageKey: "error.doc.limit",
        source: "consented-import",
      },
    };
  }

  return {
    kind: "ready",
    payload,
    sourceBytes,
    needsBlockingConfirm: payload.report.blocking,
  };
}

/**
 * Aplica la importación tras confirmación explícita.
 * No muta el perfil React: el caller sustituye solo si `ok`.
 * `expectedGeneration` protege edición concurrente.
 */
export async function applyConsentedImport(args: {
  port: CorePort;
  catalogRef: CatalogRef;
  targetRef: TargetRef;
  sourceBytes: number[];
  confirmBlockingLosses: boolean;
  expectedGeneration: number;
  /** Lectura en vivo de la generación de edición de la sesión. */
  getCurrentGeneration: () => number;
}): Promise<ApplyImportResult> {
  if (args.getCurrentGeneration() !== args.expectedGeneration) {
    return {
      ok: false,
      reason: "stale-edit",
      message: "El perfil cambió durante el informe; vuelve a importar.",
    };
  }

  const res = await args.port.importDraft({
    sourceBytes: args.sourceBytes,
    catalogRef: args.catalogRef,
    targetRef: args.targetRef,
    confirmBlockingLosses: args.confirmBlockingLosses,
  });

  if (!res.ok) {
    return {
      ok: false,
      reason: "core-error",
      error: res.error,
      message: `Importar: ${res.error.code} (${res.error.messageKey})`,
    };
  }

  const payload = parseImportPayload(res.value);
  if (!payload) {
    return {
      ok: false,
      reason: "missing-draft",
      message: "Importación sin draft.",
    };
  }

  if (args.getCurrentGeneration() !== args.expectedGeneration) {
    return {
      ok: false,
      reason: "stale-edit",
      message: "El perfil cambió durante la aplicación; no se sustituyó.",
    };
  }

  const saved = await args.port.saveDraft({
    draft: payload.draft,
    expectedRevision: 0,
  });

  if (!saved.ok) {
    return {
      ok: false,
      reason: "core-error",
      error: saved.error,
      message: `Guardar importación: ${saved.error.code} (${saved.error.messageKey})`,
    };
  }

  if (args.getCurrentGeneration() !== args.expectedGeneration) {
    return {
      ok: false,
      reason: "stale-edit",
      message: "El perfil cambió tras guardar; no se sustituyó la sesión.",
    };
  }

  return { ok: true, draft: saved.value, report: payload.report };
}
