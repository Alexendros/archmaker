import { test, expect } from "@playwright/test";
import type { Artifact } from "./src/coreport.js";
import {
  DeliveryError,
  exportFilename,
  extensionForTarget,
  mediaTypeForTarget,
  prepareDelivery,
  sha256Hex,
} from "./src/export-delivery.js";

function baseArtifact(overrides: Partial<Artifact> = {}): Artifact {
  return {
    documentVersion: "vNext",
    schemaVersion: "1",
    artifactRef: {
      binaryDigest: {
        algorithm: "sha256",
        value: "0".repeat(64),
        kind: "binary",
      },
      mediaType: "application/json",
    },
    targetRef: { id: "archmaker-profile", version: "0.1.0" },
    producer: { id: "test", version: "0.1.0" },
    mediaType: "application/json",
    binaryDigest: {
      algorithm: "sha256",
      value: "0".repeat(64),
      kind: "binary",
    },
    manifestRef: {
      contentDigest: {
        algorithm: "sha256",
        value: "1".repeat(64),
        kind: "content",
      },
    },
    ...overrides,
  };
}

async function withMatchingDigest(
  bytes: Uint8Array,
  targetId: string,
): Promise<Artifact> {
  const hex = await sha256Hex(bytes);
  const mediaType = mediaTypeForTarget(targetId);
  return baseArtifact({
    targetRef: { id: targetId, version: "0.1.0" },
    mediaType,
    artifactRef: {
      binaryDigest: { algorithm: "sha256", value: hex, kind: "binary" },
      mediaType,
    },
    binaryDigest: { algorithm: "sha256", value: hex, kind: "binary" },
    bytes: Array.from(bytes),
    content:
      targetId === "report"
        ? new TextDecoder().decode(bytes)
        : { profile: "test" },
  });
}

test.describe("export-delivery F2 bytes", () => {
  test("extensions and media types per target", () => {
    expect(extensionForTarget("archmaker-profile")).toBe(".json");
    expect(extensionForTarget("archinstall-profile")).toBe(".json");
    expect(extensionForTarget("report")).toBe(".md");
    expect(mediaTypeForTarget("report")).toBe("text/markdown");
    expect(exportFilename("token", "report")).toBe("token.md");
    expect(exportFilename("token.json", "report")).toBe("token.md");
  });

  test("delivers exact report markdown bytes, not JSON.stringify", async () => {
    const text = "# ArchMaker report\n\ndomain: archmaker:manifest:v1\n";
    const bytes = new TextEncoder().encode(text);
    const artifact = await withMatchingDigest(bytes, "report");
    // content is the string; JSON.stringify would wrap it in quotes
    expect(JSON.stringify(artifact.content)).not.toBe(text);
    const prepared = await prepareDelivery(artifact, "informe.md");
    expect(new TextDecoder().decode(prepared.bytes)).toBe(text);
    expect(prepared.mediaType).toBe("text/markdown");
    expect(prepared.filename).toBe("informe.md");
    expect(await sha256Hex(prepared.bytes)).toBe(artifact.binaryDigest.value);
  });

  test("rejects missing bytes instead of metadata fallback", async () => {
    const artifact = baseArtifact({
      content: { binaryDigest: "looks-like-profile" },
      bytes: undefined,
    });
    await expect(prepareDelivery(artifact, "x.json")).rejects.toMatchObject({
      code: "bytes-missing",
    } satisfies Partial<DeliveryError>);
  });

  test("rejects altered bytes (digest mismatch)", async () => {
    const bytes = new TextEncoder().encode('{"ok":true}');
    const artifact = await withMatchingDigest(bytes, "archmaker-profile");
    artifact.bytes = Array.from(new TextEncoder().encode('{"ok":false}'));
    await expect(prepareDelivery(artifact, "x.json")).rejects.toMatchObject({
      code: "bytes-digest-mismatch",
    });
  });

  test("rejects altered digest", async () => {
    const bytes = new TextEncoder().encode('{"ok":true}');
    const artifact = await withMatchingDigest(bytes, "archmaker-profile");
    artifact.binaryDigest = {
      algorithm: "sha256",
      value: "not-a-valid-sha256",
      kind: "binary",
    };
    await expect(prepareDelivery(artifact, "x.json")).rejects.toMatchObject({
      code: "digest-altered",
    });
  });

  test("rejects incompatible target", async () => {
    const bytes = new TextEncoder().encode("iso");
    const hex = await sha256Hex(bytes);
    const artifact = baseArtifact({
      targetRef: { id: "iso", version: "0.1.0" },
      mediaType: "application/octet-stream",
      binaryDigest: { algorithm: "sha256", value: hex, kind: "binary" },
      artifactRef: {
        binaryDigest: { algorithm: "sha256", value: hex, kind: "binary" },
        mediaType: "application/octet-stream",
      },
      bytes: Array.from(bytes),
    });
    await expect(prepareDelivery(artifact, "x.iso")).rejects.toMatchObject({
      code: "target-incompatible",
    });
  });
});
