import { test, expect } from "@playwright/test";

const BASE_URL = process.env.BASE_URL || "http://localhost:5173";

test.describe("MockCorePort flow", () => {
  test("create → select → resolve → validate → manifest → export", async ({
    page,
  }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");

    await page.getByRole("button", { name: "Crear draft" }).click();
    await expect(page.locator("#toast-region")).toContainText("Draft creado");

    await page.getByRole("checkbox", { name: /Kernel Linux/i }).click();
    await page.getByRole("checkbox", { name: /Escritorio GNOME/i }).click();

    await page.getByRole("button", { name: "Resolver draft" }).click();
    await expect(page.locator("#resolve-out")).toContainText("2 selecciones");

    await page.getByRole("button", { name: "Validar draft" }).click();
    await expect(page.locator("#validation-panel")).toContainText(
      "Sin incidencias",
    );

    await page.getByRole("button", { name: "Construir manifest" }).click();
    await expect(page.locator("#manifest-out")).toContainText(
      "archmaker:manifest:v1",
    );

    await page.getByRole("button", { name: "Exportar artefacto" }).click();
    await expect(page.locator("#export-out")).toContainText(
      "Artefacto exportado",
    );
  });
});
