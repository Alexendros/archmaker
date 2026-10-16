---
id: DOC-DEL-CLO-FIELD-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-CLO-INV-001
    expectedDocumentStatus: draft
---

# Matriz de campos F3 — UI → draft → resolución/manifiesto

- Documento: DOC-DEL-CLO-FIELD-001 · Estado: draft · Propietario: Arquitectura
- Anclas: `apps/web/src/draft-for-core.ts`, `apps/web/src/forge-face.ts`, `crates/archmaker-core/src/core.rs`
- Paridad web/nativo 20 fixtures/target: **no alcanzada** (pending). Este documento no la declara cumplida.

## Campos anunciados (`STEP_IDS`)

| Nombre | stepId | Tipo UI | Soporte producto | Representación exportada | Validación |
| --- | --- | --- | --- | --- | --- |
| Intención | `archmaker.face.intent` | single | excluido face-only | no entra a resolve | `draftForCore` + exclusión visible (`face-only-intent`) |
| Kernel | `archmaker.base.kernel` | single | soportado (catálogo) | `effectiveSelections` optionId | option en catálogo; gap bloqueado en UI |
| Arranque | `archmaker.base.boot` | single | soportado | idem | idem |
| Filesystem | `archmaker.base.filesystem` | single | soportado | idem | idem |
| Entorno | `archmaker.environment.compositor` | single | soportado / gap UI | optionId o exclusión gap | gap: no al core; diagnóstico de cara |
| Apps | `archmaker.face.apps` | multiple | soportado / gap filtrado | optionIds sin gap | `draftForCore` quita gap |
| Hostname | `archmaker.face.hostname` | text | soportado (intención) | `effectiveSelections` value.text | no vacío; no entidad de catálogo |
| Cifrado | `archmaker.face.encrypt` | boolean | soportado como **intención** | `effectiveSelections` value.boolean | no implica disco cifrado |

## Tipos de `SelectionValue`

| Tipo | Anunciado en cara | Comportamiento |
| --- | --- | --- |
| single | sí | resuelto vía catálogo |
| multiple | sí (apps) | resuelto; gap filtrado en TS |
| boolean | sí (encrypt) | preservado como intención en manifiesto |
| text | sí (hostname) | preservado en manifiesto |
| number | no anunciado | excluido con diagnóstico `AM-FACE-001` (no bloqueante) |
| secretRef | no anunciado | excluido con diagnóstico `AM-FACE-001` (no bloqueante) |

## Exclusiones visibles

1. **Intención face-only** y **paquetes gap**: `listFaceExclusions` / `draftForCore` en TypeScript; la revisión muestra el recuento.
2. **number / secretRef** si llegan al draft: diagnóstico Rust `AM-FACE-001` en validate/resolve.

## Referencias de catálogo (resolve)

| Caso | Resultado |
| --- | --- |
| ref correcta (namespace/id/version/digest) | ok |
| digest alterado | `AM-CAT-002` |
| versión incompatible | `AM-CAT-001` |
| id inexistente | `AM-CAT-001` |

## Cifrado: honestidad

El booleano en el manifiesto es intención de perfil. La UI dice que marca intención y que la cara no cifra el disco. No se usa lenguaje de «cifrado aplicado».

## Pruebas

- `crates/archmaker-core/tests/f3_fidelity.rs`
- `c03_resolve_hostname_encrypt_preserved_as_intention` en `f0_findings.rs` (actualizado por F3)
- `apps/web/forge-helpers.playwright.test.ts` (exclusiones draftForCore)
