import { APP_VERSION, PRODUCT_WINDOW_TITLE } from "./build-meta.js";

type Props = {
  dark: boolean;
  onToggleDark: () => void;
  onGlossary: () => void;
  onCommand: () => void;
  onGenerate: () => void;
};

export function ForgeHeader({
  dark,
  onToggleDark,
  onGlossary,
  onCommand,
  onGenerate,
}: Props) {
  return (
    <header className="forge-header" role="banner">
      <div className="forge-brand">
        <div className="forge-mark" aria-hidden="true">
          F
        </div>
        <div>
          <div className="forge-title">
            {PRODUCT_WINDOW_TITLE}{" "}
            <span className="forge-badge" data-app-version={APP_VERSION}>
              {APP_VERSION}
            </span>
          </div>
          <div className="forge-subtitle">
            Configura la build. La imagen se pide en Revisión.
          </div>
        </div>
      </div>
      <div className="forge-header-actions">
        <button type="button" className="forge-btn-ghost" onClick={onGlossary}>
          Glosario
        </button>
        <button
          type="button"
          className="forge-btn-icon"
          aria-pressed={dark}
          onClick={onToggleDark}
        >
          {dark ? "Claro" : "Oscuro"}
        </button>
        <button
          type="button"
          className="forge-btn-ghost forge-cmd-btn"
          onClick={onCommand}
          aria-keyshortcuts="Control+K"
          title="Abrir paleta de comandos (Ctrl+K)"
        >
          Ctrl+K
        </button>
        <button
          type="button"
          className="forge-btn-primary"
          id="btn-generate"
          onClick={onGenerate}
        >
          Generar
        </button>
      </div>
    </header>
  );
}
