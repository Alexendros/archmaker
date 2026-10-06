---
id: DOC-IF-PARITY-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-IF-CORE-001
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
  - id: ADR-0007
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
---

# Evidencia de revisión de paridad CorePort nativo ↔ WASM (G5-C03)

Fecha: 2026-10-06
Alcance: walking skeleton MVP-0 (7 operaciones CorePort v0)

## 1. Matriz de operaciones CorePort → pruebas de contrato + paridad

| Operación CorePort | Test contrato nativo                                                                                                                                | Test paridad nativo/WASM                   | Código error tipado                   | Estado            |
| ------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------------------------ | ------------------------------------- | ----------------- |
| `createDraft`      | `create_draft_valid_defaults`, `create_draft_honors_given_id`, `create_draft_rejects_bad_target`, `create_draft_rejects_unknown_catalog_and_bad_id` | `parity_create_draft_deterministic_bytes`  | AM-TGT-001, AM-CAT-001, AM-SCHEMA-001 | ✅ Verde          |
| `loadCatalog`      | `load_catalog_embedded_and_digest`, `load_catalog_digest_mismatch`, `load_catalog_file_round_trip_and_missing`                                      | — (sin paridad WASM: solo embedded + file) | AM-CAT-002, AM-IO-001, AM-DOC-001     | ✅ Verde (nativo) |
| `saveDraft`        | `save_draft_increments_and_persists`, `save_draft_conflict_on_stale_revision`                                                                       | — (sin paridad WASM: requiere FS)          | AM-DOC-002                            | ✅ Verde (nativo) |
| `resolveDraft`     | `resolve_draft_valid_and_deterministic`, `resolve_draft_rejects_unknown_option`                                                                     | `parity_resolve_and_manifest` (resolución) | AM-RES-001                            | ✅ Verde          |
| `validateDraft`    | `validate_draft_clean_and_blocking`                                                                                                                 | `parity_validate_diagnostics`              | AM-RULE-001 (blocking)                | ✅ Verde          |
| `buildManifest`    | `build_manifest_sealed_and_blocked`, `build_manifest_honors_expected_resolution_digest`                                                             | `parity_resolve_and_manifest` (manifest)   | AM-RES-002, AM-TGT-001                | ✅ Verde          |
| `exportArtifact`   | `export_artifact_byte_idempotent`, `export_artifact_rejects_bad_target_and_handle`                                                                  | — (sin paridad WASM: requiere FS)          | AM-TGT-001, AM-IO-001 (handle)        | ✅ Verde (nativo) |

**Nota**: `loadCatalog`, `saveDraft`, `exportArtifact` operan sobre FS local y no tienen paridad WASM en el walking skeleton (requieren adaptador de almacenamiento fuera de v0). La paridad cubre las 4 operaciones puras/mixtas que no dependen de E/S: `createDraft`, `resolveDraft`, `validateDraft`, `buildManifest`.

## 2. Paridad byte-idéntica verificada

| Vector                                     | Dominio                                             | Nativo (Rust)                | WASM (wasm-bindgen)          | Resultado   |
| ------------------------------------------ | --------------------------------------------------- | ---------------------------- | ---------------------------- | ----------- |
| maps.vectors.json (5 vectores)             | `archmaker:canonicalization:v1`                     | canónico + digest            | canónico + digest            | ✅ Idéntico |
| domain.vectors.json (2 vectores)           | `archmaker:canonicalization:v1`                     | canónico + digest            | canónico + digest            | ✅ Idéntico |
| unicode.vectors.json (4 vectores)          | `archmaker:canonicalization:v1`                     | canónico + digest            | canónico + digest            | ✅ Idéntico |
| `createDraft` (ID fijo `1111...`)          | `archmaker:draft:v1`                                | JSON serializado             | JSON serializado             | ✅ Idéntico |
| `validateDraft` (selecciones fixture)      | `archmaker:draft:v1`                                | `ValidationResult`           | `ValidationResult`           | ✅ Idéntico |
| `resolveDraft` + `buildManifest` (fixture) | `archmaker:resolution:v1` / `archmaker:manifest:v1` | `ResolveResult` + `Manifest` | `ResolveResult` + `Manifest` | ✅ Idéntico |

