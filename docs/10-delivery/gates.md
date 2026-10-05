# Gates G0–G10

Evaluación: 2026-10-05 (Paso 7, cierre `planning-v1`).

| Gate | Criterio | Evidencia mínima | Estado | Falta para `complete` |
|---|---|---|---|---|
| G0 | Fuentes controladas | `reference/v5.1/` íntegro (sha256 OK) + `source-register.md` + `inventory-v5.1.md` | complete | — |
| G1 | Problema/usuarios | `personas.md`, `objectives.md` (OBJ-*), `product-brief` en `requirements.md` | partial | aprobar (accepted) personas/objetivos; métricas aún no verificadas |
| G2 | Alcance/requisitos | `requirements.md` (FR/NFR con Given/When/Then), `use-cases.md`, `journeys.md` | partial | aprobar FR/NFR (hoy draft); DEC-001/DEC-008 ya resueltas |
| G3 | Arquitectura | `c4/README.md`, `module-map.md`, `dependency-rules.md`, `adr/ADR-0001..0009` (accepted) | partial | aprobar C4/módulos (hoy draft) |
| G4 | Datos | `domain-model.md`, `lifecycles.md`, `canonicalization.md`, `versioning-migrations.md`, `contracts/json-schema/` (8 schemas + corpus) | partial | meta-validación CI en verde; aprobar modelo e identidad |
| G5 | Interfaces | `core-port.md`, `dto.md`, `errors-events.md`, `tauri-commands.md`, `tauri-wasm.md`, `runner-protocol.md` | partial | aprobar CorePort/DTO; ratificar cobertura CorePort (C4) |
| G6 | UX/diseño | `interaction-matrix.md`, `component-contracts.md`, `design-system.md` | partial | validación de accesibilidad; aprobar tokens preservados |
| G7 | Validación | `operator-spec.md`, `operator-corpus.md`, `pipeline.md`, `rule-inventory.md` | partial | corpus ejecutable; ratificar operador `required`; RULE-GPU-001 no implementable (SRC-002/003) |
| G8 | Seguridad | `threat-model.md`, `privilege-model.md`, `tauri-policy.md` | partial | aprobación formal de Seguridad (sign-off); cerrar RSK-001/002/008 |
| G9 | Calidad | `test-matrix.md`, `documentation-validation.md`, `ci-cd.md`, `.github/workflows/` (pins por SHA), `packaging-release.md` | partial | CI en verde; capturar SRC-005/SRC-008; SBOM/firmas reales |
| G10 | Delivery | `roadmap.md`, `backlog.md`, `next-issues.md`, `gates.md`, `packaging-release.md` | partial | owners en backlog; releases |

`complete` exige documento accepted, evidencia ejecutable y ausencia de P0 abiertos asociados.

## Progreso (Pasos 0–7)

- DEC-001..DEC-008 **accepted** (2026-10-05); DEC-009/DEC-010 en `proposed` (no bloquean).
- ADR-0001..ADR-0009 **accepted**.
- 8 JSON Schema Draft 2020-12 cerrados (`additionalProperties:false`) + corpus válido/inválido + fixture de migración v5.1.
- Fuentes SRC-002/003/006 capturadas; SRC-007/008 partial; SRC-005 pendiente.
- CI documental con acciones fijadas por SHA.
- Gate global `planning-v1`: sigue **No-Go** (ver `go-no-go.md`).
