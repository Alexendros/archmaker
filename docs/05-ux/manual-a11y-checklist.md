---
id: DOC-UX-MAN-001
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: not-started
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - ux
reviewers:
  - independent-reviewer
---

# Checklist manual de accesibilidad WCAG 2.2 AA

ID `DOC-UX-MAN-001` · Estado `draft` · Propietario UX · Fecha 2026-10-06.

Este checklist complementa la matriz automatizada (`docs/05-ux/accessibility-matrix.md`).
Debe ejecutarse por **Alexendros (supervisor)** (distinto del autor del componente)
en cada release candidate. Cada criterio `pasa` requiere firma del revisor.

## Instrucciones

1. **Entorno:** Navegador + lector de pantalla según combinaciones soportadas
   (NVDA/WebView2, VoiceOver/WKWebView, Orca/WebKitGTK).
2. **Alcance:** Flujos completos (no pantallas aisladas) según
   `docs/05-ux/interaction-matrix.md`.
3. **Evidencia:** Para cada criterio, registrar: resultado, pasos, hallazgos, severidad.
4. **Firma:** El revisor firma cada criterio `pasa` con nombre, fecha y SHA auditado.

---

## 1. Recorrido por teclado (Principio 2 — Operable)

| SC     | Criterio                  | Pasos de verificación                                                           | Resultado | Severidad | Revisor |
| ------ | ------------------------- | ------------------------------------------------------------------------------- | --------- | --------- | ------- |
| 2.1.1  | Teclado — todo operable   | Recorrer JNY-001 extremo a extremo solo con Tab/Shift+Tab/Enter/Space/Esc       |           |           |         |
| 2.1.2  | Sin trampas de teclado    | Verificar que modales (Dialog, Inspector) atrapan foco y lo devuelven al cerrar |           |           |         |
| 2.1.4  | Atajos de un carácter     | Verificar que `⌘K`/`Ctrl+K` se puede desactivar/remapear                        |           |           |         |
| 2.2.1  | Tiempo ajustable          | Verificar que no hay timeouts bloqueantes sin extensión                         |           |           |         |
| 2.2.2  | Pausar/detener/ocultar    | Verificar que animaciones y toasts son controlables                             |           |           |         |
| 2.3.1  | Tres destellos            | Verificar ausencia de destellos > 3Hz                                           |           |           |         |
| 2.4.1  | Evitar bloques            | Verificar skip link y landmarks en cada pantalla                                |           |           |         |
| 2.4.2  | Titulado de página        | Verificar `<title>` único y descriptivo por ruta                                |           |           |         |
| 2.4.3  | Orden del foco            | Verificar que orden Tab = orden visual (JNY-001..003)                           |           |           |         |
| 2.4.4  | Propósito del enlace      | Verificar que enlaces son comprensibles fuera de contexto                       |           |           |         |
| 2.4.5  | Múltiples vías            | Verificar navegación + búsqueda/acceso directo                                  |           |           |         |
| 2.4.6  | Encabezados/etiquetas     | Verificar jerarquía h1-h6 y labels descriptivas                                 |           |           |         |
| 2.4.7  | Foco visible              | Verificar anillo `--color-focus` ≥ 3:1 en todos los estados                     |           |           |         |
| 2.4.11 | Foco no obstruido         | Verificar que foco no queda bajo cabecera/overlays                              |           |           |         |
| 2.5.1  | Gestos de puntero         | Verificar que no hay gestos obligatorios sin alternativa                        |           |           |         |
| 2.5.2  | Cancelación de puntero    | Verificar activación al soltar, cancelable                                      |           |           |         |
| 2.5.3  | Etiqueta en el nombre     | Verificar que nombre visible está en nombre accesible                           |           |           |         |
| 2.5.4  | Activación por movimiento | Verificar que no hay activación por movimiento obligatoria                      |           |           |         |
| 2.5.7  | Movimientos de arrastre   | Verificar alternativa por botón en FileDrop/Import                              |           |           |         |
| 2.5.8  | Tamaño objetivo mínimo    | Verificar objetivos ≥ 24×24 CSS px                                              |           |           |         |

---

## 2. Lectura con lector de pantalla (Principios 1, 3, 4)

