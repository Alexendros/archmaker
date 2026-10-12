# ArchMaker

Repositorio de baseline ArchMaker: walking skeleton MVP-0 bajo CONDITIONAL-GO, con documentación de producto, arquitectura, datos, contratos, UX, design system, validación, seguridad, calidad, entrega, operaciones e investigación.

## Estado

- Gate global (mecánico): **No-Go** — no todos los gates G0–G10 están `complete` (faltan G1, G2, G3 y G6). Autoridad: [`docs/00-governance/go-no-go.md`](docs/00-governance/go-no-go.md) y [`docs/10-delivery/gates.md`](docs/10-delivery/gates.md).
- Veredicto de gobierno (R13): **CONDITIONAL-GO — walking skeleton MVP-0 únicamente**. Autoriza el vertical slice de [`docs/10-delivery/walking-skeleton.md`](docs/10-delivery/walking-skeleton.md); el MVP completo, el runner, los privilegios y Enterprise quedan bloqueados.
- Desarrollo funcional: autorizado solo dentro del walking skeleton; no abre MVP-1.
- Fuentes históricas: `reference/v5.1/`, conservadas por hash.
- Rust: autoridad de dominio y casos de uso.
- React: presentación e interacción.
- Tauri 2: adaptador de escritorio.
- WASM: adaptador de navegador.
- Runner privilegiado: diferido (ADR-0004 / DEC-004).

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
