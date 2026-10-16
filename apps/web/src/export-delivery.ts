import type { Artifact } from "./coreport.js";

export type DeliveryErrorCode =
  | "bytes-missing"
  | "bytes-digest-mismatch"
  | "digest-altered"
  | "target-incompatible";

export class DeliveryError extends Error {
  readonly code: DeliveryErrorCode;

  constructor(code: DeliveryErrorCode, message: string) {
    super(message);
    this.name = "DeliveryError";
    this.code = code;
  }
}

const TARGET_EXTENSION: Record<string, string> = {
  "archmaker-profile": ".json",
  "archinstall-profile": ".json",
  report: ".md",
};

const TARGET_MEDIA: Record<string, string> = {
  "archmaker-profile": "application/json",
  "archinstall-profile": "application/json",
  report: "text/markdown",
};

export function extensionForTarget(targetId: string): string {
  return TARGET_EXTENSION[targetId] ?? ".bin";
}

export function mediaTypeForTarget(targetId: string): string {
  return TARGET_MEDIA[targetId] ?? "application/octet-stream";
}

/** Opaque destination token derived from draft id (never a filesystem path). */
export function destinationTokenFor(draftId: string): string {
  const short = draftId.replace(/-/g, "").slice(0, 12);
  return `archmaker-profile-${short}.json`;
}

export function exportFilename(
  token: string,
  targetId = "archmaker-profile",
): string {
  const ext = extensionForTarget(targetId);
  const safe = token.replace(/[^A-Za-z0-9._-]/g, "_");
  if (safe.toLowerCase().endsWith(ext)) return safe;
  const stripped = safe.replace(/\.[A-Za-z0-9]+$/, "");
  return `${stripped}${ext}`;
}

export function artifactBytes(artifact: Artifact): Uint8Array {
  if (!artifact.bytes || artifact.bytes.length === 0) {
    throw new DeliveryError(
      "bytes-missing",
      "Faltan bytes del renderizador; no se entregan metadatos.",
    );
  }
  return new Uint8Array(artifact.bytes);
}

function bufferSource(bytes: Uint8Array): ArrayBuffer {
  return bytes.buffer.slice(
    bytes.byteOffset,
    bytes.byteOffset + bytes.byteLength,
  ) as ArrayBuffer;
}

export async function sha256Hex(bytes: Uint8Array): Promise<string> {
  const digest = await crypto.subtle.digest("SHA-256", bufferSource(bytes));
  return [...new Uint8Array(digest)]
    .map((b) => b.toString(16).padStart(2, "0"))
    .join("");
}

/**
 * Prepare file bytes for delivery. Fails closed if bytes are missing,
 * digest does not match file bytes, or target/mediaType are incompatible.
 */
export async function prepareDelivery(
  artifact: Artifact,
  filename: string,
): Promise<{ bytes: Uint8Array; mediaType: string; filename: string }> {
  const targetId = artifact.targetRef?.id;
  if (!targetId || !(targetId in TARGET_EXTENSION)) {
    throw new DeliveryError(
      "target-incompatible",
      `Target incompatible para entrega: ${targetId ?? "(ausente)"}`,
    );
  }
  const expectedMedia = mediaTypeForTarget(targetId);
  if (artifact.mediaType !== expectedMedia) {
    throw new DeliveryError(
      "target-incompatible",
      `mediaType incompatible: esperado ${expectedMedia}, recibido ${artifact.mediaType}`,
    );
  }
  const bytes = artifactBytes(artifact);
  const expected = artifact.binaryDigest?.value?.toLowerCase();
  if (!expected || !/^[0-9a-f]{64}$/.test(expected)) {
    throw new DeliveryError(
      "digest-altered",
      "binaryDigest ausente o alterado; no se entrega el archivo.",
    );
  }
  const actual = await sha256Hex(bytes);
  if (actual !== expected) {
    throw new DeliveryError(
      "bytes-digest-mismatch",
      "SHA-256 del archivo no coincide con binaryDigest.",
    );
  }
  return {
    bytes,
    mediaType: artifact.mediaType,
    filename: exportFilename(filename, targetId),
  };
}

export type DeliveryOutcome =
  | {
      kind: "download-started";
      filename: string;
      digestShort: string;
    }
  | {
      kind: "file-saved";
      filename: string;
      digestShort: string;
    };

/** Toast copy: web cannot verify disk write; desktop only after confirmation. */
export function exportStatusMessage(outcome: DeliveryOutcome): string {
  const digest = outcome.digestShort;
  if (outcome.kind === "file-saved") {
    return `Archivo guardado (${digest}…): ${outcome.filename}`;
  }
  return `Descarga iniciada (${digest}…): ${outcome.filename}`;
}

/**
 * Confirmed native save (desktop / F5). Do not call from the browser download path.
 */
export function confirmedSaveOutcome(
  filename: string,
  digestHex: string,
): DeliveryOutcome {
  return {
    kind: "file-saved",
    filename,
    digestShort: digestHex.slice(0, 12),
  };
}

/** Deliver renderer bytes via browser download; outcome is never file-saved. */
export async function downloadArtifact(
  artifact: Artifact,
  filename: string,
): Promise<DeliveryOutcome> {
  const prepared = await prepareDelivery(artifact, filename);
  const blob = new Blob([bufferSource(prepared.bytes)], {
    type: prepared.mediaType,
  });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = prepared.filename;
  a.rel = "noopener";
  document.body.append(a);
  a.click();
  a.remove();
  window.setTimeout(() => URL.revokeObjectURL(url), 0);
  return {
    kind: "download-started",
    filename: prepared.filename,
    digestShort: artifact.binaryDigest.value.slice(0, 12),
  };
}
