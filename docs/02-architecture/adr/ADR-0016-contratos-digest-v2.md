---
id: ADR-0016
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0006
  - id: ADR-0007
---

# ADR-0016: Contratos y digests `schemaVersion` 2

- Estado: proposed
- Fecha: 2026-10-17
- Propietario: Arquitectura

## Contexto

El draft v1 no modela plan de disco, LUKS2 tipado ni cardinalidad de navegadores. Hay que
versionar forma y dominio de hash sin romper v1.

## Decisión

- Eje documento: `schemaVersion` `"1"` | `"2"`; versiones futuras (`"3"`+) se rechazan.
- Dominios ADR-0007:
  - Draft: `archmaker:draft:v1` / `archmaker:draft:v2`
  - Manifiesto: `archmaker:manifest:v1` / `archmaker:manifest:v2` (`domainLabel`)
- El digest semántico v2 cubre `storagePlan`, `encryptionPolicy` y `browserPolicy`.
- Enteros de capacidad/tamaño: acotados a `Number.MAX_SAFE_INTEGER` (JS/Rust).
- Migrador v1→v2 idempotente con informe; no inventa hardware ni sustituye apps.

## Consecuencias

- `create_draft` sigue naciendo en `"1"`; el skeleton construye `"2"` de forma explícita.
- Corpus JSON y tests Rust validan positivo/negativo y rechazo de futuras.

## Verificación

- `migrate_draft_v1_to_v2`, digests CorePort, fixtures `32-draft-v2.*` / `33-manifest-v2.*`.
