import { useCallback, useMemo, useRef, useState } from "react";
import { allPackages, type FacePackage } from "./forge-face.js";
import { useDialogFocusTrap } from "./use-dialog-focus.js";

type Props = {
  selectedIds: Set<string>;
  onToggle: (pkg: FacePackage) => void;
  onClose: () => void;
};

export function CommandPalette({ selectedIds, onToggle, onClose }: Props) {
  const [q, setQ] = useState("");
  const dialogRef = useRef<HTMLDivElement>(null);
  const stableClose = useCallback(() => onClose(), [onClose]);
  useDialogFocusTrap(dialogRef, stableClose);

  const items = useMemo(() => {
    // Entradas gap (incompletas) no se ofrecen como disponibles (H15).
    const all = allPackages().filter((p) => !p.gap);
    const needle = q.trim().toLowerCase();
    if (!needle) return all.slice(0, 8);
    return all.filter(
      (p) =>
        p.title.toLowerCase().includes(needle) ||
        p.description.toLowerCase().includes(needle),
    );
  }, [q]);

  return (
    <div className="forge-modal-backdrop forge-cmd-backdrop">
      <button
        type="button"
        className="forge-modal-scrim"
        aria-label="Cerrar búsqueda"
        tabIndex={-1}
        onClick={onClose}
      />
      <div
        ref={dialogRef}
        className="forge-cmd"
        role="dialog"
        aria-modal="true"
        aria-labelledby="cmd-title"
        tabIndex={-1}
      >
        <h2 id="cmd-title" className="visually-hidden">
          Buscar paquetes
        </h2>
        <input
          className="forge-cmd-input"
          aria-label="Buscar paquetes por nombre o descripción"
          placeholder="Busca: Firefox, Micro, Steam…"
          value={q}
          onChange={(e) => setQ(e.target.value)}
        />
        <ul className="forge-cmd-list">
          {items.map((p) => (
            <li key={p.optionId}>
              <button
                type="button"
                onClick={() => {
                  onToggle(p);
                  onClose();
                }}
              >
                <span>
                  {p.title}
                  {selectedIds.has(p.optionId) ? " · seleccionada" : ""}
                </span>
                <span className="forge-muted">≈{p.size} MB (estimación)</span>
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
