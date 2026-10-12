---
id: DOC-UX-INT-001
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
  - apps/web/a11y-playwright.test.ts (axe-core automatizado)
  - apps/web/visual-regression.test.ts (visual regression)
  - docs/05-ux/accessibility-matrix.md (matriz WCAG)
  - docs/05-ux/manual-a11y-checklist.md (checklist manual)
---

# Matriz de interacción

ID `DOC-UX-INT-001` · Estado `accepted` · Propietario UX/diseño · Fecha 2026-10-06.

Relaciona journeys, rutas, pantallas, componentes, acciones de CorePort, validación y prueba
(fase R10). Se complementa con los contratos por componente
(`docs/05-ux/component-contracts.md`) y con la verificación de accesibilidad
(`docs/05-ux/accessibility-matrix.md`).

## Journeys y pantallas

| Journey | Ruta               | Pantalla   | Componentes                        | Acción/CorePort                  | Validación       | Prueba   |
| ------- | ------------------ | ---------- | ---------------------------------- | -------------------------------- | ---------------- | -------- |
| JNY-001 | `/`                | Inicio     | RecentDrafts, NewDraft, Import     | `productInfo`                    | Capabilities     | E2E-HOME |
| JNY-001 | `/new`             | Nuevo      | TargetPicker, PresetPicker         | `createDraft`                    | Draft schema     | E2E-NEW  |
| JNY-001 | `/configure/:step` | Configurar | StepRail, OptionCard, Inspector    | `resolveDraft`                   | Domain/rules     | E2E-CONF |
| JNY-001 | `/review`          | Revisar    | ManifestSummary, Diff, Diagnostics | `validateDraft`, `buildManifest` | Full pipeline    | E2E-REV  |
| JNY-001 | `/export`          | Exportar   | TargetPicker, ArtifactCard         | `checkTarget`, `exportArtifact`  | Target           | E2E-EXP  |
| JNY-002 | `/import`          | Importar   | FileDrop, MigrationReport          | `importDraft`, migration         | Limits/migration | E2E-IMP  |
| JNY-003 | `/runner/plan`     | Plan v1    | PlanTree, RiskSummary              | Runner protocol                  | Plan/privilege   | E2E-RUN  |

## Orden de foco por pantalla

El orden de tabulación sigue el orden visual de lectura. No se usa `tabindex` positivo.

| Pantalla   | Orden de foco                                                    |
| ---------- | ---------------------------------------------------------------- |
| Inicio     | skip link → encabezado → RecentDrafts → NewDraft → Import        |
| Nuevo      | encabezado → TargetPicker → PresetPicker → acción primaria       |
| Configurar | skip link → StepRail → región de opciones → Inspector → acciones |
| Revisar    | encabezado → ManifestSummary → Diff → Diagnostics → acciones     |
| Exportar   | encabezado → TargetPicker → ArtifactCard → acción primaria       |
| Importar   | encabezado → FileDrop → MigrationReport → acciones               |
| Plan v1    | encabezado → PlanTree → RiskSummary → acciones                   |

## Mapa de teclado

| Tecla                   | Contexto                          | Acción                                   |
| ----------------------- | --------------------------------- | ---------------------------------------- |
| `Tab` / `Shift+Tab`     | Global                            | Avanzar/retroceder en el orden de foco   |
| `Enter`                 | Botones, enlaces, tarjetas        | Activar                                  |
| `Space`                 | Botones, checkbox, radio          | Activar/alternar sin desplazar la página |
| `↑` `↓`                 | Grupos de opciones, listas        | Mover dentro del grupo                   |
| `←` `→`                 | Grupos de opciones (según patrón) | Mover dentro del grupo                   |
| `Esc`                   | Diálogos, cajones, menús          | Cerrar y devolver el foco al disparador  |
| `⌘K` / `Ctrl+K`         | Global                            | Abrir/cerrar la paleta de comandos       |
| `Inicio` / `Fin`        | Listas y tablas                   | Ir al primer/último elemento             |
| `Page Up` / `Page Down` | Listas y tablas                   | Desplazamiento por página                |

Reglas de teclado:

- K1. Todo control operable con puntero lo es con teclado.
- K2. No existen trampas de foco; los modales atrapan el foco de forma deliberada y lo
  devuelven al cerrar.
- K3. Los atajos de una sola tecla se pueden desactivar o remapear (WCAG 2.2 `2.1.4`).
- K4. El arrastre (importar) siempre tiene alternativa por botón (WCAG 2.2 `2.5.7`).

## Anuncios de estado

| Evento                            | Región                    | Cortesía    | Regla                                     |
| --------------------------------- | ------------------------- | ----------- | ----------------------------------------- |
| Error de validación bloqueante    | ValidationPanel (resumen) | `assertive` | Anunciar el resumen, no cada revalidación |
| Aviso de validación               | ValidationPanel (resumen) | `polite`    | Sin interrumpir                           |
| Progreso de operación             | StatusMessage             | `polite`    | Progreso determinado o indeterminado      |
| Resultado (guardado, exportación) | Toast                     | `polite`    | Incluye acción de deshacer si aplica      |
| Migración con avisos              | MigrationReport           | `polite`    | Resumen al finalizar                      |

## Estados y cobertura

| Estado transversal  | Pantallas que lo requieren              | Componente responsable              |
| ------------------- | --------------------------------------- | ----------------------------------- |
| Vacío               | Inicio, Importar, Revisar               | RecentDrafts, FileDrop, Diagnostics |
| Carga               | Todas                                   | Esqueletos y StatusMessage          |
| Error               | Configurar, Revisar, Exportar, Importar | ValidationPanel, Toast              |
| Movimiento reducido | Todas                                   | Regla transversal A5                |
| Tema claro/oscuro   | Todas                                   | Tokens semánticos                   |

## Trazabilidad

| Elemento                | Referencia                                        |
| ----------------------- | ------------------------------------------------- |
| Contratos de componente | `docs/05-ux/component-contracts.md`               |
| Accesibilidad           | `docs/05-ux/accessibility-matrix.md`              |
| Tokens                  | `docs/06-design-system/design-system.md`          |
| Baseline visual         | `docs/06-design-system/visual-baseline.md`        |
| Requisito               | `NFR-ACC-001` (`docs/01-product/requirements.md`) |
