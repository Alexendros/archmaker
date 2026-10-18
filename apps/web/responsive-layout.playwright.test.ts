import { test, expect } from "@playwright/test";

const BASE_URL = process.env.BASE_URL || "http://localhost:5173/?mock=1";

const VIEWPORTS = [
  { name: "360", width: 360, height: 740 },
  { name: "768", width: 768, height: 1024 },
  { name: "1024", width: 1024, height: 768 },
  { name: "1440", width: 1440, height: 900 },
] as const;

/** CSS de cara vía /forge-face.css (CSP style-src 'self'; sin <style> de Vite). */
async function layoutCssSource(page: {
  request: {
    get: (
      url: string,
    ) => Promise<{ ok: () => boolean; text: () => Promise<string> }>;
  };
  url: () => string;
}): Promise<string> {
  const origin = new URL(page.url()).origin;
  const res = await page.request.get(`${origin}/forge-face.css`);
  expect(res.ok()).toBe(true);
  return res.text();
}

test.describe("Forge — responsive-layout", () => {
  for (const vp of VIEWPORTS) {
    test(`viewport ${vp.name}: shell usable sin zoom de página`, async ({
      page,
    }) => {
      await page.setViewportSize({ width: vp.width, height: vp.height });
      await page.goto(BASE_URL);
      await expect(
        page.getByRole("heading", { name: /Disco y particiones/i }),
      ).toBeVisible({ timeout: 30_000 });

      const zoom = await page.evaluate(() => {
        const root = document.documentElement;
        const body = document.body;
        const style = getComputedStyle(root);
        return {
          rootZoom: (style as CSSStyleDeclaration & { zoom?: string }).zoom,
          rootTransform: style.transform,
          bodyTransform: getComputedStyle(body).transform,
        };
      });
      expect(zoom.rootTransform === "none" || zoom.rootTransform === "").toBe(
        true,
      );
      expect(zoom.bodyTransform === "none" || zoom.bodyTransform === "").toBe(
        true,
      );

      const css = await layoutCssSource(page);
      expect(css).toMatch(/minmax\(\s*0\s*,\s*1fr\s*\)/);
      expect(css).not.toMatch(/\.forge-main\s*\{[^}]*max-width:\s*56rem/);
      expect(css).toMatch(/\.forge-stepper/);

      await expect(page.locator("#step-rail")).toBeAttached();
      await expect(page.locator("#step-stepper")).toBeAttached();
      // Rail y stepper coexisten en DOM; solo uno es visible por viewport (CSS).
      const visibleNavButton = page
        .locator("#step-rail:visible button, #step-stepper:visible button")
        .first();
      await expect(visibleNavButton).toBeVisible();

      await page.keyboard.press("Control+k");
      await expect(page.getByRole("dialog")).toBeVisible();
      await page.keyboard.press("Escape");

      await page
        .getByRole("button", { name: /Continuar → Sistema base/i })
        .click();
      await expect(
        page.getByRole("heading", { name: "Sistema base" }),
      ).toBeVisible();
    });
  }

  test("1440: centro flexible declarado y laterales presentes", async ({
    page,
  }) => {
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.goto(BASE_URL);
    await expect(
      page.getByRole("heading", { name: /Disco y particiones/i }),
    ).toBeVisible();

    const css = await layoutCssSource(page);
    expect(css).toMatch(/\.forge-shell\s*\{[^}]*minmax\(\s*0\s*,\s*1fr\s*\)/s);
    expect(css).toMatch(/minmax\(\s*12rem\s*,\s*17\.5rem\s*\)/);
    expect(css).toMatch(/minmax\(\s*14rem\s*,\s*22\.5rem\s*\)/);
    expect(css).not.toMatch(/max-width:\s*56rem/);

    await expect(page.locator("#step-rail")).toBeAttached();
    await expect(page.locator("#inspector")).toBeVisible();
    await expect(page.getByText("Resumen de configuración")).toBeVisible();
  });
});
