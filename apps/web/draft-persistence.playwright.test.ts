import { test, expect } from "@playwright/test";

/**
 * Persistencia IndexedDB: se ejecuta en el page de Vite (no en el worker Node).
 * Métricas 20 ciclos edit-save-reopen: no alcanzadas aquí (5 ciclos enfocados).
 */

test.describe("draft-persistence F4", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto("/?mock=1");
  });

  test("emits saving → saved status and reloads last draft", async ({
    page,
  }) => {
    const outcome = await page.evaluate(async () => {
      const mod = await import("/src/draft-persistence.ts");
      mod.resetPersistQueueForTests();
      mod.clearLastDraftId();
      const statuses: string[] = [];
      const unsub = mod.subscribePersistStatus((s) => statuses.push(s));
      const draft = {
        id: "11111111-1111-4111-8111-111111111111",
        schemaVersion: "1",
        documentVersion: "vNext",
        revision: 1,
        catalogRef: {
          namespace: "archmaker",
          id: "desktop",
          version: "0.1.0",
          digest: {
            algorithm: "sha256" as const,
            value: "a".repeat(64),
            kind: "content" as const,
          },
        },
        targetRef: { id: "arch-x86_64", version: "0.1.0" },
        selections: [
          {
            stepId: "archmaker.face.hostname",
            value: { kind: "text" as const, value: "host-a" },
          },
        ],
      };
      const result = await mod.persistDraftLocal(draft);
      unsub();
      const loaded = await mod.loadLastDraftLocal();
      return {
        result,
        statuses,
        status: mod.getPersistStatus(),
        loadedId: loaded?.id ?? null,
        hostname: loaded?.selections.find(
          (s) => s.stepId === "archmaker.face.hostname",
        )?.value,
      };
    });
    expect(outcome.result.ok).toBe(true);
    expect(outcome.statuses).toContain("saving");
    expect(outcome.statuses).toContain("saved");
    expect(outcome.status).toBe("saved");
    expect(outcome.loadedId).toBe("11111111-1111-4111-8111-111111111111");
    expect(outcome.hostname).toEqual({ kind: "text", value: "host-a" });
  });

  test("serializes concurrent writes (last write wins)", async ({ page }) => {
    const hostname = await page.evaluate(async () => {
      const mod = await import("/src/draft-persistence.ts");
      mod.resetPersistQueueForTests();
      mod.clearLastDraftId();
      const id = "22222222-2222-4222-8222-222222222222";
      const base = {
        id,
        schemaVersion: "1",
        documentVersion: "vNext",
        revision: 1,
        catalogRef: {
          namespace: "archmaker",
          id: "desktop",
          version: "0.1.0",
          digest: {
            algorithm: "sha256" as const,
            value: "a".repeat(64),
            kind: "content" as const,
          },
        },
        targetRef: { id: "arch-x86_64", version: "0.1.0" },
        selections: [] as {
          stepId: string;
          value: { kind: "text"; value: string };
        }[],
      };
      const a = mod.persistDraftLocal({
        ...base,
        selections: [
          {
            stepId: "archmaker.face.hostname",
            value: { kind: "text", value: "first" },
          },
        ],
      });
      const b = mod.persistDraftLocal({
        ...base,
        selections: [
          {
            stepId: "archmaker.face.hostname",
            value: { kind: "text", value: "second" },
          },
        ],
      });
      await Promise.all([a, b]);
      const loaded = await mod.loadLastDraftLocal();
      return loaded?.selections.find(
        (s) => s.stepId === "archmaker.face.hostname",
      )?.value;
    });
    expect(hostname).toEqual({ kind: "text", value: "second" });
  });

  test("exportDraftCopy is stable JSON with trailing newline", async ({
    page,
  }) => {
    const copy = await page.evaluate(async () => {
      const mod = await import("/src/draft-persistence.ts");
      return mod.exportDraftCopy({
        id: "33333333-3333-4333-8333-333333333333",
        schemaVersion: "1",
        documentVersion: "vNext",
        revision: 1,
        catalogRef: {
          namespace: "archmaker",
          id: "desktop",
          version: "0.1.0",
          digest: {
            algorithm: "sha256" as const,
            value: "a".repeat(64),
            kind: "content" as const,
          },
        },
        targetRef: { id: "arch-x86_64", version: "0.1.0" },
        selections: [],
      });
    });
    expect(copy.endsWith("\n")).toBe(true);
    expect(JSON.parse(copy).id).toBe("33333333-3333-4333-8333-333333333333");
  });

  test("retryPersistDraftLocal re-saves after a successful prior write", async ({
    page,
  }) => {
    const loaded = await page.evaluate(async () => {
      const mod = await import("/src/draft-persistence.ts");
      mod.resetPersistQueueForTests();
      mod.clearLastDraftId();
      const draft = {
        id: "44444444-4444-4444-8444-444444444444",
        schemaVersion: "1",
        documentVersion: "vNext",
        revision: 1,
        catalogRef: {
          namespace: "archmaker",
          id: "desktop",
          version: "0.1.0",
          digest: {
            algorithm: "sha256" as const,
            value: "a".repeat(64),
            kind: "content" as const,
          },
        },
        targetRef: { id: "arch-x86_64", version: "0.1.0" },
        selections: [
          {
            stepId: "archmaker.face.hostname",
            value: { kind: "text" as const, value: "retry" },
          },
        ],
      };
      await mod.persistDraftLocal(draft);
      await mod.retryPersistDraftLocal({
        ...draft,
        revision: 2,
        selections: [
          {
            stepId: "archmaker.face.hostname",
            value: { kind: "text", value: "retried" },
          },
        ],
      });
      return mod.loadLastDraftLocal();
    });
    expect(loaded?.revision).toBe(2);
    expect(
      loaded?.selections.find((s) => s.stepId === "archmaker.face.hostname")
        ?.value,
    ).toEqual({ kind: "text", value: "retried" });
  });

  test("edit-save-reload cycle preserves normalized text field (×5)", async ({
    page,
  }) => {
    const hostnames = await page.evaluate(async () => {
      const mod = await import("/src/draft-persistence.ts");
      mod.resetPersistQueueForTests();
      mod.clearLastDraftId();
      const id = "55555555-5555-4555-8555-555555555555";
      const out: unknown[] = [];
      for (let i = 0; i < 5; i += 1) {
        const hostname = `cycle-${i}`;
        await mod.persistDraftLocal({
          id,
          schemaVersion: "1",
          documentVersion: "vNext",
          revision: i + 1,
          catalogRef: {
            namespace: "archmaker",
            id: "desktop",
            version: "0.1.0",
            digest: {
              algorithm: "sha256" as const,
              value: "a".repeat(64),
              kind: "content" as const,
            },
          },
          targetRef: { id: "arch-x86_64", version: "0.1.0" },
          selections: [
            {
              stepId: "archmaker.face.hostname",
              value: { kind: "text", value: hostname },
            },
          ],
        });
        const loaded = await mod.loadLastDraftLocal();
        out.push(
          loaded?.selections.find((s) => s.stepId === "archmaker.face.hostname")
            ?.value,
        );
      }
      return out;
    });
    expect(hostnames).toEqual([
      { kind: "text", value: "cycle-0" },
      { kind: "text", value: "cycle-1" },
      { kind: "text", value: "cycle-2" },
      { kind: "text", value: "cycle-3" },
      { kind: "text", value: "cycle-4" },
    ]);
  });

  test("persist error path exposes exportable copy when put fails", async ({
    page,
  }) => {
    const outcome = await page.evaluate(async () => {
      const mod = await import("/src/draft-persistence.ts");
      mod.resetPersistQueueForTests();
      mod.clearLastDraftId();
      const draft = {
        id: "66666666-6666-4666-8666-666666666666",
        schemaVersion: "1",
        documentVersion: "vNext",
        revision: 1,
        catalogRef: {
          namespace: "archmaker",
          id: "desktop",
          version: "0.1.0",
          digest: {
            algorithm: "sha256" as const,
            value: "a".repeat(64),
            kind: "content" as const,
          },
        },
        targetRef: { id: "arch-x86_64", version: "0.1.0" },
        selections: [],
      };
      const idb = indexedDB as IDBFactory & { open: typeof indexedDB.open };
      const originalOpen = idb.open.bind(indexedDB);
      idb.open = (() => {
        const req = {
          result: undefined as IDBDatabase | undefined,
          error: new DOMException("indexedDB unavailable", "UnknownError"),
          onupgradeneeded: null,
          onsuccess: null as
            | ((this: IDBOpenDBRequest, ev: Event) => void)
            | null,
          onerror: null as
            | ((this: IDBOpenDBRequest, ev: Event) => void)
            | null,
        };
        queueMicrotask(() => {
          req.onerror?.call(
            req as unknown as IDBOpenDBRequest,
            new Event("error"),
          );
        });
        return req as unknown as IDBOpenDBRequest;
      }) as typeof indexedDB.open;
      try {
        const result = await mod.persistDraftLocal(draft);
        return {
          result,
          status: mod.getPersistStatus(),
        };
      } finally {
        idb.open = originalOpen;
      }
    });
    expect(outcome.result.ok).toBe(false);
    if (!outcome.result.ok) {
      expect(outcome.result.status).toBe("error");
      expect(outcome.result.exportable).toContain(
        "66666666-6666-4666-8666-666666666666",
      );
    }
    expect(outcome.status).toBe("error");
  });
});
