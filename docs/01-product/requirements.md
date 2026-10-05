# Requisitos

## Funcionales P0

| ID | Requisito | Aceptación | Fase |
|---|---|---|---|
| FR-DRAFT-001 | Crear y guardar drafts locales. | Round-trip sin pérdida y escritura atómica. | MVP |
| FR-IMPORT-001 | Importar y migrar v5.1. | Informe previo; original intacto; pérdidas explícitas. | MVP |
| FR-CAT-001 | Cargar catálogo versionado. | Digest, procedencia y validación disponibles. | MVP |
| FR-PRESET-001 | Aplicar presets como patches. | ChangeSet muestra cambios manuales y derivados. | MVP |
| FR-RESOLVE-001 | Resolver capacidades/conflictos. | Determinista e idempotente. | MVP |
| FR-VALIDATE-001 | Ejecutar pipeline de validación. | Diagnósticos tipados, localizados y estables. | MVP |
| FR-MANIFEST-001 | Construir manifest canónico. | Imposible con errores bloqueantes. | MVP |
| FR-EXPORT-001 | Exportar artefacto y reporte. | Incluye digests, versiones y target. | MVP |
| FR-RUN-001 | Ejecutar plan aprobado. | Confirmación ligada a hash y journal completo. | v1 |
| FR-ENT-001 | Aplicar perfiles/policies por organización. | Aislamiento tenant y auditoría. | Enterprise |

## No funcionales P0

| ID | Requisito | Evidencia |
|---|---|---|
| NFR-SEC-001 | Deny-by-default y mínimo privilegio. | Capabilities, scopes y tests negativos. |
| NFR-DET-001 | Mismo input/versiones → mismo resultado. | Golden parity Tauri/WASM. |
| NFR-ACC-001 | WCAG 2.2 AA. | Axe, teclado y revisión manual. |
| NFR-OFF-001 | MVP usable sin CDN ni servidor. | E2E offline. |
| NFR-MIG-001 | Migraciones explícitas y no destructivas. | Corpus histórico. |
| NFR-PORT-001 | Dominio independiente de adaptadores. | Dependency checks. |
| NFR-OBS-001 | Errores/eventos estructurados y redactados. | Contract tests. |
