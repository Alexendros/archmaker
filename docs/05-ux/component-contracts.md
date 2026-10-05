---
id: DOC-UX-COMP-001
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

# Contratos de componentes

ID `DOC-UX-COMP-001` · Estado `in-review` · Propietario UX/diseño · Fecha 2026-10-06.

Contratos `v0` de los componentes interactivos (fase R10, issues `AUD-020`/`AUD-021`). Cada
componente documenta propósito, props, slots, variantes, estados, teclado, ARIA, foco,
tokens, comportamiento responsivo, movimiento reducido, estados vacío/carga/error y
pruebas. Los tokens se nombran según `docs/06-design-system/design-system.md`; el baseline
visual, según `docs/06-design-system/visual-baseline.md`; la verificación de accesibilidad,
según `docs/05-ux/accessibility-matrix.md`.

## Plantilla de contrato

Toda ficha cubre las catorce facetas siguientes. «—» indica que la faceta no aplica y debe
justificarse en la propia ficha.

1. Propósito.
2. Props y tipos.
3. Slots.
4. Variantes.
5. Estados.
6. Teclado.
7. ARIA y nombre accesible.
8. Foco.
9. Tokens.
10. Responsive.
11. Movimiento reducido.
12. Vacío.
13. Carga.
14. Error.
15. Pruebas.

## Reglas transversales

- A1. Ningún control interactivo se simula con `div` sin rol, foco ni teclado.
- A2. El foco visible usa `--color-focus` con relación de contraste ≥ 3:1 y es estable ante
  re-render (sin pérdida ni salto).
- A3. El color nunca es el único portador de significado; todo estado añade icono, texto o
  patrón.
- A4. Las regiones vivas anuncian resúmenes, no cada revalidación.
- A5. Los componentes respetan `prefers-reduced-motion` y no animan por defecto.

## OptionCard

| Faceta | Contrato |
|---|---|
| Propósito | Presentar una opción seleccionable con evidencia, dependencias y motivo de derivación. |
| Props | `id`, `label`, `description`, `selected`, `derived`, `required`, `locked`, `incompatible`, `warning`, `onSelect`, `onShowEvidence`. |
| Slots | icono, descripción, insignia de estado, acción secundaria. |
| Variantes | radio (cardinalidad uno), checkbox (múltiple). |
| Estados | idle, hover, focus-visible, selected-manual, selected-derived, required, locked-policy, incompatible, disabled, warning. |
| Teclado | `Space` alterna; flechas mueven dentro del grupo (roving tabindex solo si el grupo lo exige); `Enter` abre evidencia. |
| ARIA | `role="radio"`/`role="checkbox"` con `aria-checked`; `aria-disabled` en locked/disabled; `aria-describedby` a motivo/derivación. |
| Foco | Anillo `--color-focus`; el estado seleccionado no depende solo del color. |
| Tokens | `--option-card-bg`, `--option-card-border`, `--option-card-selected-bg`, `--option-card-selected-border`. |
| Responsive | Una columna en móvil; rejilla en ancho amplio. |
| Movimiento reducido | Sin transición de selección. |
| Vacío | — (una opción siempre tiene contenido). |
| Carga | Esqueleto no interactivo. |
| Error | `incompatible` y `warning` visibles con texto e icono. |
| Pruebas | `TST-A11Y-001` (rol, nombre, teclado), `TST-OPT-001` (selección/derivación). |

## StepRail

| Faceta | Contrato |
|---|---|
| Propósito | Navegar entre pasos del configurador y mostrar progreso. |
| Props | `steps`, `current`, `status`, `onNavigate`. |
| Slots | icono de paso, etiqueta, indicador de estado. |
| Variantes | rail lateral (ancho amplio), stepper compacto (móvil). |
| Estados | incomplete, current, valid, warning, blocked, complete. |
| Teclado | Navegación natural de enlaces; `Enter`/`Space` activan; no roving tabindex salvo patrón demostrado. |
| ARIA | `nav` con nombre; paso actual con `aria-current="step"`; estado anunciado en texto. |
| Foco | Orden de tabulación = orden visual. |
| Tokens | `--rail-bg`, `--rail-item-current-bg`, `--rail-item-done-fg`. |
| Responsive | Rail lateral en ancho amplio; stepper compacto en móvil. |
| Movimiento reducido | Sin animación de transición de paso. |
| Vacío | — |
| Carga | — |
| Error | Estado `blocked` con motivo accesible. |
| Pruebas | `TST-A11Y-001` (nav, `aria-current`), `TST-RAIL-001`. |

