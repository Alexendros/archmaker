---
id: DOC-DEL-CLO-EVD-001
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

# Contrato de evidencia F0

- Documento: DOC-DEL-CLO-EVD-001 · Estado: draft · Propietario: Arquitectura
- Esquema: `docs/10-delivery/closure/evidence-manifest.schema.json`
- Validador: `python3 tools/validate_closure_evidence.py` (VAL-CLOSURE-EVIDENCE)
- Corridas reales, cuando existan, viven fuera del árbol en `closure-evidence/<source-sha>/<run-id>/`. Este documento no es una corrida.

## Manifiesto

Campos obligatorios: `schemaVersion`, `repository`, `phase`, `sourceSha`, `baseSha`, `eventSha`, `prHeadSha`, `workflowSha`, `mergeSha`, `workflowRunId`, `runAttempt`, `eventName`, `startedAt`, `completedAt`, `toolchain`, `platform`, `dirtyTrackedFiles`, `fixtureSetDigest`, `requiredTestIds`, `executed`, `passed`, `failed`, `skipped`, `retries`, `artifacts`, `result`, `reviewer`, `reviewedAt`, `residuals`.

Campos opcionales (F7 endurecido): `harness` (`sourceSha` del harness, `productSourceSha`, `runner`), `technicalResult` (`pending`/`running`/`passed`/`failed`/`partial`/`blocked` o `null`), `commands`, `tests`, `approval`.

En `approval` se separan identidades:

| Campo | Significado |
| --- | --- |
| `approvalRecordSha256` | SHA-256 del fichero de registro de aprobación |
| `approvalRecordCommit` | Commit git del registro si está versionado |

No usar un solo nombre (`APPROVAL_RECORD_SHA`) para ambas.

Cada artefacto lleva `name`, `sizeBytes`, `sha256`, `mediaType` y `buildSourceSha`.

Lo desconocido es `null`. `result` empieza en `pending` (aprobación). `technicalResult` describe la ejecución; `null` está en el enum (o la propiedad se omite en manifiestos antiguos). Estados de `result`: `proposed`, `pending`, `running`, `passed`, `failed`, `blocked`, `not-applicable`.

El manifiesto no incluye su propio digest. Una propiedad `sha256`, `digest`, `evidenceSha256`, `manifestDigest` o `selfDigest` es inválida. `expected` no es un campo: es `len(requiredTestIds)` y tiene que ser mayor que 0.

Aprobación del conjunto: `result` es `passed`, **`technicalResult` es `passed` (obligatorio; `null` u omitido no aprueban)**, hay `sourceSha`, `reviewer` y `reviewedAt`, `executed == expected`, `passed == expected`, `failed == 0`, `skipped == 0` y `retries == 0`. Si existe `approval`, exige `status=passed` y `approvalRecordSha256`. Exit code 0 con cero tests no cuenta. Un fichero bajo `examples/` o cuyo nombre contiene `example` no es evidencia, aunque el contenido cuadre. Los manifiestos antiguos sin `technicalResult` se leen (schema OK) pero no se aprueban. Evidencia desktop: `desktop-evidence.schema.json` + import por `--desktop-evidence` (sin bypass).

## Corpus

Tres fixtures versionados, con digest en `crates/archmaker-core/tests/fixtures/closure/fixture-digests.json`. Ese índice no guarda el digest de sí mismo.

| Fichero                    | Clase    |
| -------------------------- | -------- |
| `positive-profile.json`    | positive |
| `negative-incomplete.json` | negative |
| `binary-payload.bin`       | binary   |

## Toolchain medida en la autoría

Medido en el host que escribió F0 el 2026-10-16. No es el ejecutor de CI.

| Herramienta | Versión                       |
| ----------- | ----------------------------- |
| rustc       | 1.90.0 (1159e78c4 2025-09-14) |
| cargo       | 1.90.0 (840b83a10 2025-07-30) |
| node        | v22.22.1                      |
| pnpm        | 11.21.0                       |

`rust-toolchain.toml` pide el canal `1.90`. El SO de autoría es Ubuntu 26.04.1 LTS, Linux 7.0.0-38-generic x86_64. CPU: AMD Ryzen AI 7 350 w/ Radeon 860M, 16 CPUs lógicas. `MemTotal` leído de `/proc/meminfo`: 31421384 kB.

No se midió navegador ni WebView en esta autoría.

## Lo que no se midió en CI

El workflow `ci` pide Node `20` y `ubuntu-latest`. No se midieron aquí la versión parche de ese Node, el navegador, el WebView, la CPU ni la memoria del ejecutor. No hay cifras de esas filas.
