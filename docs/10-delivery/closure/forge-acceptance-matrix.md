---
id: DOC-DEL-CLO-FORGE-ACC-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-FORGE-DECL-001
    expectedDocumentStatus: draft
---

# Matriz de aceptación Forge — T01–T44 y HA01–HA25

Evidencia técnica (aserciones identificables). **No** es aprobación independiente ni
`publicationStatus=published`. HA26–HA28 quedan **not-applicable** (A6 excluida) y
no suman al denominador ejecutado.

Base tip al registrar: ver `git rev-parse HEAD` del PR de aceptación.

## T01–T30 (producto / UI)

| ID | Caso | Aserción / ubicación | Estado |
| -- | ---- | -------------------- | ------ |
| T01 | Título ArchMaker Forge | `mock-flow` / `product-acceptance` (cabecera/body) | covered |
| T02 | Versión visible desde metadatos (no v4.1) | `mock-flow` sin `v4.1` | covered |
| T03 | Sin Sencillo/Avanzado ni Intención | `mock-flow` arranque Disco | covered |
| T04 | Recorrido seis pasos | `mock-flow` inventario + rail | covered |
| T05 | Layout responsive sin zoom | `responsive-layout.playwright` | covered |
| T06 | DPI/maximizar | desktop-closure (blocked si no hay entorno) | pending-env |
| T07 | Progreso semántico rail | `mock-flow` `data-step-progress` | covered |
| T08 | Inspector resumen | `mock-flow` inspector | covered |
| T09 | Zen default / navegadores | `catalog_providers` + `mock-flow` Zen | covered |
| T10 | ApexShot | visible no verificado; CAT01 abierto | residual |
| T11 | ONLYOFFICE canónico | `catalog_providers` / `t40` | covered |
| T12 | Kitty/Ghostty | `catalog_providers` verified | covered |
| T13 | Oferta no verificada (Grok*) | `catalog_providers` / `t42` | covered |
| T14 | Conflictos catálogo | reglas/resolución core | covered |
| T15 | GPT/MBR/FS ortogonales | `ha24` / `t33` | covered |
| T16 | Disco desconocido | `ha22` | covered |
| T17 | Reemplazo sin acción real | `executionStatus=not-executed` | covered |
| T18 | LUKS2 tipado sin secretos | `ha21` / `t37` | covered |
| T19 | Ayuda boot/FS | `ha11` | covered |
| T20 | Migración v1→v2 | domain migrate + `t42` | covered |
| T21 | Bytes export / digest | `export-delivery` / F2 | covered |
| T22 | Persistencia | `draft-persistence` T-MIG | covered |
| T23 | Estado stale / invalidación | hardware `ha16` / F7 helpers | covered |
| T24 | Offline web | `wasm-offline` / `test-f7:offline-web` | covered |
| T25 | Cero IO destructivo | `ha24` / negative capabilities | covered |
| T26 | Teclado | `product-acceptance` teclado | covered |
| T27 | Documentación alcance | `forge-declarative-scope` + esta matriz | covered |
| T28 | Candidato F7 | `f7-candidate` / e2e wasm | covered |
| T29 | WASM recorrido | `wasm-flow` | covered |
| T30 | Tres recorridos workers=1 | `test-f7:e2e-wasm-x3` (si entorno) | partial |

## T31–T44 (contratos / hardware)

| ID | Test | Estado |
| -- | ---- | ------ |
| T31 | `archmaker-hardware::t31_imported_report_not_observed_bound` | covered |
| T32 | `archmaker-hardware::t32_session_handle_not_portable_logical_ref` | covered |
| T33 | `archmaker-hardware::t33_mbr_observed_gpt_proposal_without_write` | covered |
| T34 | `archmaker-hardware::t34_missing_table_does_not_infer_empty_or_safe` | covered |
| T35 | `archmaker-hardware::t35_esp_without_uki_path_incomplete` | covered |
| T36 | `archmaker-hardware::t36_secure_boot_enabled_without_signing_is_conditional` | covered |
| T37 | `archmaker-core::t37_luks_or_default_browser_changes_digest` | covered |
| T38 | `archmaker-core::t38_v2_document_not_labeled_as_v1_domain` | covered |
| T39 | `archmaker-core::t39_future_schema_version_rejected` | covered |
| T40 | `archmaker-core::t40_onlyoffice_alias_no_duplicate_canonical` | covered |
| T41 | `archmaker-core::t41_no_browser_policy_defined_no_orphan_default` | covered |
| T42 | `archmaker-core::t42_unverified_request_kept_not_certified_support` | covered |
| T43 | `archmaker-hardware::t43_redacted_public_snapshot_hash_differs_from_private` | covered |
| T44 | `archmaker-hardware::t44_restricted_namespace_partial_not_full_host_cert` | covered |

Comando: `cargo test -p archmaker-hardware --test t_matrix --locked` y
`cargo test -p archmaker-core --test t_matrix --locked`.

## HA01–HA25

Ubicación: `crates/archmaker-hardware/tests/ha_matrix.rs` (inventario TST-ARCHMAKER-HARDWARE-HA_MATRIX-*).

| Rango | Estado |
| ----- | ------ |
| HA01–HA25 | covered (fixtures deterministas) |
| HA26–HA28 | **not-applicable** (A6 fuera; no passed) |

## Residuales (no cierran acceptance global)

| Residual | Owner | Condición de cierre |
| -------- | ----- | ------------------- |
| CAT01 ApexShot | catálogo / producto | proveedor Arch/AUR verificable con fecha |
| T06 DPI / 3 instalaciones / firma | desktop | entorno F10 + autorización |
| HA26–HA28 | A6 futura | alcance `accepted` + helper privilegiado |
| Aprobación independiente | revisor humano | `approvalStatus` externo ≠ auto |
