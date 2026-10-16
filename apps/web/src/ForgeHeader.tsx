type Props = {
  dark: boolean;
  simpleMode: boolean;
  onToggleDark: () => void;
  onSimpleMode: (simple: boolean) => void;
  onGlossary: () => void;
  onCommand: () => void;
  onGenerate: () => void;
};

export function ForgeHeader({
  dark,
  simpleMode,
  onToggleDark,
  onSimpleMode,
  onGlossary,
  onCommand,
  onGenerate,
}: Props) {
  return (
    <header className="forge-header" role="banner">
      <div className="forge-brand">
        <div className="forge-mark" aria-hidden="true">
          N
        </div>
        <div>
          <div className="forge-title">
            NEUBAT <span className="forge-title-muted">FORGE</span>{" "}
            <span className="forge-badge">v4.1</span>
          </div>
          <div className="forge-subtitle">
            Configurador local · sin instalación
          </div>
        </div>
      </div>
      <div className="forge-header-actions">
        <button type="button" className="forge-btn-ghost" onClick={onGlossary}>
          Glosario
        </button>
        <div
          className="forge-mode-toggle"
          role="group"
          aria-label="Modo de la cara"
        >
          <button
            type="button"
            className={simpleMode ? "forge-mode-active" : undefined}
            aria-pressed={simpleMode}
            onClick={() => onSimpleMode(true)}
          >
            Sencillo
          </button>
          <button
            type="button"
            className={!simpleMode ? "forge-mode-active" : undefined}
            aria-pressed={!simpleMode}
            onClick={() => onSimpleMode(false)}
          >
            Avanzado
          </button>
        </div>
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
        >
          ⌘K
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
