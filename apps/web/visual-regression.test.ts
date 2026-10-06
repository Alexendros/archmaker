import { test, expect } from "@playwright/test";
import fs from "fs";
import path from "path";
import { fileURLToPath } from "url";
import { PNG } from "pngjs";
import pixelmatch from "pixelmatch";

const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);

const BASE_URL = process.env.BASE_URL || "http://localhost:5173";
const BASELINE_DIR = path.resolve(__dirname, "visual-baselines");
const DIFF_DIR = path.resolve(__dirname, "visual-diffs");

function ensureDir(dir: string) {
  if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
}

function compareImages(
  baselineName: string,
  actualBuffer: Buffer,
): { match: boolean; diffPercent: number } {
  ensureDir(BASELINE_DIR);
  ensureDir(DIFF_DIR);

  const baselinePath = path.join(BASELINE_DIR, `${baselineName}.png`);
  const diffPath = path.join(DIFF_DIR, `${baselineName}.diff.png`);

  // If baseline doesn't exist, create it and pass
  if (!fs.existsSync(baselinePath)) {
    fs.writeFileSync(baselinePath, actualBuffer);
    return { match: true, diffPercent: 0 };
  }

  const baselineImg = PNG.sync.read(fs.readFileSync(baselinePath));
  const actualImg = PNG.sync.read(actualBuffer);

  if (
    baselineImg.width !== actualImg.width ||
    baselineImg.height !== actualImg.height
  ) {
    return { match: false, diffPercent: 100 };
  }

  const diff = new PNG({
    width: baselineImg.width,
    height: baselineImg.height,
  });
  const diffPixels = pixelmatch(
    baselineImg.data,
    actualImg.data,
    diff.data,
    baselineImg.width,
    baselineImg.height,
    {
      threshold: 0.1,
      includeAA: true,
    },
  );

  const diffPercent =
    (diffPixels / (baselineImg.width * baselineImg.height)) * 100;

  if (diffPercent > 0.1) {
    fs.writeFileSync(diffPath, PNG.sync.write(diff));
  }

  return { match: diffPercent <= 0.1, diffPercent };
}

test.describe("Visual regression", () => {
  test.beforeEach(async ({ page }) => {
    await page.goto(BASE_URL);
    await page.waitForLoadState("networkidle");
  });

  test("home page - light theme", async ({ page }) => {
    const screenshot = await page.screenshot({ fullPage: true });
    const result = compareImages("home-light", screenshot);
    expect(result.match).toBeTruthy();
  });

  test("home page - dark theme", async ({ page }) => {
    await page.emulateMedia({ colorScheme: "dark" });
    await page.waitForTimeout(100);
    const screenshot = await page.screenshot({ fullPage: true });
    const result = compareImages("home-dark", screenshot);
    expect(result.match).toBeTruthy();
  });

  test("home page - high contrast", async ({ page }) => {
    await page.emulateMedia({ forcedColors: "active" });
    await page.waitForTimeout(100);
    const screenshot = await page.screenshot({ fullPage: true });
    const result = compareImages("home-high-contrast", screenshot);
    expect(result.match).toBeTruthy();
  });

  test("home page - reduced motion", async ({ page }) => {
    await page.emulateMedia({ reducedMotion: "reduce" });
    await page.waitForTimeout(100);
    const screenshot = await page.screenshot({ fullPage: true });
    const result = compareImages("home-reduced-motion", screenshot);
    expect(result.match).toBeTruthy();
  });
});
