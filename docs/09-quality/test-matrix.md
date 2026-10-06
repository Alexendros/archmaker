---
id: DOC-QLT-TEST-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
releaseStatus: ineligible
evidence:
  - cargo test --workspace (79 passed)
  - contracts/json-schema/examples/ (42 + fixtures)
  - crates/archmaker-core/tests/core_contract.rs (17)
  - crates/archmaker-wasm/tests/parity.rs (6)
  - crates/archmaker-core/tests/negative_capabilities.rs (6)
  - crates/archmaker-wasm/tests/invoke_negative.rs (5)
  - tools/check_negative_capabilities.py (14/14)
  - pnpm test-a11y:axe (8/8 Playwright + axe-core)
  - pnpm test-a11y:visual (4/4 Playwright + pixelmatch)
  - cargo clippy/fmt (clean)
  - validate_front_matter.py, validate_traceability.py, gates.py, validate_schema_refs.py, canonicalize.py (all ✓)
owners:
  - quality
reviewers:
  - independent-reviewer
---

# Matriz de pruebas

| Área           | Unit | Property | Golden | Contract | E2E | Fuzz/Fault |
| -------------- | ---: | -------: | -----: | -------: | --: | ---------: |
| Domain         |    ✓ |        ✓ |        |          |     |            |
| Schema         |    ✓ |          |      ✓ |        ✓ |     |          ✓ |
| Rules/resolver |    ✓ |        ✓ |      ✓ |        ✓ |     |          ✓ |
| Migrations     |    ✓ |        ✓ |      ✓ |        ✓ |   ✓ |          ✓ |
| Exporters      |    ✓ |          |      ✓ |        ✓ |   ✓ |            |
| Tauri/WASM     |    ✓ |          |      ✓ |        ✓ |   ✓ |            |
| UI/a11y        |    ✓ |          | visual |          |   ✓ |            |
| Runner         |    ✓ |        ✓ |      ✓ |        ✓ |  VM |          ✓ |
| Enterprise     |    ✓ |          |        |        ✓ |   ✓ |      fault |

## Cobertura real (fase I10, 2026-10-06, T-I10-02)

Medida local con `cargo test --workspace`, corpus `contracts/`, `tools/canonicalize.py`,
`pnpm test-a11y:all` (Playwright + axe-core + pixelmatch). Alcance: Arch Linux x86_64 (DEC-008).

| Tipo                                 | Evidencia ejecutable                                                                                                                                                                                                                                                                           | Resultado                   |
| ------------------------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- |
| Unit Rust                            | `cargo test --workspace`: **79 passed, 0 failed** (domain 5, canonicalization 4, rules 20, resolution 8, core 31, wasm 11)                                                                                                                                                                     | ✓                           |
| Contract schemas                     | `contracts/json-schema/examples/`: 21 válidos + 21 inválidos + fixtures migración; job `json-schema.yml` 4/4 verde                                                                                                                                                                             | ✓                           |
| Corpus operadores                    | `contracts/json-schema/rule-corpus/operators.corpus.json`: 43 casos (12 operadores, positivo + negativo)                                                                                                                                                                                       | ✓                           |
| Golden parity canonicalización       | `tools/canonicalize.py`: 34 vectores OK; paridad Python/JS 33/34 (`e-duplicate-keys` omitida: rechazo no reproducible en JS); `crates/archmaker-canonicalization/tests/vectors.rs`: 4 tests OK                                                                                                 | ✓ con excepción documentada |
| Contract CorePort (7 ops)            | `crates/archmaker-core/tests/core_contract.rs`: **17 tests** (válido + inválido por operación: createDraft 4, loadCatalog 3, saveDraft 2, resolveDraft 2, validateDraft 2, buildManifest 2, exportArtifact 2)                                                                                  | ✓                           |
| Paridad nativo↔WASM CorePort         | `crates/archmaker-wasm/tests/parity.rs`: **6 tests** (canonicalización 3 vectores, createDraft, validateDraft, resolveDraft+buildManifest) byte-idénticos                                                                                                                                      | ✓                           |
| Negativas Tauri/WASM                 | `crates/archmaker-core/tests/negative_capabilities.rs`: **6 tests** (FS scope, errores tipados, capabilities, CSP/shell, web surface); `crates/archmaker-wasm/tests/invoke_negative.rs`: **5 tests** (AM-PROTO-001/002, AM-SCHEMA-001); `tools/check_negative_capabilities.py`: **14/14 PASS** | ✓                           |
| JSON depth limit (TST-NEG-006)       | `crates/archmaker-core/tests/negative_capabilities.rs`: `neg_json_depth_limit_aborts` — nesting > 128 abortado con AM-DOC-001                                                                                                                                                                  | ✓                           |
| Accesibilidad WCAG 2.2 AA (axe-core) | `pnpm test-a11y:axe` (Playwright + @axe-core/playwright): **8/8 PASS** (light/dark/high-contrast, tab order, skip link, live regions, dialog, reduced-motion)                                                                                                                                  | ✓                           |
| Regresión visual                     | `pnpm test-a11y:visual` (Playwright + pixelmatch): **4/4 PASS** (baselines: home-light, home-dark, home-high-contrast, home-reduced-motion)                                                                                                                                                    | ✓                           |
| Lint/format                          | `cargo clippy --workspace --all-targets -- -D warnings` OK; `cargo fmt --check` OK                                                                                                                                                                                                             | ✓                           |
| Audit dependencias                   | `cargo audit` no instalado en runner; `pnpm-lock.yaml` no existe (auditoría JS no ejecutable local); gap I10-GAP-03 registrado                                                                                                                                                                 | pendiente                   |
| Docs/gates/trazabilidad              | `validate_front_matter.py` ✓, `validate_traceability.py` (no-strict) ✓ 0 warnings, `gates.py` ✓, `validate_schema_refs.py` ✓, `canonicalize.py` ✓                                                                                                                                              | ✓                           |

Gaps registrados para I10:

- **I10-GAP-03**: sin `pnpm-lock.yaml` → auditoría JS (`pnpm audit`) no ejecutable; requiere `pnpm install` en CI para generar lockfile.

CI: `.github/workflows/ci.yml` (5 jobs: rust, frontend, docs-python, markdownlint, audit); Renovate configurado (`.github/dependabot.yml`: cargo, npm, github-actions, semanal).

## Golden profiles

| Área           | Unit | Property | Golden | Contract | E2E | Fuzz/Fault |
| -------------- | ---: | -------: | -----: | -------: | --: | ---------: |
| Domain         |    ✓ |        ✓ |        |          |     |            |
| Schema         |    ✓ |          |      ✓ |        ✓ |     |          ✓ |
| Rules/resolver |    ✓ |        ✓ |      ✓ |        ✓ |     |          ✓ |
| Migrations     |    ✓ |        ✓ |      ✓ |        ✓ |   ✓ |          ✓ |
| Exporters      |    ✓ |          |      ✓ |        ✓ |   ✓ |            |
| Tauri/WASM     |    ✓ |          |      ✓ |        ✓ |   ✓ |            |
| UI/a11y        |    ✓ |          | visual |          |   ✓ |            |
| Runner         |    ✓ |        ✓ |      ✓ |        ✓ |  VM |          ✓ |
| Enterprise     |    ✓ |          |        |        ✓ |   ✓ |      fault |

## Alcance de plataforma

Todos los perfiles y pruebas se ejecutan sobre **Arch Linux x86_64** (DEC-008). No se define matriz aarch64; un futuro soporte requeriría decisión, hardware/CI y catálogo verificado.
