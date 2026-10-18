/**
 * Helpers del recorrido Forge de cinco pasos:
 * selección → Generar → Pedir imagen (error tipado mientras la constitución
 * no está cableada).
 */
import { expect, type Page } from "@playwright/test";

/** Hostname del ejercicio de ejemplo (F3). */
export const E2E_HOSTNAME = "arch-e2e-audit";

/** Cinco pasos del rail Forge (RAIL_STEPS). */
export const FORGE_RAIL_STEPS = [
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
    id: "review-image",
    route: "review",
    step: "Revisión",
    control: "Pedir imagen",
  },
  {
    id: "review-import",
    route: "review",
    step: "Revisión",
    control: "Importar",
  },
] as const;

/** Texto visible: sin disco, partición, tabla, ESP, LUKS, cifrado ni formateo. */
export async function assertNoDiskVocabulary(page: Page): Promise<void> {
  const body = await page.locator("body").innerText();
  expect(body).not.toMatch(/\bdiscos?\b/i);
  expect(body).not.toMatch(/partici[oó]n/i);
  expect(body).not.toMatch(/\bGPT\b/);
  expect(body).not.toMatch(/\bMBR\b/);
  expect(body).not.toMatch(/\bESP\b/);
  expect(body).not.toMatch(/\bLUKS\b/i);
  expect(body).not.toMatch(/cifrado/i);
  expect(body).not.toMatch(/formatear/i);
}

/** Recorrido: base → GNOME → identidad. Sin disco ni cifrado. */
export async function completeExampleSelection(page: Page): Promise<void> {
  await page.getByRole("radio", { name: /Linux Zen/i }).click();
  await expect(
    page.getByRole("radio", { name: /systemd-boot/i }),
  ).toBeChecked();

  await page.getByRole("button", { name: /Continuar → Entorno/i }).click();
  await expect(page.getByRole("radio", { name: /GNOME/i })).toBeChecked();

  await page.getByRole("button", { name: /Continuar → Apps/i }).click();
  await expect(
    page.getByRole("checkbox", { name: /^Zen Browser$/i }),
  ).toBeChecked();

  await page.getByRole("button", { name: /Continuar → Identidad/i }).click();
  await page.getByPlaceholder("mi-equipo").fill(E2E_HOSTNAME);

  await page.getByRole("button", { name: /Continuar → Revisión/i }).click();
  await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible();
  await expect(page.getByText(`Hostname: ${E2E_HOSTNAME}`)).toBeVisible();
}

/**
 * Generar deja el manifiesto listo. Pedir imagen llama a la operación tipada
 * y no descarga un archivo: en este runtime no hay mkarchiso.
 */
export async function generateAndAskForImage(page: Page): Promise<void> {
  await page.getByRole("button", { name: "Generar" }).click();
  await expect(page.getByRole("heading", { name: "Revisión" })).toBeVisible();
  await expect(page.locator("#toast-region")).toContainText(
    /Manifest listo|Validación/i,
    { timeout: 60_000 },
  );

  await page.getByRole("button", { name: "Pedir imagen" }).click();
  await expect(page.locator("#toast-region")).toContainText(
    /constitution-mkarchiso-absent/,
    { timeout: 15_000 },
  );
  await expect(page.getByText(/Archivo guardado/i)).toHaveCount(0);
}
