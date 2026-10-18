---
id: DOC-DEL-CLO-FORGE-INT-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Registro de hallazgos FORGE-INTEGRITY-2026-10-18

- Plan: FORGE-INTEGRITY-2026-10-18
- SHA auditado / inicial: `ff1c4fa24d8ae818fab3cc4f38c8f3cb8ab80e07`
- SHA candidato de esta serie (tip PR #167): `6e53807a15f414ee6f10f0e1a6d488d0a9bd7733`
- Dictamen: **NO-GO** de cierre completo. Subconjunto web/core corregido en PRs; desktop/offline **blocked**; revisión independiente **pending**.

## H01–H16

| ID  | Prioridad | Estado          | Evidencia / PR                                                       |
| --- | --------- | --------------- | -------------------------------------------------------------------- |
| H01 | P0        | fixed           | `storagePlan` en draft vía Rust + cara; PR #163                      |
| H02 | P0        | fixed           | Sin `uiState: validated` al generar manifiesto; PR #163              |
| H03 | P0        | fixed           | Tipos v2 en `coreport.ts`; PR #163                                   |
| H04 | P1        | fixed (parcial) | Exclusiones visibles en toast/revisión; strip documentado; PR #163   |
| H05 | P1        | fixed           | Inspector Manifiesto ← `effectiveSelections` + diagnósticos; PR #165 |
| H06 | P1        | fixed (parcial) | Títulos desde `loadCatalog`; cara editorial conservada; PR #165      |
| H07 | P1        | blocked         | Steam/multilib sin enum aprobado; nota en cara; sin ADR              |
| H08 | P1        | fixed           | Cursor unverified y fuera de preselección; PR #165                   |
| H09 | P1        | fixed           | MB como estimación de cara; PR #165                                  |
| H10 | P1        | fixed           | Etiqueta Code - OSS; PR #165                                         |
| H11 | P1        | fixed           | Deshacer vía `applyDraft`/`draftRef`; PR #164                        |
| H12 | P1        | fixed           | Generación al elegir archivo; PR #164                                |
| H13 | P1        | fixed           | `goToStep` no borra `invalid`; PR #164                               |
| H14 | P1        | fixed           | `loadLastDraftLocal` discriminado; PR #164                           |
| H15 | P1        | fixed           | Focus trap + inert; PR #166                                          |
| H16 | P1        | fixed (parcial) | T37/T40/T42 reforzados; E2E App reentrada residual                   |

## Gates

| Gate  | Estado                                                              |
| ----- | ------------------------------------------------------------------- |
| V0–V1 | parcial (core/web local verde en incrementos)                       |
| V2    | parcial (sin sonda hardware; plan declarativo)                      |
| V3    | parcial (H07 blocked)                                               |
| V4–V6 | parcial (implementado; axe/manual residual)                         |
| V7    | parcial (matriz reforzada; mutaciones no en CI)                     |
| V8    | blocked (sin binario candidato desktop; sin revisión independiente) |

## Residuales

1. H07 — ADR `ProviderChannel::Multilib` + migración de fixture Steam.
2. Conectar cara a `loadCatalog` para listas completas (no solo títulos).
3. E2E App real de reentrada/recuperación (más allá de helpers).
4. Desktop/offline GO — binario del tip + 3× GUI.
5. Revisión independiente obligatoria para cualquier GO.

## Comandos locales de referencia (serie)

```bash
cargo test -p archmaker-core --test f1_storage_fidelity --locked
cargo test -p archmaker-core --test t_matrix --locked
pnpm --filter @archmaker/web check
pnpm --filter @archmaker/web exec playwright test draft-persistence.playwright.test.ts forge-helpers.playwright.test.ts --workers=1 --retries=0
```

`technicalResult` de cierre V8: no emitido. MockCorePort no cierra gates de motor.
