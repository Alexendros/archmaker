---
id: DOC-OPS-ARCHIFY-001
phase: MVP
priority: P2
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Archify (diagramas) — instalación fijada

**Archify** (`tt-a1i/archify`) es el renderer de diagramas. **Neubat** es el configurador.
No sustituye el análisis del repositorio: el agente escribe las especificaciones; Archify
valida estructura y presenta HTML.

## Pin

```text
ca67c9f16be0c8cad409cd45190aa052e0649b84
```

## Instalar el CLI por SHA

```bash
ARCHIFY_SHA=ca67c9f16be0c8cad409cd45190aa052e0649b84
ARCHIFY_SOURCE="$HOME/.local/share/archify/$ARCHIFY_SHA"
git clone --no-checkout https://github.com/tt-a1i/archify.git "$ARCHIFY_SOURCE"
git -C "$ARCHIFY_SOURCE" checkout --detach "$ARCHIFY_SHA"
export ARCHIFY_UPDATE_CHECK_DISABLED=1
export TMPDIR=/tmp
node "$ARCHIFY_SOURCE/archify/bin/archify.mjs" doctor
```

## Regenerar las 24 vistas

```bash
python3 tools/scaffold_archify_diagrams.py
python3 tools/run_archify_diagrams.py
```

El catálogo exige `sourceSha == HEAD` y `rendererSha` del pin. D25/D26 no se generan.
