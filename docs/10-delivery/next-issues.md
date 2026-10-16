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

- Documento: DOC-DEL-ISS-001 · Estado: in-review · Propietario: Delivery · Fecha: 2026-10-12
- Formato: issues trazables en Markdown (decisión D7). Cada issue declara requisitos, riesgos, amenazas, validadores y pruebas afectados.
- Estos issues son la descomposición ejecutable de la baseline MVP-0.1. El veredicto vigente es
  **GO MVP-0.1** / **CONDITIONAL-CLEAR** del walking skeleton (`docs/00-governance/go-no-go.md`): gates G0–G10
  del slice `complete`. Autoriza el vertical slice de `docs/10-delivery/walking-skeleton.md`, no el MVP
  completo ni el runner.
- Columna **Estado slice**: «evidencia de slice» = gate `complete` o artefacto del walking skeleton; «sigue
  abierto» / «parcial» = residual fuera del Clear del slice (p. ej. DEC-006 pipeline de release). No implica
  aprobación humana adicional.

| #         | Issue                                                                                                                                                                                                                       | Depende de  | Gate      | Rol propietario                 | Criterio Done                                                                         | Estado slice                                                                                                                 |
| --------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ----------- | --------- | ------------------------------- | ------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------- |
| ISSUE-001 | Resolver DEC-001..DEC-010 en `decision-register.md` (estado `accepted` o `deferred` motivado)                                                                                                                               | —           | G0–G10    | Producto/Arquitectura/Seguridad | Cada DEC con decisión, fecha y firmante; sin `decision-required` abiertos             | Evidencia de slice: DEC-001..DEC-010 en `docs/00-governance/decision-register.md` (DEC-006/007 con residuales)               |
| ISSUE-002 | Capturar fuentes primarias (SRC-002 ArchWiki, SRC-003 archinstall, SRC-004 detail, SRC-005..008) y registrar `claimId` con URL/versión/fecha                                                                                | —           | G0        | Investigación                   | Registro de afirmaciones con procedencia y caducidad                                  | Evidencia de slice: G0 `complete`; `sha256sum --check --strict reference/v5.1/SHA256SUMS` en CI                              |
| ISSUE-003 | Aprobar `personas.md`, `objectives.md`, `journeys.md`, `use-cases.md` y `requirements.md` (FR/NFR con Given/When/Then)                                                                                                      | 001         | G1,G2     | Producto                        | Documentos `accepted`; métricas verificadas o marcadas no verificadas con plan        | Evidencia de slice: G1 y G2 `complete` (`docs/10-delivery/gates.md`)                                                         |
| ISSUE-004 | Aceptar ADR-0001/0002 y revisar/aceptar ADR-0003..0009                                                                                                                                                                      | 001         | G3        | Arquitectura                    | ADR en `accepted`; ADR-0004 con transporte decidido (DEC-004)                         | Evidencia de slice: G3 `complete`; ADR en `docs/02-architecture/adr/`                                                        |
| ISSUE-005 | Cerrar modelo de dominio: identidad de IDs (DEC-002), SelectionValue, invariantes, `lifecycles.md`                                                                                                                          | 001         | G4        | Arquitectura de datos           | Modelo `accepted`; CON-001/003/004 resueltas; INV-* sin contradicción                 | Evidencia de slice: G4 `complete`; dominio en `crates/archmaker-domain/`                                                     |
| ISSUE-006 | Crear JSON Schema Draft 2020-12 (`draft`, `catalog`, `rule`, `diagnostic`, `manifest`, `artifact`, `installation-plan`, `runner-envelope`) con `additionalProperties:false` y corpus válido/inválido + fixture de migración | 004,005     | G4,G5     | Datos/Interfaces                | Meta-validación y fixtures en verde en CI (`json-schema.yml`)                         | Evidencia de slice: G4/G5 `complete`; schemas en `contracts/json-schema/`                                                    |
| ISSUE-007 | Formalizar AST de operadores y reglas: `operator-spec.md` + `operator-corpus.md` + fichas `RULE-*` (fijar severidad AM-*)                                                                                                   | 005         | G7        | Validación                      | Operador no soportado → diagnóstico bloqueante probado; `count_gt` resuelto (CON-002) | Evidencia de slice: G7 `complete`; reglas en `crates/archmaker-rules/` y `docs/07-validation/`                               |
| ISSUE-008 | Validar contratos de interfaz: CorePort + `dto.md` + `errors-events.md` + `tauri-commands.md`/capabilities (lista permissions vacía; 8 identificadores `core:*` en TOML, denegación por defecto)                            | 006         | G5,G8     | Interfaces/Seguridad            | DTO con paridad TS/Rust/WASM; cada comando con test negativo                          | Evidencia de slice: G5/G8 `complete`; `src-tauri/capabilities/main.json`; paridad en `crates/archmaker-wasm/tests/parity.rs` |
| ISSUE-009 | Validación de UX y accesibilidad sin rediseño: `interaction-matrix.md`, `component-contracts.md`, tokens `@layer` y corrección de deuda heredada                                                                            | 003         | G6        | UX/Diseño                       | Contraste/focus/reduced-motion verificados; deuda heredada documentada y trazada      | Evidencia de slice: G6 `complete`; `tools/wcag_validate.py` y jobs a11y en CI                                                |
| ISSUE-010 | Seguridad y release: cerrar `threat-model.md`/`privilege-model.md`, `test-matrix.md`, CI, SBOM/firmas/provenance y owners del backlog                                                                                       | 004,006,008 | G8,G9,G10 | Seguridad/Calidad/Delivery      | RSK-001/002/007/008 con tratamiento; CI en verde; SBOM y política de firma definidos  | Parcial: G8/G9/G10 `complete` y CI verde; DEC-006 pipeline real de SBOM/firmas **no aplicada** (`decision-register.md`)      |

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

