---
id: DOC-ARCH-DIAG-001
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
---

# Diagramas Archify (Neubat)

Índice de especificaciones tipadas para explicar y verificar el producto **Neubat**.
**Archify** es solo el renderer (skill + CLI); no es el configurador.

## Identidades

| Campo | Uso |
| --- | --- |
| `sourceSha` | Commit de Neubat al que se fijan las vistas |
| `rendererSha` | Commit de Archify (`tt-a1i/archify`) usado para validar/entregar |
| Catálogo | [`catalog.json`](catalog.json) |
| Índice HTML | [`index.md`](index.md) tras `tools/run_archify_diagrams.py` |

## Regeneración

```bash
# Archify fijado (ejemplo)
ARCHIFY_SHA=ca67c9f16be0c8cad409cd45190aa052e0649b84
export ARCHIFY_UPDATE_CHECK_DISABLED=1
# opcional: export ARCHIFY_CLI=$HOME/.local/share/archify/$ARCHIFY_SHA/archify/bin/archify.mjs

python3 tools/scaffold_archify_diagrams.py   # reescribe sources + catalog al HEAD
python3 tools/run_archify_diagrams.py         # validate + deliver 24/24
```

## Reglas

- 24 vistas obligatorias D01–D24; D25 (waterfall) y D26 (ERD) condicionadas.
- As-built: no dibujar crates ni llamadas inexistentes (runner/ISO/Enterprise = futuro).
- Separar `contentDigest`, digest de resolución y `binaryDigest`.
- Distinguir navegador, WebView y proceso Rust; FS vs IndexedDB.
- Los HTML deben abrirse en local sin recursos externos imprescindibles.

## Relación con C4

Las vistas C4 en [`../c4/`](../c4/) siguen siendo la prosa de arquitectura.
Estos diagramas las amplían con artefactos verificables; no sustituyen el análisis del repositorio.
