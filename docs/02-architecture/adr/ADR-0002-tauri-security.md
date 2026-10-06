---
id: ADR-0002
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
---

# ADR-0002: Tauri deny-by-default

- Estado: accepted
- Fecha: 2026-10-05

## Decisión

Cada ventana obtiene únicamente capabilities explícitas. Los comandos y plugins usan permisos allow/deny y scopes mínimos. El MVP no habilita shell ni sidecars; filesystem se concede por selección explícita del usuario. CSP restringe recursos a `self` y protocolos necesarios.

## Verificación

Tests negativos deben demostrar que una ventana no autorizada no puede invocar comandos, leer rutas fuera de scope ni cargar recursos remotos.
