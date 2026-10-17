import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import pkg from "./package.json";

export default defineConfig({
  plugins: [react()],
  root: ".",
  define: {
    __FORGE_APP_VERSION__: JSON.stringify(pkg.version),
  },
  resolve: {
    alias: {
      // Relative to this config file (apps/web/).
      "archmaker-wasm": new URL(
        "../../crates/archmaker-wasm/pkg/archmaker_wasm.js",
        import.meta.url,
      ).pathname,
    },
  },
  server: {
    port: 5173,
    strictPort: true,
    fs: {
      // Allow serving wasm-pack / wasm-bindgen output from the monorepo crate.
      allow: ["../.."],
    },
  },
  optimizeDeps: {
    exclude: ["archmaker-wasm"],
  },
  assetsInclude: ["**/*.wasm"],
});
