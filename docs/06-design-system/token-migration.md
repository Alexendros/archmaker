---
id: DOC-DS-TOKEN-001
phase: MVP
priority: P1
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - ux
reviewers:
  - independent-reviewer
---

# Migración de tokens heredados

ID `DOC-DS-TOKEN-001` · Estado `in-review` · Propietario UX/diseño · Fecha 2026-10-06.

Inventario de los tokens de diseño heredados de `reference/v5.1/` y mapa
`valor legacy → token semántico` (fase R10, issue `AUD-020`). Preserva los tokens sin
rediseñar: cada valor heredado se conserva y toda divergencia queda justificada y trazable.
La arquitectura de tokens vive en `docs/06-design-system/design-system.md`; la evidencia de
accesibilidad, en `docs/05-ux/accessibility-matrix.md`.

## Fuentes heredadas (inmutables)

| Fuente | Rol visual | Define |
|---|---|---|
| `reference/v5.1/index.html` | Artefacto v5.1 (Red Hat Design + React) | `:root`, `.dark`, objeto `palette` |
| `reference/v5.1/App.tsx` | Fuente React del artefacto v5.1 | `:root`, `.dark`, objeto `palette` |
| `reference/v5.1/forge_v4_final.html` | Diseño base v4.1 (histórico) | `:root`, `.dark` y hex literales |

Ninguna fuente de `reference/` se modifica (evidencia inmutable,
`docs/00-governance/G0-SOURCE-DISPOSITION.md`).

## Inventario de tokens heredados

Definidos en `:root` y idénticos en las tres fuentes (`index.html:11`, `App.tsx:391`,
`forge_v4_final.html:11`):

| Token legacy | Hex | oklch | Rol declarado (`palette.usage`) | Token semántico destino |
|---|---|---|---|---|
| `--rh-red` | `#ee0000` | `oklch(0.62 0.24 29)` | primary action | `--color-action`, `--color-danger` |
| `--rh-blue` | `#0066cc` | `oklch(0.55 0.18 250)` | focus info | `--color-focus`, `--color-info` |
| `--rh-green` | `#3e8635` | `oklch(0.60 0.15 145)` | success | `--color-success` |
| `--rh-yellow` | `#f0ab00` | `oklch(0.80 0.16 85)` | warning | `--color-warning` |
| `--rh-bg` | `#ffffff` | `oklch(1 0 0)` | background | `--color-canvas` |
| `--rh-surface` | `#f5f5f5` | `oklch(0.97 0 0)` | surface | `--color-surface` |
| `--rh-border` | `#d2d2d2` | `oklch(0.88 0 0)` | border | `--color-border` |
| `--rh-black` | `#151515` | — | texto/ink (implícito) | `--color-text` |
| `--rh-gray` | `#6a6e73` | — | texto secundario (implícito) | `--color-text-muted` |
| `--rh-gray-dark` | `#3c3f42` | — | texto de detalle (implícito) | `--color-text-subtle` |

Los siete primeros coinciden con el objeto `palette.tokens` (`App.tsx:180-186`,
`index.html:203-209`), que declara `canonical: "oklch"` y conserva `hex` como equivalente.
`--rh-black`, `--rh-gray` y `--rh-gray-dark` se definen en `:root` pero no figuran en
`palette.tokens`; su rol se infiere del uso (`text-[var(--rh-gray-dark)]`,
`text-[var(--rh-gray)]`).

## Sobrescrituras de tema oscuro

Bloque `.dark` (`index.html:12`, `App.tsx:392`, `forge_v4_final.html:12`):

| Token | Claro | Oscuro |
|---|---|---|
| `--rh-bg` | `#ffffff` | `#151515` |
| `--rh-surface` | `#f5f5f5` | `#1e1e1e` |
| `--rh-border` | `#d2d2d2` | `#2a2a2a` |
| `--rh-black` | `#151515` | `#ffffff` |
| `--rh-gray` | `#6a6e73` | `#a8a8a8` |
| `--rh-gray-dark` | `#3c3f42` | sin sobrescritura |
| `--rh-red`, `--rh-blue`, `--rh-green`, `--rh-yellow` | invariantes | invariantes |

Defecto heredado confirmado por cálculo de contraste: `--rh-gray-dark` (`#3c3f42`) sobre
`--rh-bg` oscuro (`#151515`) rinde **1.72:1** y sobre `--rh-surface` oscuro (`#1e1e1e`)
**1.57:1**. Incumple WCAG 2.2 `1.4.3` (texto) y `1.4.11` (no textual). Se registra como
corrección de accesibilidad trazable (`AUD-021`), no como rediseño; el tema oscuro debe
sobrescribir `--color-text-subtle`.

## Valores derivados sin token (divergencia heredada)

`forge_v4_final.html` mezcla tokens con hex literales en utilidades Tailwind arbitrarias
(`bg-[#ee0000]`, `bg-[#fff5f5]`, `bg-[#2a1515]`). Se conservan exactamente y se registran
como primitivos derivados:

| Hex literal | Uso heredado | Primario propuesto |
|---|---|---|
| `#fff5f5` | fondo de opción seleccionada (claro) | `--ds-color-red-50` |
| `#2a1515` | fondo de opción seleccionada (oscuro) | `--ds-color-red-950` |
| `#f0f6ff` | fondo informativo | `--ds-color-blue-50` |
| `#9ecaff` | borde informativo | `--ds-color-blue-200` |
| `#1e3a5f` | texto/borde informativo oscuro | `--ds-color-blue-900` |
| `#2d5aa0` | variante informativa | `--ds-color-blue-700` |
| `#3a6bc5` | variante informativa | `--ds-color-blue-500` |
| `#fff8e6` | fondo de aviso | `--ds-color-yellow-50` |
| `#f0f0f0` | superficie alterna | `--ds-color-neutral-100` |
| `#e5e7eb` | borde alterno | `--ds-color-neutral-150` |

Adoptar estos primarios es **normalizar valores existentes**, no cambiar la apariencia:
mantienen el hex literal. Cualquier valor nuevo que no provenga de `reference/` exige
decisión y captura de diff.

## Reglas de preservación y migración

- P1. Los diez tokens `--rh-*` permanecen como **alias de compatibilidad**
  (`--rh-*: var(--color-*)`) en la capa de tokens. El artefacto heredado resuelve sin
  cambios visuales.
- P2. El código nuevo no consume `--rh-*` directamente; consume tokens semánticos o de
  componente.
- P3. `--rh-red` cumple doble rol heredado (acción primaria y peligro/error). Se mantiene
  la equivalencia de valor; separar ambos roles es una decisión pendiente, no una
  corrección silenciosa.
- P4. Toda divergencia visual futura se justifica por accesibilidad, consistencia,
  responsividad, seguridad de recursos o mantenibilidad, y se enlaza a `AUD-*` con
  captura y diff.
- P5. La paridad visual se verifica contra `docs/06-design-system/visual-baseline.md`.

## Trazabilidad

| Elemento | Referencia |
|---|---|
| Inventario de origen | `docs/12-research/inventory-v5.1.md` §5 y §7 |
| Disposición de fuentes | `docs/00-governance/G0-SOURCE-DISPOSITION.md` |
| Baseline visual | `AUD-020` (R10) |
| Accesibilidad | `AUD-021` (R10) |
| Requisito | `NFR-ACC-001` (`docs/01-product/requirements.md`) |
| Gate | G6 (`docs/10-delivery/gates.md`) |
