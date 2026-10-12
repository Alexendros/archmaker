---
id: DOC-DEL-ISS-001
phase: planning
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# Próximos issues (orden por dependencia)

- Documento: DOC-DEL-ISS-001 · Estado: draft · Propietario: Delivery · Fecha: 2026-10-12
- Formato: issues trazables en Markdown (decisión D7). Cada issue declara requisitos, riesgos, amenazas, validadores y pruebas afectados.
- Estos 10 issues son la descomposición ejecutable de `docs/10-delivery/backlog.md`. El veredicto
  vigente es **CONDITIONAL-GO — walking skeleton MVP-0** (`docs/00-governance/final-review-planning-v1.1.md`);
  autoriza el vertical slice de `docs/10-delivery/walking-skeleton.md`, no el MVP completo ni el runner.
- Columna **Estado slice**: «evidencia de slice» = artefactos o gates `complete` del walking skeleton; «sigue abierto» = criterio humano o gate aún `in-progress` / consecuencia no aplicada. No implica aprobación humana adicional.

| #         | Issue                                                                                                                                                                                                                       | Depende de  | Gate      | Rol propietario                 | Criterio de Done                                                                      | Estado slice                                                                                                                                                 |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | --------- | ------------------------------- | ------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| ISSUE-001 | Resolver DEC-001..DEC-010 en `decision-register.md` (estado `accepted` o `deferred` motivado)                                                                                                                               | —           | G0–G10    | Producto/Arquitectura/Seguridad | Cada DEC con decisión, fecha y firmante; sin `decision-required` abiertos             | Evidencia de slice: DEC-001..DEC-010 en `docs/00-governance/decision-register.md` (DEC-006/007 con consecuencias parciales o no aplicadas)                   |
| ISSUE-002 | Capturar fuentes primarias (SRC-002 ArchWiki, SRC-003 archinstall, SRC-004 detail, SRC-005..008) y registrar `claimId` con URL/versión/fecha                                                                                | —           | G0        | Investigación                   | Registro de afirmaciones con procedencia y caducidad                                  | Evidencia de slice: G0 `complete`; `sha256sum --check --strict reference/v5.1/SHA256SUMS` en CI (`docs-validate.yml`)                                        |
| ISSUE-003 | Aprobar `personas.md`, `objectives.md`, `journeys.md`, `use-cases.md` y `requirements.md` (FR/NFR con Given/When/Then)                                                                                                      | 001         | G1,G2     | Producto                        | Documentos `accepted`; métricas verificadas o marcadas no verificadas con plan        | Sigue abierto: G1-C03 y G2-C04 pendientes (`docs/10-delivery/gates.md`)                                                                                      |
| ISSUE-004 | Aceptar ADR-0001/0002 y revisar/aceptar ADR-0003..0009                                                                                                                                                                      | 001         | G3        | Arquitectura                    | ADR en `accepted`; ADR-0004 con transporte decidido (DEC-004)                         | Sigue abierto: G3 `in-progress` (verificación pendiente); ADR-0001..0009 existen en `docs/02-architecture/adr/`                                              |
| ISSUE-005 | Cerrar modelo de dominio: identidad de IDs (DEC-002), SelectionValue, invariantes, `lifecycles.md`                                                                                                                          | 001         | G4        | Arquitectura de datos           | Modelo `accepted`; CON-001/003/004 resueltas; INV-* sin contradicción                 | Evidencia de slice: G4 `complete`; dominio en `crates/archmaker-domain/`                                                                                     |
| ISSUE-006 | Crear JSON Schema Draft 2020-12 (`draft`, `catalog`, `rule`, `diagnostic`, `manifest`, `artifact`, `installation-plan`, `runner-envelope`) con `additionalProperties:false` y corpus válido/inválido + fixture de migración | 004,005     | G4,G5     | Datos/Interfaces                | Meta-validación y fixtures en verde en CI (`json-schema.yml`)                         | Evidencia de slice: G4/G5 `complete`; schemas en `contracts/json-schema/`                                                                                    |
| ISSUE-007 | Formalizar AST de operadores y reglas: `operator-spec.md` + `operator-corpus.md` + fichas `RULE-*` (fijar severidad AM-*)                                                                                                   | 005         | G7        | Validación                      | Operador no soportado → diagnóstico bloqueante probado; `count_gt` resuelto (CON-002) | Evidencia de slice: G7 `complete`; reglas en `crates/archmaker-rules/` y `docs/07-validation/`                                                               |
| ISSUE-008 | Cerrar contratos de interfaz: CorePort + `dto.md` + `errors-events.md` + `tauri-commands.md`/capabilities (7 comandos CorePort v0, denegación por defecto)                                                                  | 006         | G5,G8     | Interfaces/Seguridad            | DTO con paridad TS/Rust/WASM; cada comando con test negativo                          | Evidencia de slice: G5/G8 `complete`; `src-tauri/capabilities/main.json`; paridad en `crates/archmaker-wasm/tests/parity.rs`                                 |
| ISSUE-009 | Validación de UX y accesibilidad sin rediseño: `interaction-matrix.md`, `component-contracts.md`, tokens `@layer` y corrección de deuda heredada                                                                            | 003         | G6        | UX/Diseño                       | Contraste/focus/reduced-motion verificados; deuda heredada documentada y trazada      | Sigue abierto: G6-C01 y G6-C03 pendientes (`docs/10-delivery/gates.md`)                                                                                      |
| ISSUE-010 | Seguridad y release: cerrar `threat-model.md`/`privilege-model.md`, `test-matrix.md`, CI, SBOM/firmas/provenance y owners del backlog                                                                                       | 004,006,008 | G8,G9,G10 | Seguridad/Calidad/Delivery      | RSK-001/002/007/008 con tratamiento; CI en verde; SBOM y política de firma definidos  | Parcial: G8/G9/G10 `complete` y CI verde; DEC-006 pipeline real de SBOM/firmas **no aplicada** (`decision-register.md`) — sigue abierto para release firmado |

## Trazabilidad de los issues

- Requisitos: ISSUE-003 (`FR-*`, `NFR-*`), ISSUE-006 (FR-DRAFT/EXPORT, NFR-MIG), ISSUE-007 (FR-VALIDATE/NFR-DET), ISSUE-008 (NFR-PORT/SEC/OBS).
- Riesgos: ISSUE-005 (RSK-004), ISSUE-006 (RSK-004), ISSUE-009 (RSK-009), ISSUE-010 (RSK-001/002/007/008).
- Amenazas: ISSUE-007 (THR-IMP-001), ISSUE-008 (THR-IPC-001/THR-FS-001), ISSUE-010 (THR-CAT-001/THR-SUP-001/THR-RUN-*).
- Validadores: ISSUE-007 (VAL-*), ISSUE-006 (VAL-MIG/VAL-DOC).
- Pruebas: ISSUE-006 (TST-MIG-001/TST-EXP-001), ISSUE-007 (TST-RES-001), ISSUE-008 (TST-DRAFT-001/RUN-001).

## Regla de cierre del plan

El veredicto de la revisión independiente R13 es **CONDITIONAL-GO — walking skeleton MVP-0**
(`docs/00-governance/final-review-planning-v1.1.md`), con condiciones fechadas `C1`–`C8`. La
reevaluación del gate global y la autorización del MVP completo exigen G0–G10 en `complete`, sin
P0 abiertos y con las condiciones cerradas; solo entonces se actualiza
`docs/00-governance/go-no-go.md`.
