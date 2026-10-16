---
id: DOC-DEL-CLO-F7-001
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
  - id: DOC-DEL-CLO-EVD-001
    expectedDocumentStatus: draft
---

# F7 — Candidato funcional (matriz sobre un RC_SHA)

- Documento: DOC-DEL-CLO-F7-001 · Estado: draft · Propietario: Arquitectura
- Inventario ejecutable: `f7-required-cases.json` (casos **atómicos**)
- Runner / validador: `python3 tools/run_f7_candidate_matrix.py` (VAL-F7-CANDIDATE)
- Manifiesto de evidencia: `result` permanece **`pending`**. `technicalResult` refleja la
  ejecución. F7 **no** declara `RC_SHA_FUNCIONAL` ni firma `reviewer`/`reviewedAt`.

## Qué es un RC_SHA candidato

Cualquier commit de `main` (o el `HEAD` de esta PR antes del merge) sobre el que se reejecuta
la matriz obligatoria. Cambiar el SHA obliga a repetir la matriz. Un harness posterior al
tag `v0.1.0` **no** demuestra el código de `9297ed7…`; registrar por separado
`harness.sourceSha` y `harness.productSourceSha`.

## Matriz obligatoria (atómica)

| Id | Caso | Estado documentado |
| --- | --- | --- |
| F7-MOTOR-REAL | Motor real (forge) | executable en CI |
| F7-MOTOR-REAL-WASM | Motor real WASM E2E-01 | requiere `--with-e2e` + pkg completo |
| F7-GEN-EDIT-EXPORT | Generar/editar/exportar (forge) | executable en CI |
| F7-GEN-EDIT-EXPORT-WASM | E2E-04 WASM | requiere `--with-e2e` |
| F7-DBLCLICK | Reentrada / doble clic | executable |
| F7-BLOCKING-VALIDATION | Validación bloqueante | executable |
| F7-TARGET-INCOMPATIBLE | Target incompatible | executable |
| F7-REG-F0…F5 | Regresión F0–F5 | executable |
| F7-REG-F6 | Mensajes F6 | executable |
| F7-P95 | P95 producto web-ready/web-prepare n=30 | `tools/run_product_p95_harness.py` |
| F7-OFFLINE-WEB | Offline web | requiere `--with-e2e` |
| F7-OFFLINE-DESKTOP | Offline escritorio | **blocked** (P6: portal E2E / offline / 3× install) |
| F7-E2E-X3 | Tres E2E WASM (invocaciones separadas `--workers=1 --retries=0`) | requiere `--with-e2e` |

Un subconjunto de comandos **no** cierra el caso: el runner marca `partial` y `--strict` falla.

## Limitaciones registradas

1. El dueño puede mergear sin checks bloqueantes.
2. Offline **escritorio** y diálogo nativo F5 reales siguen **blocked** (P6: portal E2E /
   offline / 3× install; AppImage local + IPC no bastan).
3. `--strict` es cierre técnico; la aceptación independiente sigue siendo gobierno.

## P95

Presupuesto de **producto** (`honest-ux.md`): ready ≤ 3.000 ms; prepare ≤ 1.000 ms;
n ≥ 30. Harness: `python3 tools/run_product_p95_harness.py` (WASM real, sin mock).
Informe versionable: `docs/10-delivery/closure/product-p95-report.json`. Desktop
queda `blocked` sin evidencia nombrada; eso impide aceptación global de rendimiento.
`--allow-pending` no cierra aceptación.

Microbenchmark nativo: `python3 tools/run_f7_p95_harness.py` (`kind=microbenchmark`;
**no certifica** producto). Sus corridas viven fuera del árbol
(`~/archmaker-closure/evidence/<source-sha>/…`). No inventar cifras sin informe.

## Cómo ejecutar

```bash
python3 tools/run_f7_candidate_matrix.py --validate-only
python3 tools/test_run_f7_candidate_matrix.py

# Evidencia local (no CI): falla ante omitidos/partial/cero tests
python3 tools/run_f7_candidate_matrix.py --execute --with-e2e --strict --out-dir "$EVIDENCE/f7"
```

El manifiesto conserva `result=pending` y rellena `technicalResult`, `harness`, tiempos,
toolchain y digests de fixtures cuando hay `--execute`.

## Fuera de alcance

F8/F9 (canal y release) no sustituyen `RC_SHA_FUNCIONAL`. El tag `v0.1.0` no se mueve.
