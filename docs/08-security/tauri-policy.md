# Política Tauri

- Deny-by-default.
- Capabilities separadas por ventana y plataforma.
- Permisos custom con allow/deny explícitos.
- Scopes de filesystem mínimos; negar rutas sensibles.
- Sin `shell:*` en MVP.
- Updater solo tras firma y ADR de canal/rollback.
- CSP sin CDN en producción; `default-src 'self'`.
- Devtools desactivadas en release salvo build de diagnóstico controlada.
- Navegación externa mediante allowlist y confirmación.
- Toda capability tiene test negativo.
