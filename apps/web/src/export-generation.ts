/**
 * Pure helpers for the prepare/export generation guard (OP-3).
 * UI keeps refs; tests prove edit invalidates a prepared snapshot.
 */

export type GenerationGate = {
  generation: number;
  preparedGeneration: number | null;
};

export function bumpGeneration(gate: GenerationGate): GenerationGate {
  return {
    generation: gate.generation + 1,
    preparedGeneration: null,
  };
}

export function markPrepared(
  gate: GenerationGate,
  atGeneration: number,
): GenerationGate {
  return {
    generation: gate.generation,
    preparedGeneration: atGeneration,
  };
}

/** True when export may reuse the prepared manifest without regenerating. */
export function isPreparedCurrent(gate: GenerationGate): boolean {
  return (
    gate.preparedGeneration !== null &&
    gate.preparedGeneration === gate.generation
  );
}

/**
 * F7: doble clic / reentrada — Generar y Exportar ignoran si ya hay trabajo en curso.
 * La UI usa `busy`; este helper es la regla comprobable sin DOM.
 */
export function shouldIgnoreReentrantAction(busy: boolean): boolean {
  return busy;
}

/**
 * F7: tras editar B, un prepare/export de generación A queda obsoleto.
 * `kept` es false cuando la generación actual ya no coincide con la capturada.
 */
export function keepResultForGeneration(
  capturedGeneration: number,
  currentGeneration: number,
): boolean {
  return capturedGeneration === currentGeneration;
}
