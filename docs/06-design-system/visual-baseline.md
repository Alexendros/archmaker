---
id: DOC-DS-VISUAL-001
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

# Baseline visual de referencia

ID `DOC-DS-VISUAL-001` · Estado `in-review` · Propietario UX/diseño · Fecha 2026-10-06.

Define el baseline visual aprobado y el protocolo de captura de las golden reference
screenshots (fase R10, issue `AUD-020`). El baseline es el **diseño heredado**: no se
rediseña. La referencia canónica es `reference/v5.1/index.html` (artefacto v5.1), con
`reference/v5.1/App.tsx` como su fuente React y `reference/v5.1/forge_v4_final.html` como
diseño base v4.1 histórico. El mapa de tokens está en
`docs/06-design-system/token-migration.md`.

## Referencia aprobada

| Fuente | Autoridad visual | Uso |
|---|---|---|
| `reference/v5.1/index.html` | Primaria | Baseline de pantallas y estados v5.1 |
| `reference/v5.1/App.tsx` | Primaria (fuente) | Contrato de composición React |
| `reference/v5.1/forge_v4_final.html` | Secundaria (histórica) | Evidencia v4.1; no normativa |

El artefacto heredado depende de recursos remotos (Tailwind CDN, React/Babel en `unpkg`,
Google Fonts), prohibidos en runtime (`DEC-009`). Por tanto el baseline se define **por
referencia y protocolo**, no por render en vivo; las capturas se producirán cuando exista
un build sin CDN y se aprobarán contra esta definición. Hasta entonces el estado de
verificación es `not-verified`.

## Matriz de pantallas y estados a capturar

Rutas tomadas de `docs/05-ux/interaction-matrix.md`:

| # | Pantalla | Ruta | Estado base | Estados adicionales |
|---|---|---|---|---|
| GS-01 | Inicio | `/` | Por defecto | Vacío (sin borradores recientes) |
| GS-02 | Nuevo | `/new` | Por defecto | Preset seleccionado |
| GS-03 | Configurar | `/configure/:step` | Step actual | Selección manual, derivada, bloqueada, incompatible, warning |
| GS-04 | Revisar | `/review` | Sin diagnósticos | Error bloqueante, warning, diff |
| GS-05 | Exportar | `/export` | Target por defecto | Target inválido, artefacto listo |
| GS-06 | Importar | `/import` | Vacío | Archivo cargado, migración con avisos |
| GS-07 | Plan v1 | `/runner/plan` | Plan propuesto | Riesgo elevado, privilegio requerido |

Estados transversales por captura: **claro y oscuro**; foco visible; validación con error
y con aviso; modales (detalle de paquete, glosario, paleta de comandos `⌘K`); toast.

## Viewports y condiciones

| Condición | Valor | Criterio asociado |
|---|---|---|
| Escritorio | 1280×800 | Baseline principal |
| Portátil | 1024×768 | Layout intermedio |
| Tablet | 768×1024 | Rail colapsado |
| Móvil | 390×844 | Stepper compacto |
| Reflow | 320 CSS px | WCAG 2.2 `1.4.10` |
| Zoom | 200% y 400% | WCAG 2.2 `1.4.4`, `1.4.10` |
| Movimiento reducido | `prefers-reduced-motion: reduce` | WCAG 2.2 `2.3.1`, `2.2.2` |

## Protocolo de captura y paridad

- C1. Cada captura se nombra `GS-<n>-<pantalla>-<estado>-<tema>-<viewport>.png`.
- C2. Se captura con un build sin CDN, fuentes empaquetadas y datos deterministas
  (presets y borradores fijos); sin animaciones en curso.
- C3. Se registra el par `(commit, hash SHA-256 del PNG)` en el artefacto de evidencia de
  CI para habilitar la regresión visual.
- C4. La aprobación del baseline la ejerce el rol UX/design owner (autoridad «Baseline
  visual y UX», `docs/00-governance/remediation-plan-v1.1.md`).
- C5. Un cambio de píxel no justificado por accesibilidad, consistencia, responsividad,
  seguridad de recursos o mantenibilidad **bloquea** el gate G6.
- C6. Toda divergencia aprobada se enlaza a su `AUD-*`, requisito y captura de diff.

## Criterio de paridad

| Regla | Comprobación |
|---|---|
| V1 | Toda pantalla y estado de la matriz tiene captura aprobada. |
| V2 | Los tokens heredados se conservan (mapa en `token-migration.md`); cero cambios de valor no justificados. |
| V3 | La regresión visual compara contra la captura aprobada y falla ante diff no aprobado. |
| V4 | Las correcciones de accesibilidad (p. ej. `--color-text-subtle` en tema oscuro) se documentan con su diff. |

## Trazabilidad

| Elemento | Referencia |
|---|---|
| Tokens heredados | `docs/06-design-system/token-migration.md` |
| Design system | `docs/06-design-system/design-system.md` |
| Pantallas y estados | `docs/05-ux/interaction-matrix.md` |
| Accesibilidad | `docs/05-ux/accessibility-matrix.md` |
| Issue | `AUD-020` (R10) |
| Gate | G6 (`docs/10-delivery/gates.md`) |
