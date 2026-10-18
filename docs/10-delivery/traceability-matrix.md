---
id: DOC-DEL-TRC-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
releaseStatus: ineligible
evidence:
  - tools/traceability_matrix.py (exit 0, metricas 100% en verde)
owners:
  - release
reviewers:
  - independent-reviewer
---

# Matriz de Trazabilidad FR/NFR -> TST/VAL

- **Generado**: a56e64bb
- **Total requisitos**: 17
- **En alcance (mvp0.1)**: 13
- **Diferidos**: 3
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

| Req             | Tipo | Nombre                         | Scope    | Tests                                                                                                                                                                                                                                                       | Validators                            | Evidencia | CI  | Estado   |
| --------------- | ---- | ------------------------------ | -------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------- | --------- | --- | -------- |
| FR-CAT-001      | FR   | Cargar catálogo versionado.    | mvp0.1   | TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY, TST-ARCHMAKER-WASM-PARITY-PARITY_DESKTOP_CATALOG_MANIFEST_CANONICAL_BYTES, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-DENIED_INVOKE_RETURNS_TYPED_PROTO_001...                                | VAL-SCHEMA-REFS, VAL-NEG-CAPABILITIES | 47        | OK  | covered  |
| FR-DRAFT-001    | FR   | Crear y guardar drafts locales | mvp0.1   | TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY, TST-ARCHMAKER-WASM-PARITY-PARITY_CREATE_DRAFT_DETERMINISTIC_BYTES, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-DENIED_INVOKE_RETURNS_TYPED_PROTO_001...                                        | VAL-SCHEMA-REFS, VAL-NEG-CAPABILITIES | 80        | OK  | covered  |
| FR-ENT-001      | FR   | Aplicar perfiles/policies por  | deferred | --                                                                                                                                                                                                                                                          | --                                    | 0         | OK  | deferred |
| FR-EXPORT-001   | FR   | Exportar artefacto y reporte.  | mvp0.1   | TST-ARCHMAKER-CORE-F0_FINDINGS-C02_EXPORT_BINARY_DIGEST_MATCHES_RENDERER_BYTES, TST-ARCHMAKER-CORE-F0_FINDINGS-C06_EXPORT_CAPABILITY_SCOPED_SAVE_ONLY, TST-ARCHMAKER-CORE-CORE_CONTRACT-EXPORT_ARTIFACT_BYTE_IDEMPOTENT...                                  | --                                    | 26        | OK  | covered  |
| FR-IMPORT-001   | FR   | Importar y migrar v5.1.        | mvp1     | TST-ARCHMAKER-CORE-F0_FINDINGS-C04_IMPORT_CHECKS_FILE_SIZE_BEFORE_READ_WITH_CONSENT, TST-ARCHMAKER-CORE-CORE_CONTRACT-IMPORT_DRAFT_MIGRATES_V51_SAMPLE, TST-WEB-CONSENTED-IMPORT.PLAYWRIGHT                                                                 | VAL-MIG                               | 4         | OK  | covered  |
| FR-MANIFEST-001 | FR   | Construir manifest canónico.   | mvp0.1   | TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY, TST-ARCHMAKER-WASM-PARITY-PARITY_RESOLVE_AND_MANIFEST, TST-ARCHMAKER-WASM-PARITY-PARITY_DESKTOP_CATALOG_MANIFEST_CANONICAL_BYTES...                                                   | VAL-CANONICALIZE, VAL-SCHEMA-REFS...  | 65        | OK  | covered  |
| FR-PRESET-001   | FR   | Aplicar presets como patches.  | deferred | --                                                                                                                                                                                                                                                          | --                                    | 0         | OK  | deferred |
| FR-RESOLVE-001  | FR   | Resolver capacidades/conflicto | mvp0.1   | TST-ARCHMAKER-WASM-PARITY-PARITY_RESOLVE_AND_MANIFEST, TST-ARCHMAKER-CORE-CAPABILITIES_NEGATIVE-NAMED_ALLOWED_INVOKE_RESOLVES_OP, TST-ARCHMAKER-CORE-F0_FINDINGS-C03_RESOLVE_HOSTNAME_ENCRYPT_PRESERVED_AS_INTENTION...                                     | --                                    | 10        | OK  | covered  |
| FR-RUN-001      | FR   | Ejecutar plan aprobado.        | deferred | --                                                                                                                                                                                                                                                          | --                                    | 0         | OK  | deferred |
| FR-VALIDATE-001 | FR   | Ejecutar pipeline de validació | mvp0.1   | TST-ARCHMAKER-WASM-PARITY-PARITY_VALIDATE_DIAGNOSTICS, TST-ARCHMAKER-CORE-F0_FINDINGS-C07_FACE_NO_LONGER_INVENTS_BASE_OR_SWAY_WARN, TST-ARCHMAKER-CORE-F3_FIDELITY-UNSUPPORTED_NUMBER_AND_SECRET_REF_EMIT_EXCLUSION_DIAGNOSTIC...                           | --                                    | 62        | OK  | covered  |
| NFR-ACC-001     | NFR  | WCAG 2.2 AA.                   | mvp0.1   | TST-WEB-RESPONSIVE-LAYOUT.PLAYWRIGHT, TST-WEB-A11Y-PLAYWRIGHT, TST-WEB-VISUAL-REGRESSION                                                                                                                                                                    | --                                    | 3         | OK  | covered  |
| NFR-DET-001     | NFR  | Mismo input/versiones → mismo  | mvp0.1   | TST-ARCHMAKER-CANONICALIZATION-VECTORS-GOLDEN_CORPUS_BYTE_IDENTITY_AND_DIGEST, TST-ARCHMAKER-CANONICALIZATION-VECTORS-M_KEY_ORDER_BYTE_IDENTITY, TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY...                                    | VAL-CANONICALIZE                      | 80        | OK  | covered  |
| NFR-MIG-001     | NFR  | Migraciones explícitas y no de | mvp1     | TST-ARCHMAKER-CORE-CORE_CONTRACT-IMPORT_DRAFT_MIGRATES_V51_SAMPLE, TST-ARCHMAKER-CORE-FORGE_V2_WALKING_SKELETON-ONLYOFFICE_CANONICAL_ID_IS_CATALOG_KNOWN, TST-ARCHMAKER-CORE-FORGE_V2_WALKING_SKELETON-WALKING_SKELETON_V2_SAVE_BUILD_EXPORT_AND_MIGRATE... | VAL-MIG                               | 22        | OK  | covered  |
| NFR-OBS-001     | NFR  | Errores/eventos estructurados  | pending  | --                                                                                                                                                                                                                                                          | --                                    | 0         | OK  | pending  |
| NFR-OFF-001     | NFR  | MVP usable sin CDN ni servidor | mvp0.1   | TST-ARCHMAKER-WASM-PARITY-PARITY_MAPS_VECTORS, TST-ARCHMAKER-WASM-PARITY-PARITY_DOMAIN_VECTORS, TST-ARCHMAKER-WASM-PARITY-PARITY_UNICODE_VECTORS...                                                                                                         | --                                    | 29        | OK  | covered  |
| NFR-PORT-001    | NFR  | Dominio independiente de adapt | mvp0.1   | TST-ARCHMAKER-WASM-PARITY-PARITY_MAPS_VECTORS, TST-ARCHMAKER-WASM-PARITY-PARITY_DOMAIN_VECTORS, TST-ARCHMAKER-WASM-PARITY-PARITY_UNICODE_VECTORS...                                                                                                         | --                                    | 17        | OK  | covered  |
| NFR-SEC-001     | NFR  | Deny-by-default y mínimo privi | mvp0.1   | TST-ARCHMAKER-CANONICALIZATION-VECTORS-N_NEGATIVE_ZERO_BYTE_IDENTITY, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-DENIED_INVOKE_RETURNS_TYPED_PROTO_001, TST-ARCHMAKER-WASM-INVOKE_NEGATIVE-ALLOWED_INVOKE_PASSES_CAPABILITY_GATE...                                 | VAL-NEG-CAPABILITIES                  | 72        | OK  | covered  |

## Requisitos Diferidos (fuera de slice MVP-0.1)

- FR-PRESET-001: excluido por decision aprobada
- FR-RUN-001: excluido por decision aprobada
- FR-ENT-001: excluido por decision aprobada

## Requisitos Pendientes de Decision

- NFR-OBS-001: decision pendiente (D-05 para NFR-OBS-001)

## Validadores Transversales (control de integridad, sin requisito especifico)

- VAL-FRONT-MATTER: validacion front matter documentos
- VAL-GATES: validacion estado gates
- VAL-TRACEABILITY: validacion integridad matriz trazabilidad
- VAL-CLOSURE-EVIDENCE: manifiesto de evidencia de cierre; un ejemplo no es evidencia
