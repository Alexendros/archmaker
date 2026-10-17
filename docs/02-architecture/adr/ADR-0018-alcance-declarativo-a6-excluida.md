---
id: ADR-0018
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-FORGE-DECL-001
---

# ADR-0018: Alcance declarativo; A6 excluida

- Estado: proposed
- Fecha: 2026-10-17
- Propietario: Arquitectura

## Contexto

El veredicto MVP-5 excluye runner, root, discos ejecutables e ISO. Plan A añade detección y
planificación; la escritura en discos (A6) exige aprobación aparte.

## Decisión

- Este incremento autoriza **planificación declarativa** y, en fases siguientes, **sondas de
  solo lectura**.
- A6 (escritura en discos) **no** se implementa ni se aprueba aquí.
- `executionStatus` permanece `not-executed`.
- No se reescribe `mvp5-scope.md` como si la ejecución estuviera permitida; el alcance nuevo
  vive en `docs/10-delivery/forge-declarative-scope.md`.

## Consecuencias

- No hay botón Formatear con éxito fingido.
- HA26–HA28 y el helper privilegiado quedan para una iniciativa futura.

## Verificación

- Documentación + walking skeleton sin IO destructivo.