| SC    | Criterio                      | Pasos de verificación                                                 | Resultado | Severidad | Revisor |
| ----- | ----------------------------- | --------------------------------------------------------------------- | --------- | --------- | ------- |
| 1.1.1 | Contenido no textual          | Verificar alt/textos equivalentes en iconos e imágenes                |           |           |         |
| 1.3.1 | Información y relaciones      | Verificar landmarks, headings, listas, tablas con NVDA/VoiceOver/Orca |           |           |         |
| 1.3.2 | Secuencia significativa       | Verificar orden de lectura DOM = visual                               |           |           |         |
| 1.3.3 | Características sensoriales   | Verificar que instrucciones no dependen solo de forma/color/posición  |           |           |         |
| 1.3.5 | Identificar propósito entrada | Verificar `autocomplete` válido en campos                             |           |           |         |
| 1.4.1 | Uso del color                 | Verificar estados con icono/texto además de color                     |           |           |         |
| 2.4.4 | Propósito del enlace          | Verificar enlaces comprensibles con lector                            |           |           |         |
| 2.4.6 | Encabezados y etiquetas       | Verificar que headings y labels son descriptivos                      |           |           |         |
| 3.1.1 | Idioma de la página           | Verificar `lang="es"` en `<html>`                                     |           |           |         |
| 3.1.2 | Idioma de las partes          | Verificar cambios de idioma marcados con `lang`                       |           |           |         |
| 3.2.1 | Al recibir foco               | Verificar que foco no cambia contexto sin aviso                       |           |           |         |
| 3.2.2 | Al introducir datos           | Verificar que entrada no cambia contexto sin aviso                    |           |           |         |
| 3.3.1 | Identificación de errores     | Verificar error descrito en texto + sugerencia                        |           |           |         |
| 3.3.2 | Etiquetas/instrucciones       | Verificar labels e instrucciones presentes                            |           |           |         |
| 3.3.3 | Sugerencia ante errores       | Verificar sugerencia de corrección en errores                         |           |           |         |
| 3.3.4 | Prevención errores            | Verificar confirmación en exportar/borrar                             |           |           |         |
| 4.1.2 | Nombre, función, valor        | Verificar nombre/rol/valor con lector en todos los controles          |           |           |         |
| 4.1.3 | Mensajes de estado            | Verificar regiones vivas anuncian sin interrumpir                     |           |           |         |

---

## 3. Visual y reflow (Principio 1 — Perceptible)

| SC     | Criterio             | Pasos de verificación                                          | Resultado | Severidad | Revisor |
| ------ | -------------------- | -------------------------------------------------------------- | --------- | --------- | ------- |
| 1.4.3  | Contraste mínimo     | Verificar 4.5:1 texto, 3:1 componentes (axe + medición manual) |           |           |         |
| 1.4.4  | Redimensionar texto  | Zoom 200% y 400% — texto e interfaz operables                  |           |           |         |
| 1.4.10 | Reflow               | Viewport 320 CSS px — sin scroll bidimensional                 |           |           |         |
| 1.4.11 | Contraste no textual | Verificar bordes, foco, iconos ≥ 3:1                           |           |           |         |
| 1.4.12 | Espaciado texto      | Override propiedades tipográficas — sin recorte/solapamiento   |           |           |         |
| 1.4.13 | Contenido hover/foco | Tooltips/popovers descartables y persistentes                  |           |           |         |

---

## 4. Temas y movimiento

| Criterio            | Verificación                                                               | Resultado | Severidad | Revisor |
| ------------------- | -------------------------------------------------------------------------- | --------- | --------- | ------- |
| Tema claro          | Todas las pantallas renderizan correctamente en light                      |           |           |         |
| Tema oscuro         | Todas las pantallas renderizan correctamente en dark                       |           |           |         |
| Alto contraste      | Todas las pantallas renderizan correctamente en forcedColors/high-contrast |           |           |         |
| Movimiento reducido | `prefers-reduced-motion: reduce` desactiva transiciones                    |           |           |         |

---

## 5. Flujos completos (WCAG-EM procesos)

| Flujo                                           | Pantallas                                        | Verificación extremo a extremo   | Resultado | Revisor |
| ----------------------------------------------- | ------------------------------------------------ | -------------------------------- | --------- | ------- |
| JNY-001 Crear → Configurar → Revisar → Exportar | Inicio → Nuevo → Configurar → Revisar → Exportar | Teclado + lector + zoom + reflow |           |         |
| JNY-002 Importar y migrar                       | Importar → MigrationReport                       | Teclado + lector                 |           |         |
| JNY-003 Plan v1 (fuera MVP)                     | Plan v1                                          | —                                | n/a       |         |

---

## Registro de hallazgos

| ID  | SC  | Descripción | Severidad (critical/serious/minor) | Estado | Corrección |
| --- | --- | ----------- | ---------------------------------- | ------ | ---------- |
|     |     |             |                                    |        |            |
|     |     |             |                                    |        |            |
|     |     |             |                                    |        |            |

---

## Firma del revisor

> **Revisor:** _________________________
> **Fecha:** _________________________
> **SHA auditado:** _________________________
> **Combinación probada:** _________________________
>
> **Declaración:** He ejecutado este checklist de forma independiente. Los criterios marcados como `pasa` cumplen WCAG 2.2 AA. Los criterios `falla` tienen severidad y corrección documentada.

---

## Criterios de aceptación para release AA

- [ ] 0 defectos `critical` o `serious` abiertos
- [ ] 100% criterios A/AA aplicables con evidencia `pasa` (auto + manual)
- [ ] Revisor **Alexendros (supervisor)** distinto del autor
- [ ] Firmado por revisor con SHA auditado
- [ ] Hallazgos `critical`/`serious` tienen corrección documentada y plan de cierre
