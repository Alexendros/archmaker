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
