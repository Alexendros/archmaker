# Gates G0–G10

| Gate | Criterio | Evidencia mínima | Estado |
|---|---|---|---|
| G0 | Fuentes controladas | `reference/v5.1/` + hashes/provenance + `source-register.md` + `inventory-v5.1.md` | complete |
| G1 | Problema/usuarios | `personas.md`, `objectives.md` (OBJ-*), `product-brief` en `requirements.md` | partial |
| G2 | Alcance/requisitos | `requirements.md` (FR/NFR con Given/When/Then), `use-cases.md`, `journeys.md` | partial |
| G3 | Arquitectura | `c4/README.md`, `module-map.md`, `dependency-rules.md`, `adr/ADR-0001..0008` | partial |
| G4 | Datos | `domain-model.md`, `lifecycles.md`, `canonicalization.md`, `versioning-migrations.md`, schemas (pendientes) | partial |
| G5 | Interfaces | `core-port.md`, `dto.md`, `errors-events.md`, `tauri-commands.md`, `tauri-wasm.md`, `runner-protocol.md` | partial |
| G6 | UX/diseño | `interaction-matrix.md`, `component-contracts.md`, `design-system.md` | partial |
| G7 | Validación | `operator-spec.md`, `operator-corpus.md`, `pipeline.md`, `rule-inventory.md` | partial |
| G8 | Seguridad | `threat-model.md`, `privilege-model.md`, `tauri-policy.md` | partial |
| G9 | Calidad | `test-matrix.md`, `documentation-validation.md`, `ci-cd.md`, `.github/workflows/`, `packaging-release.md` | partial |
| G10 | Delivery | `roadmap.md`, `backlog.md`, `next-issues.md`, `gates.md`, `packaging-release.md` | partial |

`complete` exige documento accepted, evidencia ejecutable y ausencia de P0 abiertos asociados.
