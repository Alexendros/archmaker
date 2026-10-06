import type { CatalogRef, Diagnostic, Draft, Selection, TargetRef } from "./coreport.js";

export const STEPS = ["Crear draft", "Seleccionar", "Resolver", "Validar", "Manifest", "Exportar"];

export function renderStepRail(el: HTMLElement, current: number, status: string[]): void {
  el.innerHTML = "";
  const list = document.createElement("ol");
  STEPS.forEach((label, i) => {
    const li = document.createElement("li");
    const a = document.createElement("a");
    a.href = `#step-${["create", "select", "resolve", "validate", "manifest", "export"][i]}`;
    a.textContent = `${i + 1}. ${label}`;
    if (i === current) a.setAttribute("aria-current", "step");
    const badge = document.createElement("span");
    badge.className = "rail-badge";
    badge.textContent = status[i] ?? "";
    li.append(a, badge);
    list.append(li);
  });
  el.append(list);
}

export function renderOptionCard(opt: { id: string; title: string; selected: boolean; locked?: boolean }): HTMLElement {
  const btn = document.createElement("button");
  btn.type = "button";
  btn.className = "option-card";
  btn.setAttribute("role", "checkbox");
  btn.setAttribute("aria-checked", String(opt.selected));
  btn.dataset.optionId = opt.id;
  if (opt.locked) btn.setAttribute("aria-disabled", "true");
  const title = document.createElement("span");
  title.className = "option-title";
  title.textContent = opt.title;
  const state = document.createElement("span");
  state.className = "option-state";
  state.textContent = opt.selected ? "● Seleccionada" : "○ Sin seleccionar";
  btn.append(title, state);
  return btn;
}

export function renderValidationPanel(el: HTMLElement, diagnostics: Diagnostic[], onNavigate: (path: string) => void): void {
  el.innerHTML = "";
  const summary = document.createElement("p");
  summary.setAttribute("role", "status");
  const errors = diagnostics.filter((d) => d.code.startsWith("AM-") && d.source !== "").length;
  summary.textContent = diagnostics.length === 0 ? "Sin incidencias." : `${diagnostics.length} diagnósticos (${errors} con código tipado).`;
  el.append(summary);
  const list = document.createElement("ul");
  for (const d of diagnostics) {
    const li = document.createElement("li");
    const a = document.createElement("a");
    a.href = "#";
    a.textContent = `${d.code} · ${d.path}`;
    a.setAttribute("aria-describedby", d.messageKey);
    a.addEventListener("click", (ev) => {
      ev.preventDefault();
      onNavigate(d.path);
    });
    li.append(a);
    list.append(li);
  }
  el.append(list);
}

export function announce(kind: "status" | "alert", message: string): void {
  const region = document.getElementById(kind === "status" ? "toast-region" : "alert-region");
  if (!region) return;
  region.textContent = "";
  const span = document.createElement("span");
  span.className = kind === "status" ? "toast" : "toast toast-error";
  span.textContent = message;
  region.append(span);
}

export function currentCatalogRef(): CatalogRef {
  return {
    namespace: "archmaker.core",
    id: "minimal",
    version: "0.1.0",
    digest: { algorithm: "sha256", value: "embedded" },
  };
}

export function currentTarget(id: string): TargetRef {
  return { id, version: "1" };
}

export function withSelection(draft: Draft, sel: Selection): Draft {
  const rest = draft.selections.filter((s) => s.step_id !== sel.step_id);
  return { ...draft, selections: [...rest, sel] };
}
