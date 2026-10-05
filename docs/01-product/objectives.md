# Objetivos de producto

- ID: DOC-PROD-OBJ-001
- Estado: draft
- Propietario: Producto
- Última revisión: 2026-10-05
- Requisitos relacionados: OBJ-001..OBJ-006; FR-DRAFT-001, FR-IMPORT-001, FR-CAT-001, FR-PRESET-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001, FR-EXPORT-001, FR-RUN-001, FR-ENT-001
- Sustituye / sustituido por: —

## Reglas

1. Los objetivos están alineados con `docs/00-governance/traceability-matrix.md`; cualquier cambio aquí exige actualizar la matriz.
2. `OBJ-001..OBJ-004` son de producto/local (MVP). `OBJ-005` es **v1**. `OBJ-006` es **Enterprise** (`glossary.md`, regla 4).
3. La columna «requisito» de la matriz marca el requisito **primario**; aquí se añaden los requisitos de apoyo sin alterar esa correspondencia.
4. Estado `draft`: ninguna métrica está aceptada ni medida todavía.

## Resumen

| OBJ | Objetivo | Fase | Personas | Journeys | Requisito primario |
|---|---|---|---|---|---|
| OBJ-001 | Configurar sin terminal | MVP | PER-001, PER-002 | JNY-001 | FR-DRAFT-001 |
| OBJ-002 | Resultado válido | MVP | PER-001, PER-002 | JNY-001 | FR-RESOLVE-001 |
| OBJ-003 | Migrar legado | MVP | PER-001, PER-002 | JNY-002 | FR-IMPORT-001 |
| OBJ-004 | Exportar | MVP | PER-001, PER-002 | JNY-001 | FR-EXPORT-001 |
| OBJ-005 | Instalar seguro | v1 | PER-001, PER-002 | JNY-003 | FR-RUN-001 |
| OBJ-006 | Gobernar flotas | Enterprise | PER-004 | JNY-004 | FR-ENT-001 |

## OBJ-001 — Configurar sin terminal

- **Enunciado:** permitir que una persona configure el objetivo completo (crear, seleccionar y ajustar) por interfaz, sin escribir comandos ni editar shell.
- **Personas:** PER-001 (primaria), PER-002.
- **Métrica de éxito verificable:** un flujo UC-001 se completa solo con teclado o puntero, sin introducir comandos, y `TST-DRAFT-001` pasa con round-trip sin pérdida. *(Umbrales de usabilidad: no verificado — fuente primaria pendiente.)*
- **Fase:** MVP.
- **Journeys:** JNY-001.
- **Requisitos:** FR-DRAFT-001 (primario), FR-CAT-001, FR-PRESET-001; apoya NFR-ACC-001, NFR-OFF-001.

## OBJ-002 — Resultado válido

- **Enunciado:** entregar una configuración resuelta y validada, determinista y explicable, exenta de errores bloqueantes antes de materializar.
- **Personas:** PER-001, PER-002; PER-003 en tanto que fuente del catálogo.
- **Métrica de éxito verificable:** la misma entrada y versiones producen la misma `Resolution` (digest reproducible) y `TST-RES-001` pasa; ningún `Manifest` se construye con errores bloqueantes.
- **Fase:** MVP.
- **Journeys:** JNY-001.
- **Requisitos:** FR-RESOLVE-001 (primario), FR-VALIDATE-001, FR-CAT-001; apoya NFR-DET-001, NFR-PORT-001.

## OBJ-003 — Migrar legado

- **Enunciado:** importar configuraciones heredadas v5.1, detectar el formato, simular la migración y producir un nuevo draft con informe, sin alterar el original.
- **Personas:** PER-001, PER-002.
- **Métrica de éxito verificable:** `TST-MIG-001` pasa; el original permanece byte-idéntico y toda pérdida queda listada de forma explícita. *(Formato v5.1: no verificado — fuente primaria pendiente.)*
- **Fase:** MVP.
- **Journeys:** JNY-002.
- **Requisitos:** FR-IMPORT-001 (primario); apoya NFR-MIG-001.

## OBJ-004 — Exportar

- **Enunciado:** materializar el resultado canónico en uno o más artefactos y un reporte, declarando target, versiones y digests.
- **Personas:** PER-001, PER-002.
- **Métrica de éxito verificable:** `TST-EXP-001` pasa; el artefacto expone `digest`, `mediaType` y versiones, y la reexportación del mismo manifest produce digests idénticos. *(Conjunto exacto de targets: DEC-001 pendiente.)*
- **Fase:** MVP.
- **Journeys:** JNY-001.
- **Requisitos:** FR-EXPORT-001 (primario), FR-MANIFEST-001; apoya NFR-DET-001, NFR-PORT-001.

## OBJ-005 — Instalar seguro

- **Enunciado:** ejecutar un plan de instalación aprobado mediante un runner separado, con preflight, dry-run, confirmación ligada a hash y journal append-only, sin scripts arbitrarios.
- **Personas:** PER-001, PER-002.
- **Métrica de éxito verificable:** `TST-RUN-001` pasa; el journal es completo, el hash de confirmación liga la ejecución al plan y el MVP no usa root/disco/shell. *(Transporte del runner: DEC-004 pendiente; motor: DEC-003 pendiente.)*
- **Fase:** v1.
- **Journeys:** JNY-003.
- **Requisitos:** FR-RUN-001 (primario); apoya NFR-SEC-001, NFR-OBS-001.

## OBJ-006 — Gobernar flotas

- **Enunciado:** gobernar perfiles y policies por organización, con aislamiento de tenant, aprobaciones y auditoría.
- **Personas:** PER-004.
- **Métrica de éxito verificable:** `TST-TEN-001` pasa; ninguna operación cruza tenants y toda aplicación de policy queda auditada. *(Firma de catálogo/policy: DEC-006 pendiente.)*
- **Fase:** Enterprise.
- **Journeys:** JNY-004.
- **Requisitos:** FR-ENT-001 (primario); apoya NFR-SEC-001, NFR-OBS-001.
