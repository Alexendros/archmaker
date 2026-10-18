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
- SHA de trabajo (baseline residual): `dcb96b726efd2ec5e50a9732168c5e6e41898669` (`origin/main` al abrir la serie residual)
- SHA candidato de esta serie residual: `c0cbc25dd6e51d6320cac5953103bc353709e4ed` (rama `fix/forge-integrity-residual-tests`; fusionar #168/#169/#170)
- Dictamen: **NO-GO** de cierre completo. Subconjunto web/core de integridad de cara y contratos v2 corregido en PRs; desktop/offline **blocked**; revisión independiente **pending**. `technicalResult` V8: **no emitido**.

## Autoridad y bloqueos explícitos

- `forge-declarative-scope.md` sigue `draft` / `pending`; la nota «Entregado en main» de hardware no equivale a `accepted`.
- ADR-0011 y ADR-0013…0018 siguen `proposed`.
- Sin alcance `accepted` adicional de hardware, sondas, instalación, catálogo remoto ni Multilib.
- `executionStatus` permanece `not-executed`. MockCorePort no cierra gates de motor.

## H01–H17

| ID  | Prioridad | Estado     | Evidencia / PR                                                                                                   |
| --- | --------- | ---------- | ---------------------------------------------------------------------------------------------------------------- |
| H01 | P0        | fixed      | `storagePlan` en draft vía cara + Rust; cadena `withFaceV2Fields` → `draftForCore` en helpers; PR #163 + #170    |
| H02 | P0        | fixed      | Sin `uiState: validated` al generar manifiesto; PR #163                                                          |
| H03 | P0        | fixed      | Tipos v2 en `coreport.ts`; enum observación alineado al schema en #169                                           |
| H04 | P1        | superseded | Strip de intención/no verificadas documentado y visible; política de cara, no defecto silencioso; PR #163        |
| H05 | P1        | fixed      | Inspector Manifiesto ← `effectiveSelections`; PR #165                                                            |
| H06 | P1        | fixed      | Listas kernel/boot/fs desde `loadCatalog` (`mergeFaceOptionsFromCatalog`); PR #169                               |
| H07 | P1        | blocked    | Steam/multilib sin `ProviderChannel::Multilib` aprobado; nota en cara; sin ADR                                   |
| H08 | P1        | fixed      | Cursor unverified y fuera de preselección; PR #165                                                               |
| H09 | P1        | fixed      | MB como estimación de cara en ficha, inspector, filas y paleta; chips de versión editoriales; PR #165 + #169     |
| H10 | P1        | fixed      | Code - OSS en cara y fixture `local-catalog-mvp1.json`; PR #165 + #169                                           |
| H11 | P1        | fixed      | Deshacer vía `applyDraft`/`draftRef`; PR #164                                                                    |
| H12 | P1        | fixed      | Generación al elegir archivo; PR #164                                                                            |
| H13 | P1        | fixed      | `goToStep` no borra `invalid`; PR #164                                                                           |
| H14 | P1        | fixed      | `loadLastDraftLocal` discriminado; PR #164                                                                       |
| H15 | P1        | fixed      | Focus trap + inert (incl. importación, glosario, stepper); PR #166 + #168                                        |
| H16 | P1        | fixed      | T42 exige `AM-PROV-001` / `diagnostic.unverifiedProviderSelected`; buildManifest bloqueado; PR #167 + #170       |
| H17 | P0        | fixed      | CSP `style-src 'self'` bloqueaba CSS de Vite (`<style>`); `/forge-face.css` + gate `!ready`/`bootError`; PR #171 |

## Gates

| Gate | Estado                                                                                         |
| ---- | ---------------------------------------------------------------------------------------------- |
| V0   | parcial (inventario residual; SHA de trabajo registrado)                                       |
| V1   | parcial (fidelidad v2 + H01 en tests; corpus no reejecutado íntegro en este tip)               |
| V2   | parcial (plan declarativo; sin sonda hardware; sin validación runtime de plan)                 |
| V3   | parcial (H07 blocked; H06/H09/H10 cerrados en cara/fixture)                                    |
| V4   | parcial (inspector efectivo; residuales de procedencia remota fuera de alcance)                |
| V5   | parcial (H11–H14 cerrados; E2E App reentrada residual)                                         |
| V6   | parcial (H15 cerrado en PR #168; axe/manual residual)                                          |
| V7   | parcial (T42 endurecido; mutaciones no en CI)                                                  |
| V8   | **blocked** (sin binario candidato desktop; sin revisión independiente; sin `technicalResult`) |

## Residuales abiertos

1. H07 — ADR `ProviderChannel::Multilib` + migración de fixture Steam (blocked de autoridad).
2. Desktop/offline GO — binario del tip + 3× GUI + revisión independiente.
3. Aceptación de `forge-declarative-scope` / ADR-0013…0018 si se pretende certificar hardware declarativo.
4. Catálogo remoto / snapshot-diff (fuera del veredicto CONDITIONAL-CLEAR).

## Nota de mantenibilidad GUI (post-H17)

Higiene de tokens en `styles.css` (CSS muerto Sencillo/Avanzado, literales → tokens, capa `overrides` vacía; rail current documentado como herencia selected). Sin V8/GO. PR de higiene de diseño tras #171.

## PRs de la serie residual (2026-10-18)

| PR   | Tema                                  |
| ---- | ------------------------------------- |
| #168 | H15 foco residual                     |
| #169 | H06/H09/H10 catálogo y etiquetas      |
| #170 | H16 T42 + H01 cadena + este registro  |
| #171 | H17 CSP `/forge-face.css` + boot gate |

## Comandos locales de referencia (serie residual)

```bash
cargo test -p archmaker-core --test t_matrix --locked
cargo test -p archmaker-core --test f1_storage_fidelity --locked
pnpm --filter @archmaker/web check
pnpm --filter @archmaker/web exec playwright test forge-helpers.playwright.test.ts --grep "H01" --workers=1 --retries=0
```

`technicalResult` de cierre V8: no emitido. MockCorePort no cierra gates de motor.
Independent review: **pending**.
