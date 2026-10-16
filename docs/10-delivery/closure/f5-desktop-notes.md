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
