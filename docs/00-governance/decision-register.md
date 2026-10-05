# Registro de decisiones

| ID | Pregunta | Opciones | Recomendación | Propietario | Impacto | Estado |
|---|---|---|---|---|---|---|
| DEC-001 | ¿Qué exporta MVP? | Perfil ArchMaker; perfil archinstall; ISO | Perfil ArchMaker + reporte; exporter archinstall experimental | Product owner | P0 | decision-required |
| DEC-002 | ¿Ámbito de IDs? | Global; catálogo; sección | Global con namespace `vendor.kind.id` | Arquitectura | P0 | decision-required |
| DEC-003 | ¿Motor v1? | archinstall; libalpm propio; scripts | Adapter versionado de archinstall, sin acoplar dominio | Arquitectura | P0 | decision-required |
| DEC-004 | ¿Transporte runner? | Unix socket; stdio sidecar; D-Bus | Unix socket + auth de sesión; elevar fuera de WebView | Seguridad | P0 | decision-required |
| DEC-005 | ¿Licencia? | Apache-2.0; MIT/Apache; GPLv3 | Apache-2.0 o dual MIT/Apache tras revisión | Propietario | P0 | decision-required |
| DEC-006 | ¿Firma de catálogos? | Minisign; Sigstore; ambos | Sigstore para releases + firma offline evaluada | Seguridad | P1 | proposed |
| DEC-007 | ¿Actualización? | Manual; Tauri updater; repos distro | Manual/repo en MVP, updater firmado tras threat model | Release | P1 | proposed |
| DEC-008 | ¿Soporte inicial? | Arch; Arch-based; live ISO propia | Arch x86_64; aarch64 experimental separado | Producto | P0 | decision-required |
| DEC-009 | ¿Fuentes? | CDN; empaquetadas; system | Empaquetadas o system stack; sin CDN runtime | Diseño/Legal | P1 | proposed |
| DEC-010 | ¿Telemetría? | Ninguna; opt-in; enterprise | Ninguna en MVP; opt-in futuro con ADR | Producto | P1 | proposed |
