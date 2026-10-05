---
id: DOC-GOV-RSK-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
---

# Registro de riesgos

| ID | Riesgo | Prob. | Impacto | Nivel | Control | Propietario | Gate |
|---|---|---:|---:|---|---|---|---|
| RSK-001 | Borrado del disco equivocado. | M | C | P0 | Inventario estable, plan hash, doble confirmación y VM tests. | Runner | G8 |
| RSK-002 | Ejecución arbitraria desde catálogo/UI. | M | C | P0 | Sin shell; operaciones enum; firmas y capabilities. | Seguridad | G8 |
| RSK-003 | Divergencia WASM/Tauri. | M | H | P0 | Core Rust único y golden parity tests. | Core | G9 |
| RSK-004 | Migración pierde selecciones. | H | H | P0 | Plan previo, copia original y corpus golden. | Datos | G4 |
| RSK-005 | Paquetes o compatibilidad obsoletos. | H | H | P1 | Evidencia versionada, freshness y verificadores. | Catálogo | G7 |
| RSK-006 | Dependencia de CDN rompe offline/CSP. | H | M | P1 | Bundling y CSP restrictiva. | Frontend | G6/G8 |
| RSK-007 | Supply-chain Rust/JS comprometida. | M | C | P0 | Lockfiles, audits, SBOM, provenance y firmas. | Release | G9 |
| RSK-008 | Elevación insegura del runner. | M | C | P0 | Proceso separado y ADR de autenticación/elevación. | Seguridad | G8 |
| RSK-009 | Design system se degrada al modularizar. | M | M | P1 | Visual regression y tokens canónicos. | Diseño | G6/G9 |
| RSK-010 | Enterprise contamina el MVP. | M | H | P1 | Puertos opcionales y módulos separados. | Arquitectura | G3 |

## Estado de tratamiento (2026-10-05)

- RSK-001 (borrado de disco equivocado): control decidido y documentado (DEC-004; `privilege-model.md`; ADR-0004). Mitigado por diseño; verificación de implementación pendiente. No cerrado hasta evidencia.
- RSK-002 (ejecución arbitraria desde catálogo/UI): control decidido y documentado (DEC-003 adapter versionado; catálogo sin comandos; operaciones enum; ADR-0004). Mitigado por diseño; verificación pendiente.
- RSK-008 (elevación insegura del runner): control decidido y documentado (DEC-004, transporte Unix socket + autenticación de sesión y elevación fuera del WebView; `privilege-model.md`). Mitigado por diseño; verificación pendiente.
- Resto de riesgos (RSK-003..RSK-007, RSK-009, RSK-010): sin cambio de estado en esta revisión.
