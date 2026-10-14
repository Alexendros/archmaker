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
- En MVP solo existe la ventana `main` y su capability (`src-tauri/capabilities/main.json`) con `permissions: []`: esa lista vacía es la denegación definitiva del slice. Los ocho ficheros `src-tauri/permissions/*.toml` definen identificadores `core:*` y no se conceden en el JSON. No hay campo raíz `denied` (el schema Tauri 2 no lo soporta; la ausencia de permiso basta).
- Reserva v1: las ventanas `dialog` y `picker` no tienen ficheros de capability en el árbol MVP; se reintroducirán solo con alcance `accepted` de v1, cada una con allow-list vacía hasta conceder permisos tipados.
- Capabilities separadas por ventana y plataforma.
- Permisos custom con allow/deny explícitos cuando se concedan.
- Scopes de filesystem mínimos; negar rutas sensibles.
- Sin `shell:*` en MVP.
- Updater solo tras firma y ADR de canal/rollback.
- CSP sin CDN en producción; `default-src 'self'`.
- Devtools desactivadas en release salvo build de diagnóstico controlada.
- Navegación externa mediante allowlist y confirmación.
- Toda capability tiene test negativo (estático + `security-runtime`).
