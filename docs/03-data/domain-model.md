---
id: DOC-DATA-DM-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - data
reviewers:
  - independent-reviewer
---

# Modelo de dominio

- ID: DOC-DATA-DM-001
- Estado: accepted
- Propietario: Arquitectura de datos
- Última revisión: 2026-10-05
- Requisitos relacionados: NFR-DET-001, NFR-MIG-001
- Sustituye / sustituido por: —

| ID | Agregado | Identidad | Mutabilidad | Invariantes |
|---|---|---|---|---|
| DM-CATALOG | Catalog | namespace/id/version/digest | Inmutable | Referencias únicas, procedencia. |
| DM-DRAFT | Draft | UUID | Editable | Solo intención manual y refs. |
| DM-PRESET | Preset | namespace/id/version | Inmutable | Patch validable y trazable. |
| DM-RESOLUTION | Resolution | input digest | Efímera | Recalculable, determinista. |
| DM-MANIFEST | Manifest | digest | Inmutable | Cerrado y sin bloqueos. |
| DM-ARTIFACT | Artifact | digest/mediaType | Inmutable | Declara target y productor. |
| DM-PLAN | InstallationPlan | UUID + manifest hash | Inmutable | Operaciones tipadas y ordenadas. |
| DM-SESSION | RunnerSession | UUID/nonce | Estado | Una ejecución y expiración. |
| DM-POLICY | PolicyBundle | namespace/id/version/signature | Inmutable | Evaluación auditada. |

## SelectionValue

Unión discriminada: `single`, `multiple`, `boolean`, `number`, `text`, `secretRef`. `secretRef` nunca serializa el secreto.
