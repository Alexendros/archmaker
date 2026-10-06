import type { CorePort, Draft, Result } from "./coreport.js";
import { typedError } from "./coreport.js";
import { announce, currentCatalogRef, currentTarget, renderOptionCard, renderStepRail, renderValidationPanel, withSelection } from "./components.js";

declare global {
  interface Window {
    __TAURI__?: { core: { invoke: (cmd: string, args?: unknown) => Promise<unknown> } };
  }
}

class TauriCorePort implements CorePort {
  private async call<T>(cmd: string, args?: unknown): Promise<Result<T>> {
    const invoke = window.__TAURI__?.core.invoke;
    if (!invoke) return { ok: false, error: typedError("AM-PROTO-002", "archmaker-web", "error.proto.transportUnavailable") };
    try {
      const value = (await invoke(cmd, args)) as T;
      return { ok: true, value };
    } catch (e) {
      const message = e instanceof Error ? e.message : String(e);
      if (/denied|forbidden|capability|permission/i.test(message)) {
        return { ok: false, error: typedError("AM-PROTO-001", "archmaker-web", "error.proto.invokeDenied") };
      }
      return { ok: false, error: typedError("AM-PROTO-002", "archmaker-web", "error.proto.invokeFailed") };
    }
  }
  createDraft(input: { draftId?: string; catalogRef: never; targetRef: never }): Promise<Result<Draft>> {
    return this.call<Draft>("create_draft", { input });
  }
  loadCatalog(input: { source: "embedded" | "file" }): Promise<Result<unknown>> {
    return this.call<unknown>("load_catalog", { input });
  }
  saveDraft(input: { draft: Draft; expectedRevision: number }): Promise<Result<Draft>> {
    return this.call<Draft>("save_draft", { input });
  }
  resolveDraft(input: { draft: Draft; catalogRef: never }): Promise<Result<{ resolutionDigest: unknown; effectiveSelections: never[] }>> {
    return this.call("resolve_draft", { input });
  }
  validateDraft(input: { draft: Draft; catalogRef: never }): Promise<Result<{ blocking: boolean; diagnostics: never[] }>> {
    return this.call("validate_draft", { input });
  }
  buildManifest(input: { draft: Draft; catalogRef: never; targetRef: never }): Promise<Result<unknown>> {
    return this.call<unknown>("build_manifest", { input });
  }
  exportArtifact(input: { manifestDigest: unknown; targetRef: never; destination: string }): Promise<Result<unknown>> {
    return this.call<unknown>("export_artifact", { input });
  }
}

const port: CorePort = new TauriCorePort();
let draft: Draft | null = null;
let currentStep = 0;
const railStatus = ["", "", "", "", "", ""];

const OPTIONS = [
  { id: "archmaker.kernel.linux", step: "archmaker.sys.kernel", title: "Kernel Linux" },
  { id: "archmaker.desktop.gnome", step: "archmaker.sys.desktop", title: "Escritorio GNOME" },
];

function setStep(n: number, mark: string): void {
  currentStep = n;
  railStatus[n] = mark;
  const rail = document.getElementById("step-rail");
  if (rail) renderStepRail(rail, currentStep, railStatus);
}

function fail(err: { code: string; messageKey: string }, where: string): void {
  announce("alert", `${where}: ${err.code} (${err.messageKey})`);
  const out = document.getElementById("export-out");
  if (out && where === "Exportar") out.textContent = `${err.code} ${err.messageKey}`;
}

