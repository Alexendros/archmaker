import { createRoot } from "react-dom/client";
import { App } from "./App.js";
import { createCorePort } from "./create-core-port.js";
/* Estilos vía <link href="/forge-face.css"> (plugin Vite): CSP style-src 'self'
   sin <style> inyectado. El @import de tokens vive en styles.css para el plugin. */

function renderBootError(message: string): void {
  const root = document.getElementById("root");
  if (!root) throw new Error("missing #root");
  root.innerHTML = "";
  const section = document.createElement("section");
  section.className = "forge-boot-error";
  section.setAttribute("role", "alert");
  const h1 = document.createElement("h1");
  h1.textContent = "Motor no disponible";
  const p = document.createElement("p");
  p.textContent = message;
  const hint = document.createElement("p");
  hint.textContent =
    "No se ha activado el simulador. En desarrollo puedes usar ?mock=1 de forma explícita.";
  section.append(h1, p, hint);
  root.append(section);
}

async function boot(): Promise<void> {
  const root = document.getElementById("root");
  if (!root) throw new Error("missing #root");

  const resolved = await createCorePort();
  if (!resolved.ok) {
    renderBootError(resolved.error);
    return;
  }

  createRoot(root).render(
    <App port={resolved.port} portKind={resolved.kind} />,
  );
}

void boot();
