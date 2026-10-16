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
- La ventana `main` (`capabilities/main.json`) concede únicamente los `allow-*` de CorePort (nueve operaciones, incluido `open_draft`).
- `capabilities/restricted.json` tiene `permissions: []` para pruebas ACL negativas (ventana `restricted`).
- `check_invoke` en Rust es una segunda capa (matriz ventana × operación) tras la ACL de Tauri.
- Reserva v1: las ventanas `dialog` y `picker` no tienen ficheros de capability; se reintroducirán solo con alcance `accepted` de v1.
- Capabilities separadas por ventana y plataforma.
- Permisos custom con allow/deny explícitos cuando se concedan.
- Scopes de filesystem mínimos; negar rutas sensibles.
- Sin `shell:*` en MVP.
- Updater solo tras firma y ADR de canal/rollback.
- CSP sin CDN en producción; `default-src 'self'`.
- Devtools desactivadas en release salvo build de diagnóstico controlada.
- Navegación externa mediante allowlist y confirmación.
- Toda capability tiene test negativo (estático + `security-runtime`).
