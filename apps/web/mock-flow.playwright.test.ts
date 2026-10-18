import { test, expect } from "@playwright/test";
import {
  ESSENTIAL_CONTROLS,
  FORGE_RAIL_STEPS,
  completeExampleSelection,
  completeStorageDeclaration,
  generateExportAndAssertFidelity,
} from "./e2e-fidelity-helpers.js";

const BASE_URL = process.env.BASE_URL || "http://localhost:5173/?mock=1";

test.describe("Forge face — rutas del plan", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(BASE_URL);
    await expect(
      page.getByRole("heading", { name: /Disco y particiones/i }),
    ).toBeVisible();
  });

  test("inventario: seis pasos Forge y controles esenciales en DOM", async ({
    page,
  }) => {
    expect(FORGE_RAIL_STEPS).toHaveLength(6);
    expect(ESSENTIAL_CONTROLS.length).toBeGreaterThanOrEqual(18);
    const rail = page.getByRole("navigation", {
      name: /Pasos del configurador$/i,
    });
    await expect(rail).toBeVisible();
    for (const title of FORGE_RAIL_STEPS) {
      await expect(
        rail.getByRole("button", { name: new RegExp(title, "i") }),
      ).toBeVisible();
    }
    await expect(page.getByRole("button", { name: "Generar" })).toBeVisible();
    await expect(
      page.getByRole("button", { name: "Sencillo", exact: true }),
    ).toHaveCount(0);
    await expect(
      page.getByRole("button", { name: "Avanzado", exact: true }),
    ).toHaveCount(0);
    await expect(page.getByRole("button", { name: "Glosario" })).toBeVisible();
    await expect(page.getByRole("button", { name: "Ctrl+K" })).toBeVisible();
    await expect(
      page.getByRole("button", { name: /Continuar → Sistema base/i }),
    ).toBeVisible();
    await expect(page.locator(".forge-title")).toContainText("ArchMaker Forge");
    await expect(page.locator("[data-app-version]")).toBeVisible();
    await expect(page.locator("body")).not.toContainText("v4.1");
  });

  test("F3 ejercicio: selección → Generar → Exportar → SHA-256 == binaryDigest", async ({
    page,
  }) => {
    test.setTimeout(90_000);
    await completeExampleSelection(page);
    const { digest } = await generateExportAndAssertFidelity(page);
    expect(digest).toHaveLength(64);
  });

  test("arranque: Disco y particiones; sin paso Intención", async ({
    page,
  }) => {
    await expect(
      page.getByRole("heading", { name: /Disco y particiones/i }),
    ).toBeVisible();
    await expect(
      page.getByRole("navigation", { name: /Pasos/i }).first(),
    ).toBeVisible();
    // Paso de rail retirado (no confundir con el checkbox de declaración).
    await expect(
      page
        .getByRole("navigation", { name: /Pasos del configurador/i })
        .getByRole("button", {
          name: /^Intención$/i,
        }),
    ).toHaveCount(0);
    await expect(page.getByText(/no modifica discos/i).first()).toBeVisible();
  });

  test("storage: modo new-disk y acknowledgements", async ({ page }) => {
    await completeStorageDeclaration(page);
    await expect(
      page.getByRole("radio", {
        name: /Disco nuevo o sin datos a conservar/i,
      }),
    ).toBeChecked();
    await expect(
      page.getByRole("checkbox", { name: /declaración de intención/i }),
    ).toBeChecked();
  });

  test("sistema base: boot y filesystem exclusivos", async ({ page }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();

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

  test("entorno: catálogo completo con Hyprland; GNOME marcado", async ({
    page,
  }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: /Continuar → Entorno/i }).click();

    await expect(page.getByRole("radio", { name: /GNOME/i })).toBeChecked();
    await expect(page.getByRole("radio", { name: /Hyprland/i })).toBeVisible();
    await page.getByRole("radio", { name: /Hyprland/i }).click();
    await expect(page.getByRole("radio", { name: /Hyprland/i })).toBeChecked();
    await expect(page.getByRole("radio", { name: /GNOME/i })).not.toBeChecked();
  });

  test("apps: capas, toggle y ficha de paquete", async ({ page }) => {
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
    await expect(page.getByRole("group", { name: "Desarrollo" })).toBeVisible();
    await expect(page.getByRole("group", { name: "Gaming" })).toBeVisible();

    const zen = page.getByRole("checkbox", { name: /^Zen Browser$/i });
    await expect(zen).toBeChecked();

    await zen.uncheck();
    await expect(zen).not.toBeChecked();
    await expect(page.locator("#toast-region")).toContainText(
      /Quitado Zen Browser/i,
    );

    await page.getByRole("button", { name: /Zen Browser/i }).click();
    await expect(page.getByRole("dialog")).toBeVisible();
    await expect(
      page.getByRole("heading", { name: "Zen Browser" }),
    ).toBeVisible();
    await page.getByRole("button", { name: "Añadir" }).click();
    await expect(zen).toBeChecked();
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

    await page.getByPlaceholder("mi-equipo").fill("arch-forge");
    await expect(page.getByPlaceholder("mi-equipo")).toHaveValue("arch-forge");
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

  test("cabecera: glosario, tema y paleta Ctrl+K", async ({ page }) => {
    await page.getByRole("button", { name: "Glosario" }).click();
    await expect(page.getByRole("heading", { name: "Glosario" })).toBeVisible();
    await expect(page.getByText(/Disco y particiones/i).first()).toBeVisible();
    await page.getByRole("button", { name: "Cerrar glosario" }).click();

    await page.getByRole("button", { name: "Oscuro" }).click();
    await expect(page.locator("html")).toHaveClass(/dark/);
    await page.getByRole("button", { name: "Claro" }).click();
    await expect(page.locator("html")).not.toHaveClass(/dark/);

    await page.getByRole("button", { name: "Ctrl+K" }).click();
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

  test("H15: glosario atrapa Tab, inerta stepper y Escape restaura disparador", async ({
    page,
  }) => {
    const glossaryBtn = page.getByRole("button", { name: "Glosario" });
    await glossaryBtn.click();
    const dialog = page.getByRole("dialog", { name: "Glosario" });
    await expect(dialog).toBeVisible();

    await expect(page.locator(".forge-stepper")).toHaveAttribute("inert", "");
    await expect(page.locator("#main")).toHaveAttribute("inert", "");

    const focusInside = await page.evaluate(() => {
      const dlg = document.querySelector('[role="dialog"][aria-modal="true"]');
      return Boolean(dlg && dlg.contains(document.activeElement));
    });
    expect(focusInside).toBe(true);

    for (let i = 0; i < 8; i++) {
      await page.keyboard.press("Tab");
      const stillInside = await page.evaluate(() => {
        const dlg = document.querySelector(
          '[role="dialog"][aria-modal="true"]',
        );
        return Boolean(dlg && dlg.contains(document.activeElement));
      });
      expect(stillInside).toBe(true);
    }

    await page.keyboard.press("Escape");
    await expect(dialog).toHaveCount(0);
    await expect(glossaryBtn).toBeFocused();
  });

  test("H15: informe de importación atrapa foco y Escape cancela", async ({
    page,
  }) => {
    await page.getByRole("button", { name: "Generar" }).click();
    await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible({
      timeout: 60_000,
    });

    const downloadPromise = page.waitForEvent("download");
    await page.getByRole("button", { name: "Exportar" }).click();
    const download = await downloadPromise;
    const path = await download.path();
    expect(path).toBeTruthy();

    await page
      .locator('input[type="file"][accept*="json"]')
      .setInputFiles(path!);
    const dialog = page.getByRole("dialog", {
      name: /Informe de importación/i,
    });
    await expect(dialog).toBeVisible({ timeout: 30_000 });
    await expect(page.locator(".forge-stepper")).toHaveAttribute("inert", "");

    for (let i = 0; i < 6; i++) {
      await page.keyboard.press("Tab");
      const stillInside = await page.evaluate(() => {
        const dlg = document.querySelector(
          '[role="dialog"][aria-modal="true"]',
        );
        return Boolean(dlg && dlg.contains(document.activeElement));
      });
      expect(stillInside).toBe(true);
    }

    await page.keyboard.press("Escape");
    await expect(dialog).toHaveCount(0);
  });

  test("toast Deshacer restaura el paquete", async ({ page }) => {
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await page.getByRole("button", { name: /Continuar → Entorno/i }).click();
    await page.getByRole("button", { name: /Continuar → Apps/i }).click();

    const zen = page.getByRole("checkbox", { name: /^Zen Browser$/i });
    await expect(zen).toBeChecked();
    await zen.uncheck();
    await expect(zen).not.toBeChecked();
    await page.getByRole("button", { name: "Deshacer" }).click();
    await expect(zen).toBeChecked();
  });

  test("entorno: DEs y WMs visibles", async ({ page }) => {
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
      /Hyprland/i,
      /Sway/i,
      /^i3/i,
    ]) {
      await expect(page.getByRole("radio", { name })).toBeVisible();
    }
  });

  test("rail: navegar a revisión y volver a disco", async ({ page }) => {
    const rail = page.getByRole("navigation", {
      name: /Pasos del configurador$/i,
    });
    await rail.getByRole("button", { name: /Revisión/i }).click();
    await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible();
    await rail.getByRole("button", { name: /Disco y particiones/i }).click();
    await expect(
      page.getByRole("heading", { name: /Disco y particiones/i }),
    ).toBeVisible();
  });

  test("inspector: resumen y preview esquemática", async ({ page }) => {
    await expect(
      page.getByText("Resumen de configuración").first(),
    ).toBeVisible();
    await expect(page.getByText("Actividades")).toHaveCount(0);
    await expect(page.locator("[data-storage-ui-state]")).toBeVisible();
  });

  test("progreso: atributo data-step-progress en rail", async ({ page }) => {
    const current = page.locator('#step-rail [data-step-progress="current"]');
    await expect(current).toHaveCount(1);
    await page
      .getByRole("button", { name: /Continuar → Sistema base/i })
      .click();
    await expect(
      page.locator('#step-rail [data-step-progress="current"]'),
    ).toContainText(/Sistema base/i);
  });

  test("sin órdenes de instalación en la cara", async ({ page }) => {
    const body = await page.locator("body").innerText();
    expect(body).not.toMatch(/pacstrap/i);
    expect(body).not.toMatch(/pacman -S/i);
    expect(body).not.toMatch(/\bcurl\b/i);
  });
});
