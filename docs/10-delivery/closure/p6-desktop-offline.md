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

**`blocked`** (no `pending` ambiguo). Hay sesión gráfica y `xdg-desktop-portal`, pero
**no** hay binario de producción del `SOURCE_SHA` del tip bajo prueba. El AppImage
histórico `v0.1.0` (nombre público retirado) **no** cierra este gate. Una captura con MockCorePort
**no** cierra este gate. La aceptación web **no** se presenta como aceptación desktop.

## Evidencia ausente nombrada

1. AppImage o binario de release del `SOURCE_SHA` candidato (tip de `main` tras P0–P5).
2. E2E portal real: diálogo nativo cancelar / guardar / sobrescribir / destino sin permisos.
3. IPC permitido (`main`) y denegado (`restricted`) sobre ese binario.
4. Arranque sin red (`F7-OFFLINE-DESKTOP`) sobre ese binario.
5. Tres instalaciones limpias independientes (no tres aperturas de la misma sesión).

Registro versionado: [`examples/desktop-evidence.p6-blocked.json`](examples/desktop-evidence.p6-blocked.json)
(probe: [`examples/desktop-offline-p6-probe.json`](examples/desktop-offline-p6-probe.json)).

## Condición de cierre

Re-ejecutar el probe con un artefacto de producción del SHA, importar evidencia
`status=passed` vía `--desktop-evidence` (schema + hash + identidad; sin bypass) y
actualizar este documento. Owner: architecture.
