---
id: DOC-CON-NONNORM-001
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - data
reviewers:
  - independent-reviewer
---

# Ejemplos no normativos

Ejemplos **ilustrativos** de los contratos v0. No forman parte del corpus verificado: el gate
`json-schema` solo recorre `examples/*.valid.json` y `examples/*.invalid.json` en el nivel superior,
de modo que los ficheros de este directorio quedan **fuera** de la validación de corpus y del
chequeo de límites.

| Archivo | Rol |
|---|---|
| `draft-illustrative.json` | Draft comentado de forma conceptual; muestra por qué el estado derivado (`derivedSelections`) **no** se almacena. |
| `changeset-illustrative.json` | ChangeSet de ejemplo con una operación `replace` y su semántica de digest. |

Regla: si un ejemplo pasa a ser normativo, se mueve a `examples/` con la convención
`NN-<schema>[-descriptor].{valid,invalid}.json` y se añade al corpus del workflow.
