import type { Artifact } from "./coreport.js";

/** Deliver artifact bytes to the user as a browser download. */
export function downloadArtifact(artifact: Artifact, filename: string): void {
  const payload = artifact.content ?? {
    binaryDigest: artifact.binaryDigest,
    mediaType: artifact.mediaType,
    manifestRef: artifact.manifestRef,
  };
  const text = JSON.stringify(payload, null, 2);
  const blob = new Blob([text], {
    type: artifact.mediaType || "application/json",
  });
  const url = URL.createObjectURL(blob);
  const a = document.createElement("a");
  a.href = url;
  a.download = filename;
  a.rel = "noopener";
  document.body.append(a);
  a.click();
  a.remove();
  window.setTimeout(() => URL.revokeObjectURL(url), 0);
}

export function exportFilename(token: string): string {
  const safe = token.replace(/[^A-Za-z0-9._-]/g, "_");
  return safe.endsWith(".json") ? safe : `${safe}.json`;
}

/** Opaque destination token derived from draft id (never a filesystem path). */
export function destinationTokenFor(draftId: string): string {
  const short = draftId.replace(/-/g, "").slice(0, 12);
  return `archmaker-profile-${short}.json`;
}
