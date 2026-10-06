---
id: DOC-VAL-RULES-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - quality
reviewers:
  - independent-reviewer
---

# Inventario de reglas

ID `DOC-VAL-RULES-001` · Estado `draft` · Propietario Calidad · Última revisión 2026-10-06.
Requisitos relacionados: FR-CAT-001, FR-RESOLVE-001, FR-VALIDATE-001; NFR-DET-001, NFR-MIG-001.

> Documento `draft`. Ninguna ficha está `accepted`. Las fichas heredadas conservan su fila original;
> lo nuevo se marca explícitamente. Cada condición usa **solo** operadores del vocabulario cerrado de
> `contracts/json-schema/rule.schema.json#/$defs.operator` (ver `docs/07-validation/operator-spec.md`);
> no hay operadores indefinidos.

## Resumen

| ID | Fuente | Operadores | Condición | Efecto heredado | Estado nuevo | Pruebas |
|---|---|---|---|---|---|---|
| RULE-COMP-001 | `validation-rules.json` (`single-compositor`) | `count_gt` (legacy) | `count_gt(compositor,1)` | block | Formalizar cardinalidad; normalizar a `count` | 0,1,2 selecciones |
| RULE-DM-001 | `validation-rules.json` (`gdm-gnome`) | `eq` | compositor=gnome | change dm=gdm | Derivación explicable/reversible | GNOME/no GNOME/manual conflict |
| RULE-GPU-001 | `validation-rules.json` (`nvidia-sway`) | `all`, `eq` | gpu=nvidia AND compositor=sway | change sin target | **No implementable** (SRC-002/SRC-003) | NVIDIA/Sway permutations |
| RULE-KERNEL-001 | `App.tsx` (SRC-001) | `not`, `required` | ningún kernel | block | Requisito de cardinalidad | none/one/multiple |
| RULE-BROWSER-001 | `App.tsx` (SRC-001) | `eq`, `count` | ningún browser | suggest | No bloqueante | none/one |
| RULE-PKG-001 | pendiente (SRC-002/SRC-003) | `selected`, `not`, `provides` | `selected(pkg)` sin `provides` | — (nueva) | No verificado — fuente primaria pendiente | disponibilidad/no disponibilidad |
| RULE-KERNEL-002 | derivada de FR-RESOLVE-001 | `selected`, `not`, `provides` | fs seleccionado no aportado por kernel | — (nueva) | Conflicto de capabilities | combinaciones kernel/fs |
| RULE-DUP-001 | instancia v5.1 (CON-004) | `count_gt` (legacy) | identidad de paquete repetida | — (nueva) | Identidad global DEC-002 | único/duplicado |

## Reconciliación con el vocabulario de operadores

Fase R8 (AUD-017). La fuente heredada inmutable `reference/v5.1/validation-rules.json` contiene tres
reglas. Se reconcilian una a una contra el vocabulario cerrado; el resto del inventario se conserva
como referencia (no se reconstruyen artefactos de `reference/`).

| Regla heredada (`validation-rules.json`) | Rule nueva | Operador(es) requerido(s) | Estado del operador | ¿Implementable en MVP? | Disposición |
|---|---|---|---|---|---|
| `single-compositor` (`count_gt(compositor,1)`, block) | RULE-COMP-001 | `count_gt` | legacy (solo migración) | **Sí**, tras normalizar `count_gt` → `count` + umbral | Conservar el bloqueo; migrar la forma. |
| `gdm-gnome` (`eq(compositor,"gnome")`, change dm=gdm) | RULE-DM-001 | `eq` | canónico | **Sí** | Conservar la derivación con explicabilidad/reversibilidad. |
| `nvidia-sway` (`all([eq(gpu,"nvidia"), eq(compositor,"sway")])`, change **sin target**) | RULE-GPU-001 | `all`, `eq` | canónico | **No** | `evidence-required`: el efecto `change` no declara destino; depende de SRC-002/SRC-003. |

Reglas del inventario no presentes en `validation-rules.json` (con fuente declarada aparte):

