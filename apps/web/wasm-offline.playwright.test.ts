import { test, expect } from "@playwright/test";

/**
 * F7-OFFLINE (web): página ya cargada con motor WASM, luego sin red.
 * El catálogo y el draft viven en el bundle / memoria; no deben exigir red.
 * Escritorio sin red: ver docs/10-delivery/closure/f7-candidate.md (documentado,
 * sin humo de portal en este runner).
 */
const BASE_URL = process.env.BASE_URL_WASM || "http://localhost:5173/";

test.describe("WASM CorePort — offline tras carga", () => {
  test("F7-OFFLINE: draft y navegación tras setOffline", async ({
    page,
    context,
  }) => {
    test.setTimeout(120_000);
    await page.goto(BASE_URL);
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

    await context.setOffline(true);

    await page.getByRole("radio", { name: /Uso diario/i }).click();
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await expect(page.getByText(/Modo Sencillo activo/i)).toBeVisible({
      timeout: 30_000,
    });
    await expect(
      page.getByText(/Arranque rápido \(systemd-boot\)/i),
    ).toBeVisible();
    await expect(
      page.getByRole("heading", { name: /Motor no disponible/i }),
    ).toHaveCount(0);
    await expect(page.getByText(/Modo demostración/i)).toHaveCount(0);
  });
});
