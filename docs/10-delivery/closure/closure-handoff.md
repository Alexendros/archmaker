---
id: DOC-DEL-CLO-HAND-001
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
  - id: DOC-DEL-CLO-RC-001
    expectedDocumentStatus: draft
---

# P9 — Informe de handoff (12 secciones)

- Documento: DOC-DEL-CLO-HAND-001 · Estado: draft · Propietario: Release · Fecha: 2026-10-17
- Candidato: [`release-candidate.md`](release-candidate.md) (DOC-DEL-CLO-RC-001).

Mandato de cierre ArchMaker/Forge. Casillas solo con evidencia. **No publicado.**

## 1. Identidad y alcance

- Producto: ArchMaker (motor). GUI: Forge. Fórmula: «Forge, la interfaz gráfica de ArchMaker».
- Alcance: configurador local hasta exportación; sin runner/ISO/root.
- Historia: release `v0.1.0` (nombre histórico) intacta (ADR-0010); nombre visible Forge (ADR-0012).

## 2. SHA de referencia

| Rol | SHA / pin |
| --- | --- |
| Candidato (fuente) | `a09c429988a39c77f56f2795b8e12dfd0051bac8` — ver `release-candidate.md` |
| Renderer Archify | `ca67c9f16be0c8cad409cd45190aa052e0649b84` |
| Artefacto histórico | `9297ed7cc47f42a9611aed0093b5ab040daca512` / digest en `f9-release.md` |

## 3. Capas (passed / blocked / pending)

| Capa | Estado |
| --- | --- |
| Baseline de código | passed (`implementation-baseline-mvp5` + F0–F8 en main) |
| CI / front-matter (P0) | passed (PR #124) |
| Nomenclatura (P1) | passed (PR #125/#126) |
| Integridad evidencia (P2) | passed (PR #127–#129) |
| Perf producto web (P3) | passed (web, PR #131); desktop blocked |
| Archify trace (P4) | passed (PR #133) |
| Aceptación web (P5) | passed (PR #132; gates PR #134) |
| Desktop/offline (P6) | blocked (PR #135 + humo #141) |
| Docs gobernanza (P7) | passed (PR #136) |
| Candidato/handoff (P8–P9) | candidato identificado; **sin** tag ni publicación |
| Aprobación funcional F7 | pending (`RC_SHA_FUNCIONAL`) |
| Distribución nueva | pending |

## 4. Evidencia y adversarios

- Schema `technicalResult` + 20 adversarios + `check_product_closure` (solo lectura).
- Import desktop sin bypass; Mock no cierra.

## 5. Rendimiento

- Web-ready / web-prepare: harness P3 (`run_product_p95_harness.py`); informe en
  `product-p95-report.json`.
- Microbenchmark nativo: no certifica producto.
- Desktop P95: blocked (evidencia ausente nombrada).

## 6. Diagramas

- D01–D24 con AS_BUILT + `--check`; D25/D26 no generados.
- codeRefs verificables; títulos Forge/ArchMaker.

## 7. Aceptación funcional web

- Motor real WASM; corpus ≥20/target; tres invocaciones E2E separadas.
- T-MIG / teclado / stale / instancias sin colisión (P5).

## 8. Escritorio

- ACL/save_export en árbol (F5).
- P6: blocked — evidencia ausente nombrada en `p6-desktop-offline.md`.

## 9. Seguridad y supply chain

- Deny-by-default Tauri; sin shell arbitrario.
- Firma/SBOM del candidato **nuevo**: pending (no reclamados sin bytes).

## 10. Documentación

- README, CHANGELOG Unreleased, AGENTS, plan-status, go-no-go, manual instalación Forge.
- Validadores documentales exigidos en verde en CI.

## 11. Residuales abiertos

| Residual | Owner | Impacto | Condición |
| --- | --- | --- | --- |
| Desktop/offline tip | architecture | Impide aceptación desktop global | Binario prod + portal E2E |
| P95 desktop | architecture | Impide aceptación perf global | GUI + n≥30 |
| `RC_SHA_FUNCIONAL` | architecture | Aprobación funcional | Revisor independiente |
| Firma candidato nuevo | release | Distribución | Sí explícito + bytes |
| Runner/ISO | architecture | Fuera de veredicto | Alcance accepted |

## 12. Decisión

- **No publicar** en este handoff.
- Candidato = `a09c429988a39c77f56f2795b8e12dfd0051bac8` (tip `main` post-P0–P7 + P6 humo #141).
- Distribución y revisión independiente: **pending** hasta autorización y revisor
  distintos del autor.
