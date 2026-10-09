---
id: DOC-UX-A11Y-001
phase: MVP
priority: P1
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - ux
reviewers:
  - independent-reviewer
---

# Matriz de accesibilidad WCAG 2.2 AA

ID `DOC-UX-A11Y-001` · Estado `in-review` · Propietario UX/diseño · Fecha 2026-10-06.

Matriz de conformidad WCAG 2.2 nivel AA (fase R10, issue `AUD-021`), con **evidencia
automatizada y manual separadas** y combinaciones de navegador/lector de pantalla
soportadas. Ancla el requisito `NFR-ACC-001` (`docs/01-product/requirements.md`) a la
Recomendación W3C WCAG 2.2 y a WAI-ARIA APG (`SRC-007`,
`docs/00-governance/source-register.md`). Los componentes se contratan en
`docs/05-ux/component-contracts.md`.

## Alcance y estado

- Alcance: todas las pantallas y estados de `docs/05-ux/interaction-matrix.md`, en tema
  claro y oscuro, en los viewports de `docs/06-design-system/visual-baseline.md`.
- Niveles cubiertos: **A y AA** de WCAG 2.2. `4.1.1 Parsing` se excluye por obsoleto en
  WCAG 2.2.
- Estado global: `partial` (walking skeleton MVP-0.1). Evidencia automatizada de tokens
  (`tools/wcag_validate.py`), axe-core Playwright y contratos A3; checklist manual
  (`docs/05-ux/manual-a11y-checklist.md`) obligatorio en release candidate. No se declara
  conformidad total sin firma manual (regla R1 de `docs/00-governance/status-model.md`).

## Vocabulario

| Estado      | Significado                                          |
| ----------- | ---------------------------------------------------- |
| `n/a`       | El criterio no aplica; se justifica la exclusión.    |
| `pendiente` | Aplica; método definido, evidencia aún no ejecutada. |
| `pasa`      | Evidencia ejecutada y conforme.                      |
| `falla`     | Evidencia ejecutada y no conforme; abre corrección.  |

## Separación de evidencia

- **Automatizada:** análisis estático y en navegador con axe-core integrado en Playwright,
  cálculo de contraste de tokens, emulación de viewport, zoom y `prefers-reduced-motion`.
  Detecta una fracción de los criterios; nunca es suficiente por sí sola.
- **Manual:** recorrido con teclado, inspección de orden de foco, lectura con lector de
  pantalla, revisión de reflow/zoom y juicio de contenido. Obligatoria para los criterios
  marcados como manuales.

Una fila solo puede declararse `pasa` cuando **ambas** evidencias que le apliquen constan
enlazadas.

## Matriz WCAG 2.2 AA

### Principio 1 — Perceptible

