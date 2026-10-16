---
id: DOC-DEL-CLO-INV-001
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
  - id: DOC-DEL-MVP5-001
    expectedDocumentStatus: accepted
---

# Inventario F0 — targets, campos y CI

- Documento: DOC-DEL-CLO-INV-001 · Estado: draft · Propietario: Arquitectura
- `BASE_SHA` observado: `56c2a198cc36da3a30d727487f082d13fda1b1f5`
- `SOURCE_SHA_F0` es el commit que contiene este árbol. No se escribe aquí por adelantado y el manifiesto de evidencia no guarda su propio digest.
- Esto no acepta el alcance ni sustituye a un revisor independiente. No reabre OP-1..OP-8.

## Targets

Denominador de exportación: 3 ids en `EXPORT_TARGETS`, los tres con renderer y bytes.

| Id                    | Papel                         | Renderer | Anunciado en la UI |
| --------------------- | ----------------------------- | -------- | ------------------ |
| `archmaker-profile`   | export                        | sí       | sí, único          |
| `archinstall-profile` | export                        | sí       | no                 |
| `report`              | export                        | sí       | no                 |
| `arch-x86_64`         | draft                         | no       | no                 |
| `iso`                 | rechazado por `ensure_target` | no       | no                 |

La UI exporta 1/3. Los dos call sites de `exportTarget` en `apps/web/src/App.tsx` usan `archmaker-profile`. Existir en `EXPORT_TARGETS` no cierra el target.

## Campos de cara

Denominador de pasos visibles: 6/6 en `RAIL_STEPS` (`Intención`, `Sistema base`, `Entorno`, `Apps`, `Identidad`, `Revisión`).

Denominador de `STEP_IDS`: 8. De ellos, `draftForCore` quita solo `archmaker.face.intent`. Hostname (`archmaker.face.hostname`, texto) y cifrado (`archmaker.face.encrypt`, boolean) siguen en el draft. La cara dice «Esta cara no cifra nada.»

Denominador de `optionId`: 38 campos. 12 llevan `gap: true`. 26 no.

| Grupo      | Opciones | Con gap | Notas                                        |
| ---------- | -------- | ------- | -------------------------------------------- |
| Intención  | 4        | 0       | face-only; no entra a resolve                |
| Kernel     | 4        | 1       |                                              |
| Arranque   | 3        | 0       | preselección UI: systemd-boot                |
| Filesystem | 2        | 0       |                                              |
| Entorno    | 9        | 5       | 6 DE y 3 WM; Sencillo muestra 6/9 (solo DE)  |
| Paquetes   | 16       | 6       | 6 capas: 4 `simple: true`, 2 `simple: false` |

## CI

Denominador de workflows en `.github/workflows`: 7.

| Workflow           | `pull_request` | `push` a `main` | Otro                |
| ------------------ | -------------- | --------------- | ------------------- |
| `ci`               | sí             | sí              | `workflow_dispatch` |
| `docs-validate`    | sí             | sí              | `workflow_dispatch` |
| `json-schema`      | sí             | sí              | `workflow_dispatch` |
| `canonicalization` | sí             | sí              | `workflow_dispatch` |
| `release`          | no             | no              | tags `v*`           |
| `plan-release`     | no             | no              | tags de plan        |
| `assign-milestone` | no             | no              | `issues`            |

`pull_request_target`: 0. Un check de `pull_request` no valida `main`: `GITHUB_SHA` puede ser el merge sintético.

Jobs de `ci.yml`: 10 (`rust`, `security-runtime`, `frontend`, `web-wasm-prod`, `docs-python`, `traceability`, `a11y-automated`, `visual-regression`, `markdownlint`, `audit`). El validador de evidencia entra en `docs-python`.

## Hallazgos reproducidos

Clasificación estática y de comportamiento en el árbol de pruebas. No es una certificación de navegador ni de WebView.

| Id   | Clasificación        | Qué quedó demostrado                                                                                                                                                           |
| ---- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| C-01 | confirmado           | `create_draft` pisa un id ya guardado. En wasm el UUID sale de `UUID_COUNTER` en 0. El primer id de contador es `b8178f96-e05d-4c4c-9574-864fca3fdb7d`.                        |
| C-02 | confirmado           | `binaryDigest` es el SHA-256 de `rendered.bytes`. El DTO no tiene `bytes`. `JSON.stringify` de la cadena `report` no es el markdown.                                           |
| C-03 | confirmado en parte  | `draftForCore` no quita hostname ni encrypt. Resolve acepta el draft. El manifiesto no contiene `arch-neubat` ni el step de cifrado. La UI no dice que el disco quedó cifrado. |
| C-04 | confirmado en fuente | `confirmBlockingLosses: true`. El tope `2_000_000` va después de `arrayBuffer()`. No se mira `file.size`.                                                                      |
| C-05 | confirmado en fuente | No hay estados guardando/guardado. `port.saveDraft` aparece 2 veces (arranque e importación). Las ediciones van a IndexedDB.                                                   |
| C-06 | confirmado en fuente | Exportar llama a `downloadArtifact`. No existen `dialog.json` ni `picker.json`. El `catch` de WASM no construye `MockCorePort`.                                                |
| C-07 | confirmado en fuente | La suma de tamaño usa base fija 650. La advertencia Sway/kernel está en `App.tsx` y no en `core.rs`.                                                                           |

Pruebas: `crates/archmaker-core/tests/f0_findings.rs`. Corpus: `crates/archmaker-core/tests/fixtures/closure/` (1 positivo, 1 negativo, 1 binario). Eso no cumple el umbral de 20 casos por target; ese umbral queda propuesto, no medido como cumplido.

## Generados que no entran al commit

`.cursor/hooks/state/` y `target-tools/` son cachés locales no versionadas. F0 no las incluye y no cambia `.gitignore`. El vacío exigible antes de un build de aceptación es el de cambios versionados, no el de esas rutas.

## Umbrales propuestos, no resultados

Quedan `pending`. No se usaron para aprobar nada.

- Identidad: 100 contextos por 100 ids en el lote; 10.000 ids no demuestran unicidad universal.
- Corpus de entrega: 20 casos por target soportado, más 4 rechazos. F0 versiona 3 fixtures de clases.
- Rendimiento, en la máquina que se fije al medir: P95 de inicialización ≤ 3.000 ms; resolver + validar + manifiesto ≤ 1.000 ms; 30 muestras. Aquí no se tomaron muestras.
