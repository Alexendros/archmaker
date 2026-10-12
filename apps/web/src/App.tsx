import { useCallback, useMemo, useState } from "react";
import type {
  CorePort,
  Diagnostic,
  Draft,
  Manifest,
  Result,
} from "./coreport.js";
import {
  DRAFT_TARGET,
  FIXED_CATALOG_REF,
  exportTarget,
  singleSelection,
  withSelection,
} from "./coreport.js";

const STEPS = [
  "Crear draft",
  "Seleccionar",
  "Resolver",
  "Validar",
  "Manifest",
  "Exportar",
];

const OPTIONS = [
  {
    id: "archmaker.kernel.linux",
    stepId: "archmaker.sys.base",
    title: "Kernel Linux",
  },
  {
    id: "archmaker.desktop.gnome",
    stepId: "archmaker.sys.session",
    title: "Escritorio GNOME",
  },
];

const EXPORT_TARGETS = [
  "archmaker-profile",
  "archinstall-profile",
  "report",
] as const;

type Props = { port: CorePort };

function announce(kind: "status" | "alert", message: string): void {
  const region = document.getElementById(
    kind === "status" ? "toast-region" : "alert-region",
  );
  if (!region) return;
  region.textContent = "";
  const span = document.createElement("span");
  span.className = kind === "status" ? "toast" : "toast toast-error";
  span.textContent = message;
  region.append(span);
}

function fail(err: { code: string; messageKey: string }, where: string): void {
  announce("alert", `${where}: ${err.code} (${err.messageKey})`);
}

