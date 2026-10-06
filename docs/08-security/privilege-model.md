---
id: DOC-SEC-PRIV-001
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

# Modelo de privilegios

## MVP

- Web/desktop: usuario normal.
- Core: computación pura.
- Filesystem: solo ficheros seleccionados o directorios de aplicación.
- Red: no necesaria para configurar con catálogo embebido.
- Shell/sidecars/root/discos: prohibidos.

## v1

| Componente | Privilegio | Puede | No puede |
|---|---|---|---|
| WebView | Usuario | Presentar y solicitar | Abrir discos/root/socket libremente |
| Tauri host | Usuario | Validar, serializar, autenticar sesión | Ejecutar instalación |
| Runner | Elevado temporal | Preflight/operaciones allowlisted | Interpretar scripts/catálogo |
| Adapter | Dentro runner | Traducir plan a API fijada | Modificar el plan confirmado |

## Reglas

- La elevación ocurre fuera del contenido WebView.
- El runner autentica peer y sesión.
- Confirmaciones destructivas expiran y se ligan a hashes.
- Cambios del inventario invalidan el plan.
- Los secretos se suministran just-in-time y se borran de memoria cuando sea posible.

## Sign-off

- Estado del modelo: revisado 2026-10-05. Incorporadas DEC-004 (transporte Unix socket + autenticación de sesión, elevación fuera del WebView) y DEC-007 (updater firmado en MVP, fail-open offline, ADR-0009).
- MVP: sin shell/sidecars/root/discos; red solo para comprobación de actualizaciones firmada y opcional.
- Sign-off de Seguridad **aprobado formalmente** el 2026-10-05 (revisión técnica + ratificación del propietario del producto).
- Residual: verificación de implementación en MVP/v1; G8 queda condicionado a evidencia ejecutable (pruebas negativas de capabilities y de elevación).
