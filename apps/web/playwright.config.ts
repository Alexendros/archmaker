import { defineConfig } from "@playwright/test";

const port = 5173;
const baseURL = process.env.BASE_URL || `http://localhost:${port}`;

export default defineConfig({
  testDir: ".",
  fullyParallel: false,
  forbidOnly: !!process.env.CI,
  retries: process.env.CI ? 1 : 0,
  workers: 1,
  use: {
    baseURL,
  },
  webServer: {
    command: `pnpm exec vite --host localhost --port ${port}`,
    url: baseURL,
    reuseExistingServer: !process.env.CI,
    timeout: 120_000,
  },
});
