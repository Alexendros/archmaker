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

Denominador de `STEP_IDS`: 8. De ellos, `draftForCore` quita solo `archmaker.face.intent` (y opciones gap). Hostname (`archmaker.face.hostname`, texto) y cifrado (`archmaker.face.encrypt`, boolean) siguen en el draft y F3 los preserva en el manifiesto como intención. Matriz: `field-matrix.md`. La cara no afirma cifrado aplicado al disco.

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

| Id   | Clasificación   | Qué quedó demostrado                                                                                                                                                                               |
| ---- | --------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| C-01 | confirmado      | `create_draft` pisa un id ya guardado. En wasm el UUID sale de `UUID_COUNTER` en 0. El primer id de contador es `b8178f96-e05d-4c4c-9574-864fca3fdb7d`.                                            |
| C-02 | confirmado      | `binaryDigest` es el SHA-256 de `rendered.bytes`. El DTO no tiene `bytes`. `JSON.stringify` de la cadena `report` no es el markdown.                                                               |
| C-03 | corregido en F3 | Hostname y cifrado (intención) llegan al manifiesto. number/secretRef emiten `AM-FACE-001`. Exclusiones face visibles. Ver `field-matrix.md` y `f3_fidelity.rs`.                                   |
| C-04 | corregido en F4 | `file.size` antes de leer (`IMPORT_MAX_BYTES = 2_000_000`). Análisis con `confirmBlockingLosses: false`; informe y confirmación antes de aplicar.                                                  |
| C-05 | corregido en F4 | Estados tipados `guardando` / `guardado` / `error`; escrituras serializadas; copia exportable y reintento si el almacén falla.                                                                     |
| C-06 | corregido en F5 | Escritorio: `deliverArtifact` → `save_export_bytes` (diálogo acotado, atómico). Web: descarga. Solo `allow-save-export-bytes`; sin `dialog.json`/`picker.json`/FS global. Fallo IPC/WASM sin mock. |
| C-07 | corregido en F6 | Sin base fija 650 ni aviso Sway/kernel inventado. Suma solo paquetes de catálogo. Ver `honest-ux.md`.                                                                                              |

Pruebas: `crates/archmaker-core/tests/f0_findings.rs`. Corpus: `crates/archmaker-core/tests/fixtures/closure/` (1 positivo, 1 negativo, 1 binario). Eso no cumple el umbral de 20 casos por target; ese umbral queda propuesto, no medido como cumplido.

## Generados que no entran al commit

`.cursor/hooks/state/` y `target-tools/` son cachés locales no versionadas. F0 no las incluye y no cambia `.gitignore`. El vacío exigible antes de un build de aceptación es el de cambios versionados, no el de esas rutas.

## F7 — matriz de candidato

Tras F1–F6 en `main`, la reejecución obligatoria vive en `f7-candidate.md` + `f7-required-cases.json` (`VAL-F7-CANDIDATE`). Inventario versionado de IDs / denominadores / owner / estado: [`mandatory-cases.md`](mandatory-cases.md). El manifiesto de evidencia permanece `result=pending`; no declara `RC_SHA_FUNCIONAL` ni aceptación independiente.

Residuales de ejecución (agente / `--with-e2e`):

| Id         | executed/passed                                      | Sigue pending                                      |
| ---------- | ---------------------------------------------------- | -------------------------------------------------- |
| F7-E2E-X3  | executable (3 invocaciones separadas workers=1 retries=0; no repeat-each) | re-verificar con `--with-e2e`                  |
| F7-OFFLINE | web passed (`wasm-offline`)                          | escritorio **blocked** (P6; portal E2E/offline/3×) |
| F7-REG-F6  | mensajes F6 executable                               | —                                                  |
| F7-P95     | web-ready P95 123 ms; web-prepare P95 58 ms (n=30)   | desktop blocked; aceptación global no cerrada      |

## F8 — gobierno / F9 — publicación

Gobierno: `f8-governance.md` (**accepted**). Capas: baseline de código **passed**;
aprobación funcional **pending** (sin `RC_SHA_FUNCIONAL`); release binaria **published**
(Forge AppImage `v0.1.0`, `RC_SHA` `9297ed7…`) con verificación parcial (sin cosign
local, sin 3× install GUI). Procedimiento y residuales: `f9-release.md`. Sin imagen Arch.

Candidato de cierre (P8, sin publicar): `release-candidate.md` —
`6440b03acf27140c410b77d3f81f43c3d9b21898`. Handoff (P9): `closure-handoff.md`.

## Umbrales propuestos, no resultados

Quedan `pending`. No se usaron para aprobar nada.

- Identidad: 100 contextos por 100 ids en el lote; 10.000 ids no demuestran unicidad universal.
- Corpus de entrega P5: ≥20 fixtures por target en `crates/archmaker-core/tests/fixtures/closure/delivery/` (+ 4 rechazos). F0 sigue versionando 3 fixtures de clases en `closure/`.
- Rendimiento de **producto** (ready ≤ 3.000 ms P95; prepare ≤ 1.000 ms P95; n ≥ 30): web medido (ready P95 123 ms; prepare P95 58 ms) en `product-p95-report.json` vía `tools/run_product_p95_harness.py`. Desktop-ready/prepare **blocked** (evidencia ausente nombrada). Aceptación global de rendimiento **no** cerrada. El microbenchmark nativo `run_f7_p95_harness.py` **no certifica** producto (ver `honest-ux.md`). `--allow-pending` no cierra aceptación.
