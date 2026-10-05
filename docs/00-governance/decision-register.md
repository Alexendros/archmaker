---
id: DOC-GOV-DEC-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
---

# Registro de decisiones

- ID: DOC-GOV-DEC-001
- Estado: in-review
- Propietario: Gobierno
- Última revisión: 2026-10-05

| ID | Pregunta | Opciones | Recomendación | Propietario | Impacto | Estado |
|---|---|---|---|---|---|---|
| DEC-001 | ¿Qué exporta MVP? | Perfil ArchMaker; perfil archinstall; ISO | Perfil ArchMaker + reporte; exporter archinstall experimental | Product owner | P0 | accepted |
| DEC-002 | ¿Ámbito de IDs? | Global; catálogo; sección | Global con namespace `vendor.kind.id` | Arquitectura | P0 | accepted |
| DEC-003 | ¿Motor v1? | archinstall; libalpm propio; scripts | Adapter versionado de archinstall, sin acoplar dominio | Arquitectura | P0 | accepted |
| DEC-004 | ¿Transporte runner? | Unix socket; stdio sidecar; D-Bus | Unix socket + auth de sesión; elevar fuera de WebView | Seguridad | P0 | accepted |
| DEC-005 | ¿Licencia? | Apache-2.0; MIT/Apache; GPLv3 | Apache-2.0 o dual MIT/Apache tras revisión | Propietario | P0 | accepted |
| DEC-006 | ¿Firma de catálogos? | Minisign; Sigstore; ambos | Sigstore para releases + firma offline evaluada | Seguridad | P1 | accepted |
| DEC-007 | ¿Actualización? | Manual; Tauri updater; repos distro | Manual/repo en MVP, updater firmado tras threat model | Release | P1 | accepted |
| DEC-008 | ¿Soporte inicial? | Arch; Arch-based; live ISO propia | Arch x86_64; aarch64 experimental separado | Producto | P0 | accepted |
| DEC-009 | ¿Fuentes? | CDN; empaquetadas; system | Empaquetadas o system stack; sin CDN runtime | Diseño/Legal | P1 | proposed |
| DEC-010 | ¿Telemetría? | Ninguna; opt-in; enterprise | Ninguna en MVP; opt-in futuro con ADR | Producto | P1 | proposed |

## Decisiones aceptadas

| ID | Opción aceptada | Fecha | Notas |
|---|---|---|---|
| DEC-001 | Perfil ArchMaker + reporte; exporter archinstall experimental | 2026-10-05 | Alcance de exportación del MVP fijado. |
| DEC-002 | Global con namespace `vendor.kind.id` | 2026-10-05 | Desbloquea schemas y catálogos. |
| DEC-003 | Adapter versionado de archinstall, sin acoplar dominio | 2026-10-05 | Refuerza ADR-0003. |
| DEC-004 | Unix socket + autenticación de sesión; elevar fuera de WebView | 2026-10-05 | Cierra la incógnita de transporte de ADR-0004. |
| DEC-005 | Apache-2.0 (o dual MIT/Apache) | 2026-10-05 | Licencia del repositorio y recursos visuales. |
| DEC-006 | Sigstore para releases + firma offline evaluada | 2026-10-05 | Habilita ADR-0008 y la firma en MVP (ver DEC-007). |
| DEC-007 | **Tauri updater en MVP** | 2026-10-05 | **Desviación** respecto a la recomendación. Ver consecuencias. |
| DEC-008 | **Solo Arch x86_64** | 2026-10-05 | **Desviación** respecto a la recomendación. Ver consecuencias. |

## Desviaciones respecto a la recomendación

### DEC-007 — Tauri updater en MVP

La recomendación era «manual/repo en MVP, updater firmado tras threat model». La decisión aceptada introduce el **updater de Tauri ya en el MVP**. Consecuencias obligatorias:

1. `THR-UPD-001` (actualización comprometida) deja de ser diferible y debe cerrarse **antes de MVP-0**.
2. Arrastra `DEC-006` (firma) al alcance del MVP: el canal de actualización debe estar firmado.
3. Convive con `NFR-OFF-001` (operación offline): el updater es **fail-open offline** — la aplicación es plenamente usable sin red; solo comprueba actualizaciones cuando hay red.
4. Requiere ADR propio (`ADR-0009`) y política de canal firmado, versión y rollback.

### DEC-008 — Solo Arch x86_64

La recomendación era «Arch x86_64 + aarch64 experimental separado». La decisión aceptada limita el soporte a **Arch x86_64**. Consecuencias:

1. aarch64 sale del alcance soportado; `reference/v5.1/arch-info.json` (aarch64) queda solo como evidencia histórica.
2. `docs/11-operations/support-policy.md` y `docs/09-quality/test-matrix.md` se reducen a x86_64.
3. Un futuro soporte aarch64 exige nueva decisión, hardware/CI y catálogo verificado.

## Decisiones pendientes (P1)

DEC-009 (fuentes sin CDN runtime) y DEC-010 (telemetría) permanecen `proposed`; no bloquean la planificación.
