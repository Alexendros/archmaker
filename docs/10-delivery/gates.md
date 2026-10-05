# Gates G0–G10

Evaluación: 2026-10-05 (Etapa F; tras A1-A6 y Etapas B–D).

| Gate | Criterio | Evidencia mínima | Estado | Falta para `complete` |
|---|---|---|---|---|
| G0 | Fuentes controladas | `reference/v5.1/` íntegro (sha256 OK) + `source-register.md` (SRC-001..008 verificadas) + `inventory-v5.1.md` | complete | — |
| G1 | Problema/usuarios | `personas.md`, `objectives.md` (OBJ-*), `product-brief` en `requirements.md` | partial | aprobar (accepted) personas/objetivos; métricas aún no verificadas |
| G2 | Alcance/requisitos | `requirements.md` (FR/NFR con Given/When/Then), `use-cases.md`, `journeys.md` | partial | aprobar FR/NFR (hoy draft); DEC-001/DEC-008 ya resueltas |
| G3 | Arquitectura | `c4/README.md`, `module-map.md`, `dependency-rules.md`, `adr/ADR-0001..0009` (accepted) | partial | aprobar C4/módulos (hoy draft) |
| G4 | Datos | `domain-model.md`, `lifecycles.md`, `canonicalization.md`, `versioning-migrations.md`, `contracts/json-schema/` (8 schemas + corpus + job CI de corpus) | partial | CI en verde; aprobar modelo e identidad |
| G5 | Interfaces | `core-port.md`, `dto.md`, `errors-events.md`, `tauri-commands.md`, `tauri-wasm.md`, `runner-protocol.md` | partial | aprobar CorePort/DTO; ratificar cobertura CorePort (C4) |
| G6 | UX/diseño | `interaction-matrix.md`, `component-contracts.md`, `design-system.md` | partial | validación de accesibilidad; aprobar tokens preservados |
| G7 | Validación | `operator-spec.md`, `operator-corpus.md`, `pipeline.md`, `rule-inventory.md` | partial | corpus ejecutable; ratificar operador `required`; RULE-GPU-001 no implementable (SRC-002/003) |
| G8 | Seguridad | `threat-model.md`, `privilege-model.md`, `tauri-policy.md` | partial | verificación de implementación (pruebas negativas de capabilities/elevación); RSK-001/002/008 con control definido y sign-off |
| G9 | Calidad | `test-matrix.md`, `documentation-validation.md`, `ci-cd.md`, `.github/workflows/` (pins por SHA), `packaging-release.md` | partial | CI en verde; SBOM/firmas reales |
| G10 | Delivery | `roadmap.md`, `backlog.md`, `next-issues.md`, `gates.md`, `packaging-release.md` | partial | owners en backlog; releases |

`complete` exige documento accepted, evidencia ejecutable y ausencia de P0 abiertos asociados.

## Progreso (Pasos 0–7 + Etapas A–F)

- DEC-001..DEC-008 **accepted** (2026-10-05); DEC-009/DEC-010 en `proposed` (no bloquean).
- ADR-0001..ADR-0009 **accepted**.
- 8 JSON Schema Draft 2020-12 cerrados (`additionalProperties:false`) + corpus válido/inválido + fixture de migración v5.1.
- **Fuentes SRC-001..SRC-008 verificadas** (SRC-002/003/006 en Paso 2; SRC-005/007/008 en Etapa B).
- **Etapa A (A1-A6)**: marcadores obsoletos resueltos, índice ADR, plantilla de issue, severidad canónica, catálogo AM-* completo, aliases ValueRef, mapas abiertos documentados.
- **Etapa C**: job de CI `corpus-validate` (válido/inválido) + meta-validación acotada a `*.schema.json`.
- **Etapa D**: sign-off de Seguridad registrado 2026-10-05; RSK-001/002/008 con control definido (verificación de implementación pendiente).
- Gate global `planning-v1`: sigue **No-Go** (ver `go-no-go.md`).
