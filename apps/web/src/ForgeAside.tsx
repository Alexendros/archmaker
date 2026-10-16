import type { FaceOption, FacePackage } from "./forge-face.js";

type Props = {
  kernel?: FaceOption;
  bootTitle: string;
  filesystemTitle: string;
  env?: FaceOption;
  simpleMode: boolean;
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
  simpleMode,
  apps,
  appCount,
  totalSize,
  diagnostics,
  onPickEnv,
}: Props) {
  return (
    <aside className="forge-aside" aria-label="Inspector" id="inspector">
      <div className="forge-panel">
        <div className="forge-panel-head">
          <span>Vista previa</span>
          <span className="forge-chip">
            {kernel?.title ?? "—"} · {env?.protocol ?? "—"}
          </span>
        </div>
        <div className="forge-preview-screen" data-env={env?.htmlId ?? "none"}>
          <div className="forge-preview-bar">
            <span>Actividades</span>
            <span>{kernel?.title ?? "—"}</span>
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
            <dd>
              {env?.title ?? "—"} {env?.version ?? ""}
            </dd>
          </div>
          <div>
            <dt>Modo</dt>
            <dd>{simpleMode ? "Sencillo" : "Avanzado"}</dd>
          </div>
        </dl>
      </div>

      <div className="forge-panel">
        <div className="forge-panel-head">
          <span>Manifiesto</span>
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