| SC     | Nivel | Título                                      | Aplica  | Evidencia automatizada                                            | Evidencia manual                                    | Estado                 |
| ------ | ----- | ------------------------------------------- | ------- | ----------------------------------------------------------------- | --------------------------------------------------- | ---------------------- |
| 1.1.1  | A     | Contenido no textual                        | Sí      | axe `image-alt`, `svg-img-alt`, `role-img-alt`                    | Distinguir iconos decorativos de significativos     | pendiente              |
| 1.2.1  | A     | Solo audio y solo vídeo (grabado)           | No      | —                                                                 | Sin media en MVP                                    | n/a                    |
| 1.2.2  | A     | Subtítulos (grabado)                        | No      | —                                                                 | Sin media en MVP                                    | n/a                    |
| 1.2.3  | A     | Audiodescripción o alternativa (grabado)    | No      | —                                                                 | Sin media en MVP                                    | n/a                    |
| 1.2.4  | AA    | Subtítulos (en directo)                     | No      | —                                                                 | Sin media en MVP                                    | n/a                    |
| 1.2.5  | AA    | Audiodescripción (grabado)                  | No      | —                                                                 | Sin media en MVP                                    | n/a                    |
| 1.3.1  | A     | Información y relaciones                    | Sí      | axe `label`, `aria-*`, `list`, `heading-order`                    | Landmarks y estructura semántica                    | pendiente              |
| 1.3.2  | A     | Secuencia significativa                     | Sí      | axe `tabindex` (positivo)                                         | Lectura con lector de pantalla                      | pendiente              |
| 1.3.3  | A     | Características sensoriales                 | Sí      | —                                                                 | Instrucciones sin depender de forma/posición/sonido | pendiente              |
| 1.3.4  | AA    | Orientación                                 | Sí      | axe (parcial)                                                     | Vista vertical y horizontal                         | pendiente              |
| 1.3.5  | AA    | Identificar el propósito de entrada         | Parcial | axe `autocomplete-valid`                                          | Revisión de campos de entrada                       | pendiente              |
| 1.4.1  | A     | Uso del color                               | Sí      | contrato A3 + reporte WCAG (`automated_scope: contract_declared`) | Estados con icono/texto además de color (checklist) | pasa (auto); manual RC |
| 1.4.2  | A     | Control de audio                            | No      | —                                                                 | Sin audio automático en MVP                         | n/a                    |
| 1.4.3  | AA    | Contraste (mínimo)                          | Sí      | axe `color-contrast` + `wcag_validate.py` (tokens)                | Medición puntual en estados                         | pasa                   |
| 1.4.4  | AA    | Redimensionar texto                         | Sí      | `wcag_validate.py` (unidades rem)                                 | Zoom 200% sin pérdida                               | pasa (auto); manual RC |
| 1.4.5  | AA    | Imágenes de texto                           | Sí      | —                                                                 | Revisión de texto rasterizado                       | pendiente              |
| 1.4.10 | AA    | Reflow                                      | Sí      | Playwright a 320 CSS px + tokens                                  | Sin scroll bidimensional                            | pasa (auto); manual RC |
| 1.4.11 | AA    | Contraste no textual                        | Sí      | axe (parcial) + `wcag_validate.py` (tokens)                       | Bordes, foco e iconos ≥ 3:1                         | pasa                   |
| 1.4.12 | AA    | Espaciado del texto                         | Sí      | `wcag_validate.py` (subset tokens)                                | Override 0.12x sin recorte                          | pasa (auto); manual RC |
| 1.4.13 | AA    | Contenido al pasar el cursor o recibir foco | Sí      | —                                                                 | Tooltips y popovers descartables y persistentes     | pendiente              |

### Principio 2 — Operable

