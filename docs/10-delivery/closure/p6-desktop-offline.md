---
id: DOC-DEL-CLO-P6-001
phase: MVP
priority: P1
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
  - id: DOC-DEL-CLO-F5-001
    expectedDocumentStatus: draft
  - id: DOC-DEL-CLO-F7-001
    expectedDocumentStatus: draft
---

# P6 — Desktop / offline (gate)

## Resultado

**`blocked`** (no `pending` ambiguo; no `passed`). Hay sesión gráfica (`DISPLAY=:0`,
`WAYLAND_DISPLAY=wayland-0`), `xdg-desktop-portal` activo (FileChooser v4) y humo
local del AppImage Forge del tip (`Forge_0.1.0_amd64.AppImage`, no publicado). El
AppImage histórico `v0.1.0` (nombre público retirado) **no** cierra este gate. Una
captura con MockCorePort **no** cierra este gate. La aceptación web **no** se
presenta como aceptación desktop. Identidad técnica intacta (`dev.archmaker.app`).

## Casos (2026-10-17, `SOURCE_SHA` af31f8b2…)

| Id | Estado | Evidencia |
| --- | --- | --- |
| env-display | passed | `DISPLAY=:0` / `WAYLAND_DISPLAY=wayland-0` |
| env-portal | passed | `xdg-desktop-portal` + FileChooser v4 |
| appimage-build | passed | Forge AppImage local sha256 `a064aee2…` (no published) |
| appimage-launch | passed | proceso vivo ≥5s con display |
| ipc-allowed | passed | `cargo test -p archmaker --lib` (gate main) |
| ipc-denied | passed | restricted / unknown cmd denegados |
| gui-dialog-real | blocked | falta E2E rfd vía portal |
| gui-cancel | blocked | falta cancelación nativa |
| gui-overwrite | blocked | falta sobrescritura nativa |
| gui-destination-noperms | blocked | falta destino sin permisos |
| boot-offline | blocked | falta arranque con red aislada (`unshare -n` no disponible) |
| clean-install-x3 | blocked | faltan 3 instalaciones limpias independientes |

## Evidencia ausente nombrada

1. E2E portal real: diálogo nativo cancelar / guardar / sobrescribir / destino sin permisos.
2. Arranque sin red (`F7-OFFLINE-DESKTOP`) sobre el AppImage del tip (netns / host offline).
3. Tres instalaciones limpias independientes (no tres aperturas de la misma sesión).

Registro versionado: [`examples/desktop-evidence.p6-blocked.json`](examples/desktop-evidence.p6-blocked.json)
(probe: [`examples/desktop-offline-p6-probe.json`](examples/desktop-offline-p6-probe.json)).
Verificador: `python3 tools/check_desktop_offline_gate.py`.

## Condición de cierre

Completar los tres residuales con evidencia GUI/offline del operador, importar
`status=passed` vía `--desktop-evidence` (schema + hash + identidad; sin bypass) y
actualizar este documento. Owner: architecture. No publicar release ni mover tags.