| Rule | Operador(es) requerido(s) | Estado del operador | ¿Implementable en MVP? | Disposición |
|---|---|---|---|---|
| RULE-KERNEL-001 | `not`, `required` | canónico | **Sí** | Cardinalidad mínima de kernel. |
| RULE-BROWSER-001 | `eq`, `count` | canónico | **Sí** | Sugerencia no bloqueante. |
| RULE-PKG-001 | `selected`, `not`, `provides` | canónico | **Sí** | Disponibilidad no verificada (SRC-002/003). |
| RULE-KERNEL-002 | `selected`, `not`, `provides` | canónico | **Sí** | Conflicto kernel/filesystem (SRC-002). |
| RULE-DUP-001 | `count_gt` | legacy (solo migración) | **Sí**, tras normalizar | Identidad global `vendor.kind.id` (DEC-002). |

Resultado: **7 reglas implementables** en MVP (RULE-COMP-001, RULE-DM-001, RULE-KERNEL-001,
RULE-BROWSER-001, RULE-PKG-001, RULE-KERNEL-002, RULE-DUP-001) y **1 no implementable**
(RULE-GPU-001). El operador `target_is` queda **diferido a v1** y **ninguna regla heredada lo
requiere**; el operador `count_gt` es **legacy** y solo lo usan dos reglas, ambas normalizables.

## Convención de códigos de diagnóstico

Todas las fichas emiten `Diagnostic` dentro de las familias declaradas en
`docs/04-interfaces/errors-events.md`. La **asignación numérica canónica** de los códigos `AM-RULE-00x`
usados por este inventario vive en `docs/04-interfaces/errors-events.md` → «Catálogo canónico de
códigos (fuente única)»; este documento **no** redefine códigos. Para esta documentación `draft` se
reservan códigos de la familia `AM-RULE` (evaluación de reglas, etapa 8 de `pipeline.md`); un
conflicto detectado por una regla y resuelto en la etapa 9 puede reportarse con la familia `AM-RES`.
La asignación **no** es un contrato `accepted`.

Códigos referenciados por las fichas de este inventario: `AM-RULE-001` … `AM-RULE-009` (ver catálogo
canónico en `errors-events.md`).

## Ficha obligatoria

Cada regla futura declara ID, fuente, entradas, operadores, condición, prioridad, efecto, severidad,
blocking, mensaje/diagnóstico, remedio, interacciones, test cases y versión de
introducción/deprecación.

### RULE-COMP-001 — Compositor único

| Campo | Valor |
|---|---|
| ID | RULE-COMP-001 |
| Fuente | `reference/v5.1/validation-rules.json` regla `single-compositor`; instancia `reference/v5.1/archmaker.instance.v5.1.json` (`"single_compositor": true`) |
| Requisitos / amenazas | FR-RESOLVE-001, FR-VALIDATE-001; NFR-DET-001 |
| Entradas | `ValueRef: compositor`; `Integer: 1` |
| Operadores | `count_gt` (legacy, solo migración) → normalizado a `count` + umbral |
| Condición formal (AST) | `count_gt(compositor, 1)` ≡ `gt(count(compositor), Literal(1))` |
| Efecto | `block` |
| Mensaje / diagnóstico | `AM-RULE-001` |
| Severidad | error |
| Bloqueante | sí |
| Remedio / sugerencia | Reducir `compositor` a una sola selección; la UI desmarca la anterior y explica el cambio. |
| Fase | MVP |
| Interacciones | RULE-DM-001 presupone un único `compositor`; CON-001 (cardinalidad explícita). |
| Prueba positiva | 1 selección de `compositor` → sin diagnóstico. |
| Prueba negativa | 2 selecciones de `compositor` → `AM-RULE-001` bloqueante. |
| Migración v5.1→nuevo | Conserva `block` como diagnóstico; formaliza la cardinalidad en el dominio (`DM-DRAFT`). `count_gt` es `legacy-pending` (CON-002). |

### RULE-DM-001 — Derivación GNOME→GDM

| Campo | Valor |
|---|---|
| ID | RULE-DM-001 |
| Fuente | `reference/v5.1/validation-rules.json` regla `gdm-gnome`; instancia (`"auto_logic": "GNOME -> GDM bloqueado"`) |
| Requisitos / amenazas | FR-RESOLVE-001, FR-PRESET-001 |
| Entradas | `ValueRef: compositor`; `Id: gnome`; `target: dm`; `allowed: [gdm]` |
| Operadores | `eq` |
| Condición formal (AST) | `eq(compositor, "gnome")` |
| Efecto | `change` / `derive` sobre `dm` con `allowed = ["gdm"]` |
| Mensaje / diagnóstico | `AM-RULE-002` |
| Severidad | info |
| Bloqueante | no |
| Remedio / sugerencia | Fijar `dm = gdm` de forma explicable y reversible según `policy`; nunca silenciosa. |
| Fase | MVP |
| Interacciones | RULE-COMP-001 (un solo compositor); precedencia manual/derived/locked (UC-004). |
| Prueba positiva | `compositor = gnome` → `dm = gdm` con diagnóstico informativo. |
| Prueba negativa | `compositor = gnome` y `dm` manual distinto → conflicto reportado, sin mutación silenciosa. |
| Migración v5.1→nuevo | Se conserva la derivación; se añade explicabilidad/reversibilidad. |

