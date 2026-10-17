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

## Draft schemaVersion 1 → 2 (ADR-0016)

1. Decodificar `schemaVersion`; rechazar futuras (`"3"`+).
2. Si ya es `"2"`, informe idempotente (`idempotent_noop`).
3. Conservar selecciones e IDs de apps; no sustituir Firefox/LibreOffice/Flameshot.
4. No inventar firmware, disco ni capacidad.
5. Preservar booleano histórico de cifrado como `inheritedEncryptIntent` (no como LUKS2).
6. Copiar `archmaker.face.intent` a `legacyMetadata.faceIntent` (el paso UI puede desaparecer).
7. Añadir `verification` con `executionStatus: not-executed` (A6 fuera).
8. Invalidar `contentDigest` para recomputar con dominio `archmaker:draft:v2`.
9. Export público (`public_export_bundle`): plan + verificación + legacyMetadata; sin snapshot privado ni serial/WWN.
