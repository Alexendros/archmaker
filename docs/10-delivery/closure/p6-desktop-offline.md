---
id: DOC-DEL-CLO-P6-001
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
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

Residuales automatizables **cerrados** (2026-10-17, probe
`tools/run_p6_residual_probe.py`): `clean-install-x3` y `boot-offline` (bwrap
`--unshare-net` sobre AppRun extraído; `/tmp` del host es `noexec`). El gate
global sigue **blocked** por E2E portal rfd.

## Casos (2026-10-17, `SOURCE_SHA` ccab5229…)

| Id | Estado | Evidencia |
| --- | --- | --- |
| env-display | passed | `DISPLAY=:0` / `WAYLAND_DISPLAY=wayland-0` |
| env-portal | passed | `xdg-desktop-portal` + FileChooser v4 |
| appimage-build | passed | Forge AppImage local sha256 `a064aee2…` (no published) |
| appimage-launch | passed | proceso vivo ≥5s con display |
| ipc-allowed | passed | `cargo test -p archmaker --lib` (gate main) |
| ipc-denied | passed | restricted / unknown cmd denegados |
| clean-install-x3 | passed | 3 HOME independientes; FUSE AppImage; timeout 124 |
| boot-offline | passed | extract + `bwrap --unshare-net` AppRun; timeout 124 |
| gui-dialog-real | blocked | falta E2E rfd vía portal (no se sustituye por picker) |
| gui-cancel | blocked | falta cancelación nativa |
| gui-overwrite | blocked | falta sobrescritura nativa |
| gui-destination-noperms | blocked | falta destino sin permisos |

## Evidencia ausente nombrada (bloquea `passed`)

1. E2E portal real: diálogo nativo cancelar / guardar / sobrescribir / destino sin permisos
   sobre el AppImage del tip (operador + display/portal).

Registro versionado: [`examples/desktop-evidence.p6-blocked.json`](examples/desktop-evidence.p6-blocked.json)
(probe: [`examples/desktop-offline-p6-probe.json`](examples/desktop-offline-p6-probe.json)).
Verificador: `python3 tools/check_desktop_offline_gate.py`.
Probe: `python3 tools/run_p6_residual_probe.py` (requiere AppImage local + display).

## Condición de cierre

Completar el residual portal E2E con evidencia GUI del operador, importar
`status=passed` vía `--desktop-evidence` (schema + hash + identidad; sin bypass) y
actualizar este documento. Owner: architecture. No publicar release ni mover tags.
