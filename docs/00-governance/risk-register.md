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

## Riesgo residual y aceptación (corte walking skeleton, 2026-10-06)

El modelo de puntuación inherente/residual, sus escalas, campos y reglas se leen en
`docs/08-security/residual-risk-model.md` (`DOC-SEC-RISK-001`); este registro no los duplica.
Resumen del corte:

| ID | Control | Evidencia del control | Residual | Autoridad que acepta | Estado |
|---|---|---|---|---|---|
| RSK-003 | Core Rust único; paridad Rust nativo/WASM por golden parity tests. | `ADR-0007`; `canonicalization.md`; `manifest.schema.json`; `examples/05-manifest.valid.json`; `test-matrix.md`. | media×alto → baja×alto (6→3) | architecture | aceptado para el slice |
| RSK-004 | Plan de migración previo, original conservado y corpus golden de migración; digest distinto = cambio, no pérdida. | `versioning-migrations.md`; `examples/migration/v5.1-instance.sample.json`; `examples/migration/vnext-draft.expected.json`; `ADR-0007`. | alta×alto → media×alto (9→6) | data | aceptado para el slice |
| RSK-007 | Lockfiles, auditoría, SBOM, provenance y firmas Sigstore keyless con verificación offline; canal firmado. | `DEC-006`; `ADR-0008`; `ADR-0009`; `packaging-release.md`; `ci-cd.md`; `supply-chain-policy.md`. | media×crítico → baja×crítico (8→4) | security | aceptado para el slice |
| RSK-001 | Inventario estable, plan hash, doble confirmación y VM tests; elevación fuera del WebView. | `DEC-004`; `ADR-0004`; `privilege-model.md`; `mvp-negative-tests.md`. | media×crítico → baja×crítico (8→4) | security | pendiente-de-verificación |
| RSK-002 | Catálogo sin comandos, operaciones enum, sin shell, firmas y capabilities por ventana. | `ADR-0008`; `tauri-commands.md`; `tauri-policy.md`; `mvp-negative-tests.md`. | media×crítico → baja×crítico (8→4) | security | pendiente-de-verificación |
| RSK-008 | Proceso separado, Unix socket con autenticación de sesión y elevación fuera del WebView. | `DEC-004`; `ADR-0004`; `privilege-model.md`; `runner-hazard-analysis.md`. | media×crítico → baja×crítico (8→4) | security | pendiente-de-verificación |

- **RSK-003, RSK-004 y RSK-007** quedan **aceptados para el walking skeleton**: control decidido y
  evidencia de diseño enlazada (vectores golden de canonicalización de R5, corpus de migración y
  contratos). **No se declaran cerrados**: la implementación de producto no está verificada
  (`implementationStatus: partial`, `verificationStatus: partial`).
- **RSK-001, RSK-002 y RSK-008** quedan **`pendiente-de-verificación`**: control decidido y
  documentado, verificación de implementación pendiente (pruebas negativas de capabilities y de
  elevación). No admiten cierre hasta que exista evidencia ejecutable.
- Ningún riesgo P0 se marca aceptado sin evidencia del control; la aceptación se revisa si cambia
  el control, la evidencia o el corte (`DOC-SEC-RISK-001`, reglas 1–6).
- RSK-005, RSK-006, RSK-009 y RSK-010 no cambian de estado en este corte.
