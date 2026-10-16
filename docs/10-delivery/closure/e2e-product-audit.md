---
id: DOC-DEL-CLO-E2E-001
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
  - id: DOC-DEL-CLO-FIELD-001
    expectedDocumentStatus: draft
---

# Auditoría E2E de producto (cierre) — selección → export → digest

## Alcance verificable (CONDITIONAL-CLEAR)

| Verificable | Fuera de alcance |
| --- | --- |
| Recorrido NEUBAT 6 pasos | Imagen Arch real / ISO |
| Controles esenciales (rail, cabecera, Continuar, Generar, Exportar) | root, discos, pacstrap, shell |
| `archmaker-profile` bytes íntegros | Instalación en hardware |
| SHA-256(archivo) == `binaryDigest` (toast) | Runner / sidecars |
| Fidelidad F3: hostname + cifrado como **intención** | «La imagen se instala» |

Si el operador pide «imagen se instala»: no aplica bajo el veredicto vigente. Se verifica en su lugar la paridad **selección → export → digest**.

## Inventario (apps/web)

Pasos (`RAIL_STEPS`): Intención → Sistema base → Entorno → Apps → Identidad → Revisión.

Controles esenciales: ver `ESSENTIAL_CONTROLS` en `apps/web/e2e-fidelity-helpers.ts` (asserido en mock-flow).

## Pruebas

| Suite | Qué cubre |
| --- | --- |
| `mock-flow.playwright.test.ts` | Inventario + ejercicio F3 mock |
| `wasm-flow.playwright.test.ts` | Motor real + digest + ejercicio F3 WASM |
| `consented-import` / `export-delivery` / `a11y` | F4 / F2 / axe (ya existentes) |
| `pnpm --dir apps/web test-e2e:fidelity` | Subconjunto inventario + F3 mock |

## Escritorio (F5)

| Comprobación | Resultado |
| --- | --- |
| `pnpm --dir apps/web` vite build → `apps/web/dist` | ok (esta corrida) |
| `cargo build -p archmaker` → ELF `target/debug/archmaker` | ok |
| Tests unitarios `save_export*` (picker simulado) | ok (suite lib) |
| Humo GUI interactivo del diálogo `rfd` | **no ejecutado** — requiere portal de escritorio estable; no se lanza GUI en el agent |

Lógica de cancelar/sobrescribir/atómico: `f5-desktop-notes.md` + tests Tauri. Web verifica descarga + digest (paridad selección→export).

## Comandos

```bash
pnpm --dir apps/web check
pnpm --dir apps/web exec playwright install chromium
pnpm --dir apps/web test-a11y:flow
pnpm --dir apps/web test-e2e:fidelity
pnpm --dir apps/web build:wasm   # si falta pkg
pnpm --dir apps/web test-f7:e2e-wasm
pnpm --dir apps/web test-a11y:axe
```
