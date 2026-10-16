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
