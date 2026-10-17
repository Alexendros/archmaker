/**
 * Helpers compartidos para el ejercicio E2E de cierre:
 * selección Forge → Generar → Exportar → SHA-256(archivo) == binaryDigest.
 * Fuera de alcance: imagen Arch, ISO, root, discos, pacstrap, instalación real.
 */
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";
import { expect, type Page } from "@playwright/test";

/** Hostname del ejercicio de ejemplo (F3). */
export const E2E_HOSTNAME = "arch-e2e-audit";

/** Seis pasos del rail Forge (RAIL_STEPS). */
export const FORGE_RAIL_STEPS = [
  "Disco y particiones",
  "Sistema base",
  "Entorno",
  "Apps",
  "Identidad y seguridad",
  "Revisión",
] as const;

/**
 * Inventario de controles esenciales (botón / ruta / paso).
 * Cada fila debe poder ejercitarse o quedar cubierta por un test dedicado.
 */
export const ESSENTIAL_CONTROLS = [
  {
    id: "rail-storage",
    route: "rail",
    step: "Disco y particiones",
    control: "button",
  },
  { id: "rail-base", route: "rail", step: "Sistema base", control: "button" },
  { id: "rail-env", route: "rail", step: "Entorno", control: "button" },
  { id: "rail-apps", route: "rail", step: "Apps", control: "button" },
  {
    id: "rail-identity",
    route: "rail",
    step: "Identidad y seguridad",
    control: "button",
  },
  { id: "rail-review", route: "rail", step: "Revisión", control: "button" },
  {
    id: "header-generate",
    route: "header",
    step: "*",
    control: "Generar",
  },
  {
    id: "header-glossary",
    route: "header",
    step: "*",
    control: "Glosario",
  },
  {
    id: "header-theme",
    route: "header",
    step: "*",
    control: "Oscuro|Claro",
  },
  { id: "header-cmdk", route: "header", step: "*", control: "Ctrl+K" },
  {
    id: "nav-to-base",
    route: "storage",
    step: "Disco y particiones",
    control: "Continuar → Sistema base",
  },
  {
    id: "nav-to-env",
    route: "base",
    step: "Sistema base",
    control: "Continuar → Entorno",
  },
  {
    id: "nav-to-apps",
    route: "environment",
    step: "Entorno",
    control: "Continuar → Apps",
  },
  {
    id: "nav-to-identity",
    route: "apps",
    step: "Apps",
    control: "Continuar → Identidad",
  },
  {
    id: "nav-to-review",
    route: "identity",
    step: "Identidad y seguridad",
    control: "Continuar → Revisión",
  },
  {
    id: "identity-hostname",
    route: "identity",
    step: "Identidad y seguridad",
    control: "hostname text",
  },
  {
    id: "identity-encrypt",
    route: "identity",
    step: "Identidad y seguridad",
    control: "Activar cifrado",
  },
  {
    id: "review-export",
    route: "review",
    step: "Revisión",
    control: "Exportar",
  },
  {
    id: "review-import",
    route: "review",
    step: "Revisión",
    control: "Importar",
  },
] as const;

export function sha256FileHex(path: string): string {
  return createHash("sha256").update(readFileSync(path)).digest("hex");
}

/** Declara storage mínimo (modo + acks) sin fingir detección. */
export async function completeStorageDeclaration(page: Page): Promise<void> {
  await page
    .getByRole("radio", { name: /Disco nuevo o sin datos a conservar/i })
    .click();
  await page
    .getByRole("checkbox", { name: /declaración de intención/i })
    .check();
}

/** Recorrido razonable: storage → base Ext4 → GNOME → identidad. */
export async function completeExampleSelection(page: Page): Promise<void> {
  await completeStorageDeclaration(page);
  await page
    .getByRole("button", { name: /Continuar → Sistema base/i })
    .click();

  await page.getByRole("radio", { name: /Linux Zen/i }).click();
  await expect(page.getByRole("radio", { name: /systemd-boot/i })).toBeChecked();
  await page.getByRole("radio", { name: /^Ext4$/i }).click();

  await page.getByRole("button", { name: /Continuar → Entorno/i }).click();
  await expect(page.getByRole("radio", { name: /GNOME/i })).toBeChecked();

  await page.getByRole("button", { name: /Continuar → Apps/i }).click();
  await expect(
    page.getByRole("checkbox", { name: /^Zen Browser$/i }),
  ).toBeChecked();

  await page.getByRole("button", { name: /Continuar → Identidad/i }).click();
  await page.getByPlaceholder("mi-equipo").fill(E2E_HOSTNAME);
  await page.getByRole("checkbox", { name: /Activar cifrado/i }).check();

  await page.getByRole("button", { name: /Continuar → Revisión/i }).click();
  await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible();
  await expect(page.getByText(`Hostname: ${E2E_HOSTNAME}`)).toBeVisible();
  await expect(
    page.getByText(/Cifrado: intención marcada \(no aplicada al disco\)/i),
  ).toBeVisible();
}

/**
 * Generar + Exportar: comprueba SHA-256 del archivo == binaryDigest (toast)
 * y fidelidad hostname/cifrado en el JSON (F3).
 */
export async function generateExportAndAssertFidelity(
  page: Page,
): Promise<{ digest: string; filename: string }> {
  await page.getByRole("button", { name: "Generar" }).click();
  await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible();
  await expect(page.locator("#toast-region")).toContainText(
    /Manifest listo|Validación/i,
    { timeout: 60_000 },
  );

  const downloadPromise = page.waitForEvent("download");
  await page.getByRole("button", { name: "Exportar" }).click();
  const download = await downloadPromise;
  const filename = download.suggestedFilename();
  expect(filename).toMatch(/archmaker-profile-.*\.json/);

  const path = await download.path();
  expect(path).toBeTruthy();
  const digest = sha256FileHex(path!);
  expect(digest).toMatch(/^[0-9a-f]{64}$/);

  const digestShort = digest.slice(0, 12);
  await expect(page.locator("#toast-region")).toContainText(
    new RegExp(`Descarga iniciada \\(${digestShort}`),
    { timeout: 15_000 },
  );

  const text = readFileSync(path!, "utf8");
  expect(text).toContain("archmaker.face.hostname");
  expect(text).toContain(E2E_HOSTNAME);
  expect(text).toContain("archmaker.face.encrypt");
  expect(text.toLowerCase()).not.toMatch(/cifrado aplicado|disk encrypted/);

  // Paridad selección → export: arranque y filesystem del ejercicio.
  expect(text).toMatch(/systemd-boot|archmaker\.boot\.systemd-boot/);
  expect(text).toMatch(/ext4|archmaker\.filesystem\.ext4/i);
  expect(text).toMatch(/zen|archmaker\.kernel\.zen/i);

  return { digest, filename };
}