## TargetPicker

| Faceta | Contrato |
|---|---|
| Propósito | Elegir el objetivo de exportación. |
| Props | `targets`, `value`, `onChange`. |
| Slots | descripción de target. |
| Variantes | lista, select nativo si aplica. |
| Estados | idle, focus-visible, invalid, disabled. |
| Teclado | Patrón de lista o select nativo; flechas y `Enter`. |
| ARIA | `role="listbox"`/`option` o `select` nativo con `label`. |
| Foco | Anillo `--color-focus`. |
| Tokens | `--field-*`, `--option-card-*`. |
| Responsive | Columna única en móvil. |
| Movimiento reducido | — |
| Vacío | Mensaje «sin objetivos disponibles». |
| Carga | Esqueleto. |
| Error | Target inválido con texto asociado. |
| Pruebas | `TST-A11Y-001`, `TST-EXP-001`. |

## PresetPicker

| Faceta | Contrato |
|---|---|
| Propósito | Aplicar un preset de configuración. |
| Props | `presets`, `onApply`, `applied`. |
| Slots | descripción y resumen de selecciones. |
| Variantes | tarjeta, menú. |
| Estados | idle, focus-visible, applied, disabled. |
| Teclado | `Enter`/`Space` aplican; `Esc` cierra menú. |
| ARIA | `button` con nombre; confirmación anunciada. |
| Foco | Retorno al disparador al cerrar. |
| Tokens | `--button-*`, `--option-card-*`. |
| Responsive | Tarjetas apiladas en móvil. |
| Movimiento reducido | Sin animación de aplicación. |
| Vacío | — |
| Carga | Esqueleto. |
| Error | Preset incompatible con texto e icono. |
| Pruebas | `TST-A11Y-001`, `TST-PRESET-001`. |

## Inspector

| Faceta | Contrato |
|---|---|
| Propósito | Mostrar detalle, dependencias y evidencia del elemento activo. |
| Props | `entity`, `open`, `onClose`. |
| Slots | secciones de detalle. |
| Variantes | panel lateral, cajón inferior. |
| Estados | closed, open, focus-visible. |
| Teclado | `Esc` cierra; foco atrapado solo si es modal. |
| ARIA | `complementary` con nombre o `dialog` si modal; `aria-labelledby`. |
| Foco | Foco inicial en el encabezado; retorno al origen al cerrar. |
| Tokens | `--inspector-bg`, `--inspector-border`. |
| Responsive | Cajón inferior en móvil. |
| Movimiento reducido | Sin animación de apertura. |
| Vacío | Sin selección: mensaje neutro. |
| Carga | Esqueleto. |
| Error | Fallo de detalle con texto. |
| Pruebas | `TST-A11Y-001`, `TST-INSP-001`. |

## ValidationPanel

| Faceta | Contrato |
|---|---|
| Propósito | Agrupar diagnósticos por severidad y paso, enlazando cada uno a su control. |
| Props | `diagnostics`, `onNavigate`. |
| Slots | resumen por severidad. |
| Variantes | resumen, lista completa. |
| Estados | empty, has-error, has-warning, has-info. |
| Teclado | Enlaces a controles operables con `Enter`. |
| ARIA | `role="alert"`/`aria-live="polite"` solo para el resumen; no anunciar en masa. |
| Foco | Al navegar a un diagnóstico, el foco va al control afectado. |
| Tokens | `--color-danger`, `--color-warning`, `--color-info`. |
| Responsive | Columna única en móvil. |
| Movimiento reducido | — |
| Vacío | «Sin incidencias». |
| Carga | Estado de cálculo. |
| Error | Error bloqueante destacado con texto. |
| Pruebas | `TST-A11Y-001` (región viva), `TST-VAL-001`. |

