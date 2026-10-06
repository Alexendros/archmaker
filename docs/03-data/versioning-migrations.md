---
id: DOC-DATA-VER-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - data
reviewers:
  - independent-reviewer
---

# Versionado y migraciones

## Versiones independientes

`documentVersion`, `schemaVersion`, `catalogRef.version`, `targetRef.version`, `producer.version`, `protocolVersion`.

## v5.1 → vNext

1. Limitar bytes/profundidad.
2. Detectar versión.
3. Validar mínimo estructural.
4. Detectar IDs duplicados y operadores desconocidos.
5. Construir `MigrationPlan` con preserved/transformed/dropped/rejected.
6. Mostrarlo al usuario.
7. Aplicar transformaciones puras.
8. Resolver aliases.
9. Validar destino.
10. Guardar como documento nuevo; conservar original.

## Casos obligatorios

- `mode: single`.
- `count_gt` desconocido.
- Arrays en presets.
- ID `base-devel` duplicado.
- `cmd` y hooks rechazados como ejecución.
