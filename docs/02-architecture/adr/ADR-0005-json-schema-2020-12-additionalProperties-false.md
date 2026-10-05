---
id: ADR-0005
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# ADR-0005: Contratos en JSON Schema Draft 2020-12 con `additionalProperties: false`

- Estado: accepted
- Fecha: 2026-10-05
- Propietario: Arquitectura
- Requisitos: FR-DRAFT-001, FR-IMPORT-001, FR-MANIFEST-001, NFR-MIG-001, NFR-DET-001

## Contexto

El schema heredado v5.1 admite propiedades abiertas, arrays donde solo caben escalares (`CON-003`), un operador no definido (`CON-002`) y `mode: single` ausente (`CON-001`). El registro de fuentes marca JSON Schema 2020-12 (`SRC-006`) como estándar objetivo con captura de fuente pendiente. Los contratos persistidos requieren versión y migration note (`document-control.md`). Un esquema abierto permite que datos no declarados entren en el dominio y rompan la determinación (`NFR-DET-001`).

Documentos de apoyo: `docs/03-data/versioning-migrations.md`, `docs/03-data/canonicalization.md`, `docs/00-governance/contradiction-register.md`.

## Drivers

- `NFR-DET-001`: mismo input/versiones → mismo resultado.
- `NFR-MIG-001`: migraciones explícitas y no destructivas.
- Cerrar el conjunto de propiedades para que lo no declarado se rechace.
- Conservar el schema v5.1 únicamente como evidencia histórica.
- Alinear validación estructural con el pipeline de migración.

## Opciones consideradas

1. **Mantener esquemas abiertos** (sin `additionalProperties`): compatibilidad laxa, riesgo de datos no declarados.
2. **Draft 2020-12 con `additionalProperties: false` por defecto**: contratos cerrados; el schema v5.1 abierto se conserva solo como evidencia histórica.
3. **Esquemas ad-hoc por consumidor**: sin estándar común.

## Decisión propuesta

Los **contratos nuevos** se definen en **JSON Schema Draft 2020-12** y adoptan **`additionalProperties: false` por defecto**. El **schema v5.1 abierto se conserva únicamente como evidencia histórica** en `reference/` (nunca como contrato runtime). La migración documenta explícitamente el **cierre** del esquema: cada propiedad eliminada o renombrada se registra como preserved/transformed/dropped/rejected, y las diferencias de cardinalidad y operadores se resuelven antes de implementar (`CON-001`, `CON-002`, `CON-003`).

## Consecuencias

- Datos no declarados se rechazan en el borde, reduciendo ambigüedad y superficie de error.
- Los fixtures deben cubrir casos válidos e inválidos explícitos.
- Cada cambio de contrato exige nueva `schemaVersion` y migration note.
- Puede romper importaciones laxas del legado; se absorbe mediante migración explícita, no por tolerancia silenciosa.

## Riesgos

- `RSK-004` (la migración pierde selecciones): mitigado con `MigrationPlan` y corpus golden.
- Migración que descarte propiedades sin trazabilidad: mitigado por el informe preserved/transformed/dropped/rejected.

## Verificación

- Fixtures válidos e inválidos por contrato, con rechazo de propiedades no declaradas.
- Corpus histórico migrado con informe previo y original intacto (`NFR-MIG-001`).
- Vectores de canonicalización estables bajo el esquema cerrado (`NFR-DET-001`).
- La conformidad exacta con Draft 2020-12 queda **no verificada — fuente primaria pendiente** (`SRC-006`) hasta su captura.

## Sustituye

—

## Sustituido por

—

## Requisitos relacionados

FR-DRAFT-001, FR-IMPORT-001, FR-MANIFEST-001, NFR-MIG-001, NFR-DET-001. Relaciona `SRC-006`, `CON-001`, `CON-002`, `CON-003`.
