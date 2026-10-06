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

# Matriz de trazabilidad

| Objetivo | Journey/UC | Requisito | Arquitectura/datos | UX/API | Validación/amenaza | Prueba | Release |
|---|---|---|---|---|---|---|---|
| OBJ-001 Configurar sin terminal | JNY-001 / UC-001 | FR-DRAFT-001 | DM-DRAFT / ADR-0001 | `/configure`; `createDraft` | VAL-DOC / THR-IMP-001 | TST-DRAFT-001 | MVP |
| OBJ-002 Resultado válido | JNY-001 / UC-003 | FR-RESOLVE-001 | DM-RESOLUTION | `resolveDraft` | VAL-REF, VAL-RULE | TST-RES-001 | MVP |
| OBJ-003 Migrar legado | JNY-002 / UC-002 | FR-IMPORT-001 | MIG-5.1 | `importDraft` | VAL-MIG / THR-IMP-001 | TST-MIG-001 | MVP |
| OBJ-004 Exportar | JNY-001 / UC-005 | FR-EXPORT-001 | DM-MANIFEST/ARTIFACT | `/export`; `exportArtifact` | VAL-TARGET | TST-EXP-001 | MVP |
| OBJ-005 Instalar seguro | JNY-003 / UC-006 | FR-RUN-001 | DM-PLAN / ADR-0004 | Runner protocol | VAL-PLAN / THR-RUN-001 | TST-RUN-001 | v1 |
| OBJ-006 Gobernar flotas | JNY-004 / UC-007 | FR-ENT-001 | PolicyBundle | `/api/v1` | VAL-POL / THR-TEN-001 | TST-TEN-001 | Enterprise |

## Trazabilidad ampliada por requisito

| Requisito | Journey / UC | Arquitectura | Datos | Interfaz | Validación / amenaza | Prueba | Release |
|---|---|---|---|---|---|---|---|
| FR-DRAFT-001 | JNY-001 / UC-001 | ADR-0001, ADR-0002 | DM-DRAFT | `createDraft` | VAL-DOC / THR-IMP-001 | TST-DRAFT-001 | MVP |
| FR-IMPORT-001 | JNY-002 / UC-002 | ADR-0003 | DM-DRAFT (MIG-5.1) | `importDraft`, `planMigration`, `applyMigration` (fuera de `CorePort v0`; reintroducir en fase posterior) | VAL-MIG / THR-IMP-001 | TST-MIG-001 | MVP |
| FR-CAT-001 | JNY-001 / UC-001 | ADR-0008 | DM-CATALOG | `loadCatalog` | VAL-CAT / THR-CAT-001 | TST-CAT-001 | MVP |
| FR-PRESET-001 | JNY-001 / UC-001 | ADR-0003 | DM-PRESET / DM-DRAFT | `applyPreset` (fuera de `CorePort v0`; reintroducir en fase posterior) | VAL-REF / THR-IMP-001 | TST-PRESET-001 | MVP |
| FR-RESOLVE-001 | JNY-001 / UC-003 | ADR-0001 | DM-RESOLUTION | `resolveDraft` | VAL-REF, VAL-RULE | TST-RES-001 | MVP |
| FR-VALIDATE-001 | JNY-001 / UC-004 | ADR-0001 | DM-RESOLUTION | `validateDraft` | VAL-RULE / THR-IMP-001 | TST-VAL-001 | MVP |
| FR-MANIFEST-001 | JNY-001 / UC-005 | ADR-0006, ADR-0007 | DM-MANIFEST | `buildManifest` | VAL-DET (digest) | TST-MAN-001 | MVP |
| FR-EXPORT-001 | JNY-001 / UC-005 | ADR-0003 | DM-MANIFEST / DM-ARTIFACT | `exportArtifact` | VAL-TARGET | TST-EXP-001 | MVP |
| FR-RUN-001 | JNY-003 / UC-006 | ADR-0004 | DM-PLAN / DM-SESSION | Runner protocol | VAL-PLAN / THR-RUN-001, THR-RUN-002 | TST-RUN-001 | v1 |
| FR-ENT-001 | JNY-004 / UC-007 | ADR-0008 | DM-POLICY | `/api/v1` | VAL-POL / THR-TEN-001 | TST-TEN-001 | Enterprise |
| NFR-SEC-001 | JNY-001..004 | ADR-0002, ADR-0004 | — | Tauri capabilities | THR-IPC-001, THR-FS-001 | TST-SEC-001 | MVP |
| NFR-DET-001 | JNY-001 | ADR-0006, ADR-0007 | DM-MANIFEST | `buildManifest` | VAL-DET | TST-MAN-001 | MVP |
| NFR-ACC-001 | JNY-001 | — | — | componentes UI | WCAG 2.2 AA | TST-A11Y-001 | MVP |
| NFR-OFF-001 | JNY-001 | ADR-0008 | DM-CATALOG | `loadCatalog` | VAL-CAT | TST-OFF-001 | MVP |
| NFR-MIG-001 | JNY-002 / UC-002 | ADR-0003 | MIG-5.1 | `planMigration` | VAL-MIG / THR-IMP-001 | TST-MIG-001 | MVP |
| NFR-PORT-001 | JNY-001..003 | ADR-0001 | — | CorePort / WASM | paridad corpus | TST-PORT-001 | MVP/v1 |
| NFR-OBS-001 | JNY-001..004 | ADR-0001 | — | errors-events | VAL-OBS | TST-OBS-001 | MVP |

## Trazabilidad de reglas y amenazas P0

| Regla | Fuente (SRC/CON) | Diagnóstico | Amenaza | Prueba |
|---|---|---|---|---|
| RULE-COMP-001 | v5.1 validation-rules / CON-002 | AM-RULE | THR-IMP-001 | TST-RULE-COMP |
| RULE-DM-001 | v5.1 | AM-RULE | — | TST-RULE-DM |
| RULE-GPU-001 | v5.1 | AM-RULE | — | TST-RULE-GPU |
| RULE-KERNEL-001, RULE-KERNEL-002 | v5.1 | AM-RULE | — | TST-RULE-KERNEL |
| RULE-BROWSER-001 | v5.1 | AM-RULE | — | TST-RULE-BROWSER |
| RULE-DUP-001 | CON-004 | AM-DOC | — | TST-RULE-DUP |
| RULE-PKG-001 | SRC-002/SRC-003 (pendiente) | AM-RULE | THR-SUP-001 | TST-RULE-PKG |
