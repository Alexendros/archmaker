---
id: DOC-DEL-CLO-FORGE-DESK-001
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: blocked
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-CLO-P6-001
    expectedDocumentStatus: draft
  - id: DOC-DEL-CLO-FORGE-ACC-001
    expectedDocumentStatus: draft
---

# Cierre desktop Forge (candidato tip; sin publicar)

## Dictamen

| Eje | Estado | Notas |
| --- | ------ | ----- |
| `publicationStatus` | **not-published** | No tags, no assets en GitHub Releases nuevos |
| AppImage tip | **blocked** | Existe `Forge_0.1.0_amd64.AppImage` local, pero `sourceSha` de evidencia P6 (`ccab5229…`) ≠ tip Forge |
| Gate P6 desktop/offline | **blocked** | Residual portal E2E rfd; ver `p6-desktop-offline.md` |
| DPI 125/150/200 | **pending** | Sin medición en este tip |
| Tres instalaciones limpias tip | **pending** | Probe P6 cubrió un tip anterior; no reejecutado sobre tip Forge |
| Firma / cosign | **pending** | Sin autorización de firma del candidato nuevo |
| A6 escritura discos | **N/A** | Fuera de alcance (HA26–28) |
| Detección/plan Tauri | **partial** | Código en tip (`archmaker-hardware` + IPC); recorrido GUI tip no cerrado en evidencia |

## Tip de referencia

Al redactar: ver `git rev-parse origin/main` (post docs #159 / aceptación #158).

Condición para pasar de `blocked` → evaluación tip:

1. `pnpm tauri build --bundles appimage` sobre el tip (sin publicar).
2. Recorrido Tauri: título ArchMaker Forge, maximizar/restaurar, Detectar → plan, portal guardado, cero escrituras de disco.
3. Actualizar evidencia P6 con `sourceSha` = tip y residuales portal/DPI nombrados.

## Relación con aceptación

- Web / T·HA: `forge-acceptance-matrix.md` (técnico; sin autoaprobación).
- Desktop no se declara passed por CI web ni por AppImage histórico `v0.1.0`.

Owner: architecture. **No publicar.**
