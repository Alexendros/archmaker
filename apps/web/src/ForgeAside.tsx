import type { Diagnostic, ResolvedSelection } from "./coreport.js";
import type { FaceOption, FacePackage } from "./forge-face.js";
import {
  labelStorageMode,
  labelStorageUiState,
  type StorageIntent,
} from "./storage-intent.js";

type Props = {
  kernel?: FaceOption;
  bootTitle: string;
  filesystemTitle: string;
  env?: FaceOption;
  storage: StorageIntent;
  hasManifest: boolean;
  apps: FacePackage[];
  appCount: number;
  /** Suma de estimaciones de cara; no presentar como requisito total certificado. */
  totalSizeEstimate: number;
  /** Diagnósticos locales (kernel) antes de generar. */
  diagnostics: { title: string; description: string; kind: "info" | "warn" }[];
  /** Resultado efectivo del motor tras generate (H05). */
  effectiveSelections?: ResolvedSelection[];
  engineDiagnostics?: Diagnostic[];
  onPickEnv?: (optionId: string) => void;
};

export function ForgeAside({
  kernel,
  bootTitle,
  filesystemTitle,
  env,
  storage,
  hasManifest,
  apps,
  appCount,
  totalSizeEstimate,
  diagnostics,
  effectiveSelections,
  engineDiagnostics,
  onPickEnv,
}: Props) {
  const previewLabel = env?.title ?? "Sin entorno";
  const showManifest = hasManifest && Boolean(effectiveSelections);
  return (
    <aside className="forge-aside" aria-label="Inspector" id="inspector">
      <div className="forge-panel">
        <div className="forge-panel-head">
          <span>Vista previa</span>
          <span className="forge-chip">
            {kernel?.title ?? "—"} · {env?.protocol ?? "—"}
          </span>
        </div>
        <div
          className="forge-preview-screen"
          data-env={env?.htmlId ?? "none"}
          role="img"
          aria-label={`Esquema del entorno ${previewLabel}`}
        >
          <div className="forge-preview-bar">
            <span>{previewLabel}</span>
            <span>{env?.protocol ?? "—"}</span>
          </div>
          <div className="forge-preview-schematic" aria-hidden="true">
            <span className="forge-preview-dock" />
            <span className="forge-preview-pane" />
          </div>
        </div>
        <dl className="forge-summary">
          <div>
            <dt>Kernel</dt>
            <dd>{kernel?.title ?? "—"}</dd>
          </div>
          <div>
            <dt>Arranque</dt>
            <dd>{bootTitle}</dd>
          </div>
          <div>
            <dt>Sistema de archivos</dt>
            <dd>{filesystemTitle}</dd>
          </div>
          <div>
            <dt>Entorno</dt>
            <dd>{env?.title ?? "—"}</dd>
          </div>
          <div>
            <dt>Disco</dt>
            <dd>{labelStorageMode(storage.mode)}</dd>
          </div>
          <div>
            <dt>Estado del plan</dt>
            <dd data-storage-ui-state={storage.uiState}>
              {labelStorageUiState(storage.uiState)}
            </dd>
          </div>
        </dl>
      </div>

      <div className="forge-panel">
        <div className="forge-panel-head">
          <span>
            {showManifest ? "Manifiesto" : "Resumen de configuración"}
          </span>
          <span className="forge-chip">
            {showManifest
              ? `${effectiveSelections?.length ?? 0} efectivas`
              : `${appCount} apps solicitadas · ≈${totalSizeEstimate} MB (estimación)`}
          </span>
        </div>
        <div className="forge-manifest-body">
          {showManifest && engineDiagnostics && engineDiagnostics.length > 0
            ? engineDiagnostics.map((d) => (
                <div
                  key={`${d.code}-${d.path}`}
                  className={`forge-diag forge-diag-${d.severity ? "warn" : "info"}`}
                  role="status"
                >
                  <strong>
                    {d.code}
                    {d.blocking ? " (bloqueo)" : ""}
                  </strong>
                  <p>
                    {d.messageKey}
                    {d.path ? ` · ${d.path}` : ""}
                  </p>
                </div>
              ))
            : diagnostics.map((d) => (
                <div
                  key={d.title}
                  className={`forge-diag forge-diag-${d.kind}`}
                  role="status"
                >
                  <strong>{d.title}</strong>
                  <p>{d.description}</p>
                  {d.kind === "warn" && onPickEnv ? (
                    <button
                      type="button"
                      className="forge-btn-dark"
                      onClick={() => onPickEnv("archmaker.compositor.hyprland")}
                    >
                      Usar Hyprland
                    </button>
                  ) : null}
                </div>
              ))}
          {showManifest && effectiveSelections ? (
            <ul className="forge-app-list">
              {effectiveSelections.map((sel) => (
                <li key={`${sel.stepId}:${sel.optionId}`}>
                  <span>
                    {sel.optionId}
                    {sel.origin === "derived" ? " (derivada)" : ""}
                  </span>
                  <span className="forge-muted">{sel.stepId}</span>
                </li>
              ))}
            </ul>
          ) : (
            <ul className="forge-app-list">
              {apps.map((p) => (
                <li key={p.optionId}>
                  <span>{p.title}</span>
                  <span className="forge-muted">≈{p.size} MB (estimación)</span>
                </li>
              ))}
            </ul>
          )}
        </div>
      </div>
    </aside>
  );
}