## Issues MVP-1

Fuente canónica de alcance: [`mvp1-scope.md`](mvp1-scope.md).

| #        | Issue                                   | Depende de | Gate     | Owner        | Criterio Done                     | Estado |
| -------- | --------------------------------------- | ---------- | -------- | ------------ | --------------------------------- | ------ |
| MVP-1-01 | Aprobar alcance DOC-DEL-MVP1-001        | —          | G2,G10   | Product      | Documento `accepted`              | closed |
| MVP-1-02 | Cerrar CON-001..004 (dominio + schemas) | 01         | G4       | Data         | Corpus verde                      | closed |
| MVP-1-03 | Formato ejes de versión (ADR-0006)      | 01         | G3,G4    | Architecture | ADR `accepted`                    | closed |
| MVP-1-04 | Catálogo local versionado multi-entity  | 02,03      | G4,G5    | Data         | FR-CAT-001 ampliado               | closed |
| MVP-1-05 | importDraft + migración MIG-5.1         | 02,04      | G4,G7,G9 | Data         | FR-IMPORT-001 / NFR-MIG-001 en CI | closed |
| MVP-1-06 | Changeset + concurrencia                | 02         | G4,G5    | Interfaces   | Corpus changeset                  | closed |
| MVP-1-07 | Trazabilidad ampliada MVP-1             | 04,05,06   | G2,G9    | Quality      | Sin huérfanos                     | closed |
| MVP-1-08 | Tag `implementation-baseline-mvp1`      | 01–07      | G10      | Release      | Tag + CI verde                    | closed |

## Issues MVP-2

Fuente canónica de alcance: [`mvp2-scope.md`](mvp2-scope.md).

| #        | Issue                                   | Depende de | Gate     | Owner        | Criterio Done                | Estado |
| -------- | --------------------------------------- | ---------- | -------- | ------------ | ---------------------------- | ------ |
| MVP-2-01 | Aprobar alcance DOC-DEL-MVP2-001        | —          | G2,G10   | Product      | Documento `accepted`         | closed |
| MVP-2-02 | AST operadores canónicos + AM-RULE-009  | 01         | G7       | Validation   | Corpus + tests Rust          | closed |
| MVP-2-03 | Cargar reglas desde fichas del catálogo | 02         | G4,G7    | Data         | Pack + `catalog.rules`       | closed |
| MVP-2-04 | Resolver dominio + digest SHA-256       | 02,03      | G4,G5,G7 | Architecture | FR-RESOLVE-001; FNV retirado | closed |
| MVP-2-05 | validateDraft ≡ buildManifest (bloqueo) | 03,04      | G5,G7,G9 | Quality      | FR-VALIDATE-001              | closed |
| MVP-2-06 | Trazabilidad MVP-2 sin huérfanos        | 04,05      | G2,G9    | Quality      | Matriz verde                 | closed |
| MVP-2-07 | Tag `implementation-baseline-mvp2`      | 01–06      | G10      | Release      | Tag + CI verde               | closed |

## Issues MVP-3

Fuente canónica de alcance: [`mvp3-scope.md`](mvp3-scope.md).

| #        | Issue                                            | Depende de | Gate   | Owner        | Criterio Done                      | Estado |
| -------- | ------------------------------------------------ | ---------- | ------ | ------------ | ---------------------------------- | ------ |
| MVP-3-01 | Aprobar alcance DOC-DEL-MVP3-001                 | —          | G2,G10 | Product      | Documento `accepted`               | closed |
| MVP-3-02 | Adapter `archmaker-profile` + `report` + content | 01         | G4,G5  | Architecture | Content relleno; bytes canónicos   | closed |
| MVP-3-03 | Sobre experimental `archinstall-profile`         | 01,02      | G3,G5  | Architecture | Bytes ≠ ArchMaker; sin ejecutables | closed |
| MVP-3-04 | Tests divergencia / idempotencia / rechazo       | 02,03      | G5,G9  | Quality      | TST-EXP-001 / VAL-TARGET           | closed |
| MVP-3-05 | Trazabilidad MVP-3 sin huérfanos                 | 04         | G2,G9  | Quality      | Matriz verde                       | closed |
| MVP-3-06 | Tag `implementation-baseline-mvp3`               | 01–05      | G10    | Release      | Tag + CI verde                     | closed |

