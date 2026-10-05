# Corpus de operadores

- ID: DOC-VAL-CORPUS-001
- Estado: draft
- Propietario: pendiente
- Última revisión: 2026-10-05
- Requisitos relacionados: FR-VALIDATE-001, FR-RESOLVE-001, FR-CAT-001
- Sustituye/sustituido por: —

> Documento `draft`. Los ejemplos JSON son **ilustrativos y no constituyen schema definitivo**. La gramática de referencia es `docs/07-validation/operator-spec.md`. Los códigos `AM-*` provienen del **catálogo canónico** de `docs/04-interfaces/errors-events.md` → «Catálogo canónico de códigos (fuente única)»; este documento no los redefine.

## Convención

- Un operador no soportado o desconocido **produce diagnóstico bloqueante**; nunca se ignora ni se evalúa como `false` silenciosamente.
- El diagnóstico de un combinador booleano (`all`, `any`, `not`) corresponde a la regla que lo contiene.
- Severidades: `error` (bloqueante) / `warning` / `info`.

## Estados

| Operador | Firma | Resultado | Estado (operator-spec) | Código típico |
|---|---|---|---|---|
| `all` | `List<Expr>` | bool | required | el de la regla contenedora |
| `any` | `List<Expr>` | bool | required | el de la regla contenedora |
| `not` | `Expr` | bool | required | el de la regla contenedora |
| `eq` | `(T,T)` | bool | required | el de la regla contenedora |
| `ne` | `(T,T)` | bool | required | el de la regla contenedora |
| `in` | `(T,List<T>)` | bool | required | el de la regla contenedora |
| `required` | `ValueRef` | bool | proposed | `AM-RULE-004` / `AM-RULE-005` |
| `selected` | `Id` | bool | proposed | `AM-RULE-006` |
| `count` | `ValueRef` | integer | proposed | `AM-RULE-001` |
| `provides` | `Capability` | bool | proposed | `AM-RULE-007` |
| `count_gt` | `(ValueRef,integer)` | bool | **legacy-pending (CON-002)** | `AM-RULE-001` |
| `target_is` | `TargetId` | bool | proposed | `AM-RULE-003` |
| *(desconocido)* | — | — | no soportado | `AM-RULE-009` |

---

## all

- **Descripción formal:** `all(List<Expr>) → bool`; verdadero solo si **todas** las expresiones son verdaderas. La lista vacía es `true` (a confirmar en decisión).
- **Ejemplo ilustrativo (RULE-GPU-001):**
  ```json
  { "when": { "all": [ { "eq": ["gpu", "nvidia"] }, { "eq": ["compositor", "sway"] } ] } }
  ```
- **Satisface:** `gpu = nvidia` y `compositor = sway` → `all` verdadero.
- **Viola:** `gpu = amd` o `compositor = hyprland` → alguna hoja falsa → `all` falso.
- **Diagnóstico esperado:** el de la regla contenedora; aquí `AM-RULE-003`, `error` (bloqueante provisional).

## any

- **Descripción formal:** `any(List<Expr>) → bool`; verdadero si **al menos una** expresión es verdadera. La lista vacía es `false` (a confirmar).
- **Ejemplo ilustrativo:**
  ```json
  { "when": { "any": [ { "eq": ["gpu", "nvidia"] }, { "eq": ["gpu", "amd"] } ] } }
  ```
- **Satisface:** `gpu = nvidia` → `any` verdadero.
- **Viola:** `gpu = intel` → todas las hojas falsas → `any` falso.
- **Diagnóstico esperado:** el de la regla contenedora.

## not

- **Descripción formal:** `not(Expr) → bool`; negación lógica de la expresión.
- **Ejemplo ilustrativo (RULE-KERNEL-001):**
  ```json
  { "when": { "not": { "count_gt": ["kernel", 0] } } }
  ```
- **Satisface:** `count(kernel) = 0` → `not` verdadero (dispara la regla).
- **Viola (no dispara):** `count(kernel) = 1` → `not` falso.
- **Diagnóstico esperado:** `AM-RULE-004`, `error` (bloqueante).

## eq

- **Descripción formal:** `eq(T,T) → bool`; igualdad de valores del mismo tipo.
- **Ejemplo ilustrativo (RULE-DM-001):**
  ```json
  { "when": { "eq": ["compositor", "gnome"] }, "action": "change", "target": "dm", "allowed": ["gdm"] }
  ```
- **Satisface:** `compositor = gnome` → verdadero (aplica derivación).
- **Viola:** `compositor = hyprland` → falso.
- **Diagnóstico esperado:** `AM-RULE-002`, `info` (no bloqueante).

## ne

- **Descripción formal:** `ne(T,T) → bool`; desigualdad de valores del mismo tipo.
- **Ejemplo ilustrativo:**
  ```json
  { "when": { "ne": ["compositor", "gnome"] } }
  ```
- **Satisface:** `compositor = hyprland` → verdadero.
- **Viola:** `compositor = gnome` → falso.
- **Diagnóstico esperado:** el de la regla contenedora.

## in

