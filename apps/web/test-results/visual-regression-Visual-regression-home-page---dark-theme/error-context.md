# Instructions

- Following Playwright test failed.
- Explain why, be concise, respect Playwright best practices.
- Provide a snippet of code with the fix, if possible.

# Test info

- Name: visual-regression.test.ts >> Visual regression >> home page - dark theme
- Location: visual-regression.test.ts:83:3

# Error details

```
Error: page.goto: net::ERR_CONNECTION_REFUSED at http://localhost:5173/
Call log:
  - navigating to "http://localhost:5173/", waiting until "load"

```

# Test source

```ts
  1   | import { test, expect } from "@playwright/test";
  2   | import fs from "fs";
  3   | import path from "path";
  4   | import { fileURLToPath } from "url";
  5   | import { PNG } from "pngjs";
  6   | import pixelmatch from "pixelmatch";
  7   | 
  8   | const __filename = fileURLToPath(import.meta.url);
  9   | const __dirname = path.dirname(__filename);
  10  | 
  11  | const BASE_URL = process.env.BASE_URL || "http://localhost:5173";
  12  | const BASELINE_DIR = path.resolve(__dirname, "visual-baselines");
  13  | const DIFF_DIR = path.resolve(__dirname, "visual-diffs");
  14  | 
  15  | function ensureDir(dir: string) {
  16  |   if (!fs.existsSync(dir)) fs.mkdirSync(dir, { recursive: true });
  17  | }
  18  | 
  19  | function compareImages(
  20  |   baselineName: string,
  21  |   actualBuffer: Buffer,
  22  | ): { match: boolean; diffPercent: number } {
  23  |   ensureDir(BASELINE_DIR);
  24  |   ensureDir(DIFF_DIR);
  25  | 
  26  |   const baselinePath = path.join(BASELINE_DIR, `${baselineName}.png`);
  27  |   const diffPath = path.join(DIFF_DIR, `${baselineName}.diff.png`);
  28  | 
  29  |   // If baseline doesn't exist, create it and pass
  30  |   if (!fs.existsSync(baselinePath)) {
  31  |     fs.writeFileSync(baselinePath, actualBuffer);
  32  |     return { match: true, diffPercent: 0 };
  33  |   }
  34  | 
  35  |   const baselineImg = PNG.sync.read(fs.readFileSync(baselinePath));
  36  |   const actualImg = PNG.sync.read(actualBuffer);
  37  | 
  38  |   if (
  39  |     baselineImg.width !== actualImg.width ||
  40  |     baselineImg.height !== actualImg.height
  41  |   ) {
  42  |     return { match: false, diffPercent: 100 };
  43  |   }
  44  | 
  45  |   const diff = new PNG({
  46  |     width: baselineImg.width,
  47  |     height: baselineImg.height,
  48  |   });
  49  |   const diffPixels = pixelmatch(
  50  |     baselineImg.data,
  51  |     actualImg.data,
  52  |     diff.data,
  53  |     baselineImg.width,
  54  |     baselineImg.height,
  55  |     {
  56  |       threshold: 0.1,
  57  |       includeAA: true,
  58  |     },
  59  |   );
  60  | 
  61  |   const diffPercent =
  62  |     (diffPixels / (baselineImg.width * baselineImg.height)) * 100;
  63  | 
  64  |   if (diffPercent > 0.1) {
  65  |     fs.writeFileSync(diffPath, PNG.sync.write(diff));
  66  |   }
  67  | 
  68  |   return { match: diffPercent <= 0.1, diffPercent };
  69  | }
  70  | 
  71  | test.describe("Visual regression", () => {
  72  |   test.beforeEach(async ({ page }) => {
> 73  |     await page.goto(BASE_URL);
      |                ^ Error: page.goto: net::ERR_CONNECTION_REFUSED at http://localhost:5173/
  74  |     await page.waitForLoadState("networkidle");
  75  |   });
  76  | 
  77  |   test("home page - light theme", async ({ page }) => {
  78  |     const screenshot = await page.screenshot({ fullPage: true });
  79  |     const result = compareImages("home-light", screenshot);
  80  |     expect(result.match).toBeTruthy();
  81  |   });
  82  | 
  83  |   test("home page - dark theme", async ({ page }) => {
  84  |     await page.emulateMedia({ colorScheme: "dark" });
  85  |     await page.waitForTimeout(100);
  86  |     const screenshot = await page.screenshot({ fullPage: true });
  87  |     const result = compareImages("home-dark", screenshot);
  88  |     expect(result.match).toBeTruthy();
  89  |   });
  90  | 
  91  |   test("home page - high contrast", async ({ page }) => {
  92  |     await page.emulateMedia({ forcedColors: "active" });
  93  |     await page.waitForTimeout(100);
  94  |     const screenshot = await page.screenshot({ fullPage: true });
  95  |     const result = compareImages("home-high-contrast", screenshot);
  96  |     expect(result.match).toBeTruthy();
  97  |   });
  98  | 
  99  |   test("home page - reduced motion", async ({ page }) => {
  100 |     await page.emulateMedia({ reducedMotion: "reduce" });
  101 |     await page.waitForTimeout(100);
  102 |     const screenshot = await page.screenshot({ fullPage: true });
  103 |     const result = compareImages("home-reduced-motion", screenshot);
  104 |     expect(result.match).toBeTruthy();
  105 |   });
  106 | });
  107 | 
```