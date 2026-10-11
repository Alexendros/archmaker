---
id: DOC-GOV-TRC-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
---

# Matriz de trazabilidad (Gobernanza)

> **Única fuente de verdad**: La matriz de trazabilidad ejecutable y completa se genera automáticamente en `docs/10-delivery/traceability-matrix.md` y `docs/10-delivery/traceability-matrix.json` por `tools/traceability_matrix.py`.
>
> Este documento de gobernanza mantiene únicamente el mapeo de alto nivel **Objetivo → Requisito** para decisiones de alcance y prioridad. La trazabilidad detallada FR/NFR → TST/VAL, métricas C_req/C_pass, huérfanos, cobertura negativa, paridad y veredictos V-TRACE se derivan automáticamente y son la fuente autoritativa.

---

## Mapeo Objetivo → Requisito (alto nivel)

| Objetivo                        | Journey/UC       | Requisitos clave                                 | Fase           |
| ------------------------------- | ---------------- | ------------------------------------------------ | -------------- |
| OBJ-001 Configurar sin terminal | JNY-001 / UC-001 | FR-DRAFT-001, FR-CAT-001                         | MVP            |
| OBJ-002 Resultado válido        | JNY-001 / UC-003 | FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001 | MVP            |
| OBJ-003 Migrar legado           | JNY-002 / UC-002 | FR-IMPORT-001                                    | MVP (diferido) |
| OBJ-004 Exportar                | JNY-001 / UC-005 | FR-MANIFEST-001, FR-EXPORT-001                   | MVP            |
| OBJ-005 Instalar seguro         | JNY-003 / UC-006 | FR-RUN-001                                       | v1             |
| OBJ-006 Gobernar flotas         | JNY-004 / UC-007 | FR-ENT-001                                       | Enterprise     |

---

## Referencias

- **Matriz ejecutable completa**: `docs/10-delivery/traceability-matrix.md` (generada por CI)
- **Fuente máquina (JSON)**: `docs/10-delivery/traceability-matrix.json`
- **Generador**: `tools/traceability_matrix.py`
- **Inventario de tests**: `docs/10-delivery/tests-inventory.json`
- **Validadores**: `docs/10-delivery/validators.json`
- **Job CI**: `traceability` en `.github/workflows/ci.yml`
- **Veredictos V-TRACE-01..05**: Evaluados en CI, fallan si métricas < 100%

---

## Nota de gobernanza

Cualquier cambio en la trazabilidad debe hacerse en las fuentes primarias:

1. `docs/01-product/requirements.md` (campo `Scope` en tablas)
2. `tools/gen_test_inventory.py` (inferencia de cobertura)
3. `docs/10-delivery/validators.json` (enlaces validador → requisito)

La matriz de gobernanza **no se edita a mano** para trazabilidad detallada; solo se actualiza el mapeo Objetivo → Requisito cuando cambia el alcance del producto.
