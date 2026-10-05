# ArchMaker

Repositorio documental `planning-v1` para ArchMaker. Este baseline separa producto, arquitectura, datos, contratos, UX, design system, validación, seguridad, calidad, entrega, operaciones e investigación.

## Estado

- Gate global: **No-Go — planificación en curso**.
- Desarrollo funcional: bloqueado hasta aprobar G0–G10.
- Fuentes históricas: `reference/v5.1/`, conservadas por hash.
- Rust: autoridad de dominio y casos de uso.
- React: presentación e interacción.
- Tauri 2: adaptador de escritorio.
- WASM: adaptador de navegador.
- Runner privilegiado: v1, proceso separado.

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

## Regla de reemplazo

Copiar estos archivos por sus rutas canónicas. Antes de reemplazar un documento existente más completo, conservarlo en Git y fusionar requisitos o decisiones aceptadas. Los archivos de `reference/v5.1/` sí deben coincidir byte a byte con este paquete.
