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
  - cargo test --workspace --locked (86 passed, 0 failed, 19 suites)
  - contracts/json-schema/examples/ (42 + fixtures)
  - crates/archmaker-core/tests/core_contract.rs (17)
  - crates/archmaker-wasm/tests/parity.rs (6)
  - crates/archmaker-core/tests/negative_capabilities.rs (12)
  - crates/archmaker-wasm/tests/invoke_negative.rs (5)
  - tools/check_negative_capabilities.py (18/18)
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

| Tipo                                 | Evidencia ejecutable                                                                                                                                                                                                                                                                            | Resultado                   |
| ------------------------------------ | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- | --------------------------- |
| Unit Rust                            | `cargo test --workspace --locked`: **86 passed, 0 failed** en 19 suites                                                                                                                                                                                                                         | ✓                           |
| Contract schemas                     | `contracts/json-schema/examples/`: 21 válidos + 21 inválidos + fixtures migración; job `json-schema.yml` 4/4 verde                                                                                                                                                                              | ✓                           |
| Corpus operadores                    | `contracts/json-schema/rule-corpus/operators.corpus.json`: 43 casos (12 operadores, positivo + negativo)                                                                                                                                                                                        | ✓                           |
| Golden parity canonicalización       | `tools/canonicalize.py`: 34 vectores OK; paridad Python/JS 33/34 (`e-duplicate-keys` omitida: rechazo no reproducible en JS); `crates/archmaker-canonicalization/tests/vectors.rs`: 4 tests OK                                                                                                  | ✓ con excepción documentada |
| Contract CorePort (7 ops)            | `crates/archmaker-core/tests/core_contract.rs`: **17 tests** (válido + inválido por operación: createDraft 4, loadCatalog 3, saveDraft 2, resolveDraft 2, validateDraft 2, buildManifest 2, exportArtifact 2)                                                                                   | ✓                           |
| Paridad nativo↔WASM CorePort         | `crates/archmaker-wasm/tests/parity.rs`: **6 tests** (canonicalización 3 vectores, createDraft, validateDraft, resolveDraft+buildManifest) byte-idénticos                                                                                                                                       | ✓                           |
| Negativas Tauri/WASM                 | `crates/archmaker-core/tests/negative_capabilities.rs`: **12 tests** (FS scope, errores tipados, capabilities, CSP/shell, web surface); `crates/archmaker-wasm/tests/invoke_negative.rs`: **5 tests** (AM-PROTO-001/002, AM-SCHEMA-001); `tools/check_negative_capabilities.py`: **18/18 PASS** | ✓                           |
| JSON depth limit (TST-NEG-006)       | `crates/archmaker-core/tests/negative_capabilities.rs`: `neg_json_depth_limit_aborts` — nesting > 128 abortado con AM-DOC-001                                                                                                                                                                   | ✓                           |
| Accesibilidad WCAG 2.2 AA (axe-core) | `pnpm test-a11y:axe` (Playwright + @axe-core/playwright): **8/8 PASS** (light/dark/high-contrast, tab order, skip link, live regions, dialog, reduced-motion)                                                                                                                                   | ✓                           |
| Regresión visual                     | `pnpm test-a11y:visual` (Playwright + pixelmatch): **4/4 PASS** (baselines: home-light, home-dark, home-high-contrast, home-reduced-motion)                                                                                                                                                     | ✓                           |
| Lint/format                          | `cargo clippy --workspace --all-targets -- -D warnings` OK; `cargo fmt --check` OK                                                                                                                                                                                                              | ✓                           |
| Audit dependencias                   | `cargo-audit` no instalado localmente; el job `audit` de CI lo instala y ejecuta `cargo audit`, por lo que la evidencia Rust queda pendiente de CI; `pnpm-lock.yaml` existe en la raíz y `pnpm audit --audit-level high` local no reporta vulnerabilidades conocidas                            | parcial                     |
| Docs/gates/trazabilidad              | `validate_front_matter.py` ✓, `validate_traceability.py` (no-strict) ✓ 0 warnings, `gates.py` ✓, `validate_schema_refs.py` ✓, `canonicalize.py` ✓                                                                                                                                               | ✓                           |

Gaps registrados para I10:

- **I10-GAP-03 (cerrado)**: el antiguo bloqueo por falta de `pnpm-lock.yaml` ya no aplica; el lockfile existe en la raíz y la auditoría JS local pasa.
- **Brecha abierta Rust**: falta evidencia ejecutable de `cargo audit` en CI (`cargo-audit` no está instalado localmente).
- **I10-GAP-04 (abierto)**: la automatización actual es Dependabot (`.github/dependabot.yml`: cargo, npm, github-actions, semanal); el plan pide Renovate y esa migración sigue como observación de estándar. Sin decisión que acepte Dependabot o migre a Renovate, el criterio “Renovate activo” no está cumplido.

CI: `.github/workflows/ci.yml` (8 jobs: rust, frontend, docs-python, traceability, a11y-automated, visual-regression, markdownlint, audit); dependencias actuales con Dependabot, Renovate pendiente según el plan/estándar.

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
