---
id: DOC-DEL-CLO-RC-001
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-CLO-F8-001
    expectedDocumentStatus: accepted
  - id: DOC-DEL-CLO-F9-001
    expectedDocumentStatus: in-review
---

# P8 — Candidato de cierre (sin publicar)

## Definición

El **candidato** de este cierre ArchMaker/Forge es el tip de `main` **después** de
fusionar P0–P7 (CI front-matter, naming, evidencia, perf web, Archify, aceptación web,
desktop blocked, docs). No se crea tag nuevo. No se mueve `v0.1.0`. No se sustituyen
assets de la release histórica.

## SHA separados (plantilla)

| Rol | Valor | Notas |
| --- | --- | --- |
| Fuente (candidato) | tip de `main` post-P7 | Registrar con `git rev-parse origin/main` al cerrar |
| Harness | tip que contiene `tools/run_*` usados | Puede coincidir con fuente |
| Renderer Archify | `ca67c9f16be0c8cad409cd45190aa052e0649b84` | Pin; no actualizar sin revalidar |
| Artefacto publicado | `v0.1.0` / `9297ed7…` | Histórico Neubat; **no** es este candidato |

## Qué queda pending / blocked

| Ítem | Estado | Condición de cierre |
| --- | --- | --- |
| Firma / SBOM del candidato nuevo | pending | Bytes producidos + sí explícito de publicación |
| Revisión independiente | pending | Revisor distinto del autor |
| Distribución (tag/channel nuevo) | pending | Autorización explícita |
| Desktop/offline del tip | blocked | Binario prod del tip + portal E2E (P6) |
| P95 desktop | blocked | GUI + medición n≥30 |
| Runner / ISO | out of scope | Alcance `accepted` adicional |

## Prohibiciones

- Publicar, firmar o anunciar el candidato como release.
- Usar MockCorePort o AppImage `v0.1.0` como evidencia del tip.
- Declarar `RC_SHA_FUNCIONAL` sin aceptación independiente.
