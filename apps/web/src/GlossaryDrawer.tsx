import { useCallback, useRef } from "react";
import { GLOSSARY } from "./forge-face.js";
import { useDialogFocusTrap } from "./use-dialog-focus.js";

type Props = {
  onClose: () => void;
};

export function GlossaryDrawer({ onClose }: Props) {
  const drawerRef = useRef<HTMLElement>(null);
  const stableClose = useCallback(() => onClose(), [onClose]);
  useDialogFocusTrap(drawerRef, stableClose);

  return (
    <div className="forge-drawer-backdrop">
      <button
        type="button"
        className="forge-modal-scrim"
        aria-label="Cerrar glosario"
        tabIndex={-1}
        onClick={onClose}
      />
      <aside
        ref={drawerRef}
        className="forge-drawer"
        role="dialog"
        aria-modal="true"
        aria-labelledby="glossary-title"
        tabIndex={-1}
      >
        <div className="forge-modal-head">
          <h2 id="glossary-title">Glosario</h2>
          <button type="button" className="forge-btn-icon" onClick={onClose}>
            Cerrar
          </button>
        </div>
        <div className="forge-stack">
          {GLOSSARY.map((g) => (
            <div key={g.title} className="forge-info-card">
              <strong>{g.title}</strong>
              <p>{g.body}</p>
            </div>
          ))}
        </div>
      </aside>
    </div>
  );
}
