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
| SRC-005 | Rust docs / RustSec Advisory DB | verified | Primaria | Core y supply chain. |
| SRC-006 | JSON Schema 2020-12 | verified | Estándar | Contratos. |
| SRC-007 | WCAG 2.2 / WAI-ARIA APG | verified | Estándar | Accesibilidad (WCAG 2.2 y ARIA APG capturadas). |
| SRC-008 | NIST SSDF / SLSA / Sigstore | verified | Institucional | SDLC, provenance y firma (SLSA v1.2, SSDF v1.1 y Sigstore capturadas). |

## Detalle de captura

| ID | URL canónica | Versión objetivo | Fecha de captura | Verificador | Caducidad / revisión |
|---|---|---|---|---|---|
| SRC-002 | <https://wiki.archlinux.org/title/Installation_guide> | Contenido wiki vigente a la fecha | 2026-10-05 | Gobierno | Revisar por release de soporte o si cambia el flujo de instalación |
| SRC-003 | <https://github.com/archlinux/archinstall> | Repositorio oficial vigente a la fecha | 2026-10-05 | Gobierno | Revisar antes de fijar el adapter v1 (DEC-003) |
| SRC-004 | <https://v2.tauri.app/> | Tauri 2.x | 2026-10-05 (previa) | Seguridad | Revisar por minor; fijar permisos por ventana |
| SRC-005 | <https://doc.rust-lang.org/stable/> · <https://rustsec.org/> | Rust estable + RustSec Advisory DB | 2026-10-05 | Seguridad | Revisar por release estable y por aviso relevante en `cargo-audit` |
| SRC-006 | <https://json-schema.org/draft/2020-12/release-notes> | Draft 2020-12 | 2026-10-05 | Arquitectura | Estable; revisar notas de errata |
| SRC-007 | <https://www.w3.org/TR/WCAG22/> · <https://www.w3.org/WAI/ARIA/apg/> | **W3C Recommendation 12 December 2024** + ARIA APG vigente | 2026-10-05 | UX/Accesibilidad | Estable; revisar APG por cambios de patrón |
| SRC-008 | <https://slsa.dev/spec/v1.2/> · <https://csrc.nist.gov/pubs/sp/800/218/final> · <https://docs.sigstore.dev/> | **SLSA v1.2** (v1.0 retirada) · **NIST SSDF SP 800-218 v1.1** · Sigstore vigente | 2026-10-05 | Seguridad/Release | Revisar por nueva versión de especificación; fijar versiones antes de firmar releases |

## Notas de verificación

- **SRC-008 (SLSA):** la especificación `v1.0` está **retired**; la versión actual es **v1.2** (`https://slsa.dev/spec/v1.2/`). Toda referencia a SLSA en la documentación debe apuntar a v1.2, no a v1.0.
- **SRC-008 (SSDF):** capturado como **NIST SP 800-218, SSDF Version 1.1** (no v1.0). Las referencias de SDLC se anclan a v1.1.
- **SRC-008 (Sigstore):** capturado el overview oficial; firma/verificación de artefactos (release files, imágenes, binarios, SBOM) conforme a DEC-006.
- **SRC-005 (Rust):** capturados la documentación estable de Rust (`doc.rust-lang.org/stable/`, con el «Rust Bookshelf») y la RustSec Advisory Database (`rustsec.org/`, herramienta `cargo-audit` sobre `Cargo.lock`) para el core y la supply chain.
- **SRC-007 (WCAG 2.2):** capturada como W3C Recommendation de 2024-12-12; la matriz de accesibilidad (NFR-ACC-001) se ancla a esta versión. **ARIA APG** capturada en la misma fecha.
- **SRC-002/SRC-003:** capturadas para desbloquear las afirmaciones de paquetes/instalación marcadas «no verificado — fuente primaria pendiente»; siguen pendientes de conversión a datos concretos de catálogo (ver `docs/12-research/inventory-v5.1.md`).

## Regla

Toda afirmación mutable de catálogo debe almacenar URL, fecha de captura, versión objetivo, verificador y fecha de caducidad. Las referencias históricas no se convierten en verdad de producto sin verificación.
