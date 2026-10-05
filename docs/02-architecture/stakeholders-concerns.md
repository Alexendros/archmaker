---
id: DOC-ARCH-STK-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Stakeholders y concerns

- ID: DOC-ARCH-STK-001
- Estado: in-review
- Propietario: Arquitectura
- Fecha: 2026-10-06
- Requisitos relacionados: OBJ-001..OBJ-006, FR-DRAFT-001, FR-IMPORT-001, FR-CAT-001, FR-PRESET-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001, FR-EXPORT-001, FR-RUN-001, FR-ENT-001, NFR-SEC-001, NFR-DET-001, NFR-ACC-001, NFR-OFF-001, NFR-MIG-001, NFR-PORT-001, NFR-OBS-001
- Fase: R4 (issues `AUD-007`/`AUD-008`); resuelve `AUD-F-08` del `audit-baseline.md`

## Propósito

Identificar los stakeholders de ArchMaker, sus **concerns** arquitectónicos y el mapping
`concern → view → ADR`, conforme a ISO/IEC/IEEE 42010. Este documento y
[`viewpoints.md`](viewpoints.md) constituyen la descripción arquitectónica mínima que
encuadra los concerns antes de las vistas de [`c4/`](c4/README.md).

Los estados se leen del front matter y de `docs/00-governance/status-model.md`; no se repiten
narrativamente en los consumidores.

## Conceptos (ISO/IEC/IEEE 42010)

- **Stakeholder:** individuo, rol u organización con interés en el sistema de interés.
- **Concern:** interés de un stakeholder relevante para la arquitectura.
- **Viewpoint:** convención para construir, interpretar y usar una view que encuadra concerns
  concretos de stakeholders.
- **View:** expresión de la arquitectura que aborda los concerns mediante un viewpoint.
- **Architecture description:** conjunto de views, viewpoints y su justificación.

Los viewpoints que generan las views viven en [`viewpoints.md`](viewpoints.md).

## Sistema de interés y alcance

- **Sistema de interés (SoI):** ArchMaker, en sus dos productos locales (Desktop Application y
  Browser Application), más las capacidades de fases posteriores (Runner v1 y Enterprise).
- **Alcance de esta descripción:** arquitectura de software y de despliegue del SoI; no cubre la
  arquitectura del sistema operativo anfitrión ni la de terceros (repositorios, fuentes oficiales,
  proveedor Sigstore).
- **Fuera de alcance:** instalación real, root, discos y shell (v1 y posteriores permanecen
  bloqueados por gates). El `Rust Core` es un **componente interno**, no un contenedor.

## Stakeholders

| ID | Stakeholder | Tipo | Intereses principales |
|---|---|---|---|
| SH-01 | Usuario guiado (PER-001) | Persona | Configurar y exportar sin terminal; accesibilidad y operación offline. |
| SH-02 | Usuario avanzado (PER-002) | Persona | Presets, migración del legado y control fino de la resolución. |
| SH-03 | Maintainer de catálogo (PER-003) | Persona | Autoría de catálogos y presets con procedencia verificable. |
| SH-04 | Operador Enterprise (PER-004) | Persona | RBAC, policies, aislamiento de tenant, auditoría y campañas. |
| SH-05 | Product owner | Rol de gobierno | Alcance, journeys, prioridad y releases. |
| SH-06 | Arquitectura | Rol de gobierno | Boundaries, módulos, C4, viewpoints y ADR. |
| SH-07 | Data/contracts owner | Rol de gobierno | Schemas, migraciones, canonicalización y versiones. |
| SH-08 | Security owner | Rol de gobierno | Modelo de amenazas, capabilities, updater y riesgo residual. |
| SH-09 | UX/design owner | Rol de gobierno | Design system, componentes y accesibilidad. |
| SH-10 | Quality owner | Rol de gobierno | Corpus, pruebas, CI y evidencia de verificación. |
| SH-11 | Release owner | Rol de gobierno | Packaging, firma, procedencia y rollback. |
| SH-12 | Revisor independiente | Rol de gobierno | Conformidad arquitectónica y dictamen No-Go/Go. |
| SH-13 | Mantenedor de ArchMaker / integrador de distribución | Organización | Empaquetado, compatibilidad de toolchain y política de dependencias. |
| SH-14 | Auditor externo (NIST SSDF) | Organización | Trazabilidad de criterios de seguridad durante el SDLC. |

## Concerns

