import { createRoot } from "react-dom/client";
import { App } from "./App.js";
import { MockCorePort } from "./mock-core-port.js";
import { TauriCorePort } from "./tauri-core-port.js";
import "./styles.css";

function resolvePort() {
  if (typeof window !== "undefined" && window.__TAURI__?.core?.invoke) {
    return new TauriCorePort();
  }
  return new MockCorePort();
}

const root = document.getElementById("root");
if (!root) {
  throw new Error("missing #root");
}
createRoot(root).render(<App port={resolvePort()} />);