async function boot(): Promise<void> {
  const rail = document.getElementById("step-rail");
  if (rail) renderStepRail(rail, 0, railStatus);
  const themeBtn = document.getElementById("theme-toggle");
  themeBtn?.addEventListener("click", () => {
    const dark = document.documentElement.classList.toggle("dark");
    themeBtn.setAttribute("aria-pressed", String(dark));
    themeBtn.textContent = dark ? "Tema claro" : "Tema oscuro";
  });
  const opts = document.getElementById("options");
  const inspectorBody = document.getElementById("inspector-body");
  for (const o of OPTIONS) {
    const card = renderOptionCard({ id: o.id, title: o.title, selected: false });
    card.addEventListener("click", () => {
      if (!draft) {
        announce("alert", "Crea primero el draft (paso 1).");
        return;
      }
      draft = withSelection(draft, { step_id: o.step, option_id: o.id });
      const selected = draft.selections.some((s) => s.option_id === o.id);
      card.setAttribute("aria-checked", String(selected));
      const state = card.querySelector(".option-state");
      if (state) state.textContent = selected ? "● Seleccionada" : "○ Sin seleccionar";
      if (inspectorBody) inspectorBody.textContent = `${o.title}: ${o.id} en ${o.step}.`;
      setStep(1, "✓");
      announce("status", `${o.title} seleccionada.`);
    });
    opts?.append(card);
  }
  document.getElementById("inspector-evidence")?.addEventListener("click", () => {
    const dlg = document.getElementById("evidence-dialog") as HTMLDialogElement | null;
    const body = document.getElementById("evidence-body");
    if (body) body.textContent = inspectorBody?.textContent ?? "Sin selección.";
    dlg?.showModal();
  });
  document.getElementById("btn-create")?.addEventListener("click", async () => {
    const target = (document.getElementById("target-select") as HTMLSelectElement).value;
    const res = await port.createDraft({ catalogRef: currentCatalogRef() as never, targetRef: currentTarget(target) as never });
    if (!res.ok) {
      fail(res.error, "Crear draft");
      return;
    }
    draft = res.value;
    setStep(0, "✓");
    announce("status", "Draft creado.");
  });
  document.getElementById("btn-resolve")?.addEventListener("click", async () => {
    if (!draft) {
      announce("alert", "Crea primero el draft (paso 1).");
      return;
    }
    const res = await port.resolveDraft({ draft, catalogRef: currentCatalogRef() as never });
    const out = document.getElementById("resolve-out");
    if (!res.ok) {
      if (out) out.textContent = `${res.error.code} ${res.error.messageKey}`;
      fail(res.error, "Resolver");
      return;
    }
    if (out) out.textContent = `Resuelto: ${res.value.effectiveSelections.length} selecciones efectivas.`;
    setStep(2, "✓");
    announce("status", "Draft resuelto.");
  });
  document.getElementById("btn-validate")?.addEventListener("click", async () => {
    if (!draft) {
      announce("alert", "Crea primero el draft (paso 1).");
      return;
    }
    const res = await port.validateDraft({ draft, catalogRef: currentCatalogRef() as never });
    const panel = document.getElementById("validation-panel");
    if (!res.ok) {
      fail(res.error, "Validar");
      return;
    }
    if (panel) {
      renderValidationPanel(panel, res.value.diagnostics, (path) => {
        const target = document.querySelector(`[data-option-id="${path}"]`) as HTMLElement | null;
        target?.focus();
      });
    }
    setStep(3, res.value.blocking ? "⚠" : "✓");
    announce("status", res.value.blocking ? "Validación con bloqueos." : "Validación sin bloqueos.");
  });
  document.getElementById("btn-manifest")?.addEventListener("click", async () => {
    if (!draft) {
      announce("alert", "Crea primero el draft (paso 1).");
      return;
    }
    const target = (document.getElementById("target-select") as HTMLSelectElement).value;
    const res = await port.buildManifest({ draft, catalogRef: currentCatalogRef() as never, targetRef: currentTarget(target) as never });
    const out = document.getElementById("manifest-out");
    if (!res.ok) {
      if (out) out.textContent = `${res.error.code} ${res.error.messageKey}`;
      fail(res.error, "Manifest");
      return;
    }
    if (out) out.textContent = JSON.stringify(res.value, null, 2);
    setStep(4, "✓");
    announce("status", "Manifest construido.");
  });
  document.getElementById("btn-export")?.addEventListener("click", async () => {
    if (!draft) {
      announce("alert", "Crea primero el draft (paso 1).");
      return;
    }
    const target = (document.getElementById("target-select") as HTMLSelectElement).value;
    const res = await port.exportArtifact({ manifestDigest: null, targetRef: currentTarget(target) as never, destination: "picker:opaque-handle" });
    const out = document.getElementById("export-out");
    if (!res.ok) {
      if (out) out.textContent = `${res.error.code} ${res.error.messageKey}`;
      fail(res.error, "Exportar");
      return;
    }
    if (out) out.textContent = "Artefacto exportado.";
    setStep(5, "✓");
    announce("status", "Artefacto exportado.");
  });
}

void boot();
