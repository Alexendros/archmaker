import { useMemo, useState } from "react";
import { allPackages, type FacePackage } from "./neubat-face.js";

type Props = {
  selectedIds: Set<string>;
  onToggle: (pkg: FacePackage) => void;
  onClose: () => void;
};

export function CommandPalette({ selectedIds, onToggle, onClose }: Props) {
  const [q, setQ] = useState("");
  const items = useMemo(() => {
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
        onClick={onClose}
      />
      <div
        className="forge-cmd"
        role="dialog"
        aria-modal="true"
        aria-labelledby="cmd-title"
      >
        <h2 id="cmd-title" className="visually-hidden">
          Buscar paquetes
        </h2>
        <input
          autoFocus
          className="forge-cmd-input"
          placeholder="Busca: Firefox, Micro, Steam…"
          value={q}
          onChange={(e) => setQ(e.target.value)}
          onKeyDown={(e) => {
            if (e.key === "Escape") onClose();
          }}
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
                <span className="forge-muted">{p.size} MB</span>
              </button>
            </li>
          ))}
        </ul>
      </div>
    </div>
  );
}