export function App({ port }: Props) {
  const [draft, setDraft] = useState<Draft | null>(null);
  const [manifest, setManifest] = useState<Manifest | null>(null);
  const [exportTargetId, setExportTargetId] =
    useState<(typeof EXPORT_TARGETS)[number]>("archmaker-profile");
  const [currentStep, setCurrentStep] = useState(0);
  const [railStatus, setRailStatus] = useState<string[]>([
    "",
    "",
    "",
    "",
    "",
    "",
  ]);
  const [dark, setDark] = useState(false);
  const [resolveOut, setResolveOut] = useState("");
  const [diagnostics, setDiagnostics] = useState<Diagnostic[]>([]);
  const [manifestOut, setManifestOut] = useState("");
  const [exportOut, setExportOut] = useState("");
  const [inspector, setInspector] = useState("Sin selección.");
  const [dialogOpen, setDialogOpen] = useState(false);

  const markStep = useCallback((n: number, mark: string) => {
    setCurrentStep(n);
    setRailStatus((prev) => {
      const next = [...prev];
      next[n] = mark;
      return next;
    });
  }, []);

  const selectedIds = useMemo(() => {
    const set = new Set<string>();
    for (const s of draft?.selections ?? []) {
      if (s.value.kind === "single") set.add(s.value.optionId);
    }
    return set;
  }, [draft]);

  async function onCreate(): Promise<void> {
    const res = await port.createDraft({
      catalogRef: FIXED_CATALOG_REF,
      targetRef: DRAFT_TARGET,
    });
    if (!res.ok) {
      fail(res.error, "Crear draft");
      return;
    }
    setDraft(res.value);
    setManifest(null);
    markStep(0, "✓");
    announce("status", "Draft creado.");
  }

  function onToggleOption(opt: (typeof OPTIONS)[number]): void {
    if (!draft) {
      announce("alert", "Crea primero el draft (paso 1).");
      return;
    }
    const already = selectedIds.has(opt.id);
    let next: Draft;
    if (already) {
      next = {
        ...draft,
        selections: draft.selections.filter((s) => s.stepId !== opt.stepId),
      };
    } else {
      next = withSelection(draft, singleSelection(opt.stepId, opt.id));
    }
    setDraft(next);
    setInspector(`${opt.title}: ${opt.id} en ${opt.stepId}.`);
    markStep(1, "✓");
    announce("status", `${opt.title} ${already ? "quitada" : "seleccionada"}.`);
  }

  async function onResolve(): Promise<void> {
    if (!draft) {
      announce("alert", "Crea primero el draft (paso 1).");
      return;
    }
    const res = await port.resolveDraft({
      draft,
      catalogRef: FIXED_CATALOG_REF,
    });
    if (!res.ok) {
      setResolveOut(`${res.error.code} ${res.error.messageKey}`);
      fail(res.error, "Resolver");
      return;
    }
    setResolveOut(
      `Resuelto: ${res.value.effectiveSelections.length} selecciones efectivas.`,
    );
    markStep(2, "✓");
    announce("status", "Draft resuelto.");
  }

  async function onValidate(): Promise<void> {
    if (!draft) {
      announce("alert", "Crea primero el draft (paso 1).");
      return;
    }
    const res = await port.validateDraft({
      draft,
      catalogRef: FIXED_CATALOG_REF,
    });
    if (!res.ok) {
      fail(res.error, "Validar");
      return;
    }
    setDiagnostics(res.value.diagnostics);
    markStep(3, res.value.blocking ? "⚠" : "✓");
    announce(
      "status",
      res.value.blocking
        ? "Validación con bloqueos."
        : "Validación sin bloqueos.",
    );
  }

  async function onManifest(): Promise<void> {
    if (!draft) {
      announce("alert", "Crea primero el draft (paso 1).");
      return;
    }
    const res = await port.buildManifest({
      draft,
      catalogRef: FIXED_CATALOG_REF,
      targetRef: exportTarget(exportTargetId),
    });
    if (!res.ok) {
      setManifestOut(`${res.error.code} ${res.error.messageKey}`);
      fail(res.error, "Manifest");
      return;
    }
    setManifest(res.value);
    setManifestOut(JSON.stringify(res.value, null, 2));
    markStep(4, "✓");
    announce("status", "Manifest construido.");
  }

  async function onExport(): Promise<void> {
    if (!draft) {
      announce("alert", "Crea primero el draft (paso 1).");
      return;
    }
    let man = manifest;
    if (!man || man.targetRef.id !== exportTargetId) {
      const built = await port.buildManifest({
        draft,
        catalogRef: FIXED_CATALOG_REF,
        targetRef: exportTarget(exportTargetId),
      });
      if (!built.ok) {
        setExportOut(`${built.error.code} ${built.error.messageKey}`);
        fail(built.error, "Exportar");
        return;
      }
      man = built.value;
      setManifest(man);
    }
    const res: Result<unknown> = await port.exportArtifact({
      manifest: man,
      targetRef: exportTarget(exportTargetId),
      destination: { token: "artifact.json" },
    });
    if (!res.ok) {
      setExportOut(`${res.error.code} ${res.error.messageKey}`);
      fail(res.error, "Exportar");
      return;
    }
    setExportOut("Artefacto exportado.");
    markStep(5, "✓");
    announce("status", "Artefacto exportado.");
  }

  return (
    <>
      <a className="skip-link" href="#main">
        Saltar al contenido
      </a>
      <header className="app-header">
        <h1>ArchMaker MVP-0</h1>
        <p className="app-sub">
          Walking skeleton: crear draft → seleccionar → resolver → validar →
          manifest → exportar
        </p>
        <div className="header-actions">
          <button
            id="theme-toggle"
            type="button"
            aria-pressed={dark}
            onClick={() => {
              const next = !dark;
              setDark(next);
              document.documentElement.classList.toggle("dark", next);
            }}
          >
            {dark ? "Tema claro" : "Tema oscuro"}
          </button>
        </div>
      </header>
      <div className="app-shell">
        <nav
          className="rail"
          aria-label="Pasos del configurador"
          id="step-rail"
        >
          <ol>
            {STEPS.map((label, i) => (
              <li key={label}>
                <a
                  href={`#step-${["create", "select", "resolve", "validate", "manifest", "export"][i]}`}
                  aria-current={i === currentStep ? "step" : undefined}
                >
                  {i + 1}. {label}
                </a>
                <span className="rail-badge">{railStatus[i] ?? ""}</span>
              </li>
            ))}
          </ol>
        </nav>
        <main id="main" className="workspace" tabIndex={-1}>
          <section aria-labelledby="s1-h" id="step-create">
            <h2 id="s1-h">1. Crear draft</h2>
            <div className="field-row">
              <label htmlFor="target-select">Target de exportación</label>
              <select
                id="target-select"
                value={exportTargetId}
                onChange={(e) =>
                  setExportTargetId(
                    e.target.value as (typeof EXPORT_TARGETS)[number],
                  )
                }
              >
                {EXPORT_TARGETS.map((t) => (
                  <option key={t} value={t}>
                    {t}
                  </option>
                ))}
              </select>
            </div>
            <button
              id="btn-create"
              type="button"
              onClick={() => void onCreate()}
            >
              Crear draft
            </button>
          </section>
          <section aria-labelledby="s2-h" id="step-select">
            <h2 id="s2-h">2. Seleccionar</h2>
            <div
              id="options"
              role="group"
              aria-label="Opciones del catálogo embebido"
            >
              {OPTIONS.map((o) => {
                const selected = selectedIds.has(o.id);
                return (
                  <button
                    key={o.id}
                    type="button"
                    className="option-card"
                    role="checkbox"
                    aria-checked={selected}
                    data-option-id={o.id}
                    onClick={() => onToggleOption(o)}
                  >
                    <span className="option-title">{o.title}</span>
                    <span className="option-state">
                      {selected ? "● Seleccionada" : "○ Sin seleccionar"}
                    </span>
                  </button>
                );
              })}
            </div>
          </section>
          <section aria-labelledby="s3-h" id="step-resolve">
            <h2 id="s3-h">3. Resolver</h2>
            <button
              id="btn-resolve"
              type="button"
              onClick={() => void onResolve()}
            >
              Resolver draft
            </button>
            <output id="resolve-out" aria-live="off">
              {resolveOut}
            </output>
          </section>
          <section aria-labelledby="s4-h" id="step-validate">
            <h2 id="s4-h">4. Validar</h2>
            <button
              id="btn-validate"
              type="button"
              onClick={() => void onValidate()}
            >
              Validar draft
            </button>
            <div id="validation-panel">
              <p role="status">
                {diagnostics.length === 0
                  ? "Sin incidencias."
                  : `${diagnostics.length} diagnósticos.`}
              </p>
              <ul>
                {diagnostics.map((d) => (
                  <li key={`${d.code}-${d.path}`}>
                    <a
                      href="#"
                      aria-describedby={d.messageKey}
                      onClick={(ev) => {
                        ev.preventDefault();
                        const target = document.querySelector(
                          `[data-option-id="${d.path}"]`,
                        ) as HTMLElement | null;
                        target?.focus();
                      }}
                    >
                      {d.code} · {d.path}
                    </a>
                  </li>
                ))}
              </ul>
            </div>
          </section>
          <section aria-labelledby="s5-h" id="step-manifest">
            <h2 id="s5-h">5. Manifest</h2>
            <button
              id="btn-manifest"
              type="button"
              onClick={() => void onManifest()}
            >
              Construir manifest
            </button>
            <pre
              id="manifest-out"
              tabIndex={0}
              aria-label="Resumen del manifest"
            >
              {manifestOut}
            </pre>
          </section>
          <section aria-labelledby="s6-h" id="step-export">
            <h2 id="s6-h">6. Exportar</h2>
            <button
              id="btn-export"
              type="button"
              onClick={() => void onExport()}
            >
              Exportar artefacto
            </button>
            <output id="export-out" aria-live="off">
              {exportOut}
            </output>
          </section>
        </main>
        <aside className="inspector" aria-label="Inspector" id="inspector">
          <h2>Inspector</h2>
          <p id="inspector-body">{inspector}</p>
          <button
            id="inspector-evidence"
            type="button"
            onClick={() => setDialogOpen(true)}
          >
            Ver evidencia
          </button>
          {dialogOpen ? (
            <dialog id="evidence-dialog" aria-labelledby="evidence-title" open>
              <h2 id="evidence-title">Evidencia</h2>
              <p id="evidence-body">{inspector}</p>
              <form
                method="dialog"
                onSubmit={(e) => {
                  e.preventDefault();
                  setDialogOpen(false);
                }}
              >
                <button type="submit">Cerrar</button>
              </form>
            </dialog>
          ) : null}
        </aside>
      </div>
      <div id="toast-region" aria-live="polite" role="status" />
      <div id="alert-region" aria-live="assertive" role="alert" />
    </>
  );
}
