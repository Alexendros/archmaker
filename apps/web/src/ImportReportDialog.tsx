import { useCallback, useRef } from "react";
import type { CoreError } from "./coreport.js";
import {
  summarizeReport,
  type ImportDraftPayload,
} from "./consented-import.js";
import { useDialogFocusTrap } from "./use-dialog-focus.js";

export type ImportReportPending =
  | {
      kind: "ready";
      payload: ImportDraftPayload;
      needsBlockingConfirm: boolean;
    }
  | {
      kind: "needs_blocking_confirm";
      error: CoreError;
    };

type Props = {
  pending: ImportReportPending;
  busy: boolean;
  onConfirm: (confirmBlockingLosses: boolean) => void;
  onClose: () => void;
};

export function ImportReportDialog({
  pending,
  busy,
  onConfirm,
  onClose,
}: Props) {
  const dialogRef = useRef<HTMLDivElement>(null);
  const stableClose = useCallback(() => onClose(), [onClose]);
  useDialogFocusTrap(dialogRef, stableClose);

  const confirmBlocking =
    pending.kind === "needs_blocking_confirm" ||
    (pending.kind === "ready" && pending.needsBlockingConfirm);

  return (
    <div className="forge-modal-backdrop">
      <button
        type="button"
        className="forge-modal-scrim"
        aria-label="Cancelar importación"
        tabIndex={-1}
        onClick={onClose}
      />
      <div
        ref={dialogRef}
        className="forge-modal"
        role="dialog"
        aria-modal="true"
        aria-labelledby="import-report-title"
        tabIndex={-1}
      >
        <div className="forge-modal-head">
          <div>
            <h2 id="import-report-title">Informe de importación</h2>
            <p className="forge-muted">
              El perfil actual no se sustituye hasta que confirmes.
            </p>
          </div>
          <button type="button" className="forge-btn-icon" onClick={onClose}>
            Cerrar
          </button>
        </div>
        {pending.kind === "needs_blocking_confirm" ? (
          <p role="alert">
            Hay pérdidas bloqueantes ({pending.error.code}). Confirma para
            aplicarlas o cancela para conservar el perfil actual.
          </p>
        ) : (
          <>
            <p role="status">{summarizeReport(pending.payload.report)}</p>
            {pending.payload.report.entries.length > 0 ? (
              <ul className="forge-stack">
                {pending.payload.report.entries.map((entry) => (
                  <li key={`${entry.path}-${entry.messageKey}`}>
                    <strong>{entry.path}</strong>: {entry.messageKey}
                    {entry.blocking ? " (bloqueante)" : ""}
                    {entry.detail ? ` — ${entry.detail}` : ""}
                  </li>
                ))}
              </ul>
            ) : (
              <p>Sin entradas de migración detalladas.</p>
            )}
          </>
        )}
        <div className="forge-modal-actions">
          <button
            type="button"
            className="forge-btn-primary"
            disabled={busy}
            onClick={() => onConfirm(confirmBlocking)}
          >
            {busy ? "Aplicando…" : "Confirmar e importar"}
          </button>
          <button
            type="button"
            className="forge-btn-ghost"
            disabled={busy}
            onClick={onClose}
          >
            Cancelar
          </button>
        </div>
      </div>
    </div>
  );
}
