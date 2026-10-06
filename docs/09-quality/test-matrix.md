---
id: DOC-QLT-TEST-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - quality
reviewers:
  - independent-reviewer
---

# Matriz de pruebas

| Área | Unit | Property | Golden | Contract | E2E | Fuzz/Fault |
|---|---:|---:|---:|---:|---:|---:|
| Domain | ✓ | ✓ |  |  |  |  |
| Schema | ✓ |  | ✓ | ✓ |  | ✓ |
| Rules/resolver | ✓ | ✓ | ✓ | ✓ |  | ✓ |
| Migrations | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Exporters | ✓ |  | ✓ | ✓ | ✓ |  |
| Tauri/WASM | ✓ |  | ✓ | ✓ | ✓ |  |
| UI/a11y | ✓ |  | visual |  | ✓ |  |
| Runner | ✓ | ✓ | ✓ | ✓ | VM | ✓ |
| Enterprise | ✓ |  |  | ✓ | ✓ | fault |

## Cobertura real (fase I10, 2026-10-06, T-I10-02)

Medida local con `cargo test --workspace`, corpus `contracts/` y
`tools/canonicalize.py`. Alcance: Arch Linux x86_64 (DEC-008).

| Tipo | Evidencia ejecutable | Resultado |
|---|---|---|
| Unit Rust | `cargo test --workspace`: 37 passed, 0 failed (rules 20, resolution 8, domain 5, canonicalization-vectors 4) | ✓ parcial |
| Unit Rust (stubs) | `archmaker-core` y `archmaker-wasm`: 0 tests (crates stub, gap I10-GAP-01) | pendiente |
| Contract schemas | `contracts/json-schema/examples/`: 21 válidos + 21 inválidos + fixtures migración; job `json-schema.yml` | ✓ |
| Corpus operadores | `contracts/json-schema/rule-corpus/operators.corpus.json`: 43 casos (positivo + negativo) | ✓ |
| Golden parity canonicalización | `tools/canonicalize.py`: 34 vectores OK; paridad Python/JS 33/34 (`e-duplicate-keys` omitida: rechazo no reproducible en JS); `vectors.rs`: 4 tests OK | ✓ con excepción documentada |
| Contract CorePort (7 ops) | Sin tests de contrato por operación (gap I10-GAP-01) | pendiente |
| Visual regression | Sin app ni baseline ejecutable (gap I10-GAP-02) | pendiente |
| Accesibilidad WCAG 2.2 AA | axe no cableado; evidencia solo documental en `docs/05-ux/accessibility-matrix.md` (gap I10-GAP-02) | pendiente |
| Lint/format | `cargo clippy --workspace -- -D warnings` OK; `cargo fmt --check` FALLA local (diffs en `archmaker-core`, `archmaker-domain`; ver `cargo fmt --check`) | 1 fallo preexistente |
| Audit dependencias | `cargo audit` no instalado; `npm/pnpm audit` sin `pnpm-lock.yaml` (gap I10-GAP-03) | no ejecutable local |
| Docs/gates/trazabilidad | `validate_front_matter.py`, `validate_traceability.py` (no-strict, 8 advertencias conocidas), `gates.py`, `validate_schema_refs.py` | ✓ |

Gaps registrados para I10: I10-GAP-01 (tests `archmaker-core`/`archmaker-wasm` + contratos
CorePort), I10-GAP-02 (frontend: sin script `check`, sin visual/a11y ejecutable),
I10-GAP-03 (sin `pnpm-lock.yaml`: auditoría JS no ejecutable). CI: `.github/workflows/ci.yml`;
Renovate ya configurado (`.github/dependabot.yml`: cargo, npm, github-actions, semanal).

## Golden profiles

| Área | Unit | Property | Golden | Contract | E2E | Fuzz/Fault |
|---|---:|---:|---:|---:|---:|---:|
| Domain | ✓ | ✓ |  |  |  |  |
| Schema | ✓ |  | ✓ | ✓ |  | ✓ |
| Rules/resolver | ✓ | ✓ | ✓ | ✓ |  | ✓ |
| Migrations | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| Exporters | ✓ |  | ✓ | ✓ | ✓ |  |
| Tauri/WASM | ✓ |  | ✓ | ✓ | ✓ |  |
| UI/a11y | ✓ |  | visual |  | ✓ |  |
| Runner | ✓ | ✓ | ✓ | ✓ | VM | ✓ |
| Enterprise | ✓ |  |  | ✓ | ✓ | fault |

## Golden profiles

Minimal UEFI/ext4; GNOME/Btrfs; Hyprland/AMD; Sway/NVIDIA conflict; gaming/multilib; dev/Podman; v5.1 migration; aliases; catálogo manipulado; target incompatible.

## Alcance de plataforma

Todos los perfiles y pruebas se ejecutan sobre **Arch Linux x86_64** (DEC-008). No se define matriz aarch64; un futuro soporte requeriría decisión, hardware/CI y catálogo verificado.
