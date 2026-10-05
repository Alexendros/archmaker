---
id: DOC-GOV-DOC-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
---

# Control documental

## Estados

`draft`, `in-review`, `accepted`, `superseded`, `deferred`, `rejected`.

## Cabecera mínima

- ID y título.
- Estado.
- Propietario.
- Última revisión.
- Requisitos relacionados.
- Sustituye/sustituido por.

## Cambios

- Los contratos persistidos requieren versión y migration note.
- Las decisiones estructurales requieren ADR.
- Los originales de `reference/` nunca se corrigen in-place.
- Un documento accepted solo cambia mediante PR revisado por owner.
