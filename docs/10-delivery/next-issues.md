# Próximos issues (orden por dependencia)

- Documento: DOC-DEL-ISS-001 · Estado: draft · Propietario: Delivery · Fecha: 2026-10-05
- Formato: issues trazables en Markdown (decisión D7). Cada issue declara requisitos, riesgos, amenazas, validadores y pruebas afectados.
- Estos 10 issues son la descomposición ejecutable de `docs/10-delivery/backlog.md`; **no** autorizan código funcional de producto mientras el gate siga en No-Go.

| # | Issue | Depende de | Gate | Rol propietario | Criterio de Done |
|---|---|---|---|---|---|
| ISSUE-001 | Resolver DEC-001..DEC-008 en `decision-register.md` (estado `accepted` o `deferred` motivado) | — | G0–G10 | Producto/Arquitectura/Seguridad | Cada DEC con decisión, fecha y firmante; sin `decision-required` abiertos |
| ISSUE-002 | Capturar fuentes primarias (SRC-002 ArchWiki, SRC-003 archinstall, SRC-004 detail, SRC-005..008) y registrar `claimId` con URL/versión/fecha | — | G0 | Investigación | Registro de afirmaciones con procedencia y caducidad |
| ISSUE-003 | Aprobar `personas.md`, `objectives.md`, `journeys.md`, `use-cases.md` y `requirements.md` (FR/NFR con Given/When/Then) | 001 | G1,G2 | Producto | Documentos `accepted`; métricas verificadas o marcadas no verificadas con plan |
| ISSUE-004 | Aceptar ADR-0001/0002 y revisar/aceptar ADR-0003..0008 | 001 | G3 | Arquitectura | 8 ADR en `accepted`; ADR-0004 con transporte decidido (DEC-004) |
| ISSUE-005 | Cerrar modelo de dominio: identidad de IDs (DEC-002), SelectionValue, invariantes, `lifecycles.md` | 001 | G4 | Arquitectura de datos | Modelo `accepted`; CON-001/003/004 resueltas; INV-* sin contradicción |
| ISSUE-006 | Crear JSON Schema Draft 2020-12 (`draft`, `catalog`, `rule`, `diagnostic`, `manifest`, `artifact`, `installation-plan`, `runner-envelope`) con `additionalProperties:false` y corpus válido/inválido + fixture de migración | 004,005 | G4,G5 | Datos/Interfaces | Meta-validación y fixtures en verde en CI (`json-schema.yml`) |
| ISSUE-007 | Formalizar AST de operadores y reglas: `operator-spec.md` + `operator-corpus.md` + fichas `RULE-*` (fijar severidad AM-*) | 005 | G7 | Validación | Operador no soportado → diagnóstico bloqueante probado; `count_gt` resuelto (CON-002) |
| ISSUE-008 | Cerrar contratos de interfaz: CorePort + `dto.md` + `errors-events.md` + `tauri-commands.md`/capabilities (8 comandos, denegación por defecto) | 006 | G5,G8 | Interfaces/Seguridad | DTO con paridad TS/Rust/WASM; cada comando con test negativo |
| ISSUE-009 | Validación de UX y accesibilidad sin rediseño: `interaction-matrix.md`, `component-contracts.md`, tokens `@layer` y corrección de deuda heredada | 003 | G6 | UX/Diseño | Contraste/focus/reduced-motion verificados; deuda heredada documentada y trazada |
| ISSUE-010 | Seguridad y release: cerrar `threat-model.md`/`privilege-model.md`, `test-matrix.md`, CI, SBOM/firmas/provenance y owners del backlog | 004,006,008 | G8,G9,G10 | Seguridad/Calidad/Delivery | RSK-001/002/007/008 con tratamiento; CI en verde; SBOM y política de firma definidos |

## Trazabilidad de los issues

- Requisitos: ISSUE-003 (FR-*, NFR-*), ISSUE-006 (FR-DRAFT/EXPORT, NFR-MIG), ISSUE-007 (FR-VALIDATE/NFR-DET), ISSUE-008 (NFR-PORT/SEC/OBS).
- Riesgos: ISSUE-005 (RSK-004), ISSUE-006 (RSK-004), ISSUE-009 (RSK-009), ISSUE-010 (RSK-001/002/007/008).
- Amenazas: ISSUE-007 (THR-IMP-001), ISSUE-008 (THR-IPC-001/THR-FS-001), ISSUE-010 (THR-CAT-001/THR-SUP-001/THR-RUN-*).
- Validadores: ISSUE-007 (VAL-*), ISSUE-006 (VAL-MIG/VAL-DOC).
- Pruebas: ISSUE-006 (TST-MIG-001/TST-EXP-001), ISSUE-007 (TST-RES-001), ISSUE-008 (TST-DRAFT-001/RUN-001).

## Regla de cierre del plan

Tras ISSUE-001..010 en verde y sin P0 abiertos, se reevalúa `docs/00-governance/go-no-go.md`; solo entonces se autoriza `MVP-0`.
