---
id: DOC-DS-001
phase: MVP
priority: P1
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
releaseStatus: ineligible
owners:
  - ux
reviewers:
  - independent-reviewer
evidence:
  - packages/tokens/tokens.css (tokens canónicos implementados)
  - tools/wcag_validate.py (validación WCAG automatizada)
  - docs/10-delivery/wcag-validation-report.json (evidencia WCAG)
---

# Design system

## Política

El diseño heredado es baseline. Solo se corrigen defectos demostrables de accesibilidad, consistencia, responsividad, seguridad de recursos o mantenibilidad. Toda corrección necesita captura/diff y requisito.

## Arquitectura de tokens

Cuatro niveles resueltos en cascada. El inventario de valores heredados y su mapa a
semánticos viven en `docs/06-design-system/token-migration.md`; aquí se define la
arquitectura, no se duplican valores.

| Nivel      | Prefijo                                                                                                 | Contenido                   | Consumidor        |
| ---------- | ------------------------------------------------------------------------------------------------------- | --------------------------- | ----------------- |
| Primitivo  | `--ds-*`                                                                                                | valor crudo sin significado | tokens semánticos |
| Semántico  | `--color-*`, `--space-*`, `--font-*`, `--radius-*`, `--shadow-*`, `--duration-*`, `--easing-*`, `--z-*` | rol de uso                  | componentes       |
| Componente | `--<componente>-*`                                                                                      | decisiones locales          | CSS de componente |
| Tema       | `.dark`, `.hc`                                                                                          | reasignación de semánticos  | runtime           |

Reglas de niveles:

- T1. Un componente no referencia primitivos directamente; solo semánticos o tokens de
  componente.
- T2. Un tema reasigna semánticos; nunca reescribe primitivos ni componentes.
- T3. Un token de componente deriva de semánticos, salvo valor heredado que se conserva
  con justificación trazable.

### Primitivos

Categorías y prefijos: color (`--ds-color-*`), tipografía (`--ds-font-*`, `--ds-size-*`),
espacio (`--ds-space-*`), radio (`--ds-radius-*`), borde (`--ds-border-*`), sombra
(`--ds-shadow-*`), duración (`--ds-duration-*`), easing (`--ds-easing-*`) y profundidad
(`--ds-z-*`). Los primitivos de color heredados y derivados se registran en
`docs/06-design-system/token-migration.md`.

### Semánticos

| Token                   | Claro                    | Oscuro                   | Rol                         |
| ----------------------- | ------------------------ | ------------------------ | --------------------------- |
| `--color-canvas`        | `--ds-color-neutral-0`   | `--ds-color-neutral-900` | fondo de aplicación         |
| `--color-surface`       | `--ds-color-neutral-50`  | `--ds-color-neutral-850` | superficies elevadas        |
| `--color-surface-hover` | `--ds-color-neutral-100` | `--ds-color-neutral-800` | hover de superficie         |
| `--color-border`        | `--ds-color-neutral-200` | `--ds-color-neutral-200` | bordes (3:1 en ambos temas) |
| `--color-text`          | `--ds-color-neutral-900` | `--ds-color-neutral-0`   | texto principal             |
| `--color-text-muted`    | `--ds-color-neutral-500` | `--ds-color-neutral-350` | texto secundario            |
| `--color-text-subtle`   | `--ds-color-neutral-600` | `--ds-color-neutral-350` | texto de detalle            |
| `--color-action`        | `--ds-color-red-600`     | `--ds-color-red-650`     | acción primaria             |
| `--color-action-hover`  | `--ds-color-red-500`     | `--ds-color-red-400`     | hover de acción             |
| `--color-danger`        | `--ds-color-red-600`     | `--ds-color-red-650`     | error                       |
| `--color-focus`         | `--ds-color-blue-600`    | `--ds-color-blue-350`    | anillo de foco              |
| `--color-info`          | `--ds-color-blue-600`    | `--ds-color-blue-350`    | información                 |
| `--color-success`       | `--ds-color-green-600`   | `--ds-color-green-400`   | éxito                       |
| `--color-warning`       | `--ds-color-yellow-400`  | `--ds-color-yellow-300`  | aviso                       |

Correcciones de accesibilidad (`AUD-021`, no rediseño): el valor heredado de
`--color-text-subtle` no se sobrescribía en tema oscuro y rendía 1.72:1; los
neutros claros se reescalaron (`neutral-200/500/600` actuales) para 3:1/4.5:1; el
tema oscuro usa primitivos dedicados (`neutral-350`, `red-400/650`, `blue-350`,
`green-400`, `yellow-300`) para 3:1/4.5:1 sobre fondo oscuro. El tema `.hc` usa
valores absolutos intencionales (sin primitivo) y mapea a colores de sistema en
`forced-colors`. Evidencia automática en `docs/10-delivery/wcag-validation-report.json`.

### Componentes

