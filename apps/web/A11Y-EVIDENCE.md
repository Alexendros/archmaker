# Evidencia WCAG 2.2 AA — apps/web v0 (I8, T-I8-04)

Fecha: 2026-10-06 · Alcance: flujo crear draft → seleccionar → resolver → validar →
manifest → exportar, temas claro/oscuro. Criterio: `docs/05-ux/accessibility-matrix.md`.

## Automatizada — `python3 apps/web/a11y-check.py`: 19/19 PASS

| Grupo                                                                      | Resultado |
| -------------------------------------------------------------------------- | --------- |
| Contraste texto/canvas claro 18.26:1, oscuro 18.26:1 (umbral 4.5)          | PASS ×2   |
| Contraste secundario claro 5.13:1, oscuro 7.68:1 (umbral 4.5)              | PASS ×2   |
| Contraste detalle claro 10.60:1, oscuro corregido 7.68:1 (umbral 4.5)      | PASS ×2   |
| Contraste foco 5.57:1 (umbral 3.0), CTA blanco/acción 4.53:1 (umbral 4.5)  | PASS ×2   |
| Foco visible `--color-focus`, `prefers-reduced-motion`, `lang="es"`        | PASS ×3   |
| Skip link, landmarks `nav`/`main`/`aside`, regiones vivas `status`+`alert` | PASS ×3   |
| `dialog` con `aria-labelledby`, OptionCard `checkbox`+`aria-checked`       | PASS ×2   |
| `aria-current="step"`, sin `div` interactivo sin rol, objetivos táctiles   | PASS ×3   |

## Automatizada axe-core — `pnpm test-a11y:axe` (Playwright): 8/8 PASS

| Test                                                       | Resultado |
| ---------------------------------------------------------- | --------- |
| Home page - light theme (wcag2aa, wcag21aa, best-practice) | PASS      |
| Home page - dark theme                                     | PASS      |
| Home page - high contrast (forcedColors)                   | PASS      |
| Keyboard navigation - tab order                            | PASS      |
| Skip link works                                            | PASS      |
| Live regions present (status + alert)                      | PASS      |
| Dialog accessibility (aria-labelledby, aria-modal)         | PASS      |
| Reduced motion respected                                   | PASS      |

## Regresión visual — `pnpm test-a11y:visual` (Playwright + pixelmatch): 4/4 PASS

| Baseline            | Resultado              |
| ------------------- | ---------------------- |
| home-light          | PASS (creado baseline) |
| home-dark           | PASS (creado baseline) |
| home-high-contrast  | PASS (creado baseline) |
| home-reduced-motion | PASS (creado baseline) |

## Correcciones aplicadas desde la matriz

- `.dark --color-text-subtle: #a8a8a8` (heredado 1.72:1 → 7.68:1, AUD-021).
- Anillo `:focus-visible` 3px `--color-focus` ≥ 3:1 sobre canvas.
- `prefers-reduced-motion: reduce` desactiva transiciones en `tokens.css`.
- Warning/borde nunca solos como portadores: OptionCard y ValidationPanel añaden icono+texto.

## Pendiente (requiere entorno con navegador)

- Recorrido manual con teclado y lector (NVDA/WebView2, VoiceOver/WKWebView, Orca/WebKitGTK).
- Zoom 200%/400% y reflow 320 CSS px con render real.

Estado global WCAG: automatizado PASS (19 + 8 + 4 = 31 checks); conformidad AA completa `pendiente` (revisión manual no ejecutable en CI sin hardware).
