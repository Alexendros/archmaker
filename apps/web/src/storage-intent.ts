/**
 * Intención de disco declarativa en la cara (schema v2 pending en core).
 * No finge detección ni escritura. Estados ortogonales: no hay «ejecutado».
 */

export type StorageMode = "new-disk" | "replace-disk";

/** Procedencia / ciclo visibles en UI (ortogonales; A6 fuera). */
export type StorageUiState =
  | "declared"
  | "proposed"
  | "validated"
  | "unspecified";

export type StorageIntent = {
  mode: StorageMode | null;
  intentAcknowledged: boolean;
  lossRiskAcknowledged: boolean;
  /** Solo declaración manual en esta entrega. */
  uiState: StorageUiState;
};

export const INITIAL_STORAGE_INTENT: StorageIntent = {
  mode: null,
  intentAcknowledged: false,
  lossRiskAcknowledged: false,
  uiState: "unspecified",
};

export function storageStepComplete(intent: StorageIntent): boolean {
  if (intent.mode === null || !intent.intentAcknowledged) return false;
  if (intent.mode === "replace-disk" && !intent.lossRiskAcknowledged) {
    return false;
  }
  return true;
}

export function labelStorageMode(mode: StorageMode | null): string {
  switch (mode) {
    case "new-disk":
      return "Disco nuevo o sin datos a conservar";
    case "replace-disk":
      return "Reemplazo completo";
    default:
      return "Sin modo";
  }
}

export function labelStorageUiState(state: StorageUiState): string {
  switch (state) {
    case "declared":
      return "Declarado";
    case "proposed":
      return "Propuesto";
    case "validated":
      return "Validado";
    default:
      return "Sin declarar";
  }
}
