# Inventario de reglas

- ID: DOC-VAL-RULES-001
- Estado: draft
- Propietario: pendiente
- Última revisión: 2026-10-05
- Requisitos relacionados: FR-CAT-001, FR-RESOLVE-001, FR-VALIDATE-001; NFR-DET-001, NFR-MIG-001
- Sustituye/sustituido por: —

> Documento `draft`. Ninguna ficha está `accepted`. Las fichas heredadas conservan su fila original; lo nuevo se marca explícitamente.

## Resumen

| ID | Fuente | Condición | Efecto heredado | Estado nuevo | Pruebas |
|---|---|---|---|---|---|
| RULE-COMP-001 | `validation-rules.json` | `count_gt(compositor,1)` | block | Formalizar cardinalidad en dominio; conservar diagnóstico | 0,1,2 selecciones |
| RULE-DM-001 | v5.1 | compositor=gnome | change dm=gdm | Derivación explicable/reversible según policy | GNOME/no GNOME/manual conflict |
| RULE-GPU-001 | v5.1 | gpu=nvidia AND compositor=sway | change sin target | Incompleta; evidence-required | NVIDIA/Sway permutations |
| RULE-KERNEL-001 | App.tsx | ningún kernel | block | Requisito de cardinalidad | none/one/multiple |
| RULE-BROWSER-001 | App.tsx | ningún browser | suggest | No bloqueante | none/one |
| RULE-PKG-001 | pendiente (SRC-002/SRC-003) | `selected(pkg)` sin `provides` | — (nueva) | No verificado — fuente primaria pendiente | disponibilidad/no disponibilidad |
| RULE-KERNEL-002 | derivada de FR-RESOLVE-001 | fs seleccionado no aportado por kernel | — (nueva) | Conflicto de capabilities | combinaciones kernel/fs |
| RULE-DUP-001 | instancia v5.1 (CON-004) | identidad de paquete repetida | — (nueva) | Identidad pendiente DEC-002 | único/duplicado |

## Convención de códigos de diagnóstico

Todas las fichas emiten `Diagnostic` dentro de las familias declaradas en `docs/04-interfaces/errors-events.md`. Para esta documentación `draft` se reservan códigos de la familia `AM-RULE` (evaluación de reglas, etapa 8 de `pipeline.md`); un conflicto detectado por una regla y resuelto en la etapa 9 puede reportarse con la familia `AM-RES`. La asignación numérica exacta queda pendiente de catálogo y **no** es un contrato aceptado.

| Código | Mensaje | Severidad | Bloqueante |
|---|---|---|---|
| `AM-RULE-001` | cardinalidad de compositor excedida | error | sí |
| `AM-RULE-002` | derivación de `dm` aplicada | info | no |
| `AM-RULE-003` | conflicto GPU/compositor sin resolución | error | sí (provisional) |
| `AM-RULE-004` | cardinalidad de kernel inválida | error | sí |
| `AM-RULE-005` | ausencia de navegador | warning | no |
| `AM-RULE-006` | disponibilidad de paquete no verificada | warning | no |
| `AM-RULE-007` | conflicto kernel/filesystem | error | sí |
| `AM-RULE-008` | identidad de paquete duplicada | error | sí |
| `AM-RULE-009` | operador de regla no soportado | error | sí |

## Ficha obligatoria

Cada regla futura declara ID, fuente, tipos, condición, prioridad, efecto, severidad, blocking, diagnóstico, remedy, interacciones, test cases y versión de introducción/deprecación.

### RULE-COMP-001 — Compositor único

| Campo | Valor |
|---|---|
| ID | RULE-COMP-001 |
| Fuente | `reference/v5.1/validation-rules.json` regla `single-compositor`; instancia `reference/v5.1/archmaker.instance.v5.1.json` (`"single_compositor": true`) |
| Requisitos / amenazas | FR-RESOLVE-001, FR-VALIDATE-001; NFR-DET-001 |
| Tipos | `ValueRef: compositor`; `Integer: 1` |
| Condición formal (AST) | `count_gt(compositor, 1)` ≡ `gt(count(compositor), Literal(1))` |
| Efecto | `block` |
| Diagnóstico | `AM-RULE-001` |
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
| Tipos | `ValueRef: compositor`; `Id: gnome`; `target: dm`; `allowed: [gdm]` |
| Condición formal (AST) | `eq(compositor, "gnome")` |
| Efecto | `change` / `derive` sobre `dm` con `allowed = ["gdm"]` |
| Diagnóstico | `AM-RULE-002` |
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
| Tipos | `ValueRef: gpu`; `Id: nvidia`; `ValueRef: compositor`; `Id: sway` |
| Condición formal (AST) | `all([eq(gpu, "nvidia"), eq(compositor, "sway")])` |
| Efecto | `change` heredado **sin `target`** (incompleto) |
| Diagnóstico | `AM-RULE-003` |
| Severidad | error |
| Bloqueante | sí (provisional) |
| Remedio / sugerencia | `evidence-required`: definir el cambio efectivo o el conflicto explícito. **No verificado — fuente primaria pendiente (SRC-002/SRC-003)**. |
| Fase | MVP |
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
| Tipos | `ValueRef: kernel` |
| Condición formal (AST) | `not(count_gt(kernel, 0))` ≡ `eq(count(kernel), 0)` |
| Efecto | `block` |
| Diagnóstico | `AM-RULE-004` |
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
| Tipos | `ValueRef: browser` |
| Condición formal (AST) | `eq(count(browser), 0)` |
| Efecto | `suggest` |
| Diagnóstico | `AM-RULE-005` |
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
| Tipos | `ValueRef: pkg`; `Capability: pkg.<id>` |
| Condición formal (AST) | `all([selected(pkg), not(provides(pkg.<id>))])` |
| Efecto | `suggest` / diagnóstico de disponibilidad |
| Diagnóstico | `AM-RULE-006` |
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
| Tipos | `ValueRef: fs`; `Capability: fs.<id>` |
| Condición formal (AST) | `all([selected(fs), not(provides(fs.<id>))])` |
| Efecto | `block` (conflicto de capabilities) |
| Diagnóstico | `AM-RULE-007` |
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
| Tipos | `ValueRef: packageId`; `Integer: 1` |
| Condición formal (AST) | `count_gt(packageId, 1)` |
| Efecto | `block` |
| Diagnóstico | `AM-RULE-008` |
| Severidad | error |
| Bloqueante | sí |
| Remedio / sugerencia | Unificar la definición duplicada o resolver el ámbito de identidad (DEC-002: global `vendor.kind.id` vs scoped). |
| Fase | MVP |
| Interacciones | CON-004; DEC-002; catálogo con referencias únicas (`DM-CATALOG`). |
| Prueba positiva | identidad de paquete única → sin diagnóstico. |
| Prueba negativa | `base-devel` definido dos veces → `AM-RULE-008` bloqueante. |
| Migración v5.1→nuevo | Detecta el duplicado; la resolución de identidad queda supeditada a DEC-002. |