### RULE-GPU-001 — NVIDIA con Sway

| Campo | Valor |
|---|---|
| ID | RULE-GPU-001 |
| Fuente | `reference/v5.1/validation-rules.json` regla `nvidia-sway` |
| Requisitos / amenazas | FR-RESOLVE-001, FR-VALIDATE-001 |
| Entradas | `ValueRef: gpu`; `Id: nvidia`; `ValueRef: compositor`; `Id: sway` |
| Operadores | `all`, `eq` (ambos canónicos) |
| Condición formal (AST) | `all([eq(gpu, "nvidia"), eq(compositor, "sway")])` |
| Efecto | `change` heredado **sin `target`** (incompleto) |
| Mensaje / diagnóstico | `AM-RULE-003` |
| Severidad | error |
| Bloqueante | n/a — no implementable |
| Estado de implementación | **No implementable** hasta disponer de evidencia primaria (SRC-002/SRC-003) que defina el `target` del cambio o el conflicto explícito. La condición es expresable con el vocabulario; lo que falta es el **efecto** (destino del cambio), no el operador. |
| Remedio / sugerencia | `evidence-required`: definir el cambio efectivo o el conflicto explícito. **No verificado — fuente primaria pendiente (SRC-002/SRC-003)**. |
| Fase | deferred (evidencia pendiente SRC-002/SRC-003) |
| Interacciones | RULE-COMP-001; resolución de capabilities (etapa 9). |
| Prueba positiva | `gpu = nvidia` y `compositor = sway` con cambio definido → diagnóstico trazable. |
| Prueba negativa | `gpu = nvidia` y `compositor = sway` sin `target` ni conflicto → regla incompleta, no evaluable. |
| Migración v5.1→nuevo | **Incompleta**: el efecto `change` no declara destino; requiere evidencia antes de implementar. |

### RULE-KERNEL-001 — Kernel obligatorio

| Campo | Valor |
|---|---|
| ID | RULE-KERNEL-001 |
| Fuente | prototipo heredado `App.tsx` (SRC-001) |
| Requisitos / amenazas | FR-VALIDATE-001, FR-MANIFEST-001 |
| Entradas | `ValueRef: kernel` |
| Operadores | `not`, `required` |
| Condición formal (AST) | `not(required(kernel))` ≡ `eq(count(kernel), 0)` |
| Efecto | `block` |
| Mensaje / diagnóstico | `AM-RULE-004` |
| Severidad | error |
| Bloqueante | sí |
| Remedio / sugerencia | Seleccionar exactamente un `kernel` (opción `single`). |
| Fase | MVP |
| Interacciones | Invariante de cardinalidad de la definición `kernel` (`single: true`). |
| Prueba positiva | 1 selección de `kernel` → sin diagnóstico. |
| Prueba negativa | 0 selecciones de `kernel` → `AM-RULE-004` bloqueante. |
| Migración v5.1→nuevo | Requisito de cardinalidad movido de UI embebida a regla/Rule del core. |

### RULE-BROWSER-001 — Navegador sugerido

| Campo | Valor |
|---|---|
| ID | RULE-BROWSER-001 |
| Fuente | prototipo heredado `App.tsx` (SRC-001) |
| Requisitos / amenazas | FR-VALIDATE-001 |
| Entradas | `ValueRef: browser` |
| Operadores | `eq`, `count` |
| Condición formal (AST) | `eq(count(browser), 0)` |
| Efecto | `suggest` |
| Mensaje / diagnóstico | `AM-RULE-005` |
| Severidad | warning |
| Bloqueante | no |
| Remedio / sugerencia | Sugerir un navegador; el usuario puede continuar sin él. |
| Fase | MVP |
| Interacciones | Ninguna bloqueante. |
| Prueba positiva | 1 selección de `browser` → sin diagnóstico. |
| Prueba negativa | 0 selecciones de `browser` → `AM-RULE-005` no bloqueante. |
| Migración v5.1→nuevo | Se conserva `suggest`; no bloquea manifest. |

