import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

export default defineConfig({
  plugins: [react()],
  root: ".",
  server: {
    port: 5173,
    strictPort: true,
    fs: {
      // Allow serving wasm-pack / wasm-bindgen output from the monorepo crate.
      allow: ["../.."],
    },
  },
  optimizeDeps: {
    exclude: ["archmaker_wasm"],
  },
  assetsInclude: ["**/*.wasm"],
});
