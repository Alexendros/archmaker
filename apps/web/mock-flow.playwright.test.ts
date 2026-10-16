import { test, expect } from "@playwright/test";
import {
  ESSENTIAL_CONTROLS,
  NEUBAT_RAIL_STEPS,
  completeExampleSelection,
  generateExportAndAssertFidelity,
} from "./e2e-fidelity-helpers.js";

const BASE_URL = process.env.BASE_URL || "http://localhost:5173/?mock=1";

test.describe("NEUBAT FORGE face — rutas del plan", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(
      page.getByRole("heading", { name: /Para qué usarás/i }),
    ).toBeVisible();
  });

  test("inventario: seis pasos NEUBAT y controles esenciales en DOM", async ({
    page,
  }) => {
    expect(NEUBAT_RAIL_STEPS).toHaveLength(6);
    expect(ESSENTIAL_CONTROLS.length).toBeGreaterThanOrEqual(20);
    const rail = page.getByRole("navigation", { name: /Pasos/i });
    await expect(rail).toBeVisible();
    for (const title of NEUBAT_RAIL_STEPS) {
      await expect(
        rail.getByRole("button", { name: new RegExp(title, "i") }),
      ).toBeVisible();
    }
    await expect(page.getByRole("button", { name: "Generar" })).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Sencillo", exact: true }),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Avanzado", exact: true }),
    ).toBeVisible();
    await expect(page.getByRole("button", { name: "Glosario" })).toBeVisible();
    await expect(page.getByRole("button", { name: "⌘K" })).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Continuar → Sistema base/i }),
    ).toBeVisible();
  });

  test("F3 ejercicio: selección → Generar → Exportar → SHA-256 == binaryDigest", async ({
    page,
  }) => {
    test.setTimeout(90_000);
    await completeExampleSelection(page);
    const { digest } = await generateExportAndAssertFidelity(page);
    expect(digest).toHaveLength(64);
  });

  test("arranque: Intención visible y desarrollo premarcado", async ({
    page,
  }) => {
    await expect(
      page.getByRole("radio", { name: /Desarrollo con IA/i }),
    ).toBeChecked();
    await expect(
      page.getByRole("navigation", { name: /Pasos/i }),
    ).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Intención/i }),
    ).toBeVisible();
  });

  test("intención: cambiar tarjeta actualiza selección exclusiva", async ({
    page,
  }) => {
    await page.getByRole("radio", { name: /Uso diario/i }).click();
    await expect(
      page.getByRole("radio", { name: /Uso diario/i }),
    ).toBeChecked();
    await expect(
      page.getByRole("radio", { name: /Desarrollo con IA/i }),
    ).not.toBeChecked();
  });

  test("sistema base sencillo: resumen con systemd-boot y btrfs", async ({
    page,
  }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await expect(page.getByText(/Modo Sencillo activo/i)).toBeVisible();
    await expect(
      page.getByText(/Arranque rápido \(systemd-boot\)/i),
    ).toBeVisible();
    await expect(page.getByText(/Protección \(Btrfs\)/i)).toBeVisible();
    await expect(page.getByRole("radio", { name: /^GRUB$/i })).toHaveCount(0);
  });

  test("sistema base avanzado: boot y filesystem exclusivos", async ({
    page,
  }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: "Avanzado", exact: true }).click();

    const systemd = page.getByRole("radio", { name: /systemd-boot/i });
    await expect(systemd).toBeChecked();
    await expect(page.getByRole("radio", { name: /^Btrfs$/i })).toBeChecked();

    await page.getByRole("radio", { name: /^GRUB$/i }).click();
    await expect(page.getByRole("radio", { name: /^GRUB$/i })).toBeChecked();
    await expect(systemd).not.toBeChecked();

    await page.getByRole("radio", { name: /^Limine$/i }).click();
    await expect(page.getByRole("radio", { name: /^Limine$/i })).toBeChecked();
    await systemd.click();
    await expect(systemd).toBeChecked();

    await page.getByRole("radio", { name: /^Ext4$/i }).click();
    await expect(page.getByRole("radio", { name: /^Ext4$/i })).toBeChecked();
    await expect(
      page.getByRole("radio", { name: /^Btrfs$/i }),
    ).not.toBeChecked();

    await page.getByRole("radio", { name: /Linux Zen/i }).click();
    await expect(page.getByRole("radio", { name: /Linux Zen/i })).toBeChecked();
  });

  test("sencillo/avanzado: no borra arranque al cambiar de modo", async ({
    page,
  }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: "Avanzado", exact: true }).click();
    await page.getByRole("radio", { name: /^GRUB$/i }).click();
    await page.getByRole("button", { name: "Sencillo", exact: true }).click();
    await expect(page.getByText(/Arranque rápido \(GRUB\)/i)).toBeVisible();
    await page.getByRole("button", { name: "Avanzado", exact: true }).click();
    await expect(page.getByRole("radio", { name: /^GRUB$/i })).toBeChecked();
  });

  test("entorno: sencillo oculta Hyprland; avanzado lo muestra; GNOME marcado", async ({
    page,
  }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: /Continuar → Entorno/i }).click();

    await expect(page.getByRole("radio", { name: /GNOME/i })).toBeChecked();
    await expect(page.getByRole("radio", { name: /Hyprland/i })).toHaveCount(0);
    await expect(page.getByRole("radio", { name: /^i3/i })).toHaveCount(0);

    await page.getByRole("button", { name: "Avanzado", exact: true }).click();
    await expect(page.getByRole("radio", { name: /Hyprland/i })).toBeVisible();
    await page.getByRole("radio", { name: /Hyprland/i }).click();
    await expect(page.getByRole("radio", { name: /Hyprland/i })).toBeChecked();
    await expect(page.getByRole("radio", { name: /GNOME/i })).not.toBeChecked();
  });

  test("apps: capas sencillas, toggle y ficha de paquete", async ({ page }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: /Continuar → Entorno/i }).click();
    await page.getByRole("button", { name: /Continuar → Apps/i }).click();

    await expect(
      page.getByRole("group", { name: "Herramientas base" }),
    ).toBeVisible();
    await expect(
      page.getByRole("group", { name: "Navegación web" }),
    ).toBeVisible();
    await expect(page.getByRole("group", { name: "Desarrollo" })).toHaveCount(
      0,
    );
    await expect(page.getByRole("group", { name: "Gaming" })).toHaveCount(0);

    const firefox = page.getByRole("checkbox", { name: /^Firefox$/i });
    await expect(firefox).toBeChecked();

    await page.getByRole("button", { name: "Avanzado", exact: true }).click();
    await expect(page.getByRole("group", { name: "Desarrollo" })).toBeVisible();
    await expect(page.getByRole("group", { name: "Gaming" })).toBeVisible();

    await firefox.uncheck();
    await expect(firefox).not.toBeChecked();
    await expect(page.locator("#toast-region")).toContainText(
      /Quitado Firefox/i,
    );

    await page.getByRole("button", { name: /Firefox/i }).click();
    await expect(page.getByRole("dialog")).toBeVisible();
    await expect(page.getByRole("heading", { name: "Firefox" })).toBeVisible();
    await page.getByRole("button", { name: "Añadir" }).click();
    await expect(firefox).toBeChecked();
  });

  test("identidad: hostname y cifrado se guardan en la cara", async ({
    page,
  }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: /Continuar → Entorno/i }).click();
    await page.getByRole("button", { name: /Continuar → Apps/i }).click();
    await page.getByRole("button", { name: /Continuar → Identidad/i }).click();

    await page.getByPlaceholder("mi-equipo").fill("arch-neubat");
    await expect(page.getByPlaceholder("mi-equipo")).toHaveValue("arch-neubat");
    await page.getByRole("checkbox", { name: /Activar cifrado/i }).check();
    await expect(
      page.getByRole("checkbox", { name: /Activar cifrado/i }),
    ).toBeChecked();
  });

  test("revisión: Generar valida y Exportar entrega archivo", async ({
    page,
  }) => {
    await page.getByRole("button", { name: "Generar" }).click();
    await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible();
    await expect(page.locator("#toast-region")).toContainText(
      /Manifest listo|Validación/i,
    );
    await expect(page.getByText(/Arranque systemd-boot/i)).toBeVisible();
    const downloadPromise = page.waitForEvent("download");
    await page.getByRole("button", { name: "Exportar" }).click();
    const download = await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/archmaker-profile-.*\.json/);
    await expect(page.locator("#toast-region")).toContainText(
      /Descarga iniciada/i,
    );
  });

  test("cabecera: glosario, tema y paleta ⌘K", async ({ page }) => {
    await page.getByRole("button", { name: "Glosario" }).click();
    await expect(page.getByRole("heading", { name: "Glosario" })).toBeVisible();
    await expect(page.getByText(/Modo Sencillo \/ Avanzado/i)).toBeVisible();
    await page.getByRole("button", { name: "Cerrar glosario" }).click();

    await page.getByRole("button", { name: "Oscuro" }).click();
    await expect(page.locator("html")).toHaveClass(/dark/);
    await page.getByRole("button", { name: "Claro" }).click();
    await expect(page.locator("html")).not.toHaveClass(/dark/);

    await page.getByRole("button", { name: "⌘K" }).click();
    await expect(page.getByRole("dialog")).toBeVisible();
    await page.getByPlaceholder(/Busca/i).fill("Steam");
    await page.getByRole("button", { name: /Steam/i }).click();
    await expect(page.locator("#toast-region")).toContainText(/Steam/i);
  });

  test("Escape cierra paleta y ficha de paquete", async ({ page }) => {
    await page.keyboard.press("Control+k");
    await expect(page.getByRole("dialog")).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(page.getByRole("dialog")).toHaveCount(0);

    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: /Continuar → Entorno/i }).click();
    await page.getByRole("button", { name: /Continuar → Apps/i }).click();
    await page.getByRole("button", { name: /Firefox/i }).click();
    await expect(page.getByRole("dialog")).toBeVisible();
    await page.keyboard.press("Escape");
    await expect(page.getByRole("dialog")).toHaveCount(0);
  });

  test("toast Deshacer restaura el paquete", async ({ page }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: /Continuar → Entorno/i }).click();
    await page.getByRole("button", { name: /Continuar → Apps/i }).click();

    const firefox = page.getByRole("checkbox", { name: /^Firefox$/i });
    await expect(firefox).toBeChecked();
    await firefox.uncheck();
    await expect(firefox).not.toBeChecked();
    await page.getByRole("button", { name: "Deshacer" }).click();
    await expect(firefox).toBeChecked();
  });

  test("entorno sencillo: DEs visibles, WMs ocultos", async ({ page }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: /Continuar → Entorno/i }).click();

    for (const name of [
      /GNOME/i,
      /KDE Plasma/i,
      /Cinnamon/i,
      /XFCE/i,
      /COSMIC/i,
      /Budgie/i,
    ]) {
      await expect(page.getByRole("radio", { name })).toBeVisible();
    }
    for (const name of [/Hyprland/i, /Sway/i, /^i3/i]) {
      await expect(page.getByRole("radio", { name })).toHaveCount(0);
    }
  });

  test("rail: navegar a revisión y volver a intención", async ({ page }) => {
    await page.getByRole("button", { name: /Revisión/i }).click();
    await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible();
    await page.getByRole("button", { name: /Intención/i }).click();
    await expect(
      page.getByRole("heading", { name: /Para qué usarás/i }),
    ).toBeVisible();
  });

  test("sin órdenes de instalación en la cara", async ({ page }) => {
    const body = await page.locator("body").innerText();
    expect(body).not.toMatch(/pacstrap/i);
    expect(body).not.toMatch(/pacman -S/i);
    expect(body).not.toMatch(/\bcurl\b/i);
  });
});
