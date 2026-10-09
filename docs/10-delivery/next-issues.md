---
id: DOC-DEL-ISS-001
phase: planning
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# Próximos issues (orden por dependencia)

- Documento: DOC-DEL-ISS-001 · Estado: in-review · Propietario: Delivery · Fecha: 2026-10-06
- Formato: issues trazables en Markdown (decisión D7). Cada issue declara requisitos, riesgos, amenazas, validadores y pruebas afectadas.
- Estos issues son la descomposición ejecutable de la baseline MVP-0.1. El veredicto vigente es
  **GO MVP-0.1** (baseline firmada, artefactos verificables, gates del slice completos, P0/P1 cerrados o con
  plan de cierre fechado). Autoriza el vertical slice de `docs/10-delivery/walking-skeleton.md`, no el MVP
  completo ni el runner.

| #         | Issue                                                                                                                                                                                                                       | Depende de  | Gate      | Rol propietario                 | Criterio Done                                                                         |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | --------- | ------------------------------- | ------------------------------------------------------------------------------------- |
| ISSUE-001 | Resolver DEC-001..DEC-008 en `decision-register.md` (estado `accepted` o `deferred` motivado)                                                                                                                               | —           | G0–G10    | Producto/Arquitectura/Seguridad | Cada DEC con decisión, fecha y firmante; sin `decision-required` abiertos             |
| ISSUE-002 | Capturar fuentes primarias (SRC-002 ArchWiki, SRC-003 archinstall, SRC-004 detail, SRC-005..008) y registrar `claimId` con URL/versión/fecha                                                                                | —           | G0        | Investigación                   | Registro de afirmaciones con procedencia y caducidad                                  |
| ISSUE-003 | Aprobar `personas.md`, `objectives.md`, `journeys.md`, `use-cases.md` y `requirements.md` (FR/NFR con Given/When/Then)                                                                                                      | 001         | G1,G2     | Producto                        | Documentos `accepted`; métricas verificadas o marcadas no verificadas con plan        |
| ISSUE-004 | Aceptar ADR-0001/0002 y revisar/aceptar ADR-0003..0008                                                                                                                                                                      | 001         | G3        | Arquitectura                    | 8 ADR en `accepted`; ADR-0004 con transporte decidido (DEC-004)                       |
| ISSUE-005 | Cerrar modelo de dominio: identidad de IDs (DEC-002), SelectionValue, invariantes, `lifecycles.md`                                                                                                                          | 001         | G4        | Arquitectura de datos           | Modelo `accepted`; CON-001/003/004 resueltas; INV-* sin contradicción                 |
| ISSUE-006 | Crear JSON Schema Draft 2020-12 (`draft`, `catalog`, `rule`, `diagnostic`, `manifest`, `artifact`, `installation-plan`, `runner-envelope`) con `additionalProperties:false` y corpus válido/inválido + fixture de migración | 004,005     | G4,G5     | Datos/Interfaces                | Meta-validación y fixtures en verde en CI (`json-schema.yml`)                         |
| ISSUE-007 | Formalizar AST de operadores y reglas: `operator-spec.md` + `operator-corpus.md` + fichas `RULE-*` (fijar severidad AM-*)                                                                                                   | 005         | G7        | Validación                      | Operador no soportado → diagnóstico bloqueante probado; `count_gt` resuelto (CON-002) |
| ISSUE-008 | Validar contratos de interfaz: CorePort + `dto.md` + `errors-events.md` + `tauri-commands.md`/capabilities (8 comandos, denegación por defecto)                                                                             | 006         | G5,G8     | Interfaces/Seguridad            | DTO con paridad TS/Rust/WASM; cada comando con test negativo                          |
| ISSUE-009 | Validación de UX y accesibilidad sin rediseño: `interaction-matrix.md`, `component-contracts.md`, tokens `@layer` y corrección de deuda heredada                                                                            | 003         | G6        | UX/Diseño                       | Contraste/focus/reduced-motion verificados; deuda heredada documentada y trazada      |
| ISSUE-010 | Seguridad y release: cerrar `threat-model.md`/`privilege-model.md`, `test-matrix.md`, CI, SBOM/firmas/provenance y owners del backlog                                                                                       | 004,006,008 | G8,G9,G10 | Seguridad/Calidad/Delivery      | RSK-001/002/007/008 con tratamiento; CI en verde; SBOM y política de firma definidos  |

