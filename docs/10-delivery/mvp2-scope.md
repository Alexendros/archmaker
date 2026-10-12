---
id: DOC-DEL-MVP2-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - quality
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-MVP1-001
    expectedDocumentStatus: accepted
  - id: DOC-GOV-GNG-001
  - id: DOC-VAL-RULES-001
---

# Alcance MVP-2 — rules / resolver / validation

- Documento: DOC-DEL-MVP2-001 · Estado: accepted · Propietario: Calidad/Arquitectura · Fecha: 2026-10-12
- Entrada: baseline `implementation-baseline-mvp1` + gates G0–G10 `complete` (slice).
- Alcance **accepted** (MVP-2-01). Implementación de reglas de catálogo, resolución de dominio y
  pipeline de validación autorizada bajo las exclusiones de este documento; runner y privilegios
  siguen fuera.

## Objetivo

Hacer que `resolveDraft` y `validateDraft` evalúen las fichas `Rule` del catálogo local y
resuelvan contra el catálogo de dominio con huella SHA-256 canónica, bloqueando el manifiesto
ante diagnóstico o conflicto tipado.

## Incluido

| Área       | Contenido                                                                                          | Anclas                                      |
| ---------- | -------------------------------------------------------------------------------------------------- | ------------------------------------------- |
| Operadores | AST ejecutable canónico (`any`/`ne`/`in` incluidos); `count_gt` normalizado; `target_is` rechazado | FR-VALIDATE-001, operator-spec, AM-RULE-009 |
| Reglas     | Fichas del pack canónico referenciadas por `catalog.rules`; sin inventar reglas                    | FR-VALIDATE-001, rule-inventory             |
| Resolución | Catálogo de dominio (`provides`/`requires`); conflictos tipados; digest SHA-256                    | FR-RESOLVE-001, NFR-DET-001                 |
| Pipeline   | `validateDraft` ≡ etapas 6–9 de resolve/buildManifest                                              | FR-VALIDATE-001, pipeline.md                |

## Excluido (explícito)

- Runner v1, root, discos, shell, sidecars, catálogos remotos.
- Presets (`FR-PRESET-001` deferred).
- `RULE-GPU-001` (sin target; SRC-002/SRC-003 pendientes).
- `target_is` (diferido a v1).
- Etapas 10–12 del pipeline (hardware, firma/freshness, exportador → MVP-3).
- Rediseño visual.

## Issues trazables

| #        | Issue                                         | Depende de | Gate     | Owner        | Criterio Done                         |
| -------- | --------------------------------------------- | ---------- | -------- | ------------ | ------------------------------------- |
| MVP-2-01 | Aprobar alcance DOC-DEL-MVP2-001              | —          | G2,G10   | Product      | Documento `accepted`                  |
| MVP-2-02 | AST operadores canónicos + AM-RULE-009        | 01         | G7       | Validation   | Corpus operadores + tests Rust        |
| MVP-2-03 | Cargar reglas desde fichas del catálogo       | 02         | G4,G7    | Data         | Pack + `catalog.rules`; oráculo verde |
| MVP-2-04 | Resolver dominio + digest SHA-256             | 02,03      | G4,G5,G7 | Architecture | FR-RESOLVE-001; FNV retirado          |
| MVP-2-05 | validateDraft bloquea igual que buildManifest | 03,04      | G5,G7,G9 | Quality      | FR-VALIDATE-001                       |
| MVP-2-06 | Trazabilidad MVP-2 sin huérfanos              | 04,05      | G2,G9    | Quality      | Matriz verde                          |
| MVP-2-07 | Tag `implementation-baseline-mvp2`            | 01–06      | G10      | Release      | Tag anotado; CI verde; go-no-go       |

## Trazabilidad

- Requisitos: FR-RESOLVE-001, FR-VALIDATE-001, NFR-DET-001.
- Validadores: VAL-RULE, corpus operadores, VAL-DOC.
- Pruebas: TST-RULE-\*, parity resolve/validate, core_contract resolve/validate/build.
- Amenazas: sin superficie nueva de shell/red/disco.

## Criterio de salida

- Issues MVP-2-01..07 cerrados o con residual fechado.
- Tag `implementation-baseline-mvp2` emitido; tags MVP-0.1 y MVP-1 intactos.
