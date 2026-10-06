# Matriz de Trazabilidad FR/NFR -> TST/VAL

- **Generado**: 4eb7390c
- **Total requisitos**: 17
- **En alcance (mvp0.1)**: 11
- **Diferidos**: 5
- **Pendientes**: 1

## Resumen de Metricas

- **Cobertura de enlace (C_req)**: 100.0%
- **Cobertura verde (C_pass)**: 100.0%
- **Pruebas sin requisito**: 0
- **Validadores sin requisito (no transversal)**: 0
- **Cobertura negativa**: 100.0%
- **Cobertura por workflow**: 100.0%
- **Cobertura de paridad**: 100.0%
- **Diferidos documentados**: 100.0%

## Veredictos V-TRACE

- **V-TRACE-01**: 100% enlace -- OK
- **V-TRACE-02**: 100% verde -- OK
- **V-TRACE-03**: 0 huerfanos (no transversal) -- OK
- **V-TRACE-04**: 100% cobertura negativa -- OK
- **V-TRACE-05**: Reporte CI fail -- OK

## Matriz de Requisitos

| Req | Tipo | Nombre | Scope | Tests | Validators | Evidencia | CI | Estado |
|-----|------|--------|-------|-------|------------|-----------|----|--------|
| FR-CAT-001 | FR | Cargar catálogo versionado. | mvp0.1 | TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-DENIED_INVOKE_RETURNS_TYPED_PROTO_001, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-ALLOWED_INVOKE_PASSES_CAPABILITY_GATE... | VAL-SCHEMA-REFS, VAL-NEG-CAPABILITIES | 33 | OK | covered |
| FR-DRAFT-001 | FR | Crear y guardar drafts locales | mvp0.1 | TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY, TST-ARCHMAKER-WASM-PARITY-PARITY_CREATE_DRAFT_DETERMINISTIC_BYTES, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-DENIED_INVOKE_RETURNS_TYPED_PROTO_001... | VAL-SCHEMA-REFS, VAL-NEG-CAPABILITIES | 39 | OK | covered |
| FR-ENT-001 | FR | Aplicar perfiles/policies por  | deferred | -- | -- | 0 | OK | deferred |
| FR-EXPORT-001 | FR | Exportar artefacto y reporte. | mvp0.1 | TST-ARCHMAKER-CORE-CORE_CONTRACT-EXPORT_ARTIFACT_BYTE_IDEMPOTENT, TST-ARCHMAKER-CORE-CORE_CONTRACT-EXPORT_ARTIFACT_REJECTS_BAD_TARGET_AND_HANDLE, TST-ARCHMAKER-CORE-CORE_CONTRACT-END_TO_END_CREATE_SAVE_RESOLVE_VALIDATE_BUILD_EXPORT | -- | 3 | OK | covered |
| FR-IMPORT-001 | FR | Importar y migrar v5.1. | deferred | -- | -- | 0 | OK | deferred |
| FR-MANIFEST-001 | FR | Construir manifest canónico. | mvp0.1 | TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY, TST-ARCHMAKER-WASM-PARITY-PARITY_RESOLVE_AND_MANIFEST, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-DENIED_INVOKE_RETURNS_TYPED_PROTO_001... | VAL-CANONICALIZE, VAL-SCHEMA-REFS... | 33 | OK | covered |
| FR-PRESET-001 | FR | Aplicar presets como patches. | deferred | -- | -- | 0 | OK | deferred |
| FR-RESOLVE-001 | FR | Resolver capacidades/conflicto | mvp0.1 | TST-ARCHMAKER-WASM-PARITY-PARITY_RESOLVE_AND_MANIFEST, TST-ARCHMAKER-CORE-CAPABILITIES_NEGATIVE-NAMED_ALLOWED_INVOKE_RESOLVES_OP, TST-ARCHMAKER-CORE-CORE_CONTRACT-RESOLVE_DRAFT_VALID_AND_DETERMINISTIC... | -- | 6 | OK | covered |
| FR-RUN-001 | FR | Ejecutar plan aprobado. | deferred | -- | -- | 0 | OK | deferred |
| FR-VALIDATE-001 | FR | Ejecutar pipeline de validació | mvp0.1 | TST-ARCHMAKER-WASM-PARITY-PARITY_VALIDATE_DIAGNOSTICS, TST-ARCHMAKER-CORE-CORE_CONTRACT-VALIDATE_DRAFT_CLEAN_AND_BLOCKING, TST-ARCHMAKER-CORE-CORE_CONTRACT-END_TO_END_CREATE_SAVE_RESOLVE_VALIDATE_BUILD_EXPORT | -- | 3 | OK | covered |
| NFR-ACC-001 | NFR | WCAG 2.2 AA. | mvp0.1 | TST-WEB-A11Y-PLAYWRIGHT, TST-WEB-VISUAL-REGRESSION | -- | 2 | OK | covered |
| NFR-DET-001 | NFR | Mismo input/versiones → mismo  | mvp0.1 | TST-ARCHMAKER-CANONICALIZATION-VECTORS-GOLDEN_CORPUS_BYTE_IDENTITY_AND_DIGEST, TST-ARCHMAKER-CANONICALIZATION-VECTORS-M_KEY_ORDER_BYTE_IDENTITY, TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY... | VAL-CANONICALIZE | 13 | OK | covered |
| NFR-MIG-001 | NFR | Migraciones explícitas y no de | deferred | -- | -- | 0 | OK | deferred |
| NFR-OBS-001 | NFR | Errores/eventos estructurados  | pending | -- | -- | 0 | OK | pending |
| NFR-OFF-001 | NFR | MVP usable sin CDN ni servidor | mvp0.1 | TST-WEB-VISUAL-REGRESSION | -- | 1 | OK | covered |
| NFR-PORT-001 | NFR | Dominio independiente de adapt | mvp0.1 | TST-ARCHMAKER-WASM-PARITY-PARITY_MAPS_VECTORS, TST-ARCHMAKER-WASM-PARITY-PARITY_DOMAIN_VECTORS, TST-ARCHMAKER-WASM-PARITY-PARITY_UNICODE_VECTORS... | -- | 6 | OK | covered |
| NFR-SEC-001 | NFR | Deny-by-default y mínimo privi | mvp0.1 | TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-DENIED_INVOKE_RETURNS_TYPED_PROTO_001, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-ALLOWED_INVOKE_PASSES_CAPABILITY_GATE... | VAL-NEG-CAPABILITIES | 28 | OK | covered |

## Requisitos Diferidos (fuera de slice MVP-0.1)

- FR-IMPORT-001: excluido por decision aprobada
- FR-PRESET-001: excluido por decision aprobada
- FR-RUN-001: excluido por decision aprobada
- FR-ENT-001: excluido por decision aprobada
- NFR-MIG-001: excluido por decision aprobada

## Requisitos Pendientes de Decision

- NFR-OBS-001: decision pendiente (D-05 para NFR-OBS-001)

## Validadores Transversales (control de integridad, sin requisito especifico)

- VAL-FRONT-MATTER: validacion front matter documentos
- VAL-GATES: validacion estado gates
- VAL-TRACEABILITY: validacion integridad matriz trazabilidad
