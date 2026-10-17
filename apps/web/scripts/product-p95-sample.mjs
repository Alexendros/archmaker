#!/usr/bin/env node
/**
 * Una muestra de rendimiento de producto (WASM real, sin MockCorePort).
 *
 * Emite una línea JSON: { readyMs, prepareMs, valid, error? }
 * ready = navegación → Forge interactiva (heading + Draft listo, sin mock)
 * prepare = clic Generar → toast Manifest listo
 *
 * Uso (servidor ya arriba, sin ?mock=1):
 *   BASE_URL_WASM=http://127.0.0.1:4173/ node scripts/product-p95-sample.mjs
 */

import { chromium } from "playwright";

const BASE_URL = process.env.BASE_URL_WASM || "http://127.0.0.1:4173/";
const TIMEOUT_MS = Number(process.env.PRODUCT_P95_TIMEOUT_MS || 60_000);

function emit(payload) {
  process.stdout.write(JSON.stringify(payload) + "\n");
}

async function measureOne() {
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({
    viewport: { width: 1280, height: 720 },
  });
  try {
    if (BASE_URL.includes("mock=1")) {
      throw new Error("BASE_URL_WASM no debe incluir mock=1");
    }

    const readyStart = Date.now();
    await page.goto(BASE_URL, { waitUntil: "commit", timeout: TIMEOUT_MS });
    await page
      .getByRole("heading", { name: /Disco y particiones/i })
      .waitFor({ state: "visible", timeout: TIMEOUT_MS });
    await page.locator("#toast-region").getByText(/Draft listo/i).waitFor({
      state: "visible",
      timeout: TIMEOUT_MS,
    });
    const mockCount = await page.getByText(/Modo demostración/i).count();
    if (mockCount > 0) {
      throw new Error("MockCorePort activo (Modo demostración)");
    }
    const motorDown = await page
      .getByRole("heading", { name: /Motor no disponible/i })
      .count();
    if (motorDown > 0) {
      throw new Error("Motor WASM no disponible");
    }
    const readyMs = Date.now() - readyStart;

    const prepareStart = Date.now();
    await page.getByRole("button", { name: "Generar" }).click();
    await page
      .getByRole("heading", { name: "Revisión" })
      .waitFor({ state: "visible", timeout: TIMEOUT_MS });
    await page.locator("#toast-region").getByText(/Manifest listo/i).waitFor({
      state: "visible",
      timeout: TIMEOUT_MS,
    });
    const prepareMs = Date.now() - prepareStart;

    emit({ readyMs, prepareMs, valid: true });
  } finally {
    await browser.close();
  }
}

measureOne().catch((err) => {
  emit({
    readyMs: null,
    prepareMs: null,
    valid: false,
    error: String(err && err.message ? err.message : err),
  });
  process.exitCode = 1;
});
