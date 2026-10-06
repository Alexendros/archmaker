---
id: DOC-DEL-AUD-MVP0-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# Auditoría congelada — baseline MVP-0.1

**Rama**: `stabilization/mvp0.1`  
**Commit base**: `4eb7390c009c23ffad3555bf362ea09f8943fba9`  
**Fecha**: 2026-10-06  
**Estado**: PLAN MODE — solo lectura, no ejecutar

## Inventario de evidencia (E0)

### Control de git

- **SHA confirmado**: `4eb7390c009c23ffad3555bf362ea09f8943fba9`
- **Rama actual**: `stabilization/mvp0.1` (derivada de `main` en `4eb7390`)
- **Working tree**: limpio (`git status --porcelain` vacío)

### Workflows (.github/workflows/)

- `canonicalization.yml` — verde
- `ci.yml` — verde
- `docs-validate.yml` — verde
- `json-schema.yml` — verde
- `release.yml` — definida (sin release público)

### Tags anotados

- `implementation-baseline-mvp0` → `68e3e6a` (6 commits detrás de HEAD actual → D-10)
- `planning-v1.1` → `68e3e6a` (confirmado D-10: 6 commits detrás)
- `planning-v1` → `187121a`
- `planning-v1-audit-baseline` → `cf2fa70`

### Crates (Rust workspace, 6 crates)

- `archmaker-core` — 7 operaciones CorePort
- `archmaker-domain` — tipos de dominio con serde
- `archmaker-canonicalization` — RFC 8785 JCS + 12 decisiones perfil v1
- `archmaker-rules` — AST + evaluador de reglas
- `archmaker-resolution` — resolución + capabilities + conflictos
- `archmaker-wasm` — `wasm-bindgen` + parity golden nativo/WASM

### Paquetes (Node.js/pnpm, 1 proyecto)

- `apps/web` — app Tauri/WASM, scripts Playwright, axe, visual regression
- `packages/tokens` — design system tokens

### Schemas y contratos (`contracts/`)

- `json-schema/` — schemas Draft 2020-12: `draft`, `catalog`, `rule`, `diagnostic`, `manifest`, `artifact`, `installation-plan`, `runner-envelope`, `common`, `core-error`, `changeset`
- `events/` — eventos del sistema
- `governance/` — gobernanza contractual
- `openapi/` — spec OpenAPI (Enterprise)
- `protocol/` — protocolos de comunicación
- `test-vectors/` — vectores de prueba canónicos

### Fixtures y tests

- `crates/archmaker-core/fixtures/embedded-catalog.json` — catálogo embebido (sin red)
- `trusted_root.json` — 1585 bytes, _placeholder_ (D-03)
- `apps/web/a11y-playwright.test.ts` — regresión accesibilidad
- `apps/web/visual-regression.test.ts` — regresión visual light/dark

### Validadores locales (OK)

- `python3 tools/validate_front_matter.py` — front matter coherente
- `python3 tools/validate_traceability.py --strict` — trazabilidad estricta
- `python3 tools/gates.py --strict` — gates en verde (parciales)
- `python3 tools/validate_schema_refs.py` — referencias schema OK
- `python3 tools/canonicalize.py` — canonicalización 34 vectores, paridad Python/JS
- `python3 tools/check_negative_capabilities.py` — capacidades negativas

### Comandos de aceptación (pending fix en entorno local)

- `python3 tools/validate_front_matter.py`
- `python3 tools/validate_traceability.py --strict`
- `python3 tools/gates.py --strict`
- `python3 tools/validate_schema_refs.py`
- `python3 tools/canonicalize.py`
- `python3 tools/check_negative_capabilities.py`
- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace --locked`
- `cargo build --workspace --locked`
- `cargo build -p archmaker-wasm --target wasm32-unknown-unknown --locked`
- `corepack pnpm install --frozen-lockfile` — _entorno roto (módulo missing)_
- `corepack pnpm check` — _pending_
- `corepack pnpm test` — _pending_
- `corepack pnpm test:e2e` — _pending_
- `corepack pnpm test:a11y` — _pending_
- `corepack pnpm build:web` — _pending_
- `corepack pnpm build:tauri` — _pending (tauri CLI disponibles, schema por alinear)_
- `corepack pnpm audit --audit-level high` — _pending_
- `cargo audit` — _pending_
- `cosign verify-blob --trusted-root trusted_root.json` — _pending (root placeholder)_

### Deviasiones prioritarias registradas (D-01..D-10)

Ver `docs/00-governance/contradiction-register.md` (CON-013..CON-022).

### Gate E0: Auditoría congelada

| Afirmación                                        | Estado        | Verificación                                                                             |
| ------------------------------------------------- | ------------- | ---------------------------------------------------------------------------------------- |
| V-GIT: `git status --porcelain` vacío + SHA igual | **PASA**      | SHA `4eb7390` confirmado                                                                 |
| V-DOC: estados citan evidencia                    | **PASA**      | walking-skeleton.md + implementation-plan-mvp0.md tienen `not-started` (D-01 confirmado) |
| V-IND: revisor ajeno confirma inventario          | **PENDIENTE** | Revisor independiente aún no valida                                                      |

**Gate E0 pasa** si cada afirmación tiene archivo/comando/workflow asociado; falla ante evidencia narrativa.

## Próximo paso

E1: Reconciliación de control — actualizar README, `implementation-plan-mvp0.md`, `walking-skeleton.md` a `partial/in-review`; transacción plan-status/go-no-go/gates/roadmap/backlog/next-issues; cerrar/superseder issues AUD; validador de contradicciones.
