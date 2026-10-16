---
id: DOC-DEL-CLO-F5-001
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
  - id: DOC-DEL-CLO-INV-001
    expectedDocumentStatus: draft
---

# F5 — Escritorio (notas de verificación)

## Alcance entregado

- Binario Tauri con `frontendDist: ../apps/web/dist` (frontend real; sin MockRuntime).
- Comando `save_export_bytes`: diálogo nativo (`rfd`), cancelar / sobrescribir / escritura atómica.
- Capability `allow-save-export-bytes` en `main`; sin `dialog.json`, `picker.json`, `dialog:allow-open`, `fs:*` ni shell.
- ACL: `main` autorizado, `restricted` denegado; comando desconocido y transporte sin handler fallan tipados.
- Web: `deliverArtifact` usa descarga; escritorio: guardado nativo. Fallo IPC sin fallback a mock.

## Comandos de prueba

```bash
cargo test -p archmaker --lib
cargo test -p archmaker-core --test negative_capabilities --test f0_findings
python3 tools/check_negative_capabilities.py
pnpm --dir apps/web check
pnpm --dir apps/web exec playwright test export-delivery.playwright.test.ts
cargo build -p archmaker
```

## Humo de binario

En este entorno se pudo compilar `target/debug/archmaker` (ELF) con `save_export_bytes` enlazado y `apps/web/dist/index.html` presente. No se ejecutó GUI interactiva del diálogo (headless / sin sesión de portal para humo E2E de picker).

Bloqueo residual de humo GUI: requiere display/`xdg-desktop-portal` para ejercitar `rfd::FileDialog` de punta a punta; la lógica de cancelar/sobrescribir/atómico queda cubierta por picker simulado en tests unitarios.

### Evidencia GUI real (pendiente)

Para cerrar el residual hay que, en un entorno con display y portal:

1. Abrir el binario / AppImage del SHA bajo prueba.
2. Completar selección → validación → exportación con digest correcto.
3. Diálogo nativo: cancelar, guardar, rechazar sobrescritura, aceptar sobrescritura.
4. Editar, cerrar y reabrir (persistencia).
5. Repetir el recorrido esencial con red deshabilitada (`F7-OFFLINE-DESKTOP`).

Tres aperturas en la misma sesión no cuentan como tres instalaciones independientes (capa F9).

## P6 — Desktop / offline (2026-10-17)

Estado del tip: **blocked**. Ver [`p6-desktop-offline.md`](p6-desktop-offline.md) y
`examples/desktop-evidence.p6-blocked.json`. Display/portal presentes; AppImage Forge
local del tip + launch + IPC ACL en verde; residuales ausentes: diálogo portal E2E,
arranque offline y 3× instalaciones limpias. El AppImage `v0.1.0` histórico y MockCorePort
no cierran P6.

