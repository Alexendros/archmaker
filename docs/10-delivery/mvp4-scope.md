---
id: DOC-DEL-MVP4-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - ux
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-MVP3-001
    expectedDocumentStatus: accepted
  - id: DOC-GOV-GNG-001
  - id: ADR-0001
    expectedDocumentStatus: accepted
---

# Alcance MVP-4 — React + MockCorePort + design system

- Documento: DOC-DEL-MVP4-001 · Estado: accepted · Propietario: UX/Arquitectura · Fecha: 2026-10-12
- Entrada: baseline `implementation-baseline-mvp3` + gates G0–G10 `complete` (slice).
- Alcance **accepted** (MVP-4-01). UI React contra mock determinista autorizada bajo las
  exclusiones de este documento; runner y privilegios siguen fuera.

## Objetivo

Recorrer en React, contra un `MockCorePort` determinista, el flujo crear draft → seleccionar →
resolver → validar → manifest → exportar. La semántica permanece en Rust; TypeScript solo declara
frontera y presenta DTO y errores tipados. Se reutilizan tokens y contratos de componentes sin
rediseño.

## Incluido

| Área          | Contenido                                                            | Anclas                                    |
| ------------- | -------------------------------------------------------------------- | ----------------------------------------- |
| CorePort TS   | Declaración alineada con DTO camelCase (7 ops v0 + `importDraft`)    | DOC-IF-CORE-001, DOC-IF-DTO-001, ADR-0001 |
| MockCorePort  | Fixtures deterministas; sin reglas, sin digest de dominio, sin disco | FR-DRAFT-001, FR-EXPORT-001, NFR-OFF-001  |
| Flujo React   | StepRail, OptionCard, ValidationPanel, inspector; mismos tokens CSS  | DOC-UX-COMP-001, DOC-DS-001               |
| Accesibilidad | WCAG 2.2 AA auto + baseline visual sin regresión no aprobada         | NFR-ACC-001, visual-baseline              |

## Excluido (explícito)

- Runner v1, root, discos, shell, sidecars, catálogos remotos, firma de catálogo.
- Paridad Tauri/WASM de la UI (MVP-5).
- Presets (`FR-PRESET-001`), `listExportTargets`, rediseño visual, Enterprise.

## Issues trazables

| #        | Issue                                             | Depende de | Gate      | Owner        | Criterio Done                       |
| -------- | ------------------------------------------------- | ---------- | --------- | ------------ | ----------------------------------- |
| MVP-4-01 | Aprobar alcance DOC-DEL-MVP4-001                  | —          | G2,G10    | Product      | Documento `accepted`                |
| MVP-4-02 | Declaración CorePort TS alineada con DTO Rust     | 01         | G5        | Interfaces   | Tipos camelCase; sin reglas         |
| MVP-4-03 | MockCorePort determinista (7 ops + importDraft)   | 02         | G5,G9     | Architecture | Flujo offline sin Tauri             |
| MVP-4-04 | Flujo React con tokens / contratos existentes     | 02,03      | G6        | UX           | Crear→exportar en React             |
| MVP-4-05 | WCAG + baseline visual sin regresión              | 04         | G6,G9     | UX           | `pnpm test-a11y:all` verde          |
| MVP-4-06 | Trazabilidad + tag `implementation-baseline-mvp4` | 01–05      | G2,G9,G10 | Quality      | Matriz + tag; tags previos intactos |

## Trazabilidad

- Requisitos: FR-DRAFT-001, FR-CAT-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001,
  FR-EXPORT-001, NFR-ACC-001, NFR-OFF-001, NFR-PORT-001.
- ADR: ADR-0001.
- Pruebas: flujo mock + a11y/visual existentes.

## Criterio de salida

- Issues MVP-4-01..06 cerrados o con residual fechado.
- Tag `implementation-baseline-mvp4` emitido; tags MVP-0.1..MVP-3 intactos.
- TypeScript no contiene operadores ni resolución de reglas.
