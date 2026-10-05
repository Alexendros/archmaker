# Gates G0–G10

Evaluación: 2026-10-05 (Etapa E; tras aceptación de producto/arquitectura/datos).

| Gate | Criterio | Evidencia mínima | Estado | Falta para `complete` |
|---|---|---|---|---|
| G0 | Fuentes controladas | `reference/v5.1/` originales íntegros (sha256 OK) + disposición `G0-SOURCE-DISPOSITION.md` + `ALIASES.json`/`UNAVAILABLE-SOURCES.json` + `source-register.md` (SRC-001 `controlled-incomplete`; SRC-002..008 verificadas) + `inventory-v5.1.md` | complete | — (SRC-001 `controlled-incomplete`; sin dependencia P0 de las fuentes ausentes) |
| G1 | Problema/usuarios | `personas.md`, `objectives.md` (OBJ-*), `product-brief` en `requirements.md` | complete | — (docs `accepted` 2026-10-05; métricas cuantitativas aún no verificadas) |
| G2 | Alcance/requisitos | `requirements.md` (FR/NFR con Given/When/Then), `use-cases.md`, `journeys.md` | complete | — (docs `accepted` 2026-10-05; DEC-001/DEC-008 resueltas) |
| G3 | Arquitectura | `c4/README.md`, `module-map.md`, `dependency-rules.md`, `adr/ADR-0001..0009` (accepted) | complete | — (docs `accepted` 2026-10-05; ADR-0001..0009 accepted, ADR-0007 proposed) |
| G4 | Datos | `domain-model.md`, `lifecycles.md`, `canonicalization.md`, `versioning-migrations.md`, `contracts/json-schema/` (8 schemas + corpus + job CI de corpus) | partial | CI en verde (meta-validación + corpus); canonicalización ADR-0007 en `proposed` |
| G5 | Interfaces | `core-port.md`, `dto.md`, `errors-events.md`, `tauri-commands.md`, `tauri-wasm.md`, `runner-protocol.md` | partial | aprobar CorePort/DTO; ratificar cobertura CorePort (C4) |
| G6 | UX/diseño | `interaction-matrix.md`, `component-contracts.md`, `design-system.md` | partial | validación de accesibilidad; aprobar tokens preservados |
| G7 | Validación | `operator-spec.md`, `operator-corpus.md`, `pipeline.md`, `rule-inventory.md` | partial | corpus ejecutable en CI; RULE-GPU-001 no implementable (SRC-002/003); `target_is` diferido a v1 |
| G8 | Seguridad | `threat-model.md`, `privilege-model.md`, `tauri-policy.md` | partial | sign-off aprobado formalmente 2026-10-05; falta verificación de implementación (pruebas negativas de capabilities/elevación) y cierre de RSK-001/002/008 |
| G9 | Calidad | `test-matrix.md`, `documentation-validation.md`, `ci-cd.md`, `.github/workflows/` (pins por SHA), `packaging-release.md` | partial | CI en verde; SBOM/firmas reales |
| G10 | Delivery | `roadmap.md`, `backlog.md`, `next-issues.md`, `gates.md`, `packaging-release.md` | partial | owners en backlog; releases |

`complete` exige documentos `accepted`, ausencia de P0 abiertos asociados y, en los gates con artefacto ejecutable (G4, G9), evidencia ejecutable (CI en verde). En gates documentales (G0–G3, G5–G8, G10), `complete` = documentos `accepted` + sin P0 asociados.

## Progreso (Pasos 0–7 + Etapas A–F)

- DEC-001..DEC-008 **accepted** (2026-10-05); DEC-009/DEC-010 en `proposed` (no bloquean).
- ADR-0001..ADR-0009 **accepted**.
- 8 JSON Schema Draft 2020-12 cerrados (`additionalProperties:false`) + corpus válido/inválido + fixture de migración v5.1.
- **Fuentes**: once originales SRC-001 recibidos íntegros y controlados; `archmaker-v10.4-p3.yaml` e `i18n.es.json` `unavailable-declared-only`; `neubat_forge_v4_final.html` como alias byte a byte (`probable-rename-not-proven`). SRC-002..SRC-008 verificadas (SRC-002/003/006 en Paso 2; SRC-005/007/008 en Etapa B). SRC-001 pasa a `controlled-incomplete` (`G0-SOURCE-DISPOSITION.md`).
- **Etapa A (A1-A6)**: marcadores obsoletos resueltos, índice ADR, plantilla de issue, severidad canónica, catálogo AM-* completo, aliases ValueRef, mapas abiertos documentados.
- **Etapa C**: job de CI `corpus-validate` (válido/inválido) + meta-validación acotada a `*.schema.json`.
- **Etapa D**: sign-off de Seguridad registrado 2026-10-05; RSK-001/002/008 con control definido (verificación de implementación pendiente).
- **Etapa E**: aceptados FR/NFR, modelo de dominio, C4/módulos, personas, objetivos, journeys y casos de uso (2026-10-05); ADR-0007 rebajado a `proposed`; operadores `required` ratificado y `target_is` diferido a v1; sign-off de Seguridad aprobado formalmente.
- Gate global `planning-v1`: sigue **No-Go** (ver `go-no-go.md`).
