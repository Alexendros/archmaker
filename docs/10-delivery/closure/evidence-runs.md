---
id: DOC-DEL-CLO-EVD-RUNS-001
phase: MVP
priority: P2
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Corridas de evidencia (índice)

Las corridas reales viven **fuera del árbol** bajo `~/archmaker-closure/`. Este
documento solo apunta identidades; no declara `RC_SHA_FUNCIONAL`.

## Histórica `v0.1.0`

| Campo | Valor |
| --- | --- |
| Tag | `v0.1.0` (inamovible) |
| `RC_SHA` | `9297ed7cc47f42a9611aed0093b5ab040daca512` |
| Artefacto | `Neubat_0.1.0_amd64.AppImage` |
| `ARTIFACT_SHA256` | `6567464f5e2f21398fd3eef35592f0e9d377fd2cc7887c1e46df092b5a8e6d81` |
| Checksum local | **verified** (`sha256sum --check`) |
| cosign | **deferred** — la release pública no lista `.sig`/`.pem` |
| install_gui ×3 | **not-verified** |
| Carpeta | `~/archmaker-closure/releases/v0.1.0/` |

No atribuye aceptación al tip de `main` posterior.

## Candidato / harness (post-c90919c)

| Campo | Nota |
| --- | --- |
| P95 producto n=30 | `docs/10-delivery/closure/product-p95-report.json` (web); microbenchmark en `~/archmaker-closure/evidence/<sha>/p95-*/p95-report.json` |
| Matriz `--strict` | Pendiente de reejecución sobre el SHA **final** del PR (tras merge) |
| GUI / offline desktop | Compuerta del operador; `F7-OFFLINE-DESKTOP` sigue pending |

Tras merge: regenerar diagramas Archify (`sourceSha == HEAD`) y repetir
`--execute --with-e2e --strict` antes de cualquier aprobación independiente.