| ID | Concern | Prioridad | Stakeholders | Origen | Vistas que lo abordan | ADR |
|---|---|---|---|---|---|---|
| CN-01 | Configurar el objetivo completo por UI, sin terminal ni shell. | P0 | SH-01, SH-02 | OBJ-001, FR-DRAFT-001 | VW-01, VW-02, VW-03, VW-04, VW-07, VW-08, VW-11 | ADR-0001, ADR-0002 |
| CN-02 | Determinismo y reproducibilidad de digests entre adaptadores y máquinas. | P0 | SH-06, SH-10 | OBJ-002, NFR-DET-001, FR-RESOLVE-001, FR-MANIFEST-001 | VW-03, VW-04, VW-05, VW-11 | ADR-0001, ADR-0007 |
| CN-03 | Resultado válido y explicable mediante diagnósticos tipados y estables. | P0 | SH-01, SH-02, SH-10 | OBJ-002, FR-VALIDATE-001 | VW-05, VW-11 | ADR-0001 |
| CN-04 | Migrar el legado v5.1 de forma explícita y no destructiva. | P0 | SH-01, SH-02, SH-07 | OBJ-003, FR-IMPORT-001, NFR-MIG-001 | VW-05, VW-11 | ADR-0005, ADR-0006 |
| CN-05 | Exportar artefactos reproducibles con target, versiones y digests. | P0 | SH-01, SH-02, SH-11 | OBJ-004, FR-EXPORT-001 | VW-02, VW-05, VW-11 | ADR-0003, ADR-0006 |
| CN-06 | Ejecutar un plan aprobado de forma segura y fuera del WebView. | P0 | SH-01, SH-02, SH-08 | OBJ-005, FR-RUN-001 | VW-06, VW-09, VW-11 | ADR-0004 |
| CN-07 | Gobernar flotas con aislamiento de tenant y auditoría. | P0 | SH-04, SH-08 | OBJ-006, FR-ENT-001 | VW-02, VW-10 | ADR-0003 |
| CN-08 | Deny-by-default y mínimo privilegio en capabilities y scopes. | P0 | SH-08 | NFR-SEC-001 | VW-02, VW-03, VW-07, VW-09 | ADR-0002, ADR-0004 |
| CN-09 | Portabilidad del dominio y una sola autoridad semántica en Rust. | P0 | SH-06, SH-07, SH-10 | NFR-PORT-001 | VW-03, VW-04, VW-05 | ADR-0001 |
| CN-10 | Operar sin CDN ni servidor obligatorio en el MVP. | P0 | SH-01, SH-02, SH-11 | NFR-OFF-001 | VW-07, VW-08, VW-09 | ADR-0009 |
| CN-11 | Observabilidad estructurada con redacción de secretos. | P0 | SH-08, SH-10 | NFR-OBS-001 | VW-05, VW-06, VW-11 | ADR-0004 |
| CN-12 | Proteger la cadena de suministro y la actualización firmada. | P0 | SH-08, SH-11 | RSK-007, THR-UPD-001 | VW-02, VW-07, VW-09, VW-11 | ADR-0008, ADR-0009 |
| CN-13 | Cumplir WCAG 2.2 AA en todos los flujos. | P0 | SH-01, SH-09 | NFR-ACC-001 | VW-03, VW-04 | — |
| CN-14 | Evolucionar contratos y ejes de versión de forma independiente. | P0 | SH-06, SH-07 | NFR-MIG-001, NFR-DET-001 | VW-02, VW-05 | ADR-0005, ADR-0006 |
| CN-15 | Cargar catálogos firmados con procedencia verificable. | P0 | SH-03, SH-08 | FR-CAT-001, NFR-SEC-001 | VW-01, VW-02, VW-05 | ADR-0008 |
| CN-17 | Aplicar presets como patches explicables sin sobrescribir la intención. | P0 | SH-01, SH-02 | FR-PRESET-001 | VW-05, VW-11 | ADR-0001, ADR-0006 |
| CN-16 | Preservar el diseño visual heredado sin regresiones no aprobadas. | P1 | SH-01, SH-09 | — | VW-03, VW-04 | — |

## Mapping concern → view → ADR

Las views se materializan en:

- VW-01 Contexto y VW-02 Contenedores: [`c4/README.md`](c4/README.md).
- VW-03..VW-06 Componentes: [`c4/component-views.md`](c4/component-views.md).
- VW-07..VW-10 Deployment: [`c4/deployment-views.md`](c4/deployment-views.md).
- VW-11 Secuencias de runtime: [`c4/runtime-sequences.md`](c4/runtime-sequences.md).

| Concern | VW-01 | VW-02 | VW-03 | VW-04 | VW-05 | VW-06 | VW-07 | VW-08 | VW-09 | VW-10 | VW-11 |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| CN-01 | ● | ● | ● | ● | | | ● | ● | | | ● |
| CN-02 | | | ● | ● | ● | | | | | | ● |
| CN-03 | | | | | ● | | | | | | ● |
| CN-04 | | | | | ● | | | | | | ● |
| CN-05 | | ● | | | ● | | | | | | ● |
| CN-06 | | | | | | ● | | | ● | | ● |
| CN-07 | | ● | | | | | | | | ● | |
| CN-08 | | ● | ● | | | | ● | | ● | | |
| CN-09 | | | ● | ● | ● | | | | | | |
| CN-10 | | | | | | | ● | ● | ● | | |
| CN-11 | | | | | ● | ● | | | | | ● |
| CN-12 | | ● | | | | | ● | | ● | | ● |
| CN-13 | | | ● | ● | | | | | | | |
| CN-14 | | ● | | | ● | | | | | | |
| CN-15 | ● | ● | | | ● | | | | | | |
| CN-17 | | | | | ● | | | | | | ● |
| CN-16 | | | ● | ● | | | | | | | |

## Cobertura de concerns P0

Todo concern `P0` (`CN-01`..`CN-15` y `CN-17`) tiene al menos una view asociada; `CN-16` es `P1`
y también queda cubierto. La verificación de esta propiedad es el criterio de salida R4 «todos los
concerns P0 tienen view» y se comprueba en la revisión independiente (R13).

| Prioridad | Concerns | Con view | Sin view |
|---|---|---:|---:|
| P0 | CN-01..CN-15, CN-17 | 16 | 0 |
| P1 | CN-16 | 1 | 0 |

## Relación con otros documentos

- Viewpoints y correspondencia: [`viewpoints.md`](viewpoints.md).
- Módulos y dependencias internas del `Rust Core`: [`module-map.md`](module-map.md) y
  [`dependency-rules.md`](dependency-rules.md).
- Requisitos y objetivos: `docs/01-product/requirements.md` y `docs/01-product/objectives.md`.
- Personas y roles: `docs/01-product/personas.md` y `docs/00-governance/remediation-plan-v1.1.md`.
