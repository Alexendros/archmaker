---
id: DOC-SEC-TAURI-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
---

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