- **Descripción formal:** `in(T,List<T>) → bool`; verdadero si el valor pertenece a la lista.
- **Ejemplo ilustrativo:**
  ```json
  { "when": { "in": ["compositor", ["gnome", "cosmic"]] } }
  ```
- **Satisface:** `compositor = cosmic` → verdadero.
- **Viola:** `compositor = sway` → falso.
- **Diagnóstico esperado:** el de la regla contenedora.

## required

- **Descripción formal:** `required(ValueRef) → bool` (proposed); verdadero si la cardinalidad de la referencia es **≥ 1**. Declarado en `operator-spec.md` como `proposed`; se usa en las reglas de cardinalidad mínima (RULE-KERNEL-001, RULE-BROWSER-001). Su promoción a operador estable queda pendiente de ratificación.
- **Ejemplo ilustrativo (equivalente a RULE-KERNEL-001):**
  ```json
  { "when": { "not": { "required": "kernel" } } }
  ```
- **Satisface:** `kernel` presente → `required` verdadero (la regla no dispara con `not`).
- **Viola:** `kernel` ausente → `required` falso → `not` verdadero (dispara la regla).
- **Diagnóstico esperado:** `AM-RULE-004`, `error` (bloqueante) para kernel; `AM-RULE-005`, `warning` (no bloqueante) para browser.

## selected

- **Descripción formal:** `selected(Id) → bool` (proposed); verdadero si la opción `Id` está seleccionada en el draft.
- **Ejemplo ilustrativo (RULE-PKG-001):**
  ```json
  { "when": { "all": [ { "selected": "pkg.firefox" }, { "not": { "provides": "pkg.firefox" } } ] } }
  ```
- **Satisface:** `pkg.firefox` seleccionado y sin `provides` → `all` verdadero.
- **Viola:** `pkg.firefox` no seleccionado → `all` falso.
- **Diagnóstico esperado:** `AM-RULE-006`, `warning` (no bloqueante).

## count

- **Descripción formal:** `count(ValueRef) → integer` (proposed); número de selecciones efectivas de la referencia. Devuelve entero, no booleano.
- **Ejemplo ilustrativo (RULE-COMP-001, como argumento de `count_gt`):**
  ```json
  { "when": { "count_gt": ["compositor", 1] } }
  ```
- **Satisface (regla dispara):** `count(compositor) = 2`.
- **Viola (no dispara):** `count(compositor) = 1`.
- **Diagnóstico esperado:** `AM-RULE-001`, `error` (bloqueante). Un `count` sobre referencia desconocida produce diagnóstico bloqueante.

## provides

- **Descripción formal:** `provides(Capability) → bool` (proposed); verdadero si la resolución aporta la capability indicada.
- **Ejemplo ilustrativo (RULE-KERNEL-002):**
  ```json
  { "when": { "all": [ { "selected": "fs.btrfs" }, { "not": { "provides": "fs.btrfs" } } ] } }
  ```
- **Satisface:** `fs.btrfs` seleccionado y no aportado por el kernel → `all` verdadero.
- **Viola:** `provides("fs.btrfs")` resuelto → `not` falso.
- **Diagnóstico esperado:** `AM-RULE-007`, `error` (bloqueante). Compatibilidad concreta: **no verificado — fuente primaria pendiente (SRC-002)**.

## count_gt

- **Estado:** **`legacy-pending` (CON-002).** Operador heredado no definido en el schema original; se conserva solo por compatibilidad con v5.1.
- **Descripción formal:** `count_gt(ValueRef, integer) → bool`; equivalente a `gt(count(ValueRef), Literal(integer))`.
- **Ejemplo ilustrativo (RULE-COMP-001):**
  ```json
  { "when": { "count_gt": ["compositor", 1] } }
  ```
- **Satisface (regla dispara):** `count(compositor) = 2` → `2 > 1`.
- **Viola (no dispara):** `count(compositor) = 1`.
- **Diagnóstico esperado:** `AM-RULE-001`, `error` (bloqueante). Requiere definir AST y tipos antes de implementarlo (CON-002).

## target_is

- **Estado:** **proposed.** No usado por ninguna regla v5.1.
- **Descripción formal:** `target_is(TargetId) → bool`; verdadero si el target de exportación/ejecución coincide con el indicado.
- **Ejemplo ilustrativo:**
  ```json
  { "when": { "target_is": "archinstall" } }
  ```
- **Satisface:** target `archinstall` seleccionado → verdadero.
- **Viola:** target `yaml` → falso.
- **Diagnóstico esperado:** el de la regla contenedora; fase v1/`VAL-TARGET`. Conjunto exacto de targets: DEC-001 pendiente.

## Operador desconocido o no soportado

- **Descripción formal:** cualquier clave de operador fuera de la tabla de `operator-spec.md`.
- **Ejemplo ilustrativo:**
  ```json
  { "when": { "count_gte": ["compositor", 1] } }
  ```
- **Caso que satisface (válido):** no aplica; el operador no existe.
- **Caso que viola:** aparece `count_gte` (o cualquier operador no declarado) en una condición.
- **Diagnóstico esperado:** `AM-RULE-009` (`operatorUnsupported`), `error`, **bloqueante**; nunca se evalúa como `false` silenciosamente. Determinista e independiente del valor de la referencia.
