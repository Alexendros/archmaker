---
id: DOC-DS-001
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

# Design system

## Política

El diseño heredado es baseline. Solo se corrigen defectos demostrables de accesibilidad, consistencia, responsividad, seguridad de recursos o mantenibilidad. Toda corrección necesita captura/diff y requisito.

## Tokens

- Primitivos: color, font, size, space, radius, border, shadow, duration, easing, z-index.
- Semánticos: canvas, surface, text, border, action, focus, success, warning, danger.
- Componentes: button, option-card, field, dialog, rail, inspector.
- Temas: light, dark y high-contrast mediante semánticos.

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