**Total vectores canónicos verificados**: 11 (maps 5 + domain 2 + unicode 4)
**Total operaciones CorePort con paridad byte-idéntica**: 4/7 (las 3 restantes son I/O-bound)

## 3. Tests negativos de protocolo y capacidades (AM-PROTO-001/002)

| Escenario                                   | Ventana/Comando                               | Código esperado    | Test                                        |
| ------------------------------------------- | --------------------------------------------- | ------------------ | ------------------------------------------- |
| Comando permitido en ventana denegada       | `dialog` → `export_artifact`                  | AM-PROTO-001       | `denied_invoke_returns_typed_proto_001`     |
| Comando desconocido en ventana permitida    | `main` → `import_draft`                       | AM-PROTO-002       | `unknown_command_returns_typed_proto_002`   |
| Ventana desconocida                         | `attacker` → `create_draft`                   | AM-PROTO-002       | `unknown_window_returns_typed_proto_002`    |
| JSON malformado                             | `main` → `create_draft`                       | AM-SCHEMA-001      | `malformed_json_returns_typed_schema_error` |
| Capacidad `main` solo 7 permisos            | `src-tauri/capabilities/main.json`            | 7 permisos exactos | `neg_capability_deny_by_default`            |
| Capacidad `dialog`/`picker` deny-by-default | `src-tauri/capabilities/{dialog,picker}.json` | 0 permisos         | `neg_capabilities` (5 tests)                |

## 4. Resultados de ejecución CI local (2026-10-06)

```text
cargo test --workspace
  archmaker-domain       5 passed
  archmaker-canonicalization 4 passed
  archmaker-rules        20 passed
  archmaker-resolution   8 passed
  archmaker-core         31 passed (17 contract + 9 capabilities + 5 negativas)
  archmaker-wasm         11 passed (6 paridad + 5 negativas invoke)

Total: 79 tests passed, 0 failed

cargo clippy --workspace --all-targets -- -D warnings
  → OK (0 warnings)

cargo fmt --check
  → OK

python3 tools/check_negative_capabilities.py
  → 14/14 PASS
```

## 5. Veredicto G5-C03

### CERRADO para el walking skeleton MVP-0

- ✅ 7 operaciones CorePort v0 tienen casos válido + inválido con códigos `AM-*` tipados
- ✅ 4 operaciones puras/mixtas (`createDraft`, `resolveDraft`, `validateDraft`, `buildManifest`) tienen paridad byte-idéntica nativo ↔ WASM verificada por tests automatizados
- ✅ 3 operaciones I/O-bound (`loadCatalog`, `saveDraft`, `exportArtifact`) cubiertas por tests de contrato nativo (no aplicable paridad WASM en v0)
- ✅ Tests negativos de protocolo (`AM-PROTO-001/002`) y capacidades (deny-by-default, CSP, shell) pasan 14/14
- ✅ 79 tests workspace verdes, clippy/fmt limpios

**Pendiente para v1**: paridad WASM para operaciones con E/S (requiere adaptador de almacenamiento WASM-compatible), contrato `importDraft`/`planMigration`/`applyMigration` (fuera de CorePort v0).

## 6. Referencias cruzadas

- `DOC-IF-CORE-001` — CorePort v0 (7 ops, pre/post, límites, códigos)
- `ADR-0001` — Rust única autoridad semántica
- `ADR-0007` — Canonicalización JCS + digest SHA-256
- `crates/archmaker-core/tests/core_contract.rs` — 17 tests de contrato
- `crates/archmaker-wasm/tests/parity.rs` — 6 tests de paridad
- `crates/archmaker-wasm/tests/invoke_negative.rs` — 5 tests negativos invoke
- `crates/archmaker-core/tests/negative_capabilities.rs` — 9 tests capacidades/CSP
- `tools/check_negative_capabilities.py` — 14 checks estáticos
