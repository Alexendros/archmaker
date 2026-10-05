---
id: DOC-GOV-IDX-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
---

# ArchMaker — Índice documental (planning-v1)

- Estado del plan: in-review · Gate global: **No-Go** · Baseline: planning-v1 · Corte: 2026-10-05
- Autonomía documental: permitida. Código funcional de producto: **bloqueado** hasta aprobar G0–G10.
- Estados válidos (`docs/00-governance/document-control.md`): `draft`, `in-review`, `accepted`, `superseded`, `deferred`, `rejected`.
- Regla de reemplazo: copiar por rutas canónicas; `reference/v5.1/` debe permanecer byte a byte.

## 00 — Gobernanza

| Documento | ID | Estado |
|---|---|---|
| `plan-status.md` | DOC-GOV-STATUS | in-review |
| `document-control.md` | DOC-GOV-CONTROL | accepted (política) |
| `glossary.md` (12 términos canónicos) | DOC-GOV-001 | draft |
| `decision-register.md` (DEC-001..010) | DOC-GOV-DEC | accepted |
| `contradiction-register.md` (CON-001..012) | DOC-GOV-CON | in-review |
| `risk-register.md` (RSK-001..010) | DOC-GOV-RSK | in-review |
| `source-register.md` (SRC-001..008) | DOC-GOV-SRC | in-review |
| `traceability-matrix.md` | DOC-GOV-TRACE | in-review |
| `go-no-go.md` | DOC-GOV-GNG-001 | draft |

## 01 — Producto

| Documento | ID | Estado |
|---|---|---|
| `objectives.md` (OBJ-001..006) | DOC-PROD-OBJ-001 | draft |
| `personas.md` (PER-001..004) | DOC-PROD-PER-001 | draft |
| `journeys.md` (JNY-001..004) | DOC-PROD-JNY-001 | draft |
| `use-cases.md` (UC-001..007) | DOC-PROD-UC-001 | draft |
| `requirements.md` (FR/NFR + Given/When/Then) | DOC-PROD-REQ-001 | draft |

## 02 — Arquitectura

| Documento | ID | Estado |
|---|---|---|
| `c4/README.md` | DOC-ARCH-C4-001 | draft |
| `module-map.md` (15 crates) | DOC-ARCH-MOD-001 | draft |
| `dependency-rules.md` | DOC-ARCH-DEP-001 | draft |
| `adr/ADR-0001..ADR-0009` | ADR-0001..0009 | accepted |

## 03 — Datos

| Documento | ID | Estado |
|---|---|---|
| `domain-model.md` (DM-*) | DOC-DATA-DM-001 | draft |
| `lifecycles.md` | DOC-DATA-LIFE-001 | draft |
| `canonicalization.md` (superseded) | DOC-DATA-CANON-000 | superseded |
| `canonicalization-profile-v1.md` (perfil inmutable v1) | DOC-DATA-CANON-001 | accepted |
| `versioning-migrations.md` | DOC-DATA-VER-001 | draft |

## 04 — Interfaces

| Documento | ID | Estado |
|---|---|---|
| `core-port.md` (CorePort, 15 métodos) | DOC-IF-CORE-001 | draft |
| `dto.md` | DOC-IF-DTO-001 | draft |
| `errors-events.md` (CoreError, Diagnostic, AM-*) | DOC-IF-ERR-001 | draft |
| `tauri-commands.md` (8 comandos + capabilities) | DOC-IF-TAURI-001 | draft |
| `tauri-wasm.md` | DOC-IF-WASM-001 | draft |
| `runner-protocol.md` (protocolo v1 draft) | DOC-IF-PROTO-001 | draft |

## 05–06 — UX y diseño

| Documento | ID | Estado |
|---|---|---|
| `05-ux/interaction-matrix.md` | DOC-UX-IA-001 | draft |
| `05-ux/component-contracts.md` | DOC-UX-CMP-001 | draft |
| `06-design-system/design-system.md` | DOC-DS-001 | draft |

## 07 — Validación

| Documento | ID | Estado |
|---|---|---|
| `operator-spec.md` | DOC-VAL-OP-001 | draft |
| `operator-corpus.md` | DOC-VAL-CORPUS-001 | draft |
| `pipeline.md` (15 pasos) | DOC-VAL-PIPE-001 | draft |
| `rule-inventory.md` (RULE-*) | DOC-VAL-RULE-001 | draft |

## 08 — Seguridad

| Documento | ID | Estado |
|---|---|---|
| `threat-model.md` (THR-*) | DOC-SEC-THR-001 | draft |
| `privilege-model.md` | DOC-SEC-PRIV-001 | draft |
| `tauri-policy.md` | DOC-SEC-TAURI-001 | draft |

## 09 — Calidad

| Documento | ID | Estado |
|---|---|---|
| `test-matrix.md` | DOC-QA-TEST-001 | draft |
| `ci-cd.md` | DOC-QA-CI-001 | draft |
| `documentation-validation.md` | DOC-QA-DOCVAL-001 | draft |

## 10 — Delivery

| Documento | ID | Estado |
|---|---|---|
| `roadmap.md` (PD/MVP/v1/ENT) | DOC-DEL-ROAD-001 | draft |
| `backlog.md` | DOC-DEL-BACK-001 | draft |
| `next-issues.md` (10 issues) | DOC-DEL-ISS-001 | draft |
| `gates.md` (G0–G10) | DOC-DEL-GATE-001 | in-review |
| `packaging-release.md` | DOC-DEL-PKG-001 | draft |

## 11–12 — Operaciones e investigación

| Documento | ID | Estado |
|---|---|---|
| `11-operations/support-policy.md` | DOC-OPS-SUP-001 | draft |
| `12-research/evidence-plan.md` | DOC-RES-EVID-001 | draft |
| `12-research/inventory-v5.1.md` | DOC-RES-INV-001 | draft |

## Contratos (`contracts/`)

| Documento | Estado | Nota |
|---|---|---|
| `events/README.md` | draft | eventos públicos; sin schemas aún |
| `json-schema/README.md` | draft | pendiente de modelo; DEC-002 aplicada |
| `openapi/README.md` | draft | Enterprise, no ejecutable en MVP |
| `protocol/README.md` | draft | runner v1 tras aceptar transporte/elevación |

## Plantillas

`templates/adr.md`, `templates/requirement.md`. No existe `templates/issue.md` (los issues viven en `docs/10-delivery/next-issues.md`).
