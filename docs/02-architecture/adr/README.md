---
id: DOC-ARCH-ADR-001
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

# Registro de decisiones de arquitectura (ADR)

- Estado: in-review
- Propietario: Arquitectura
- Última revisión: 2026-10-05
- Requisitos relacionados: NFR-DET-001, NFR-PORT-001, NFR-SEC-001

Índice de los ADR vigentes. Cada ADR es un documento independiente con cabecera (Estado, Fecha,
Propietario, Requisitos) y las secciones de `templates/adr.md`. Estados posibles: `proposed`,
`accepted`, `superseded`, `rejected`, `deferred`.

## ADR vigentes

| ADR | Título | Estado | Fecha |
|---|---|---|---|
| [ADR-0001](ADR-0001-rust-authority.md) | Rust como autoridad semántica del dominio | accepted | 2026-10-05 |
| [ADR-0002](ADR-0002-tauri-security.md) | Seguridad de Tauri: capabilities y deny-by-default | accepted | 2026-10-05 |
| [ADR-0003](ADR-0003-modelo-canonico-separado-del-target.md) | Modelo canónico de dominio separado del target | accepted | 2026-10-05 |
| [ADR-0004](ADR-0004-runner-separado-transporte-y-elevacion.md) | Runner separado: transporte y elevación | accepted | 2026-10-05 |
| [ADR-0005](ADR-0005-json-schema-2020-12-additionalProperties-false.md) | JSON Schema 2020-12 con `additionalProperties:false` | accepted | 2026-10-05 |
| [ADR-0006](ADR-0006-ejes-versionado-independientes.md) | Ejes de versionado independientes | accepted | 2026-10-05 |
| [ADR-0007](ADR-0007-canonicalizacion-y-hashing.md) | Canonicalización y hashing | proposed | 2026-10-05 |
| [ADR-0008](ADR-0008-catalogos-firmados-y-procedencia.md) | Catálogos firmados y procedencia | accepted | 2026-10-05 |
| [ADR-0009](ADR-0009-actualizacion-firmada-mvp.md) | Actualización firmada en MVP | accepted | 2026-10-05 |

## Crear un ADR

1. Copiar `templates/adr.md` a `ADR-NNNN-<slug>.md` con el siguiente número libre.
2. Rellenar contexto, drivers, opciones, decisión, consecuencias, riesgos y verificación.
3. Enlazar requisitos (FR/NFR) y decisiones (DEC-*) afectadas.
4. Estado inicial `proposed`; pasa a `accepted` solo con aprobación del propietario.
5. Si sustituye a otro ADR, marcar `superseded` en ambos documentos.

## Decisiones aún propuestas

ADR-0007 (canonicalización y hashing) está en estado `proposed`: RFC 8785 sigue siendo candidata y
su conformidad exacta permanece «no verificada — fuente primaria pendiente», por lo que no puede
figurar como `accepted`. Las decisiones P1 abiertas (DEC-009 fuentes, DEC-010 telemetría) se rigen
por `docs/00-governance/decision-register.md` y no requieren ADR hasta su implementación.
