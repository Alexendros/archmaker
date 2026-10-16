# ArchMaker

Configurador local para diseñar, validar y exportar un perfil. Rust es la autoridad semántica; la interfaz es React en Tauri y WASM. Este repositorio contiene la evidencia reproducible hasta la baseline `implementation-baseline-mvp5` y el cierre funcional F0–F7 en `main`: CorePort, adapters, canonicalización, reglas, resolución, manifest, export, security, CI y supply chain. No instala el sistema ni ejecuta un plan. El runner v1 y los privilegios se autorizan en fases posteriores.

## Estado

Tres capas distintas (detalle: [`docs/10-delivery/closure/f8-governance.md`](docs/10-delivery/closure/f8-governance.md)):

| Capa                 | Estado                                                                                                     |
| -------------------- | ---------------------------------------------------------------------------------------------------------- |
| Baseline de código   | **passed** — tag `implementation-baseline-mvp5`; F0–F7 fusionados en `main`                                |
| Aprobación funcional | **pending** — matriz F7 ejecutable; sin `RC_SHA_FUNCIONAL` ni aceptación independiente (`f7-candidate.md`) |
| Release binaria      | **blocked** — MVP-6 `in-review`, ADR-0010 `proposed`; F9 no abierto                                        |

- Gate global (mecánico): **Clear** — todos los gates G0–G10 están `complete`. Autoridad: [`docs/00-governance/go-no-go.md`](docs/00-governance/go-no-go.md) y [`docs/10-delivery/gates.md`](docs/10-delivery/gates.md).
- Veredicto de gobierno: **CONDITIONAL-CLEAR** hasta MVP-5 (sin runner). Autoriza el walking skeleton y los slices MVP-1..MVP-5; runner, privilegios, ISO y Enterprise siguen bloqueados.
- Baseline vigente: `implementation-baseline-mvp5` (commit anotado `b13c200…`; tags previos intactos). Remediación operativa: [`docs/10-delivery/remediation/`](docs/10-delivery/remediation/).
- Alcance MVP-5 **accepted** y cerrado (MVP-5-01..05): [`docs/10-delivery/mvp5-scope.md`](docs/10-delivery/mvp5-scope.md). Hueco residual: [`mvp5-gap-spec.md`](docs/10-delivery/mvp5-gap-spec.md) (draft). Empaquetado ([`mvp6-scope.md`](docs/10-delivery/mvp6-scope.md)) e ISO siguen `in-review` — **no** autorizan F9.
- Fuentes históricas: `reference/v5.1/`, conservadas por hash; walking skeleton en [`docs/10-delivery/walking-skeleton.md`](docs/10-delivery/walking-skeleton.md).
- Rust: autoridad de dominio y casos de uso (nativo + WASM).
- React: cara NEUBAT; escritorio → `TauriCorePort` (`invoke` de `@tauri-apps/api/core`); navegador → `WasmCorePort`. `MockCorePort` solo con `?mock=1` en desarrollo.
- Tauri 2: ACL vía `AppManifest::commands` + capabilities mínimas; `check_invoke` como segunda capa; sin `shell:*`.
- WASM: módulo real (`tools/build-wasm.sh`); sesión `RealCore` con catálogo desktop.
- Runner privilegiado: diferido (ADR-0004 / DEC-004).
- Manual de uso: [`docs/11-operations/manual/`](docs/11-operations/manual/) y [wiki](https://github.com/Alexendros/archmaker/wiki).

## Documentación

- Índice documental: [`docs/index.md`](docs/index.md).
- Estado del plan: [`docs/00-governance/plan-status.md`](docs/00-governance/plan-status.md).
- Resultado Go/No-Go: [`docs/00-governance/go-no-go.md`](docs/00-governance/go-no-go.md).
- Gobierno del cierre (F8): [`docs/10-delivery/closure/f8-governance.md`](docs/10-delivery/closure/f8-governance.md).
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
