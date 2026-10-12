---
id: DOC-DEL-MVP3-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
  - data
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-MVP2-001
    expectedDocumentStatus: accepted
  - id: DOC-GOV-GNG-001
  - id: ADR-0003
    expectedDocumentStatus: accepted
---

# Alcance MVP-3 — manifest / exporters

- Documento: DOC-DEL-MVP3-001 · Estado: accepted · Propietario: Arquitectura/Datos · Fecha: 2026-10-12
- Entrada: baseline `implementation-baseline-mvp2` + gates G0–G10 `complete` (slice).
- Alcance **accepted** (MVP-3-01). Implementación de adapters de exportación versionados
  autorizada bajo las exclusiones de este documento; runner y privilegios siguen fuera.

## Objetivo

Hacer que `exportArtifact` cumpla ADR-0003 y FR-EXPORT-001: cada `ExportTarget` de DEC-001 es
un adapter versionado que traduce un `Manifest` sellado a un `Artifact` con bytes propios,
sin alterar el draft, sin shell y sin cambiar la superficie de CorePort.

## Incluido

| Área                  | Contenido                                                                                             | Anclas                           |
| --------------------- | ----------------------------------------------------------------------------------------------------- | -------------------------------- |
| Perfil ArchMaker      | Bytes = manifest canónico JSON; `mediaType` `application/json`; `content` = payload del perfil        | FR-EXPORT-001, DEC-001           |
| Reporte               | Markdown determinista ordenado; `mediaType` `text/markdown`                                           | FR-EXPORT-001, DEC-001           |
| Sobre archinstall     | Envelope JSON experimental distinto del manifest; `experimental: true`; sin cmd/hooks/shell/rutas     | FR-EXPORT-001, ADR-0003, DEC-001 |
| Compatibilidad target | Etapa 12 del pipeline: target soportado, alineado al manifest, digest recomputable                    | VAL-TARGET, pipeline.md etapa 12 |
| Determinismo          | Reexportar el mismo manifest sellado ⇒ mismos bytes/`binaryDigest`; adapters producen bytes distintos | NFR-DET-001, TST-EXP-001         |

## Excluido (explícito)

- Runner v1, root, discos reales, shell arbitraria, sidecars.
- Catálogos remotos / firma criptográfica de catálogo en carga (etapa 11).
- Schema upstream de `archinstall` ni config ejecutable.
- Etapa 10 como schema nuevo de hardware (el draft ya fija `arch-x86_64`, DEC-008).
- Nuevas operaciones CorePort (`listExportTargets`, etc.).
- Presets, Enterprise, telemetría, auto-updater, rediseño visual.

## Inventario de gaps (vs MVP-2)

| Gap                                         | Estado en MVP-2                                 | Trabajo MVP-3                                   |
| ------------------------------------------- | ----------------------------------------------- | ----------------------------------------------- |
| `archmaker-profile` ≡ `archinstall-profile` | Mismos bytes canónicos del manifest             | Sobre experimental distinto + `content` relleno |
| `Artifact.content`                          | Siempre `None`                                  | Payload del adapter por target                  |
| ADR-0003                                    | `accepted`, `implementationStatus: not-started` | Evidencia de adapters versionados               |
| Etapa 12 exporter compatibility             | Diferida explícitamente a MVP-3                 | Rechazo tipado + tests VAL-TARGET / TST-EXP-001 |

## Issues trazables (orden por dependencia)

| #        | Issue                                                         | Depende de | Gate   | Owner        | Criterio Done                                    |
| -------- | ------------------------------------------------------------- | ---------- | ------ | ------------ | ------------------------------------------------ |
| MVP-3-01 | Aprobar alcance DOC-DEL-MVP3-001 + actualizar roadmap/backlog | —          | G2,G10 | Product      | Documento `accepted`; exclusiones firmadas       |
| MVP-3-02 | Adapter `archmaker-profile` + `report` con `Artifact.content` | 01         | G4,G5  | Architecture | Bytes canónicos / markdown; content relleno      |
| MVP-3-03 | Sobre experimental `archinstall-profile` (ADR-0003)           | 01,02      | G3,G5  | Architecture | Bytes ≠ perfil ArchMaker; sin campos ejecutables |
| MVP-3-04 | Tests divergencia / idempotencia / rechazo (TST-EXP-001)      | 02,03      | G5,G9  | Quality      | core_contract verde; VAL-TARGET evidenciado      |
| MVP-3-05 | Trazabilidad FR-EXPORT-001 / NFR-DET-001 / ADR-0003 ampliada  | 04         | G2,G9  | Quality      | Matriz sin huérfanos del slice MVP-3             |
| MVP-3-06 | Tag `implementation-baseline-mvp3` tras review                | 01–05      | G10    | Release      | Tag anotado; CI verde; go-no-go reevaluado       |

## Trazabilidad

- Requisitos: FR-EXPORT-001, FR-MANIFEST-001 (entrada), NFR-DET-001, NFR-PORT-001.
- Riesgos: RSK-005 (adapter obsoleto — residual aceptado para experimental).
- Amenazas: sin superficie nueva de shell/red/disco (CON-005/006/012 siguen).
- Validadores: VAL-TARGET, corpus schema `artifact`.
- Pruebas: TST-EXP-001 (ampliación), paridad export si el golden lo exige.
- Decisiones/ADR: DEC-001, DEC-008, ADR-0003.

## Criterio de entrada

1. Baseline `implementation-baseline-mvp2` presente; tags previos intactos.
2. `python3 tools/gates.py` → Clear; validadores locales en verde.
3. Aprobación explícita de este alcance antes de código de producto MVP-3.

## Criterio de salida

- Issues MVP-3-01..06 cerrados o con residual fechado.
- Tag `implementation-baseline-mvp3` emitido; tags MVP-0.1, MVP-1 y MVP-2 intactos.
- `archinstall-profile` y `archmaker-profile` no son byte-idénticos.
- Reexportar el mismo manifest produce los mismos bytes y el mismo `binaryDigest`.
