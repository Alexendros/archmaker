---
id: DOC-DEL-MVP1-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - product
  - data
reviewers:
  - architecture
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-WSK-001
    expectedDocumentStatus: in-review
  - id: DOC-GOV-GNG-001
  - id: DOC-CON-JSONSCHEMA-001
---

# Alcance MVP-1 — domain / schema / catalog / migrations

- Documento: DOC-DEL-MVP1-001 · Estado: draft · Propietario: Producto/Datos · Fecha: 2026-10-09
- Entrada: baseline `implementation-baseline-mvp0.1` + gates G0–G10 `complete` (slice).
- Veredicto vigente: **CONDITIONAL-CLEAR — walking skeleton únicamente**
  (`docs/00-governance/go-no-go.md`). Este documento **planifica** MVP-1; no autoriza
  implementación de producto hasta aprobación explícita del alcance.

## Objetivo

Ampliar el dominio de datos y los contratos v0 del walking skeleton hacia un modelo de
catálogo, identidad y migración v5.1→vNext verificable en CI, sin introducir runner,
privilegios, discos, shell ni catálogos remotos.

## Incluido

| Área       | Contenido                                                                               | Anclas                                           |
| ---------- | --------------------------------------------------------------------------------------- | ------------------------------------------------ |
| Domain     | Completar tipos de dominio (SelectionValue discriminado, invariantes INV-*, lifecycles) | CON-001..004, DEC-002, ADR-0005/0006             |
| Schema     | Congelar ejes de versión pendientes; corpus adicional válido/inválido; cambiosets       | `contracts/json-schema/*`, ADR-0006              |
| Catalog    | Catálogo local versionado más allá del fixture embebido mínimo; digest + procedencia    | FR-CAT-001, DM-CATALOG                           |
| Migrations | Importación no destructiva v5.1→draft vNext + informe tipado + corpus                   | FR-IMPORT-001, NFR-MIG-001, VAL-MIG, THR-IMP-001 |

## Excluido (explícito)

- Runner v1, root, discos reales, shell arbitraria, sidecars.
- Catálogos remotos / actualización de catálogo por red.
- `installation-plan` / `runner-envelope` como superficie de ejecución (permanecen contratos
  documentales v0; no se implementan operaciones privilegiadas).
- Enterprise (ENT-*), telemetría, auto-updater activado por defecto.
- Rediseño visual no trazable a a11y/consistencia.

## Inventario de gaps (vs MVP-0.1)

| Gap                            | Estado en MVP-0.1                                                       | Trabajo MVP-1                                |
| ------------------------------ | ----------------------------------------------------------------------- | -------------------------------------------- |
| Catálogo embebido mínimo       | Fixture `embedded-catalog.json`                                         | Catálogo local multi-paquete con procedencia |
| Migración v5.1                 | Fixtures `examples/migration/*` + schema; sin `importDraft` de producto | Operación tipada + informe + corpus CI       |
| SelectionValue / arrays        | Modelo parcial; CON-003 abierta                                         | Tipo discriminado + tests                    |
| Identidad IDs (DEC-002)        | Parcial en `common.schema.json`                                         | Detector duplicados (CON-004) + invariantes  |
| `count_gt` / operadores legacy | Normalizado en reglas (CON-002 parcial)                                 | Cierre formal en ficha RULE + corpus         |
| Formato exacto de versiones    | ADR-0006 pendiente de formato                                           | ADR o enmienda + validadores                 |
| Changeset / concurrencia       | Schema `changeset` v0                                                   | Uso en save/import con corpus                |

## Issues trazables (orden por dependencia)

| #        | Issue                                                         | Depende de | Gate     | Owner           | Criterio Done                                             |
| -------- | ------------------------------------------------------------- | ---------- | -------- | --------------- | --------------------------------------------------------- |
| MVP-1-01 | Aprobar alcance DOC-DEL-MVP1-001 + actualizar roadmap/backlog | —          | G2,G10   | Product         | Documento `accepted`; exclusiones firmadas                |
| MVP-1-02 | Cerrar CON-001..004 en modelo de dominio + schemas            | 01         | G4       | Data            | SelectionValue, cardinalidad, duplicados; corpus verde    |
| MVP-1-03 | ADR/enmienda de formato de ejes de versión (ADR-0006)         | 01         | G3,G4    | Architecture    | ADR `accepted`; validadores actualizados                  |
| MVP-1-04 | Catálogo local versionado (multi-entity) + digest/procedencia | 02,03      | G4,G5    | Data            | FR-CAT-001 evidenciado más allá del fixture mínimo        |
| MVP-1-05 | `importDraft` + informe migración + corpus MIG-5.1            | 02,04      | G4,G7,G9 | Data/Validation | FR-IMPORT-001 / NFR-MIG-001; VAL-MIG en CI; sin cmd/hooks |
| MVP-1-06 | Changeset + concurrencia en save/import                       | 02         | G4,G5    | Interfaces      | Corpus changeset; conflictos tipados                      |
| MVP-1-07 | Trazabilidad FR/NFR→TST/VAL ampliada a MVP-1                  | 04,05,06   | G2,G9    | Quality         | Matriz sin huérfanos del slice MVP-1                      |
| MVP-1-08 | Tag `implementation-baseline-mvp1` tras review                | 01–07      | G10      | Release         | Tag anotado; CI verde; go-no-go reevaluado                |

## Trazabilidad

- Requisitos: FR-CAT-001, FR-IMPORT-001, FR-DRAFT-001 (extensión), NFR-MIG-001, NFR-DET-001.
- Riesgos: RSK-004 (compatibilidad datos).
- Amenazas: THR-IMP-001 (importación).
- Validadores: VAL-MIG, VAL-DOC, corpus schema.
- Contradicciones: CON-001..004 (cierre); CON-005/006 siguen como rechazo de ejecución.
- Pruebas: TST-MIG-001, TST-CAT-_, TST-DRAFT-_ (ampliación).

## Criterio de entrada

1. Working tree de integridad post-E7 integrado.
2. `python3 tools/gates.py` → Clear; validadores locales en verde.
3. CON-013..022 cerradas o con residual fechado.
4. Aprobación explícita de este alcance antes de código de producto MVP-1.

## Criterio de salida (de esta fase documental)

- Issues MVP-1-01..08 publicados en `next-issues.md` / `backlog.md`.
- Sin commits de implementación de dominio/migración hasta que MVP-1-01 pase a `accepted`.
