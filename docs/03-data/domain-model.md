# Modelo de dominio

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
