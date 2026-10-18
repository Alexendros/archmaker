import fs from "node:fs";
import path from "node:path";
import { fileURLToPath } from "node:url";
import { defineConfig, type Plugin } from "vite";
import react from "@vitejs/plugin-react";
import pkg from "./package.json";

const webRoot = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(webRoot, "../..");
const tokensPath = path.join(repoRoot, "packages/tokens/tokens.css");
const stylesPath = path.join(webRoot, "src/styles.css");

/** CSS concatenado como text/css real (CSP style-src 'self'; sin <style> de Vite). */
function readForgeFaceCss(): string {
  const tokens = fs.readFileSync(tokensPath, "utf8");
  const styles = fs
    .readFileSync(stylesPath, "utf8")
    .replace(/@import\s+["'][^"']+["']\s*;\s*/g, "");
  return `${tokens}\n${styles}`;
}

function forgeFaceCssPlugin(): Plugin {
  const urlPath = "/forge-face.css";
  return {
    name: "forge-face-css",
    configureServer(server) {
      server.middlewares.use((req, res, next) => {
        const pathname = req.url?.split("?", 1)[0];
        if (pathname !== urlPath) {
          next();
          return;
        }
        res.setHeader("Content-Type", "text/css; charset=utf-8");
        res.setHeader("Cache-Control", "no-cache");
        res.end(readForgeFaceCss());
      });
    },
    transformIndexHtml(html) {
      if (html.includes(urlPath)) return html;
      return html.replace(
        "</title>",
        `</title>\n    <link rel="stylesheet" href="${urlPath}" />`,
      );
    },
    generateBundle() {
      this.emitFile({
        type: "asset",
        fileName: "forge-face.css",
        source: readForgeFaceCss(),
      });
    },
  };
}

export default defineConfig({
  plugins: [react(), forgeFaceCssPlugin()],
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
