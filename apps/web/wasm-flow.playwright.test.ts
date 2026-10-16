import { test, expect } from "@playwright/test";
import {
  completeExampleSelection,
  generateExportAndAssertFidelity,
  sha256FileHex,
} from "./e2e-fidelity-helpers.js";

/**
 * Product path: no ?mock=1. Requires wasm-pack output under crates/archmaker-wasm/pkg
 * (produced by `pnpm build:wasm` / `pnpm build` in apps/web).
 */
const BASE_URL = process.env.BASE_URL_WASM || "http://localhost:5173/";

test.describe("WASM CorePort — recorrido real", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(BASE_URL);
  });

  test("E2E-01: arranque con motor WASM (sin mock)", async ({ page }) => {
    await expect(
      page.getByRole("heading", { name: /Motor no disponible/i }),
    ).toHaveCount(0);
    await expect(
      page.getByRole("heading", { name: /Para qué usarás/i }),
    ).toBeVisible({ timeout: 60_000 });
    await expect(page.locator("#toast-region")).toContainText(/Draft listo/i, {
      timeout: 60_000,
    });
    await expect(page.getByText(/Modo demostración/i)).toHaveCount(0);
  });

  test("E2E-04/10: generar, editar, exportar entrega bytes", async ({
    page,
  }) => {
    test.setTimeout(120_000);
    await expect(
      page.getByRole("heading", { name: /Para qué usarás/i }),
    ).toBeVisible({ timeout: 60_000 });
    await expect(page.locator("#toast-region")).toContainText(/Draft listo/i, {
      timeout: 60_000,
    });
    await page.getByRole("button", { name: "Generar" }).click();
    await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible();
    await expect(page.locator("#toast-region")).toContainText(
      /Manifest listo|Validación/i,
      { timeout: 60_000 },
    );
    // Edit via rail (invalidates prepare).
    await page.getByRole("button", { name: /Intención/i }).click();
    await expect(
      page.getByRole("heading", { name: /Para qué usarás/i }),
    ).toBeVisible();
    await page.getByRole("radio", { name: /Uso diario/i }).click();
    await page.getByRole("button", { name: "Generar" }).click();
    await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible();
    await expect(page.locator("#toast-region")).toContainText(
      /Manifest listo|Validación/i,
      { timeout: 60_000 },
    );
    const downloadPromise = page.waitForEvent("download");
    await page.getByRole("button", { name: "Exportar" }).click();
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/\.json$/);
    const path = await download.path();
    expect(path).toBeTruthy();
    const digest = sha256FileHex(path!);
    expect(digest).toMatch(/^[0-9a-f]{64}$/);
    await expect(page.locator("#toast-region")).toContainText(
      new RegExp(`Descarga iniciada \\(${digest.slice(0, 12)}`),
      { timeout: 15_000 },
    );
  });

  test("F3 ejercicio WASM: selección → export → SHA-256 == binaryDigest", async ({
    page,
  }) => {
    test.setTimeout(180_000);
    await expect(
      page.getByRole("heading", { name: /Para qué usarás/i }),
    ).toBeVisible({ timeout: 60_000 });
    await expect(page.locator("#toast-region")).toContainText(/Draft listo/i, {
      timeout: 60_000,
    });
    await completeExampleSelection(page);
    const { digest } = await generateExportAndAssertFidelity(page);
    expect(digest).toHaveLength(64);
  });
});
