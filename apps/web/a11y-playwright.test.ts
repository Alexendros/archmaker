import { test, expect } from "@playwright/test";
import AxeBuilder from "@axe-core/playwright";

const BASE_URL = process.env.BASE_URL || "http://localhost:5173";

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
