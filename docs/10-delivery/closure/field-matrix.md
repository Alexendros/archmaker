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

| Nombre              | stepId / campo                      | Tipo UI                       | Soporte producto                               | Representación exportada         | Validación                                |
| ------------------- | ----------------------------------- | ----------------------------- | ---------------------------------------------- | -------------------------------- | ----------------------------------------- |
| Disco y particiones | `storagePlan` (draft v2)            | plan declarativo              | soportado (sin IO)                             | plan + estados ortogonales       | motor Rust; UEFI64+GPT certifica política |
| Intención (legado)  | `archmaker.face.intent`             | —                             | no es paso activo                              | `legacyMetadata` tras migración  | no entra a resolve                        |
| Kernel              | `archmaker.base.kernel`             | single                        | soportado (catálogo)                           | `effectiveSelections` optionId   | option en catálogo                        |
| Arranque            | `archmaker.base.boot`               | single                        | soportado                                      | idem                             | idem; systemd-boot no válido en BIOS      |
| Filesystem          | `archmaker.base.filesystem`         | single                        | soportado                                      | idem                             | idem                                      |
| Entorno             | `archmaker.environment.compositor`  | single                        | soportado                                      | optionId                         | catálogo                                  |
| Apps                | `archmaker.face.apps`               | multiple                      | soportado / unverified                         | optionIds sin unverified         | `draftForCore` quita unverified           |
| Navegador default   | `defaultBrowserId`                  | derivado                      | soportado                                      | campo draft v2                   | debe estar entre seleccionados o null     |
| Hostname            | `archmaker.face.hostname`           | text                          | soportado (intención)                          | `effectiveSelections` value.text | no vacío                                  |
| Cifrado             | `encryptionPolicy` / encrypt legado | LUKS2 o none / boolean legado | política tipada; booleano = intención heredada | policy o boolean                 | sin secretos; no implica disco cifrado    |

## Tipos de `SelectionValue`

| Tipo      | Anunciado en cara | Comportamiento                                         |
| --------- | ----------------- | ------------------------------------------------------ |
| single    | sí                | resuelto vía catálogo                                  |
| multiple  | sí (apps)         | resuelto; unverified filtrado en TS                    |
| boolean   | sí (encrypt)      | preservado como intención en manifiesto                |
| text      | sí (hostname)     | preservado en manifiesto                               |
| number    | no anunciado      | excluido con diagnóstico `AM-FACE-001` (no bloqueante) |
| secretRef | no anunciado      | excluido con diagnóstico `AM-FACE-001` (no bloqueante) |

## Exclusiones visibles

1. **Intención face-only** y **solicitudes unverified**: `listFaceExclusions` / `draftForCore` en TypeScript; la revisión muestra el recuento.
2. **number / secretRef** si llegan al draft: diagnóstico Rust `AM-FACE-001` en validate/resolve.

## Referencias de catálogo (resolve)

| Caso                                       | Resultado    |
| ------------------------------------------ | ------------ |
| ref correcta (namespace/id/version/digest) | ok           |
| digest alterado                            | `AM-CAT-002` |
| versión incompatible                       | `AM-CAT-001` |
| id inexistente                             | `AM-CAT-001` |

## Cifrado: honestidad

El booleano en el manifiesto es intención de perfil. La UI dice que marca intención y que la cara no cifra el disco. No se usa lenguaje de «cifrado aplicado».

## Pruebas

- `crates/archmaker-core/tests/f3_fidelity.rs`
- `c03_resolve_hostname_encrypt_preserved_as_intention` en `f0_findings.rs` (actualizado por F3)
- `apps/web/forge-helpers.playwright.test.ts` (exclusiones draftForCore)
