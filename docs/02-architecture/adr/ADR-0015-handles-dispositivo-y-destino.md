---
id: ADR-0015
phase: planning
priority: P0
documentStatus: superseded
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# ADR-0015: `deviceHandle` de sesión vs `logicalTargetRef` portable

- Estado: superseded
- Superseded by: ADR-0020

- Estado: proposed
- Fecha: 2026-10-17
- Propietario: Arquitectura

## Contexto

Las rutas `/dev/sdX` no son identidad estable ni exportable. El frontend no debe enviar paths.

## Decisión

- `deviceHandle`: token opaco de **sesión** emitido por el backend de sonda; no viaja en el
  perfil portable por defecto.
- `logicalTargetRef`: identificador opaco **portable** opcional en `storagePlan` para
  revalidación sin filtrar serial/WWN en evidencia pública.

Observaciones confiables llevan `status`, `source`, `observedAt` y diagnóstico; `unknown` /
`unavailable` no se convierten en `incompatible`.

## Consecuencias

- El walking skeleton fundacional no inventa handles observados.
- Las sondas futuras (fuera de este PR) emiten handles tipados por IPC.

## Verificación

- Tipos en `archmaker-domain::schema_v2` y corpus JSON.