### RULE-PKG-001 — Disponibilidad de paquete (nueva)

| Campo | Valor |
|---|---|
| ID | RULE-PKG-001 |
| Fuente | **no verificado — fuente primaria pendiente (SRC-002/SRC-003)** |
| Requisitos / amenazas | FR-CAT-001; NFR-OFF-001 |
| Entradas | `ValueRef: pkg`; `Capability: pkg.<id>` |
| Operadores | `selected`, `not`, `provides` |
| Condición formal (AST) | `all([selected(pkg), not(provides(pkg.<id>))])` |
| Efecto | `suggest` / diagnóstico de disponibilidad |
| Mensaje / diagnóstico | `AM-RULE-006` |
| Severidad | warning |
| Bloqueante | no |
| Remedio / sugerencia | Informar que la disponibilidad no puede verificarse; **no verificado — fuente primaria pendiente (SRC-002/SRC-003)**. No afirmar compatibilidad. |
| Fase | MVP |
| Interacciones | RULE-DUP-001; catálogo como fuente de definiciones (FR-CAT-001). |
| Prueba positiva | `pkg` seleccionado y `provides(pkg.<id>)` resuelto → sin diagnóstico. |
| Prueba negativa | `pkg` seleccionado sin `provides` → `AM-RULE-006` no bloqueante, con marca de no verificado. |
| Migración v5.1→nuevo | Nueva; sin correspondencia v5.1. No verificado. |

### RULE-KERNEL-002 — Conflicto kernel/filesystem (nueva)

| Campo | Valor |
|---|---|
| ID | RULE-KERNEL-002 |
| Fuente | derivada de FR-RESOLVE-001; capacidades `fs.*`; **no verificado — fuente primaria pendiente (SRC-002)** |
| Requisitos / amenazas | FR-RESOLVE-001, FR-VALIDATE-001; NFR-DET-001 |
| Entradas | `ValueRef: fs`; `Capability: fs.<id>` |
| Operadores | `selected`, `not`, `provides` |
| Condición formal (AST) | `all([selected(fs), not(provides(fs.<id>))])` |
| Efecto | `block` (conflicto de capabilities) |
| Mensaje / diagnóstico | `AM-RULE-007` |
| Severidad | error |
| Bloqueante | sí |
| Remedio / sugerencia | Elegir un `fs` aportado por el kernel seleccionado, o cambiar de kernel. Compatibilidad concreta: **no verificado — fuente primaria pendiente (SRC-002)**. |
| Fase | MVP |
| Interacciones | RULE-KERNEL-001; etapa 9 de `pipeline.md` (resolución de capabilities y conflictos). |
| Prueba positiva | `fs` seleccionado y `provides(fs.<id>)` presente → sin diagnóstico. |
| Prueba negativa | `fs` seleccionado sin `provides` → `AM-RULE-007` bloqueante; no se promueve a manifest. |
| Migración v5.1→nuevo | Nueva; formaliza un conflicto latente en la resolución. |

### RULE-DUP-001 — Identidad de paquete duplicada (nueva)

| Campo | Valor |
|---|---|
| ID | RULE-DUP-001 |
| Fuente | instancia v5.1 (`base-devel` aparece dos veces, CON-004); `docs/03-data/versioning-migrations.md` |
| Requisitos / amenazas | FR-CAT-001; NFR-MIG-001 |
| Entradas | `ValueRef: packageId`; `Integer: 1` |
| Operadores | `count_gt` (legacy, solo migración) |
| Condición formal (AST) | `count_gt(packageId, 1)` |
| Efecto | `block` |
| Mensaje / diagnóstico | `AM-RULE-008` |
| Severidad | error |
| Bloqueante | sí |
| Remedio / sugerencia | Unificar la definición duplicada aplicando el ámbito de identidad global `vendor.kind.id` (DEC-002). |
| Fase | MVP |
| Interacciones | CON-004; DEC-002; catálogo con referencias únicas (`DM-CATALOG`). |
| Prueba positiva | identidad de paquete única → sin diagnóstico. |
| Prueba negativa | `base-devel` definido dos veces → `AM-RULE-008` bloqueante. |
| Migración v5.1→nuevo | Detecta el duplicado; la resolución de identidad sigue DEC-002. |
