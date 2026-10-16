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
  - id: DOC-DEL-CLO-P6-001
    expectedDocumentStatus: draft
---

# P8 — Candidato de cierre (sin publicar)

- Documento: DOC-DEL-CLO-RC-001 · Estado: draft · Propietario: Release · Fecha: 2026-10-17
- Handoff: [`closure-handoff.md`](closure-handoff.md) (DOC-DEL-CLO-HAND-001).

## Definición

El **candidato** de este cierre ArchMaker/Forge es el tip de `main` **después** de
fusionar P0–P7 (CI front-matter, naming, evidencia, perf web, Archify, aceptación web,
desktop blocked, docs). No se crea tag nuevo. No se mueve `v0.1.0`. No se sustituyen
assets de la release histórica.

## SHA registrados

| Rol | Valor | Notas |
| --- | --- | --- |
| Fuente (candidato) | `6440b03acf27140c410b77d3f81f43c3d9b21898` | `origin/main` post-P7 (merge PR #136); tip al fijar este documento |
| Harness | `6440b03acf27140c410b77d3f81f43c3d9b21898` | Coincide con fuente (herramientas en el mismo tip) |
| Renderer Archify | `ca67c9f16be0c8cad409cd45190aa052e0649b84` | Pin; no actualizar sin revalidar |
| Artefacto publicado | `9297ed7cc47f42a9611aed0093b5ab040daca512` | Histórico `v0.1.0` (ADR-0010); **no** es este candidato |

Digest AppImage histórico: ver [`f9-release.md`](f9-release.md). Este documento **no**
reclama digest, SBOM ni firma del tip candidato: no se produjeron bytes de canal nuevo.

## Qué queda pending / blocked

| Ítem | Estado | Condición de cierre |
| --- | --- | --- |
| Firma / SBOM del candidato nuevo | pending | Bytes producidos + sí explícito de publicación |
| Revisión independiente | pending | Revisor distinto del autor |
| Distribución (tag/channel nuevo) | pending | Autorización explícita |
| Desktop/offline del tip | blocked | Binario prod del tip + portal E2E ([`p6-desktop-offline.md`](p6-desktop-offline.md)) |
| P95 desktop | blocked | GUI + medición n≥30 |
| Runner / ISO | out of scope | Alcance `accepted` adicional |

## Prohibiciones

- Publicar, firmar o anunciar el candidato como release.
- Usar MockCorePort o AppImage `v0.1.0` como evidencia del tip.
- Declarar `RC_SHA_FUNCIONAL` sin aceptación independiente.
