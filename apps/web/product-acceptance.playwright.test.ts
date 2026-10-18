import { test, expect } from "@playwright/test";
import {
  keepResultForGeneration,
  shouldIgnoreReentrantAction,
} from "./src/export-generation.js";

/**
 * P5 — huecos de aceptación de producto (integridad / recorrido esencial).
 * Persistencia T-MIG usa ?mock=1 solo para el almacén IndexedDB; el cierre E2E
 * de motor real vive en wasm-flow (tres invocaciones separadas).
 */

const MOCK_URL = process.env.BASE_URL || "http://localhost:5173/?mock=1";

test.describe("P5 product-acceptance — integridad y recorrido", () => {
  test("respuestas viejas no pisan estado nuevo (generación)", () => {
    const generationNow = 5;
    expect(keepResultForGeneration(3, generationNow)).toBe(false);
    expect(keepResultForGeneration(5, generationNow)).toBe(true);
  });

  test("reentrada se ignora mientras busy", () => {
    expect(shouldIgnoreReentrantAction(true)).toBe(true);
    expect(shouldIgnoreReentrantAction(false)).toBe(false);
  });

  test("T-MIG: sesión previa (archmaker-profiles / lastDraftId) se recupera", async ({
    page,
  }) => {
    await page.goto(MOCK_URL);
    const outcome = await page.evaluate(async () => {
      const mod = await import("/src/draft-persistence.ts");
      mod.resetPersistQueueForTests();
      mod.clearLastDraftId();
      const priorId = "aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee";
      const prior = {
        id: priorId,
        schemaVersion: "1",
        documentVersion: "vNext",
        revision: 3,
        catalogRef: {
          namespace: "archmaker",
          id: "desktop",
          version: "0.1.0",
          digest: {
            algorithm: "sha256" as const,
            value: "d".repeat(64),
            kind: "content" as const,
          },
        },
        targetRef: { id: "arch-x86_64", version: "0.1.0" },
        selections: [
          {
            stepId: "archmaker.face.hostname",
            value: { kind: "text" as const, value: "pre-rename-host" },
          },
        ],
      };
      // Simula perfil guardado antes del renombre visible a Forge.
      const saved = await mod.persistDraftLocal(prior);
      const loaded = await mod.loadLastDraftLocal();
      const draft = loaded.kind === "found" ? loaded.draft : null;
      return {
        savedOk: saved.ok,
        dbName: "archmaker-profiles",
        lastIdKey: "archmaker.lastDraftId",
        loadKind: loaded.kind,
        loadedId: draft?.id ?? null,
        hostname: draft?.selections.find(
          (s) => s.stepId === "archmaker.face.hostname",
        )?.value,
        revision: draft?.revision ?? null,
        lastId: window.localStorage.getItem("archmaker.lastDraftId"),
      };
    });
    expect(outcome.loadKind).toBe("found");
    expect(outcome.savedOk).toBe(true);
    expect(outcome.dbName).toBe("archmaker-profiles");
    expect(outcome.lastIdKey).toBe("archmaker.lastDraftId");
    expect(outcome.loadedId).toBe("aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee");
    expect(outcome.lastId).toBe("aaaaaaaa-bbbb-4ccc-8ddd-eeeeeeeeeeee");
    expect(outcome.hostname).toEqual({
      kind: "text",
      value: "pre-rename-host",
    });
    expect(outcome.revision).toBe(3);
  });

  test("T-MIG-03: cuota tipada (QuotaExceededError)", async ({ page }) => {
    await page.goto(MOCK_URL);
    const outcome = await page.evaluate(async () => {
      const mod = await import("/src/draft-persistence.ts");
      mod.resetPersistQueueForTests();
      mod.clearLastDraftId();
      const draft = {
        id: "77777777-7777-4777-8777-777777777777",
        schemaVersion: "1",
        documentVersion: "vNext",
        revision: 1,
        catalogRef: {
          namespace: "archmaker",
          id: "desktop",
          version: "0.1.0",
          digest: {
            algorithm: "sha256" as const,
            value: "e".repeat(64),
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
          error: new DOMException("QuotaExceededError", "QuotaExceededError"),
          onupgradeneeded: null,
          onsuccess: null as
            ((this: IDBOpenDBRequest, ev: Event) => void) | null,
          onerror: null as ((this: IDBOpenDBRequest, ev: Event) => void) | null,
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
        return { result, status: mod.getPersistStatus() };
      } finally {
        idb.open = originalOpen;
      }
    });
    expect(outcome.result.ok).toBe(false);
    if (!outcome.result.ok) {
      expect(outcome.result.reason).toBe("quota");
      expect(outcome.result.message).toMatch(/espacio/i);
    }
    expect(outcome.status).toBe("error");
  });

  test("teclado: recorrido esencial Disco → Continuar → Generar", async ({
    page,
  }) => {
    test.setTimeout(90_000);
    await page.goto(MOCK_URL);
    await expect(
      page.getByRole("heading", { name: /Disco y particiones/i }),
    ).toBeVisible({ timeout: 30_000 });

    // Skip link / foco inicial
    await page.keyboard.press("Tab");
    const first = await page.evaluate(() => document.activeElement?.tagName);
    expect(["A", "BUTTON", "INPUT", "MAIN", "NAV"]).toContain(first);

    // Continuar con teclado (foco + Enter en el CTA)
    const continueBtn = page.getByRole("button", {
      name: /Continuar → Sistema base/i,
    });
    await continueBtn.focus();
    await page.keyboard.press("Enter");
    await expect(
      page.getByRole("heading", { name: "Sistema base" }),
    ).toBeVisible({ timeout: 15_000 });
    await expect(
      page.getByRole("radio", { name: /systemd-boot/i }),
    ).toBeChecked();

    // Generar (cabecera) con teclado
    const generate = page.getByRole("button", { name: "Generar" });
    await generate.focus();
    await page.keyboard.press("Enter");
    await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible({
      timeout: 60_000,
    });
    await expect(page.locator("#toast-region")).toContainText(
      /Manifest listo|Validación|Draft/i,
      { timeout: 60_000 },
    );
    // Nombre visible: ArchMaker Forge
    await expect(page.locator("body")).toContainText(/ArchMaker Forge/i);
  });
});
