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
  totalSize: number;
  diagnostics: { title: string; description: string; kind: "info" | "warn" }[];
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
  totalSize,
  diagnostics,
  onPickEnv,
}: Props) {
  const previewLabel = env?.title ?? "Sin entorno";
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
            {hasManifest ? "Manifiesto" : "Resumen de configuración"}
          </span>
          <span className="forge-chip">
            {appCount} apps · {totalSize} MB
          </span>
        </div>
        <div className="forge-manifest-body">
          {diagnostics.map((d) => (
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
          <ul className="forge-app-list">
            {apps.map((p) => (
              <li key={p.optionId}>
                <span>{p.title}</span>
                <span className="forge-muted">{p.size} MB</span>
              </li>
            ))}
          </ul>
        </div>
      </div>
    </aside>
  );
}
