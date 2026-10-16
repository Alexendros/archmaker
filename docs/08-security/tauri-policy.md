---
id: DOC-SEC-TAURI-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
---

# Política Tauri

- Deny-by-default.
- Los comandos propios se registran en `src-tauri/build.rs` vía `AppManifest::commands` (Tauri 2). Sin ese registro, la ACL de aplicación no restringe la superficie de invoke.
- La ventana `main` (`capabilities/main.json`) concede los `allow-*` de CorePort (nueve operaciones, incluido `open_draft`) más `allow-save-export-bytes` (guardado nativo acotado F5).
- El comando `save_export_bytes` abre un diálogo de guardado y escribe solo la ruta elegida (atómico); no concede `dialog:allow-open`, `dialog:default`, `fs:*` ni shell.
- `capabilities/restricted.json` tiene `permissions: []` para pruebas ACL negativas (ventana `restricted`).
- `check_invoke` en Rust es una segunda capa (matriz ventana × operación) tras la ACL de Tauri; `save_export_bytes` solo desde ventana `main`.
- No hay ficheros `dialog.json` / `picker.json`: el guardado acotado no es una ventana dialog con permisos libres.
- Capabilities separadas por ventana y plataforma.
- Permisos custom con allow/deny explícitos cuando se concedan.
- Scopes de filesystem mínimos; negar rutas sensibles.
- Sin `shell:*` en MVP.
- Updater solo tras firma y ADR de canal/rollback.
- CSP sin CDN en producción; `default-src 'self'`.
- Devtools desactivadas en release salvo build de diagnóstico controlada.
- Navegación externa mediante allowlist y confirmación.
- Toda capability tiene test negativo (estático + `security-runtime`).