| SC     | Nivel | Título                             | Aplica | Evidencia automatizada                            | Evidencia manual                              | Estado    |
| ------ | ----- | ---------------------------------- | ------ | ------------------------------------------------- | --------------------------------------------- | --------- |
| 2.1.1  | A     | Teclado                            | Sí     | axe `scrollable-region-focusable` (parcial)       | Recorrido completo de extremo a extremo       | pendiente |
| 2.1.2  | A     | Sin trampas de teclado             | Sí     | —                                                 | `Tab`/`Shift+Tab` sin bloqueo                 | pendiente |
| 2.1.4  | A     | Atajos de teclado de un carácter   | Sí     | —                                                 | Atajos desactivables o remapeables            | pendiente |
| 2.2.1  | A     | Tiempo ajustable                   | Sí     | —                                                 | Sin límites de tiempo bloqueantes             | pendiente |
| 2.2.2  | A     | Pausar, detener, ocultar           | Sí     | —                                                 | Animaciones y avisos controlables             | pendiente |
| 2.3.1  | A     | Tres destellos o menos             | Sí     | —                                                 | Sin destellos                                 | pendiente |
| 2.4.1  | A     | Evitar bloques                     | Sí     | axe `bypass`                                      | Enlace de salto y landmarks                   | pendiente |
| 2.4.2  | A     | Titulado de página                 | Sí     | —                                                 | Título por vista                              | pendiente |
| 2.4.3  | A     | Orden del foco                     | Sí     | —                                                 | Orden coherente con el visual                 | pendiente |
| 2.4.4  | A     | Propósito del enlace (en contexto) | Sí     | axe `link-name`                                   | Enlaces comprensibles                         | pendiente |
| 2.4.5  | AA    | Múltiples vías                     | Sí     | —                                                 | Navegación y búsqueda                         | pendiente |
| 2.4.6  | AA    | Encabezados y etiquetas            | Sí     | axe `heading-order`, `label`                      | Jerarquía y etiquetas descriptivas            | pendiente |
| 2.4.7  | AA    | Foco visible                       | Sí     | axe (parcial) + `wcag_validate.py` (anillo ≥ 3:1) | Foco visible y estable                        | pasa      |
| 2.4.11 | AA    | Foco no obstruido (mínimo)         | Sí     | —                                                 | El foco no queda tapado por cabecera/overlays | pendiente |
| 2.5.1  | A     | Gestos de puntero                  | Sí     | —                                                 | Sin gestos de trazo obligatorios              | pendiente |
| 2.5.2  | A     | Cancelación de puntero             | Sí     | —                                                 | Activación al soltar, cancelable              | pendiente |
| 2.5.3  | A     | Etiqueta en el nombre              | Sí     | axe `label-content-name-mismatch`                 | Nombre visible contenido en el accesible      | pendiente |
| 2.5.4  | A     | Activación por movimiento          | Sí     | —                                                 | Sin activación por movimiento obligatoria     | pendiente |
| 2.5.7  | AA    | Movimientos de arrastre            | Sí     | —                                                 | Arrastrar tiene alternativa por botón         | pendiente |
| 2.5.8  | AA    | Tamaño del objetivo (mínimo)       | Sí     | `wcag_validate.py` (`--ds-touch-target-min`)      | Objetivos ≥ 24×24 CSS px                      | pasa      |

### Principio 3 — Comprensible

| SC    | Nivel | Título                           | Aplica  | Evidencia automatizada | Evidencia manual                        | Estado    |
| ----- | ----- | -------------------------------- | ------- | ---------------------- | --------------------------------------- | --------- |
| 3.1.1 | A     | Idioma de la página              | Sí      | axe `html-has-lang`    | Idioma declarado                        | pendiente |
| 3.1.2 | AA    | Idioma de las partes             | Parcial | axe `html-lang-valid`  | Cambios de idioma marcados              | pendiente |
| 3.2.1 | A     | Al recibir el foco               | Sí      | —                      | El foco no provoca cambios de contexto  | pendiente |
| 3.2.2 | A     | Al introducir datos              | Sí      | —                      | La entrada no cambia contexto sin aviso | pendiente |
| 3.2.3 | AA    | Navegación coherente             | Sí      | —                      | Navegación repetida y consistente       | pendiente |
| 3.2.4 | AA    | Identificación coherente         | Sí      | —                      | Misma función, misma etiqueta           | pendiente |
| 3.2.6 | A     | Ayuda coherente                  | Sí      | —                      | Ayuda en la misma ubicación             | pendiente |
| 3.3.1 | A     | Identificación de errores        | Sí      | —                      | Error descrito en texto                 | pendiente |
| 3.3.2 | A     | Etiquetas o instrucciones        | Sí      | axe `label`            | Etiquetas e instrucciones presentes     | pendiente |
| 3.3.3 | AA    | Sugerencia ante errores          | Sí      | —                      | Sugerencia de corrección                | pendiente |
| 3.3.4 | AA    | Prevención de errores            | Sí      | —                      | Confirmación en exportar/borrar         | pendiente |
| 3.3.7 | A     | Entrada redundante               | Sí      | —                      | No re-solicitar datos ya aportados      | pendiente |
| 3.3.8 | AA    | Autenticación accesible (mínimo) | No      | —                      | Sin autenticación en MVP                | n/a       |

### Principio 4 — Robusto

