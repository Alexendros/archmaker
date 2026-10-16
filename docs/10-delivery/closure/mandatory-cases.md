---
id: DOC-DEL-CLO-MAND-001
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

# Inventario versionado — casos obligatorios (P0)

- Documento: DOC-DEL-CLO-MAND-001 · Estado: draft · Propietario: Arquitectura
- Fuente: [`inventory.md`](inventory.md) (DOC-DEL-CLO-INV-001). Matriz ejecutable F7:
  [`f7-required-cases.json`](f7-required-cases.json).
- Este inventario **no** inventa resultados `passed`. Los estados reflejan lo ya
  documentado en F0/F7; la aceptación firmada sigue fuera de alcance.

## Denominadores (de `inventory.md`)

| Id | Denominador | Valor | Owner | Estado |
| --- | --- | --- | --- | --- |
| DEN-EXPORT | ids en `EXPORT_TARGETS` con renderer/bytes | 3 | architecture | documentado |
| DEN-RAIL | pasos visibles `RAIL_STEPS` | 6/6 | architecture | documentado |
| DEN-STEP-IDS | `STEP_IDS` en cara | 8 | architecture | documentado |
| DEN-OPTION | campos `optionId` (con gap / sin gap) | 38 (12 gap, 26 no) | architecture | documentado |
| DEN-WF | workflows en `.github/workflows` | 7 | architecture | documentado |
| DEN-CI-JOBS | jobs de `ci.yml` | 10 | architecture | documentado |

## Hallazgos F0 (C-01…C-07)

Clasificación estática del inventario; no es certificación de navegador ni WebView.

| Id | Owner | Estado (inventario) |
| --- | --- | --- |
| C-01 | architecture | confirmado |
| C-02 | architecture | confirmado |
| C-03 | architecture | corregido en F3 |
| C-04 | architecture | corregido en F4 |
| C-05 | architecture | corregido en F4 |
| C-06 | architecture | corregido en F5 |
| C-07 | architecture | corregido en F6 |

Corpus versionado: 3 fixtures en `crates/archmaker-core/tests/fixtures/closure/`. El umbral
propuesto de 20 casos por target **no** se marca cumplido.

## Matriz F7 — casos obligatorios

Denominador: `len(cases)` en `f7-required-cases.json`. Estados según inventario F7 /
`defaultStatus` del JSON; **sin** declarar `RC_SHA_FUNCIONAL` ni aceptación independiente.

| Id | Owner | Estado documentado |
| --- | --- | --- |
| F7-MOTOR-REAL | architecture | executable (CI) |
| F7-MOTOR-REAL-WASM | architecture | executable (`--with-e2e`) |
| F7-GEN-EDIT-EXPORT | architecture | executable (CI) |
| F7-GEN-EDIT-EXPORT-WASM | architecture | executable (`--with-e2e`) |
| F7-DBLCLICK | architecture | executable (CI) |
| F7-BLOCKING-VALIDATION | architecture | executable (CI) |
| F7-TARGET-INCOMPATIBLE | architecture | executable (CI) |
| F7-REG-F0 | architecture | executable (CI) |
| F7-REG-F1 | architecture | executable (CI) |
| F7-REG-F2 | architecture | executable (CI) |
| F7-REG-F3 | architecture | executable (CI) |
| F7-REG-F4 | architecture | executable (CI) |
| F7-REG-F5 | architecture | executable (CI) |
| F7-REG-F6 | architecture | executable (CI); P95 n=30 sin muestras |
| F7-P95 | architecture | pending de medición de producto; harness nativo no certifica producto |
| F7-OFFLINE-WEB | architecture | executable (`--with-e2e`); no implica escritorio |
| F7-OFFLINE-DESKTOP | architecture | pending (humo GUI/red) |
| F7-E2E-X3 | architecture | executable (`--with-e2e`) |

Residuales ya observados en inventario (no cierran aceptación):

| Id | executed/passed (inventario) | Sigue pending |
| --- | --- | --- |
| F7-E2E-X3 | passed (3 intentos wasm-flow) | — |
| F7-OFFLINE | web passed (`wasm-offline`) | escritorio sin humo de red/portal |
| F7-REG-F6 | mensajes F6 executable | P95 n=30 (sin muestras; no se fabrican cifras) |

## Umbrales propuestos (no resultados)

Quedan `pending`. No se usaron para aprobar nada. Detalle de producto vs harness nativo:
[`honest-ux.md`](honest-ux.md).

- Corpus de entrega: 20 casos por target soportado + 4 rechazos (propuesto).
- Rendimiento de **producto**: ready ≤ 3.000 ms P95; prepare ≤ 1.000 ms P95; n ≥ 30;
  máquina de referencia **no fijada**.
