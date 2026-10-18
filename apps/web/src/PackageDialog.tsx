import { useCallback, useRef } from "react";
import type { FacePackage } from "./forge-face.js";
import { useDialogFocusTrap } from "./use-dialog-focus.js";

type Props = {
  pkg: FacePackage;
  selected: boolean;
  unavailable?: boolean;
  onToggle: () => void;
  onClose: () => void;
};

export function PackageDialog({
  pkg,
  selected,
  unavailable = false,
  onToggle,
  onClose,
}: Props) {
  const dialogRef = useRef<HTMLDivElement>(null);
  const stableClose = useCallback(() => onClose(), [onClose]);
  useDialogFocusTrap(dialogRef, stableClose);

  return (
    <div className="forge-modal-backdrop">
      <button
        type="button"
        className="forge-modal-scrim"
        aria-label="Cerrar ficha"
        tabIndex={-1}
        onClick={onClose}
      />
      <div
        ref={dialogRef}
        className="forge-modal"
        role="dialog"
        aria-modal="true"
        aria-labelledby="package-title"
        tabIndex={-1}
      >
        <div className="forge-modal-head">
          <div>
            <h2 id="package-title">{pkg.title}</h2>
            <p className="forge-muted">
              ≈{pkg.size} MB (estimación de cara) · {pkg.tag}
            </p>
          </div>
          <button type="button" className="forge-btn-icon" onClick={onClose}>
            Cerrar
          </button>
        </div>
        <p>{pkg.description}</p>
        <div className="forge-why">
          <strong>Por qué:</strong> {pkg.why}
        </div>
        <div className="forge-modal-actions">
          <button
            type="button"
            className="forge-btn-primary"
            disabled={unavailable}
            onClick={onToggle}
          >
            {unavailable ? "No disponible" : selected ? "Quitar" : "Añadir"}
          </button>
          <button type="button" className="forge-btn-ghost" onClick={onClose}>
            Cerrar
          </button>
        </div>
      </div>
    </div>
  );
}
