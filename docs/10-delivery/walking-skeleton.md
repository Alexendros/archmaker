---
id: DOC-DEL-WSK-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0007
  - id: DOC-IF-CORE-001
  - id: DOC-DATA-CANON-001
  - id: DOC-VAL-RULES-001
  - id: DOC-SEC-RISK-001
  - id: DOC-DS-VISUAL-001
  - id: DOC-CON-JSONSCHEMA-001
---

# Walking skeleton — planning-v1.1

- Documento: DOC-DEL-WSK-001 · Estado: in-review · Propietario: Delivery · Fecha: 2026-10-06
- Autoridad de estados: `docs/00-governance/status-model.md`.
- Implementación del slice: `complete` (baseline `implementation-baseline-mvp0.1`); verificación `partial`.
- Autoriza únicamente un vertical slice (AUD-023); no autoriza el MVP completo ni el runner.

## Objetivo y principio de rebanada vertical

El walking skeleton es el primer código funcional autorizable: un corte vertical mínimo que
atraviesa interfaz, contrato, dominio y persistencia controlada manteniendo la misma semántica
en nativo y en WASM. El principio de rebanada vertical exige incluir en cada capa solo lo
estrictamente necesario para un flujo verificable de extremo a extremo, en lugar de completar
capas horizontales. La lógica semántica reside en Rust; TypeScript solo adapta y presenta.

```mermaid
flowchart LR
  UI[Tauri / WASM] --> CP[CorePort v0]
  CP --> CORE[Rust Core]
  CORE --> MAN[Manifiesto canonico]
  MAN --> OUT[Exportacion JSON]
```

## Alcance incluido

- Workspace Rust mínimo con el core de dominio compilable para nativo y WASM.
- Tipos de dominio mínimos derivados de los contratos v0.
- Catálogo fixture embebido (sin descarga remota).
- Borrador en memoria y guardado local controlado con escritura atómica y detección de
  conflicto de revisión (`expectedRevision`).
- Una regla resoluble: `RULE-DM-001` o `RULE-COMP-001`, con corpus mínimo por operador.
- Diagnóstico tipado conforme a `diagnostic.schema.json` y `core-error.schema.json`.
- Manifiesto canónico conforme a `manifest.schema.json`.
- Exportación JSON reproducible.
- Adaptador Tauri mínimo con denegación por defecto y scopes acotados.
- Adaptador WASM/Web Worker mínimo con la misma interfaz que el adaptador Tauri.
- Paridad golden nativo/WASM sobre bytes y digest.

## Alcance excluido

- Runner v1 y cualquier operación privilegiada.
- Acceso a root y a discos reales.
- Shell arbitraria o ejecución de scripts heredados.
- Migración completa de `reference/v5.1`.
- Presets completos.
- Catálogos remotos.
- Capacidades Enterprise.
- Campañas.
- Telemetría.
- Auto-updater activado por defecto.

## Criterios de aceptación

- La misma entrada produce el mismo manifiesto y el mismo digest en nativo y en WASM.
- Cero reglas duplicadas en TypeScript.
- Errores públicos tipados (sin cadenas libres).
- Recorrido E2E verificable en modo offline.
- Prueba de escritura atómica y de conflicto de revisión.
- Baseline visual sin regresiones no aprobadas.
- CI en verde sobre el commit protegido.

## Propietarios por workstream

| Workstream                                | Código | Rol propietario |
| ----------------------------------------- | ------ | --------------- |
| Producto y alcance del slice              | WS-01  | product         |
| Arquitectura y CorePort                   | WS-05  | architecture    |
| Datos, canonicalización y contratos       | WS-04  | data            |
| Seguridad, capabilities y riesgo residual | WS-07  | security        |
| UX y baseline visual                      | WS-08  | ux              |
| Calidad, corpus y CI                      | WS-09  | quality         |
| Delivery y evidencia                      | WS-10  | release         |

## Dependencias

- [CorePort v0](../04-interfaces/core-port.md): frontera contractual del slice.
- [ADR-0007](../02-architecture/adr/ADR-0007-canonicalizacion-y-hashing.md): canonicalización y
  hashing de los que dependen los digests.
- [Perfil de canonicalización v1](../03-data/canonicalization-profile-v1.md): semántica
  reproducible del manifiesto.
- [Contratos v0](../../contracts/json-schema/README.md): schemas del borrador, catálogo,
  diagnóstico, manifiesto y errores.
- [Inventario de reglas](../07-validation/rule-inventory.md): definición de `RULE-*` y de los
  operadores usados por la regla resoluble.
- [Modelo de riesgo residual](../08-security/residual-risk-model.md): riesgos aceptados para el
  slice.
- [Baseline visual](../06-design-system/visual-baseline.md): referencia de no regresión visual.

## Riesgos

- Divergencia de paridad nativo/WASM: mitigada con golden vectors sobre bytes y digest.
- Duplicación de reglas en TypeScript: mitigada manteniendo la semántica en Rust.
- Escritura no atómica del borrador: mitigada con escritura atómica y conflicto de revisión.
- Ampliación indebida del alcance hacia runner o privilegios: mitigada por gate separado.

## Gates fuera de alcance

- Runner v1, privilegios, discos y shell permanecen bloqueados hasta sus gates de versión.
- SBOM, firmas reales y provenance pertenecen a artefactos de producto y no se cierran aquí.
- Enterprise, campañas y telemetría quedan fuera del MVP.
