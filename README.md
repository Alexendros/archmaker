# ArchMaker

Configurador local para diseñar, validar y exportar un perfil. Rust es la autoridad semántica; la interfaz es React en Tauri y WASM. Este repositorio contiene la evidencia reproducible hasta la baseline `implementation-baseline-mvp5` y el cierre funcional F0–F7 en `main`: CorePort, adapters, canonicalización, reglas, resolución, manifest, export, security, CI y supply chain. No instala el sistema ni ejecuta un plan. El runner v1 y los privilegios se autorizan en fases posteriores.

## Estado

Tres capas distintas (detalle: [`docs/10-delivery/closure/f8-governance.md`](docs/10-delivery/closure/f8-governance.md)):

| Capa                 | Estado                                                                                                                             |
| -------------------- | ---------------------------------------------------------------------------------------------------------------------------------- |
| Baseline de código   | **passed** — tag `implementation-baseline-mvp5`; F0–F8 fusionados en `main`                                                        |
| Aprobación funcional | **pending** — matriz F7 ejecutable; sin `RC_SHA_FUNCIONAL` ni aceptación independiente (`f7-candidate.md`)                         |
| Release binaria      | **published** — [Neubat v0.1.0](https://github.com/Alexendros/archmaker/releases/tag/v0.1.0) AppImage x86_64; verificación parcial |
| Versión de código    | **0.2.0** — tag `v0.2.0`; sin GitHub Release de paquete hasta un AppImage validado con tests                                       |

- Código actual de `main` y el AppImage publicado son identidades distintas. La versión de
  código es `0.2.0`. El tag `v0.1.0` no se mueve y sigue siendo la única release binaria.
- Canal publicado: AppImage Linux x86_64 (`v0.1.0` como **Neubat**); nombre visible vigente **Forge**. `RC_SHA` de la release:
  `9297ed7cc47f42a9611aed0093b5ab040daca512`; `ARTIFACT_SHA256`
  `6567464f5e2f21398fd3eef35592f0e9d377fd2cc7887c1e46df092b5a8e6d81`. Changelog: [`CHANGELOG.md`](CHANGELOG.md).
- Limitaciones honestas de v0.1.0: cosign local y tres instalaciones GUI siguen residuales F9;
  F7 sin `RC_SHA_FUNCIONAL`; sin imagen Arch / runner / `.deb`/`.rpm`. Detalle:
  [`docs/10-delivery/closure/f9-release.md`](docs/10-delivery/closure/f9-release.md).
- Diagramas Archify (Forge as-built): [`docs/02-architecture/diagrams/`](docs/02-architecture/diagrams/).
  Archify es herramienta de diagramación; no confundir con Forge.
- Gate global (mecánico): **Clear** — todos los gates G0–G10 están `complete`. Autoridad: [`docs/00-governance/go-no-go.md`](docs/00-governance/go-no-go.md) y [`docs/10-delivery/gates.md`](docs/10-delivery/gates.md).
- Veredicto de gobierno: **CONDITIONAL-CLEAR** hasta MVP-5, más el slice ISO-01 de imagen viva x86_64 UEFI. Autoriza el walking skeleton, los slices MVP-1..MVP-5 y el canal AppImage de MVP-6. Siguen bloqueados el runner de instalación, la escritura de discos, la shell arbitraria y Enterprise.
- Baseline vigente: `implementation-baseline-mvp5` (commit anotado `b13c200…`; tags previos intactos). Remediación operativa: [`docs/10-delivery/remediation/`](docs/10-delivery/remediation/).
- Alcance MVP-5 **accepted** y cerrado (MVP-5-01..05): [`docs/10-delivery/mvp5-scope.md`](docs/10-delivery/mvp5-scope.md). Hueco residual: [`mvp5-gap-spec.md`](docs/10-delivery/mvp5-gap-spec.md) (draft). Empaquetado MVP-6 **accepted solo AppImage** ([`mvp6-scope.md`](docs/10-delivery/mvp6-scope.md)); imagen viva [`iso-scope.md`](docs/10-delivery/iso-scope.md) **accepted** (ISO-01). El target `archmaker-live-profile` está en el árbol; la cara no invoca `mkarchiso`.
- Fuentes históricas: `reference/v5.1/`, conservadas por hash; walking skeleton en [`docs/10-delivery/walking-skeleton.md`](docs/10-delivery/walking-skeleton.md).
- Rust: autoridad de dominio y casos de uso (nativo + WASM).
- React: cara Forge; escritorio → `TauriCorePort` (`invoke` de `@tauri-apps/api/core`); navegador → `WasmCorePort`. `MockCorePort` solo con `?mock=1` en desarrollo.
- Tauri 2: ACL vía `AppManifest::commands` + capabilities mínimas; `check_invoke` como segunda capa; sin `shell:*`.
- WASM: módulo real (`tools/build-wasm.sh`); sesión `RealCore` con catálogo desktop.
- Runner privilegiado: diferido (ADR-0004 / DEC-004).
- Manual de uso: [`docs/11-operations/manual/`](docs/11-operations/manual/) y [wiki](https://github.com/Alexendros/archmaker/wiki).
- Instalación de Forge: [`docs/11-operations/manual/Instalacion-Forge.md`](docs/11-operations/manual/Instalacion-Forge.md).
- Desktop/offline del tip: [`docs/10-delivery/closure/p6-desktop-offline.md`](docs/10-delivery/closure/p6-desktop-offline.md) (**blocked** sin binario prod).

## Documentación

- Índice documental: [`docs/index.md`](docs/index.md).
- Estado del plan: [`docs/00-governance/plan-status.md`](docs/00-governance/plan-status.md).
- Resultado Go/No-Go: [`docs/00-governance/go-no-go.md`](docs/00-governance/go-no-go.md).
- Gobierno del cierre (F8): [`docs/10-delivery/closure/f8-governance.md`](docs/10-delivery/closure/f8-governance.md).
- Diagramas verificables: [`docs/02-architecture/diagrams/index.md`](docs/02-architecture/diagrams/index.md).
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
