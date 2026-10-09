# ArchMaker

Repositorio de baseline MVP-0.1 — walking skeleton stabilizado. Este repositorio contiene la evidencia reproducible del vertical slice: CorePort, adapters Tauri/WASM, canonicalización, reglas, resolución, manifest, export, security, CI y supply chain. El producto completo, runner v1 y los privilegios se autorizan en fases posteriores sobre la baseline.

## Estado

- Gate global: **Clear** mecánico (G0–G10 `complete`) / **CONDITIONAL-CLEAR — walking skeleton MVP-0 únicamente**.
- Desarrollo funcional: autorizado walking skeleton MVP-0.1 (rebanada vertical, sin runner ni privilegios).
- Siguiente fase planificada: MVP-1 (domain/schema/catalog/migrations) — ver `docs/10-delivery/mvp1-scope.md`.
- Fuentes históricas: `reference/v5.1/`, conservadas por hash; walking skeleton en `docs/10-delivery/walking-skeleton.md` y `docs/10-delivery/implementation-plan-mvp0.md`.
- Rust: autoridad de dominio y casos de uso (compilable nativo + WASM).
- React: presentación e interacción (adapters Tauri y WASM).
- Tauri 2: adaptador de escritorio (deny-by-default, scopes acotados, sin `shell:*`).
- WASM: adaptador de navegador (paridad golden nativo/WASM).
- Runner privilegiado: diferido a fase posterior a baseline MVP-0.1.

## Documentación

- Índice documental: [`docs/index.md`](docs/index.md).
- Estado del plan: [`docs/00-governance/plan-status.md`](docs/00-governance/plan-status.md).
- Resultado Go/No-Go: [`docs/00-governance/go-no-go.md`](docs/00-governance/go-no-go.md).
- Cobertura de gates: [`docs/10-delivery/gates.md`](docs/10-delivery/gates.md).
- Próximos issues: [`docs/10-delivery/next-issues.md`](docs/10-delivery/next-issues.md).

## Lectura

1. `AGENTS.md`
2. `docs/00-governance/plan-status.md`
3. `docs/00-governance/decision-register.md`
4. `docs/00-governance/contradiction-register.md`
5. `docs/01-product/personas.md`
6. `docs/01-product/use-cases.md`
7. `docs/02-architecture/c4/README.md`
8. `docs/03-data/canonicalization.md`
9. `docs/04-interfaces/errors-events.md`
10. `docs/05-ux/interaction-matrix.md`
11. `docs/07-validation/rule-inventory.md`
12. `docs/08-security/privilege-model.md`
13. `docs/10-delivery/gates.md`

## Estructura

- `docs/00-governance` … `docs/12-research`: documentación por área.
- `contracts/`: JSON Schema, protocolo, eventos y OpenAPI (Enterprise).
- `reference/v5.1/`: evidencia heredada inmutable (verificada por SHA-256).
- `templates/`: plantillas de ADR, requisito e issue.
- `.github/`: CI de validación documental, plantillas de issue y PR.

## Regla de reemplazo

Copiar estos archivos por sus rutas canónicas. Antes de reemplazar un documento existente más completo, conservarlo en Git y fusionar requisitos o decisiones aceptadas. Los archivos de `reference/v5.1/` sí deben coincidir byte a byte con este paquete.

## Licencia

Apache-2.0. Ver [`LICENSE`](LICENSE).
