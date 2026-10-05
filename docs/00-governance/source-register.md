# Registro de fuentes

ID `DOC-GOV-SRC-001` · Estado `in-review` · Propietario Gobierno · Última revisión 2026-10-05.

Toda afirmación mutable (paquetes, versiones, compatibilidad, comportamiento de estándar) debe apoyarse en una fuente de este registro con URL, versión objetivo, fecha de captura, verificador y caducidad. Las referencias históricas (`reference/v5.1/`) no se convierten en verdad de producto sin verificación.

## Fuentes registradas

| ID | Fuente | Estado | Autoridad | Uso |
|---|---|---|---|---|
| SRC-001 | `reference/v5.1/*` | received-partial | Histórica | Migraciones, requisitos y comparación visual. Nunca verdad de producto sin verificación. |
| SRC-002 | ArchWiki — Installation guide | verified | Primaria comunitaria | Instalación, paquetes y compatibilidad Arch. |
| SRC-003 | archinstall oficial (repositorio) | verified | Primaria | Adapter v1 y profile schema. |
| SRC-004 | Tauri 2 docs | verified | Primaria | Capabilities, permissions, scopes, CSP y updater. |
| SRC-005 | Rust docs / RustSec Advisory DB | pending-capture | Primaria | Core y supply chain. |
| SRC-006 | JSON Schema 2020-12 | verified | Estándar | Contratos. |
| SRC-007 | WCAG 2.2 / WAI-ARIA APG | partial | Estándar | Accesibilidad (WCAG 2.2 capturada; APG pendiente). |
| SRC-008 | NIST SSDF / SLSA / Sigstore | partial | Institucional | SDLC, provenance y firma (SLSA capturada; SSDF y Sigstore pendientes). |

## Detalle de captura

| ID | URL canónica | Versión objetivo | Fecha de captura | Verificador | Caducidad / revisión |
|---|---|---|---|---|---|
| SRC-002 | https://wiki.archlinux.org/title/Installation_guide | Contenido wiki vigente a la fecha | 2026-10-05 | Gobierno | Revisar por release de soporte o si cambia el flujo de instalación |
| SRC-003 | https://github.com/archlinux/archinstall | Repositorio oficial vigente a la fecha | 2026-10-05 | Gobierno | Revisar antes de fijar el adapter v1 (DEC-003) |
| SRC-004 | https://v2.tauri.app/ | Tauri 2.x | 2026-10-05 (previa) | Seguridad | Revisar por minor; fijar permisos por ventana |
| SRC-005 | https://docs.rs/ · https://rustsec.org/ | Rust estable + advisory DB | pendiente-captura | Seguridad | Capturar antes de G9 |
| SRC-006 | https://json-schema.org/draft/2020-12/release-notes | Draft 2020-12 | 2026-10-05 | Arquitectura | Estable; revisar notas de errata |
| SRC-007 | https://www.w3.org/TR/WCAG22/ | **W3C Recommendation 12 December 2024** | 2026-10-05 | UX/Accesibilidad | Estable; APG (`https://www.w3.org/WAI/ARIA/apg/`) pendiente-captura |
| SRC-008 | https://slsa.dev/spec/v1.2/ · https://csrc.nist.gov/pubs/sp/800/218/final · https://docs.sigstore.dev/ | **SLSA v1.2** (v1.0 retirada) | 2026-10-05 (SLSA) | Seguridad/Release | SSDF y Sigstore pendiente-captura; fijar versión antes de G9 |

## Notas de verificación

- **SRC-008 (SLSA):** la especificación `v1.0` está **retired**; la versión actual es **v1.2** (`https://slsa.dev/spec/v1.2/`). Toda referencia a SLSA en la documentación debe apuntar a v1.2, no a v1.0.
- **SRC-007 (WCAG 2.2):** capturada como W3C Recommendation de 2024-12-12; la matriz de accesibilidad (NFR-ACC-001) se ancla a esta versión.
- **SRC-002/SRC-003:** capturadas para desbloquear las afirmaciones de paquetes/instalación marcadas «no verificado — fuente primaria pendiente»; siguen pendientes de conversión a datos concretos de catálogo (ver `docs/12-research/inventory-v5.1.md`).

## Regla

Toda afirmación mutable de catálogo debe almacenar URL, fecha de captura, versión objetivo, verificador y fecha de caducidad. Las referencias históricas no se convierten en verdad de producto sin verificación.
