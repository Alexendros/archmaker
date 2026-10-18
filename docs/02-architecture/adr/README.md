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

| ADR                                                                    | Título                                               | Estado   | Fecha      |
| ---------------------------------------------------------------------- | ---------------------------------------------------- | -------- | ---------- |
| [ADR-0001](ADR-0001-rust-authority.md)                                 | Rust como autoridad semántica del dominio            | accepted | 2026-10-05 |
| [ADR-0002](ADR-0002-tauri-security.md)                                 | Seguridad de Tauri: capabilities y deny-by-default   | accepted | 2026-10-05 |
| [ADR-0003](ADR-0003-modelo-canonico-separado-del-target.md)            | Modelo canónico de dominio separado del target       | accepted | 2026-10-05 |
| [ADR-0004](ADR-0004-runner-separado-transporte-y-elevacion.md)         | Runner separado: transporte y elevación              | accepted | 2026-10-05 |
| [ADR-0005](ADR-0005-json-schema-2020-12-additionalProperties-false.md) | JSON Schema 2020-12 con `additionalProperties:false` | accepted | 2026-10-05 |
| [ADR-0006](ADR-0006-ejes-versionado-independientes.md)                 | Ejes de versionado independientes                    | accepted | 2026-10-05 |
| [ADR-0007](ADR-0007-canonicalizacion-y-hashing.md)                     | Canonicalización y hashing                           | accepted | 2026-10-06 |
| [ADR-0008](ADR-0008-catalogos-firmados-y-procedencia.md)               | Catálogos firmados y procedencia                     | accepted | 2026-10-05 |
| [ADR-0009](ADR-0009-actualizacion-firmada-mvp.md)                      | Actualización firmada en MVP                         | accepted | 2026-10-05 |
| [ADR-0010](ADR-0010-marca-y-nombre-de-artefacto.md)                    | Marca y nombre de artefacto público                  | accepted | 2026-10-14 |
| [ADR-0011](ADR-0011-contrato-de-aplicacion.md)                         | Contrato de aplicación desde el configurador         | proposed | 2026-10-14 |
| [ADR-0012](ADR-0012-nombre-visible-forge.md)                           | Nombre visible Forge (GUI de ArchMaker)              | accepted | 2026-10-17 |
| [ADR-0013](ADR-0013-uki-on-esp.md)                                     | UKI en ESP (`uki-on-esp`)                            | proposed | 2026-10-17 |
| [ADR-0014](ADR-0014-estados-ortogonales-verificacion.md)               | Estados ortogonales de verificación y ejecución      | proposed | 2026-10-17 |
| [ADR-0015](ADR-0015-handles-dispositivo-y-destino.md)                  | Handles de sesión vs destino portable                | proposed | 2026-10-17 |
| [ADR-0016](ADR-0016-contratos-digest-v2.md)                            | Contratos y digests schemaVersion 2                  | proposed | 2026-10-17 |
| [ADR-0017](ADR-0017-navegadores-y-onlyoffice.md)                       | ONLYOFFICE canónico y cardinalidad de navegadores    | proposed | 2026-10-17 |
| [ADR-0018](ADR-0018-alcance-declarativo-a6-excluida.md)                | Alcance declarativo; A6 excluida                     | proposed | 2026-10-17 |
| [ADR-0019](ADR-0019-perfil-vivo-archiso.md)                            | Target de perfil vivo y proceso de constitución      | accepted | 2026-10-18 |

## Crear un ADR

1. Copiar `templates/adr.md` a `ADR-NNNN-<slug>.md` con el siguiente número libre.
2. Rellenar contexto, drivers, opciones, decisión, consecuencias, riesgos y verificación.
3. Enlazar requisitos (FR/NFR) y decisiones (DEC-*) afectadas.
4. Estado inicial `proposed`; pasa a `accepted` solo con aprobación del propietario.
5. Si sustituye a otro ADR, marcar `superseded` en ambos documentos.

## Decisiones aún propuestas

Ningún ADR queda en estado `proposed`: ADR-0007 (canonicalización y hashing) pasó a `accepted` en
la fase R5, adopta RFC 8785 (JCS) de forma definitiva y fija el perfil
`archmaker-canonicalization-profile@v1`; su conformidad byte a byte con el RFC permanece «no
verificada — fuente primaria pendiente», pero el perfil v1 y su corpus son la autoridad ejecutable.
Las decisiones P1 abiertas (DEC-009 fuentes, DEC-010 telemetría) se rigen por
`docs/00-governance/decision-register.md` y no requieren ADR hasta su implementación.
