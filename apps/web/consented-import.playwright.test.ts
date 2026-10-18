import { test, expect } from "@playwright/test";
import type { CorePort, Draft, Result } from "./src/coreport.js";
import { DRAFT_TARGET, FIXED_CATALOG_REF, typedError } from "./src/coreport.js";
import {
  IMPORT_MAX_BYTES,
  analyzeImportFile,
  applyConsentedImport,
  checkImportFileSize,
  parseImportPayload,
  summarizeReport,
} from "./src/consented-import.js";
import { MockCorePort } from "./src/mock-core-port.js";

function fakeFile(bytes: Uint8Array, name = "profile.json"): File {
  return new File([bytes], name, { type: "application/json" });
}

const V51_BYTES = new TextEncoder().encode(
  JSON.stringify({ archmaker: { version: "5.1.0" } }),
);

test.describe("consented-import F4", () => {
  test("rejects oversized files via file.size before reading", async () => {
    let read = false;
    const huge = {
      size: IMPORT_MAX_BYTES + 1,
      arrayBuffer: async () => {
        read = true;
        return new ArrayBuffer(0);
      },
    };
    const size = checkImportFileSize(huge);
    expect(size.ok).toBe(false);
    if (!size.ok) {
      expect(size.code).toBe("too-large");
      expect(size.max).toBe(2_000_000);
    }

    const analysis = await analyzeImportFile({
      file: huge as File,
      port: new MockCorePort(),
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(analysis.kind).toBe("too-large");
    expect(read).toBe(false);
  });

  test("rejects a document that is not v5.1", async () => {
    const file = fakeFile(new TextEncoder().encode('{"ok":true}'));
    expect(checkImportFileSize(file).ok).toBe(true);
    const analysis = await analyzeImportFile({
      file,
      port: new MockCorePort(),
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(analysis.kind).toBe("rejected");
    if (analysis.kind === "rejected") {
      expect(analysis.error.code).toBe("AM-MIG-001");
      expect(analysis.error.messageKey).toBe("error.migration.unsupported");
    }
  });

  test("analyzes a minimal v5.1 document and reads its bytes", async () => {
    const file = fakeFile(V51_BYTES);
    expect(checkImportFileSize(file).ok).toBe(true);
    const analysis = await analyzeImportFile({
      file,
      port: new MockCorePort(),
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(analysis.kind).toBe("ready");
    if (analysis.kind === "ready") {
      expect(analysis.needsBlockingConfirm).toBe(false);
      expect(analysis.payload.draft.id).toBeTruthy();
      expect(analysis.sourceBytes.length).toBe(V51_BYTES.length);
      expect(analysis.payload.report.entries.length).toBeGreaterThan(0);
    }
  });

  test("analyze uses confirmBlockingLosses false (no hardcode true)", async () => {
    const seen: boolean[] = [];
    const port: CorePort = {
      ...(new MockCorePort() as CorePort),
      async importDraft(input) {
        seen.push(input.confirmBlockingLosses);
        return (new MockCorePort() as CorePort).importDraft(input);
      },
    };
    await analyzeImportFile({
      file: fakeFile(V51_BYTES),
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(seen).toEqual([false]);
  });

  test("blocking losses require explicit confirmation", async () => {
    const port = new MockCorePort();
    const analysis = await analyzeImportFile({
      file: fakeFile(new TextEncoder().encode('{"__archmakerBlocking":true}')),
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(analysis.kind).toBe("needs_blocking_confirm");
    if (analysis.kind !== "needs_blocking_confirm") return;

    const denied = await applyConsentedImport({
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
      sourceBytes: analysis.sourceBytes,
      confirmBlockingLosses: false,
      expectedGeneration: 1,
      getCurrentGeneration: () => 1,
    });
    expect(denied.ok).toBe(false);
    if (!denied.ok) {
      expect(denied.error?.code).toBe("AM-MIG-002");
    }

    const applied = await applyConsentedImport({
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
      sourceBytes: analysis.sourceBytes,
      confirmBlockingLosses: true,
      expectedGeneration: 1,
      getCurrentGeneration: () => 1,
    });
    expect(applied.ok).toBe(true);
    if (applied.ok) {
      expect(applied.report.blocking).toBe(true);
    }
  });

  test("apply does not succeed on concurrent edit (stale generation)", async () => {
    const port = new MockCorePort();
    const analysis = await analyzeImportFile({
      file: fakeFile(V51_BYTES),
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(analysis.kind).toBe("ready");
    if (analysis.kind !== "ready") return;

    let generation = 3;
    const stale = await applyConsentedImport({
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
      sourceBytes: analysis.sourceBytes,
      confirmBlockingLosses: false,
      expectedGeneration: 3,
      getCurrentGeneration: () => {
        generation = 4;
        return generation;
      },
    });
    expect(stale.ok).toBe(false);
    if (!stale.ok) expect(stale.reason).toBe("stale-edit");
  });

  test("apply refuses mid-flight edit after importDraft succeeds", async () => {
    let generation = 1;
    const base = new MockCorePort();
    const port: CorePort = {
      ...(base as CorePort),
      async importDraft(input) {
        const res = await base.importDraft(input);
        generation = 9;
        return res;
      },
      async saveDraft(input) {
        return base.saveDraft(input);
      },
    };
    const analysis = await analyzeImportFile({
      file: fakeFile(V51_BYTES),
      port: base,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(analysis.kind).toBe("ready");
    if (analysis.kind !== "ready") return;

    const applied = await applyConsentedImport({
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
      sourceBytes: analysis.sourceBytes,
      confirmBlockingLosses: false,
      expectedGeneration: 1,
      getCurrentGeneration: () => generation,
    });
    expect(applied.ok).toBe(false);
    if (!applied.ok) expect(applied.reason).toBe("stale-edit");
  });

  test("parseImportPayload normalizes report.blocking from camelCase", () => {
    const payload = parseImportPayload({
      draft: {
        id: "d1",
        schemaVersion: "1",
        documentVersion: "vNext",
        revision: 0,
        catalogRef: FIXED_CATALOG_REF,
        targetRef: DRAFT_TARGET,
        selections: [],
      } satisfies Draft,
      report: {
        entries: [
          {
            path: "/x",
            fate: "dropped",
            messageKey: "migration.dropped",
            blocking: true,
          },
        ],
        blocking: true,
      },
    });
    expect(payload?.report.blocking).toBe(true);
    expect(summarizeReport(payload!.report)).toContain("bloqueante");
  });

  test("empty source is rejected by mock without replacing profile", async () => {
    const analysis = await analyzeImportFile({
      file: fakeFile(new Uint8Array()),
      port: new MockCorePort(),
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(analysis.kind).toBe("rejected");
  });

  test("saveDraft failure surfaces typed error and keeps session", async () => {
    const base = new MockCorePort();
    const port: CorePort = {
      ...(base as CorePort),
      async importDraft(input) {
        return base.importDraft(input);
      },
      async saveDraft(): Promise<Result<Draft>> {
        return {
          ok: false,
          error: typedError("AM-DOC-002", "mock-core", "error.doc.conflict"),
        };
      },
    };
    const analysis = await analyzeImportFile({
      file: fakeFile(V51_BYTES),
      port: base,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(analysis.kind).toBe("ready");
    if (analysis.kind !== "ready") return;
    const applied = await applyConsentedImport({
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
      sourceBytes: analysis.sourceBytes,
      confirmBlockingLosses: false,
      expectedGeneration: 0,
      getCurrentGeneration: () => 0,
    });
    expect(applied.ok).toBe(false);
    if (!applied.ok) {
      expect(applied.reason).toBe("core-error");
      expect(applied.error?.code).toBe("AM-DOC-002");
    }
  });

  test("v5.1 import uses a new id so it does not collide with the session draft", async () => {
    const port = new MockCorePort();
    const created = await port.createDraft({
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(created.ok).toBe(true);
    if (!created.ok) return;
    const stored = await port.saveDraft({
      draft: created.value,
      expectedRevision: 0,
    });
    expect(stored.ok).toBe(true);
    const analysis = await analyzeImportFile({
      file: fakeFile(V51_BYTES),
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    expect(analysis.kind).toBe("ready");
    if (analysis.kind !== "ready") return;
    expect(analysis.payload.draft.id).not.toBe(created.value.id);
    const applied = await applyConsentedImport({
      port,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
      sourceBytes: analysis.sourceBytes,
      confirmBlockingLosses: false,
      expectedGeneration: 1,
      getCurrentGeneration: () => 1,
    });
    expect(applied.ok).toBe(true);
  });

  test("IMPORT_MAX_BYTES matches F0 limit 2_000_000", () => {
    expect(IMPORT_MAX_BYTES).toBe(2_000_000);
  });
});