| Componente  | Tokens                                                                                                   | Derivan de                   |
| ----------- | -------------------------------------------------------------------------------------------------------- | ---------------------------- |
| button      | `--button-bg`, `--button-fg`, `--button-border`, `--button-radius`                                       | action, text, border, radius |
| option-card | `--option-card-bg`, `--option-card-border`, `--option-card-selected-bg`, `--option-card-selected-border` | surface, border, action      |
| field       | `--field-bg`, `--field-border`, `--field-invalid-border`, `--field-radius`                               | canvas, border, danger       |
| dialog      | `--dialog-bg`, `--dialog-border`, `--dialog-shadow`                                                      | surface, border, shadow      |
| rail        | `--rail-bg`, `--rail-item-current-bg`, `--rail-item-done-fg`                                             | surface, action, success     |
| inspector   | `--inspector-bg`, `--inspector-border`                                                                   | surface, border              |

### Temas

`light` (base en `:root`), `dark` (`.dark`) y `high-contrast` (`.hc`) reasignan los
semánticos de la tabla anterior. Los alias de compatibilidad `--rh-*` apuntan a semánticos
(`--rh-bg: var(--color-canvas)`, etc.) según `docs/06-design-system/token-migration.md`.

## Estructura

```text
src/styles/
├── styles.css
├── tokens/{primitives,semantic,components}.css
├── themes/{light,dark,high-contrast}.css
├── base/{reset,typography,document}.css
├── layout/{app-shell,workspace}.css
├── components/{button,option-card,field,dialog,step-rail,inspector}.css
└── utilities/{accessibility,layout}.css
```

## styles.css

```css
@layer reset, tokens, base, layout, components, utilities, overrides;
@import "./tokens/primitives.css" layer(tokens);
@import "./tokens/semantic.css" layer(tokens);
@import "./tokens/components.css" layer(tokens);
@import "./themes/light.css" layer(tokens);
@import "./themes/dark.css" layer(tokens);
@import "./themes/high-contrast.css" layer(tokens);
@import "./base/reset.css" layer(reset);
@import "./base/typography.css" layer(base);
@import "./base/document.css" layer(base);
@import "./layout/app-shell.css" layer(layout);
@import "./layout/workspace.css" layer(layout);
@import "./components/button.css" layer(components);
@import "./components/option-card.css" layer(components);
@import "./components/field.css" layer(components);
@import "./components/dialog.css" layer(components);
@import "./components/step-rail.css" layer(components);
@import "./components/inspector.css" layer(components);
@import "./utilities/accessibility.css" layer(utilities);
@import "./utilities/layout.css" layer(utilities);
```

## Capas de styles.css

`@layer` fija la precedencia: `reset` < `tokens` < `base` < `layout` < `components` <
`utilities` < `overrides`. Una capa solo contiene lo que le corresponde.

| Capa       | Contiene                                    | No contiene                         | Ejemplo                             |
| ---------- | ------------------------------------------- | ----------------------------------- | ----------------------------------- |
| reset      | normalización de agente de usuario          | tokens ni estilos de marca          | `box-sizing`, `margin: 0`           |
| tokens     | primitivos, semánticos, componentes y temas | selectores de elementos de producto | `--color-canvas`                    |
| base       | tipografía y documento globales             | layout ni componentes               | `body`, encabezados                 |
| layout     | app-shell, workspace, rail y regiones       | apariencia de controles             | grid del workspace                  |
| components | un archivo por componente                   | tokens globales                     | `.option-card`                      |
| utilities  | accesibilidad y utilidades atómicas         | lógica de componente                | `.visually-hidden`, `.focus-ring`   |
| overrides  | excepciones justificadas y trazables        | uso rutinario                       | parche de accesibilidad con `AUD-*` |

Reglas de capas:

- L1. Cada capa importa una sola vez desde `styles.css`; ningún archivo importa CSS de
  otra capa.
- L2. `overrides` es la última capa y solo aloja correcciones justificadas; su contenido
  debe estar vacío salvo excepción documentada.
- L3. Los temas se cargan en `tokens`; no crean capas nuevas.

## Imports CSS por componente

Cada componente importa únicamente su hoja y consume tokens semánticos o de componente.

| Componente | Hoja                         | Tokens consumidos | Depende de        |
| ---------- | ---------------------------- | ----------------- | ----------------- |
| Button     | `components/button.css`      | `--button-*`      | tokens, base      |
| OptionCard | `components/option-card.css` | `--option-card-*` | tokens, utilities |
| Field      | `components/field.css`       | `--field-*`       | tokens, base      |
| Dialog     | `components/dialog.css`      | `--dialog-*`      | tokens, layout    |
| StepRail   | `components/step-rail.css`   | `--rail-*`        | tokens, layout    |
| Inspector  | `components/inspector.css`   | `--inspector-*`   | tokens, layout    |

## Regresión y accesibilidad

- El baseline visual y las golden screenshots se definen en
  `docs/06-design-system/visual-baseline.md`.
- La matriz WCAG 2.2 AA, el foco, el contraste y `prefers-reduced-motion` se verifican
  según `docs/05-ux/accessibility-matrix.md`.
- Toda utilidad de `utilities/accessibility.css` (`.visually-hidden`, `.focus-ring`) es
  obligatoria en los componentes que la referencian en sus contratos
  (`docs/05-ux/component-contracts.md`).