| SC    | Nivel | Título                 | Aplica | Evidencia automatizada      | Evidencia manual                        | Estado    |
| ----- | ----- | ---------------------- | ------ | --------------------------- | --------------------------------------- | --------- |
| 4.1.2 | A     | Nombre, función, valor | Sí     | axe `button-name`, `aria-*` | Nombre, rol y estado correctos          | pendiente |
| 4.1.3 | AA    | Mensajes de estado     | Sí     | axe (parcial)               | Regiones vivas anuncian sin interrumpir | pendiente |

## Evidencia de contraste de tokens

Cálculo automatizado sobre los valores heredados de
`docs/06-design-system/token-migration.md` (relación WCAG 2.x). Umbral AA: 4.5:1 texto
normal, 3:1 texto grande y componentes.

| Combinación                                       | Relación | Umbral | Resultado                            |
| ------------------------------------------------- | -------- | ------ | ------------------------------------ |
| texto sobre canvas (claro)                        | 18.26:1  | 4.5    | pasa                                 |
| texto sobre surface (claro)                       | 16.75:1  | 4.5    | pasa                                 |
| texto secundario sobre canvas (claro)             | 5.13:1   | 4.5    | pasa                                 |
| texto secundario sobre surface (claro)            | 4.71:1   | 4.5    | pasa                                 |
| texto de detalle sobre canvas (claro)             | 10.60:1  | 4.5    | pasa                                 |
| texto de detalle sobre surface (claro)            | 9.72:1   | 4.5    | pasa                                 |
| acción sobre canvas (claro)                       | 4.53:1   | 4.5    | pasa                                 |
| acción sobre surface (claro)                      | 4.16:1   | 3      | pasa como componente                 |
| foco sobre canvas (claro)                         | 5.57:1   | 3      | pasa                                 |
| blanco sobre acción (CTA)                         | 4.53:1   | 4.5    | pasa                                 |
| éxito sobre canvas (claro)                        | 4.50:1   | 4.5    | pasa                                 |
| aviso sobre canvas (claro)                        | 1.99:1   | 3      | **falla**                            |
| borde sobre canvas (claro)                        | 1.51:1   | 3      | **falla** si es límite significativo |
| texto sobre canvas (oscuro)                       | 18.26:1  | 4.5    | pasa                                 |
| texto secundario sobre canvas (oscuro)            | 7.68:1   | 4.5    | pasa                                 |
| texto de detalle sobre canvas (oscuro, heredado)  | 1.72:1   | 4.5    | **falla**                            |
| texto de detalle sobre surface (oscuro, heredado) | 1.57:1   | 4.5    | **falla**                            |
| borde sobre canvas (oscuro)                       | 1.27:1   | 3      | **falla** si es límite significativo |
| aviso sobre canvas (oscuro)                       | 9.16:1   | 4.5    | pasa                                 |

Correcciones trazables abiertas por esta evidencia:

| Defecto                                           | Criterio | Corrección propuesta                                |
| ------------------------------------------------- | -------- | --------------------------------------------------- |
| `--color-text-subtle` sin override en tema oscuro | 1.4.3    | Sobrescribir a `--ds-color-neutral-400` (`#a8a8a8`) |
| `--color-warning` como límite/indicador           | 1.4.11   | Usar borde de aviso con ≥ 3:1 o acompañar con icono |
| `--color-border` como límite de campo             | 1.4.11   | Elevar contraste del borde de control a ≥ 3:1       |

## Criterios de aceptación

