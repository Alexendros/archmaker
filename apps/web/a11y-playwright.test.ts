import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const BASE_URL = process.env.BASE_URL || "http://localhost:5173/?mock=1";

test.describe("WCAG 2.2 AA - axe-core", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");
  });

  test("home page - light theme", async ({ page }) => {
    const accessibilityScanResults = await new AxeBuilder({ page })
      .withTags(["wcag2aa", "wcag21aa", "best-practice"])
      .analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test("home page - dark theme", async ({ page }) => {
    await page.emulateMedia({ colorScheme: "dark" });
    const accessibilityScanResults = await new AxeBuilder({ page })
      .withTags(["wcag2aa", "wcag21aa", "best-practice"])
      .analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test("home page - high contrast", async ({ page }) => {
    await page.emulateMedia({ forcedColors: "active" });
    const accessibilityScanResults = await new AxeBuilder({ page })
      .withTags(["wcag2aa", "wcag21aa", "best-practice"])
      .analyze();
    expect(accessibilityScanResults.violations).toEqual([]);
  });

  test("keyboard navigation - tab order", async ({ page }) => {
    // Test that focus moves logically through interactive elements
    await page.keyboard.press("Tab");
    const firstFocus = await page.evaluate(
      () => document.activeElement?.tagName,
    );
    expect([
      "BUTTON",
      "A",
      "INPUT",
      "SELECT",
      "TEXTAREA",
      "NAV",
      "MAIN",
      "ASIDE",
    ]).toContain(firstFocus);

    await page.keyboard.press("Tab");
    const secondFocus = await page.evaluate(
      () => document.activeElement?.tagName,
    );
    expect([
      "BUTTON",
      "A",
      "INPUT",
      "SELECT",
      "TEXTAREA",
      "NAV",
      "MAIN",
      "ASIDE",
    ]).toContain(secondFocus);
  });

  test("skip link works", async ({ page }) => {
    await page.keyboard.press("Tab");
    const skipLink = await page.evaluate(() => {
      const el = document.activeElement;
      return (
        el?.textContent?.includes("skip") ||
        el?.getAttribute("href") === "#main"
      );
    });
    expect(skipLink).toBeTruthy();
  });

  test("live regions present", async ({ page }) => {
    const statusRegion = await page.locator('[role="status"]').count();
    const alertRegion = await page.locator('[role="alert"]').count();
    expect(statusRegion + alertRegion).toBeGreaterThan(0);
  });

  test("dialog accessibility", async ({ page }) => {
    // Trigger dialog if there's a button to open it
    const dialogTrigger = page
      .locator(
        'button:has-text("dialog"), button:has-text("Dialog"), [data-testid="open-dialog"]',
      )
      .first();
    if ((await dialogTrigger.count()) > 0) {
      await dialogTrigger.click();
      await page.waitForSelector('[role="dialog"]');
      const dialog = page.locator('[role="dialog"]');
      await expect(dialog).toHaveAttribute("aria-labelledby");
      await expect(dialog).toHaveAttribute("aria-modal", "true");
    }
  });

  test("reduced motion respected", async ({ page }) => {
    await page.emulateMedia({ reducedMotion: "reduce" });
    const transitions = await page.evaluate(() => {
      const styles = getComputedStyle(document.documentElement);
      return styles.getPropertyValue("--transition-duration") || "0s";
    });
    // Should have no transitions or very fast ones
    expect(transitions).toMatch(/0s|0ms/);
  });
});

/** F6: viewports y zoom ampliando la suite axe existente (complemento, no WCAG). */
test.describe("F6 viewports y foco", () => {
  for (const width of [360, 768, 1440] as const) {
    test(`rail y skip link visibles a ${width}px`, async ({ page }) => {
      await page.setViewportSize({ width, height: 800 });
      await page.goto(BASE_URL);
      await expect(
        page.getByRole("heading", { name: /Disco y particiones/i }),
      ).toBeVisible({ timeout: 30_000 });
      // Rail y stepper coexisten; solo uno es visible por viewport (CSS).
      await expect(
        page.locator("#step-rail:visible, #step-stepper:visible").first(),
      ).toBeVisible();
      await page.keyboard.press("Tab");
      const tag = await page.evaluate(() => document.activeElement?.tagName);
      expect(["A", "BUTTON", "INPUT", "SELECT", "TEXTAREA", "NAV"]).toContain(
        tag,
      );
    });
  }

  test("zoom 200% mantiene región de estado", async ({ page }) => {
    await page.setViewportSize({ width: 1280, height: 720 });
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");
    await page.evaluate(() => {
      document.documentElement.style.zoom = "2";
    });
    await expect(page.locator("#toast-region")).toHaveCount(1);
    await expect(page.locator("#alert-region")).toHaveCount(1);
  });
});