## Issues MVP-4

Fuente canónica de alcance: [`mvp4-scope.md`](mvp4-scope.md).

| #        | Issue                                           | Depende de | Gate   | Owner        | Criterio Done               | Estado |
| -------- | ----------------------------------------------- | ---------- | ------ | ------------ | --------------------------- | ------ |
| MVP-4-01 | Aprobar alcance DOC-DEL-MVP4-001                | —          | G2,G10 | Product      | Documento `accepted`        | closed |
| MVP-4-02 | Declaración CorePort TS alineada con DTO Rust   | 01         | G5     | Interfaces   | Tipos camelCase; sin reglas | closed |
| MVP-4-03 | MockCorePort determinista (7 ops + importDraft) | 02         | G5,G9  | Architecture | Flujo offline sin Tauri     | closed |
| MVP-4-04 | Flujo React con tokens / contratos existentes   | 02,03      | G6     | UX           | Crear→exportar en React     | closed |
| MVP-4-05 | WCAG + baseline visual sin regresión            | 04         | G6,G9  | UX           | `pnpm test-a11y:all` verde  | closed |
| MVP-4-06 | Tag `implementation-baseline-mvp4`              | 01–05      | G10    | Release      | Tag + CI verde              | closed |

## Issues MVP-5

Fuente canónica de alcance: [`mvp5-scope.md`](mvp5-scope.md).

| #        | Issue                                          | Depende de | Gate    | Owner        | Criterio Done                                              | Estado |
| -------- | ---------------------------------------------- | ---------- | ------- | ------------ | ---------------------------------------------------------- | ------ |
| MVP-5-01 | Aceptar alcance DOC-DEL-MVP5-001               | —          | G2, G10 | Product      | Documento `accepted`                                       | closed |
| MVP-5-02 | Ampliar el catálogo local con la matriz        | 01         | G4      | Data         | Cada id HTML está en catálogo o hueco explícito            | closed |
| MVP-5-03 | UI de cinco pasos con los contratos existentes | 02         | G6      | UX           | Seleccionar, resolver, validar y exportar sin escribir ISO | closed |
| MVP-5-04 | Paridad Tauri/WASM del mismo draft             | 02         | G5, G9  | Architecture | Mismos bytes de manifiesto                                 | closed |
| MVP-5-05 | Tag `implementation-baseline-mvp5`             | 04         | G10     | Release      | Tag nuevo; los anteriores no se mueven                     | closed |

## Trazabilidad de los issues

- Requisitos: ISSUE-003 (`FR-*`, `NFR-*`), ISSUE-006 (FR-DRAFT/EXPORT, NFR-MIG), ISSUE-007 (FR-VALIDATE/NFR-DET), ISSUE-008 (NFR-PORT/SEC/OBS).
- Riesgos: ISSUE-005 (RSK-004), ISSUE-006 (RSK-004), ISSUE-009 (RSK-009), ISSUE-010 (RSK-001/002/007/008).
- Amenazas: ISSUE-007 (THR-IMP-001), ISSUE-008 (THR-IPC-001/THR-FS-001), ISSUE-010 (THR-CAT-001/THR-SUP-001/THR-RUN-*).
- Validadores: ISSUE-007 (VAL-*), ISSUE-006 (VAL-MIG/VAL-DOC).
- Pruebas: ISSUE-006 (TST-MIG-001/TST-EXP-001), ISSUE-007 (TST-RES-001), ISSUE-008 (TST-DRAFT-001/RUN-001).

## Regla de cierre del plan

El veredicto vigente es **CONDITIONAL-CLEAR — walking skeleton MVP-0 + slices MVP-1..MVP-5**
(`docs/00-governance/go-no-go.md`), con gates G0–G10 `complete`. MVP-1-01..08 están `closed`;
MVP-2-01..07 están `closed`; MVP-3-01..06 están `closed`; MVP-4-01..06 están `closed`;
MVP-5-01..05 están `closed`; baseline `implementation-baseline-mvp5` emitida (tags previos
intactos). Cierre F0–F9: gobierno F8 en `closure/f8-governance.md`; release binaria
**published** (Neubat AppImage `v0.1.0`, verificación parcial); aprobación funcional F7
**pending** (sin `RC_SHA_FUNCIONAL`). La autorización del MVP completo, del runner y de
familias de paquete distintas de AppImage exige su propio gate y `accepted` explícito.