| MVP-0.1 Issues | Estado               | Dueño        | Gate | Criterio Done                                 | Evidencia                                        |
| -------------- | -------------------- | ------------ | ---- | --------------------------------------------- | ------------------------------------------------ |
| MVP-0.1-01     | closed               | Architecture | E2   | Dos ejecuciones limpias con digests idénticos | PR-02                                            |
| MVP-0.1-02     | closed               | Delivery     | E1   | Cero contradicciones P0/P1                    | PR-01; CON-013..022                              |
| MVP-0.1-03     | closed               | Data         | E3   | Cobertura completa + cero huérfanos           | PR-03                                            |
| MVP-0.1-04     | closed               | Architecture | E4   | Mismos bytes/digest/diagnósticos              | PR-04                                            |
| MVP-0.1-05     | closed               | Quality      | E4   | Flujo completo mismo Core, golden output      | PR-04                                            |
| MVP-0.1-06     | closed               | UX           | E5   | Auto + manual sin defectos críticos           | PR-05; `wcag_validate.py`                        |
| MVP-0.1-07     | closed-with-residual | Security     | E6   | Raíz materializada + artefactos verificables  | `_placeholder:false`; release en primer tag push |
| MVP-0.1-08     | closed               | Release      | E7   | Tag anotado + release verificada              | PR-07; tag `implementation-baseline-mvp0.1`      |

## Issues MVP-1 (planificados — sin implementación hasta MVP-1-01 `accepted`)

Fuente canónica de alcance: [`mvp1-scope.md`](mvp1-scope.md).

| #        | Issue                                   | Depende de | Gate     | Owner        | Criterio Done                     |
| -------- | --------------------------------------- | ---------- | -------- | ------------ | --------------------------------- |
| MVP-1-01 | Aprobar alcance DOC-DEL-MVP1-001        | —          | G2,G10   | Product      | Documento `accepted`              |
| MVP-1-02 | Cerrar CON-001..004 (dominio + schemas) | 01         | G4       | Data         | Corpus verde                      |
| MVP-1-03 | Formato ejes de versión (ADR-0006)      | 01         | G3,G4    | Architecture | ADR `accepted`                    |
| MVP-1-04 | Catálogo local versionado multi-entity  | 02,03      | G4,G5    | Data         | FR-CAT-001 ampliado               |
| MVP-1-05 | importDraft + migración MIG-5.1         | 02,04      | G4,G7,G9 | Data         | FR-IMPORT-001 / NFR-MIG-001 en CI |
| MVP-1-06 | Changeset + concurrencia                | 02         | G4,G5    | Interfaces   | Corpus changeset                  |
| MVP-1-07 | Trazabilidad ampliada MVP-1             | 04,05,06   | G2,G9    | Quality      | Sin huérfanos                     |
| MVP-1-08 | Tag `implementation-baseline-mvp1`      | 01–07      | G10      | Release      | Tag + CI verde                    |

## Trazabilidad de los issues

- Requisitos: ISSUE-003 (FR-_, NFR-_), ISSUE-006 (FR-DRAFT/EXPORT, NFR-MIG), ISSUE-007 (FR-VALIDATE/NFR-DET), ISSUE-008 (NFR-PORT/SEC/OBS).
- Riesgos: ISSUE-005 (RSK-004), ISSUE-006 (RSK-004), ISSUE-009 (RSK-009), ISSUE-010 (RSK-001/002/007/008).
- Amenazas: ISSUE-007 (THR-IMP-001), ISSUE-008 (THR-IPC-001/THR-FS-001), ISSUE-010 (THR-CAT-001/THR-SUP-001/THR-RUN-*).
- Validadores: ISSUE-007 (VAL-*), ISSUE-006 (VAL-MIG/VAL-DOC).
- Pruebas: ISSUE-006 (TST-MIG-001/TST-EXP-001), ISSUE-007 (TST-RES-001), ISSUE-008 (TST-DRAFT-001/RUN-001).

## Regla de cierre del plan

El veredicto de la revisión independiente R13 es **GO MVP-0.1**
(`docs/00-governance/final-review-planning-v1.1.md`), con condiciones fechadas `C1`–`C8` cerradas o con
plan de cierre fechado. La reevaluación del gate global y la autorización del MVP completo exigen G0–G10 en
`complete`, sin P0 abiertos y con las condiciones cerradas; solo entonces se actualiza
`docs/00-governance/go-no-go.md`.
