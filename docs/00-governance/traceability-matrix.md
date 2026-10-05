# Matriz de trazabilidad

| Objetivo | Journey/UC | Requisito | Arquitectura/datos | UX/API | Validación/amenaza | Prueba | Release |
|---|---|---|---|---|---|---|---|
| OBJ-001 Configurar sin terminal | JNY-001 / UC-001 | FR-DRAFT-001 | DM-DRAFT / ADR-0001 | `/configure`; `createDraft` | VAL-DOC / THR-IMP-001 | TST-DRAFT-001 | MVP |
| OBJ-002 Resultado válido | JNY-001 / UC-003 | FR-RESOLVE-001 | DM-RESOLUTION | `resolveDraft` | VAL-REF, VAL-RULE | TST-RES-001 | MVP |
| OBJ-003 Migrar legado | JNY-002 / UC-002 | FR-IMPORT-001 | MIG-5.1 | `importDraft` | VAL-MIG / THR-IMP-001 | TST-MIG-001 | MVP |
| OBJ-004 Exportar | JNY-001 / UC-005 | FR-EXPORT-001 | DM-MANIFEST/ARTIFACT | `/export`; `exportArtifact` | VAL-TARGET | TST-EXP-001 | MVP |
| OBJ-005 Instalar seguro | JNY-003 / UC-006 | FR-RUN-001 | DM-PLAN / ADR-0004 | Runner protocol | VAL-PLAN / THR-RUN-001 | TST-RUN-001 | v1 |
| OBJ-006 Gobernar flotas | JNY-004 / UC-007 | FR-ENT-001 | PolicyBundle | `/api/v1` | VAL-POL / THR-TEN-001 | TST-TEN-001 | Enterprise |