## Dialog

| Faceta | Contrato |
|---|---|
| Propósito | Presentar contenido modal (detalle, glosario, paleta de comandos). |
| Props | `open`, `title`, `onClose`, `modal`. |
| Slots | cabecera, cuerpo, acciones. |
| Variantes | modal centrado, cajón lateral. |
| Estados | closed, open, closing. |
| Teclado | `Esc` cierra; foco atrapado; `Tab`/`Shift+Tab` cíclicos. |
| ARIA | `role="dialog"`/`alertdialog` con `aria-modal` y `aria-labelledby`. |
| Foco | Foco inicial al primer control; retorno al disparador. |
| Tokens | `--dialog-bg`, `--dialog-border`, `--dialog-shadow`. |
| Responsive | Ocupa el ancho disponible en móvil. |
| Movimiento reducido | Sin animación de entrada. |
| Vacío | — |
| Carga | — |
| Error | — |
| Pruebas | `TST-A11Y-001` (trampa de foco, `Esc`), `TST-DIALOG-001`. |

## FileDrop

| Faceta | Contrato |
|---|---|
| Propósito | Recibir un borrador por arrastre o selección. |
| Props | `accept`, `onFiles`, `disabled`. |
| Slots | instrucciones. |
| Variantes | zona de arrastre, botón de selección. |
| Estados | idle, drag-over, focus-visible, invalid, disabled. |
| Teclado | Alternativa obligatoria por botón (no exige arrastre). |
| ARIA | `button`/`label` con nombre; anuncio de archivo cargado. |
| Foco | Anillo `--color-focus`. |
| Tokens | `--field-*`. |
| Responsive | Ancho completo en móvil. |
| Movimiento reducido | Sin animación de `drag-over`. |
| Vacío | Instrucción visible. |
| Carga | Progreso anunciado. |
| Error | Archivo inválido con motivo. |
| Pruebas | `TST-A11Y-001` (alternativa a arrastre), `TST-IMP-001`. |

## Toast y StatusMessage

| Faceta | Contrato |
|---|---|
| Propósito | Anunciar resultados y progreso (guardado, exportación, migración). |
| Props | `message`, `severity`, `action`, `duration`. |
| Slots | acción de deshacer. |
| Variantes | toast transitorio, mensaje persistente. |
| Estados | visible, dismissed, action-pending. |
| Teclado | La acción es operable con teclado; el toast no roba el foco. |
| ARIA | `role="status"` (educado) o `role="alert"` (asertivo); `aria-live`. |
| Foco | No mueve el foco salvo error bloqueante. |
| Tokens | `--color-success`, `--color-warning`, `--color-danger`, `--color-info`. |
| Responsive | Anclado inferior en móvil. |
| Movimiento reducido | Sin deslizamiento; aparece sin transición. |
| Vacío | — |
| Carga | Progreso determinado/indeterminado anunciado. |
| Error | Mensaje con severidad y acción de recuperación. |
| Pruebas | `TST-A11Y-001` (región viva), `TST-OBS-001`. |

## ManifestSummary, Diff y Diagnostics

| Componente | Propósito | Estados | Teclado y ARIA | Tokens | Pruebas |
|---|---|---|---|---|---|
| ManifestSummary | Resumen del manifiesto canónico | idle, loading, ready, error | lectura secuencial; tablas con encabezados | `--color-*` | `TST-MAN-001`, `TST-A11Y-001` |
| Diff | Comparar cambios antes/después | empty, changed, unchanged | navegación por bloques; `role` de lista | `--color-success`, `--color-danger` | `TST-REV-001`, `TST-A11Y-001` |
| Diagnostics | Lista tipada de diagnósticos | empty, has-items | enlaces a controles; `aria-live` en resumen | `--color-danger`, `--color-warning` | `TST-VAL-001`, `TST-A11Y-001` |