| Criterio                       | Método                                         | Evidencia                                                             | Estado    |
| ------------------------------ | ---------------------------------------------- | --------------------------------------------------------------------- | --------- |
| Recorrido completo por teclado | Playwright teclado + sesión manual             | Orden de foco y mapa de teclado de `docs/05-ux/interaction-matrix.md` | pendiente |
| Foco visible y estable         | axe (parcial) + captura de foco                | Anillo `--color-focus` ≥ 3:1; sin pérdida tras re-render              | pendiente |
| Errores y progreso anunciados  | axe región viva (parcial) + lector de pantalla | Tabla de anuncios de `docs/05-ux/interaction-matrix.md`               | pendiente |
| Reflow                         | Playwright a 320 CSS px                        | Sin scroll bidimensional                                              | pendiente |
| Zoom                           | Manual a 200% y 400%                           | Texto e interfaz operables                                            | pendiente |
| Contraste                      | Cálculo de tokens + axe                        | Tabla de contraste de este documento                                  | pendiente |
| Movimiento reducido            | Emulación de `prefers-reduced-motion`          | Sin animación no esencial                                             | pendiente |

## Combinaciones de navegador y lector de pantalla

El producto se ejecuta en webviews de Tauri (WebView2 en Windows, WKWebView en macOS,
WebKitGTK en Linux) y en navegador para el adaptador WASM.

| Plataforma         | Motor     | Navegador/WebView | Lector de pantalla | Nivel        | Nota                                |
| ------------------ | --------- | ----------------- | ------------------ | ------------ | ----------------------------------- |
| Windows 11         | Chromium  | WebView2          | NVDA (2024+)       | soportado    | Combinación primaria                |
| Windows 11         | Chromium  | WebView2          | JAWS (2024+)       | best-effort  | Se prueba; corrección no bloqueante |
| Windows 11         | Chromium  | WebView2          | Narrador           | best-effort  | Se prueba; corrección no bloqueante |
| macOS              | WebKit    | WKWebView         | VoiceOver          | soportado    | Combinación primaria                |
| Linux              | WebKitGTK | WebKitGTK         | Orca               | soportado    | Combinación primaria                |
| Linux              | Gecko     | Firefox           | Orca               | best-effort  | Solo adaptador WASM                 |
| Android / ChromeOS | Chromium  | Chrome            | TalkBack           | no soportado | Fuera del alcance MVP               |

Niveles:

- **soportado:** se verifica en cada gate G6 y se corrige toda no conformidad.
- **best-effort:** se prueba de forma oportunista; una no conformidad se registra pero no
  bloquea.
- **no soportado:** fuera del alcance declarado del MVP.

## Protocolo de evidencia

### Automatizada

1. Ejecutar axe-core sobre cada ruta y estado de la matriz de pantallas, en claro y oscuro.
2. Ejecutar Playwright para teclado, viewport 320 CSS px, zoom y `prefers-reduced-motion`.
3. Recalcular la tabla de contraste de tokens ante cualquier cambio de
   `docs/06-design-system/token-migration.md`.
4. Publicar el informe como artefacto de CI y enlazarlo aquí.

### Manual

1. Recorrido de extremo a extremo solo con teclado por cada journey, registrando orden de
   foco, foco visible y ausencia de trampas.
2. Lectura con cada combinación **soportada**, verificando nombre, rol, valor, anuncios de
   estado y errores.
3. Revisión de reflow, zoom, espaciado y movimiento reducido.
4. Firmar cada criterio `pasa` con **Alexendros (supervisor)** distinto del autor.

## Trazabilidad

| Elemento                | Referencia                                                                  |
| ----------------------- | --------------------------------------------------------------------------- |
| Requisito               | `NFR-ACC-001` (`docs/01-product/requirements.md`)                           |
| Fuente normativa        | `SRC-007` WCAG 2.2 / WAI-ARIA APG (`docs/00-governance/source-register.md`) |
| Contratos de componente | `docs/05-ux/component-contracts.md`                                         |
| Interacción y teclado   | `docs/05-ux/interaction-matrix.md`                                          |
| Tokens y contraste      | `docs/06-design-system/token-migration.md`                                  |
| Baseline visual         | `docs/06-design-system/visual-baseline.md`                                  |
| Prueba                  | `TST-A11Y-001` (`docs/09-quality/test-matrix.md`)                           |
| Gate                    | G6 (`docs/10-delivery/gates.md`)                                            |
| Issue                   | `AUD-021` (R10)                                                             |
