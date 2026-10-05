---
id: DOC-VAL-OPS-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - quality
reviewers:
  - independent-reviewer
---

# Especificación de operadores

ID `DOC-VAL-OPS-001` · Estado `in-review` · Propietario Calidad. Especifica el **vocabulario
cerrado** de operadores del AST de reglas. Autoridad de forma: `contracts/json-schema/rule.schema.json`
(`$defs.operator` y los `$defs` de cada expresión). Autoridad de códigos:
`docs/04-interfaces/errors-events.md` (catálogo canónico).

Un operador fuera de este vocabulario **produce diagnóstico bloqueante** (`AM-RULE-009`), nunca se
ignora ni se evalúa como `false` silenciosamente.

## Capas de especificación

La especificación separa explícitamente siete capas. Cada operador se describe una vez y cada capa
responde a una pregunta distinta.

| Capa | Qué fija | Dónde vive |
|---|---|---|
| **Sintaxis** | Nombre/clave del operador y forma de dato (posición y tipo de los argumentos). | Este documento y `rule.schema.json#/$defs.operator`. |
| **Schema** | Restricción formal: tipos, cardinalidad, cierre (`additionalProperties:false`). | `rule.schema.json#/$defs/<operador>Expr`. |
| **Referencia** | Vocabulario de los identificadores resueltos. | `common#/$defs/entityId` (DEC-002), `common#/$defs/capabilityId`, `$defs.targetId`. |
| **Regla** | Integración en el documento `Rule` (`when`/`action`). | `rule.schema.json` (raíz). |
| **Resolución** | Etapa del pipeline donde se evalúa. | `docs/07-validation/pipeline.md` (etapa 8; `provides` en la 9). |
| **Objetivo** | Efecto sobre un `target` (`change`/`derive`/`lock`). | `rule.schema.json` (`target`, `allowed`). |
| **Procedencia** | Origen y estado del operador (canónico / legacy / v1). | Este documento y el `$comment` del schema. |

Los operadores de **condición** no producen objetivo: el objetivo (`target`/`allowed`) pertenece al
**efecto** de la regla contenedora. `count` es un **término** (devuelve entero, no booleano) y por
eso no es miembro de la unión booleana `condition`: aparece como operando de `eq`/`ne`/`in`.

## Vocabulario normativo

Vocabulario cerrado, token a token (`rule.schema.json#/$defs.operator`):

| Operador | Forma (sintaxis) | `$defs` (schema) | Entradas (tipos) | Resultado | Estado / procedencia |
|---|---|---|---|---|---|
| `all` | `{"all": [Expr, …]}` | `allExpr` | `List<condition>`, ≥ 1 | bool | canónico |
| `any` | `{"any": [Expr, …]}` | `anyExpr` | `List<condition>`, ≥ 1 | bool | canónico |
| `not` | `{"not": Expr}` | `notExpr` | `condition` | bool | canónico |
| `eq` | `{"eq": [A, B]}` | `eqExpr` | `(operand, operand)` | bool | canónico |
| `ne` | `{"ne": [A, B]}` | `neExpr` | `(operand, operand)` | bool | canónico |
| `in` | `{"in": [A, [B, …]]}` | `inExpr` | `(operand, List<operand> ≥ 1)` | bool | canónico |
| `required` | `{"required": Ref}` | `requiredExpr` | `entityId` | bool | canónico (ratificado 2026-10-05) |
| `selected` | `{"selected": Id}` | `selectedExpr` | `entityId` | bool | canónico |
| `count` | `{"count": Ref}` | `countExpr` | `entityId` | integer | canónico (término) |
| `provides` | `{"provides": Cap}` | `providesExpr` | `capabilityId` | bool | canónico |
| `count_gt` | `{"count_gt": [Ref, n]}` | `countGtExpr` | `(entityId, integer ≥ 0)` | bool | **legacy (solo migración)** |
| `target_is` | `{"target_is": Target}` | `targetIsExpr` | `targetId` | bool | **proposed (diferido a v1)** |

`operand` es una unión tipada (`rule.schema.json#/$defs/operand`): `entityId` | `boolean` | `integer`
| `countExpr`. **No existe string literal sin tipar**: las opciones se comparan como `entityId`
(`vendor.kind.id`, DEC-002) y las literales como `boolean`/`integer`.

## Contrato por operador

### `all`

| Campo | Valor |
|---|---|
| Entradas | `List<condition>` con al menos un elemento. |
| Condición | Verdadero sii **todas** las hojas son verdaderas (conjunción; cortocircuito). La lista vacía no es representable (`minItems: 1`). |
| Efecto | Ninguno propio: hereda el `action` de la regla contenedora. |
| Mensaje | El de la regla contenedora; combinador sin código propio. |
| Remedio | El de la regla contenedora. |
| Referencia / Resolución / Objetivo / Procedencia | Sin referencias propias; etapa 8; sin objetivo; canónico. |

### `any`

| Campo | Valor |
|---|---|
| Entradas | `List<condition>` con al menos un elemento. |
| Condición | Verdadero sii **al menos una** hoja es verdadera (disyunción; cortocircuito). La lista vacía no es representable. |
| Efecto | Ninguno propio: hereda el `action` de la regla contenedora. |
| Mensaje | El de la regla contenedora. |
| Remedio | El de la regla contenedora. |
| Referencia / Resolución / Objetivo / Procedencia | Sin referencias propias; etapa 8; sin objetivo; canónico. |

### `not`

| Campo | Valor |
|---|---|
| Entradas | Una `condition`. |
| Condición | Negación lógica de la sub-expresión. |
| Efecto | Ninguno propio: hereda el `action` de la regla contenedora. |
| Mensaje | El de la regla contenedora. |
| Remedio | El de la regla contenedora. |
| Referencia / Resolución / Objetivo / Procedencia | Sin referencias propias; etapa 8; sin objetivo; canónico. |

### `eq`

| Campo | Valor |
|---|---|
| Entradas | Exactamente dos `operand`. |
| Condición | Igualdad de los dos operando. El schema **no** impone que ambos compartan tipo; el motor compara valores del mismo tipo y un par heterogéneo se evalúa como falso (nunca lanza). |
| Efecto | Ninguno propio como condición; habilita el efecto de la regla (p. ej. `change` en RULE-DM-001). |
| Mensaje | El de la regla contenedora (p. ej. `AM-RULE-002`). |
| Remedio | El de la regla contenedora. |
| Referencia / Resolución / Objetivo / Procedencia | `entityId` (DEC-002) u operando literal; etapa 8; sin objetivo; canónico. |

### `ne`

| Campo | Valor |
|---|---|
| Entradas | Exactamente dos `operand`. |
| Condición | Desigualdad de los dos operandos (complemento de `eq`). |
| Efecto | Ninguno propio: hereda el `action` de la regla contenedora. |
| Mensaje | El de la regla contenedora. |
| Remedio | El de la regla contenedora. |
| Referencia / Resolución / Objetivo / Procedencia | Como `eq`; etapa 8; canónico. |

### `in`

| Campo | Valor |
|---|---|
| Entradas | Un `operand` y una `List<operand>` con al menos un elemento. |
| Condición | Pertenencia del primer operando a la lista. |
| Efecto | Ninguno propio: hereda el `action` de la regla contenedora. |
| Mensaje | El de la regla contenedora. |
| Remedio | El de la regla contenedora. |
| Referencia / Resolución / Objetivo / Procedencia | `entityId`/literales; etapa 8; canónico. |

### `required`

| Campo | Valor |
|---|---|
| Entradas | Un `entityId` (ValueRef). |
| Condición | Verdadero sii la cardinalidad de la referencia es **≥ 1**. |
| Efecto | Típicamente `block` con `AM-RULE-004` (kernel) o `suggest` con `AM-RULE-005` (browser). |
| Mensaje | `AM-RULE-004` (`error`, bloqueante) o `AM-RULE-005` (`warning`, no bloqueante), según la regla. |
| Remedio | «Seleccionar exactamente un `kernel`» / «Sugerir un navegador y permitir continuar». |
| Referencia | `entityId` con forma `vendor.kind.id` (DEC-002). |
| Resolución / Objetivo / Procedencia | Etapa 8; sin objetivo; canónico (ratificado). |

### `selected`

| Campo | Valor |
|---|---|
| Entradas | Un `entityId` (Id de opción). |
| Condición | Verdadero sii la opción está seleccionada en el draft. |
| Efecto | Ninguno propio; habilita diagnósticos como `AM-RULE-006` (`warning`). |
| Mensaje | `AM-RULE-006` cuando la regla lo asocia. |
| Remedio | El de la regla contenedora (p. ej. informar disponibilidad no verificada). |
| Referencia / Resolución / Objetivo / Procedencia | `entityId`; etapa 8; sin objetivo; canónico. |

### `count`

| Campo | Valor |
|---|---|
| Entradas | Un `entityId` (ValueRef). |
| Condición | **Término**, no booleano: número de selecciones efectivas de la referencia. No es miembro de la unión `condition`; se usa como `operand`. |
| Efecto | Ninguno propio; participa en `eq`/`ne`/`in` (p. ej. `eq(count(compositor), 0)`). |
| Mensaje | El de la regla contenedora (p. ej. `AM-RULE-004`). |
| Remedio | El de la regla contenedora. |
| Referencia / Resolución / Objetivo / Procedencia | `entityId`; etapa 8; sin objetivo; canónico. Un `count` sobre referencia desconocida produce diagnóstico bloqueante. |

### `provides`

| Campo | Valor |
|---|---|
| Entradas | Un `capabilityId` (`vendor.kind`). |
| Condición | Verdadero sii la resolución aporta la capability indicada. Se evalúa sobre el resultado de la etapa 9. |
| Efecto | Ninguno propio; habilita conflictos bloqueantes como `AM-RULE-007`. |
| Mensaje | `AM-RULE-007` (`error`, bloqueante) en RULE-KERNEL-002; `AM-RULE-006` (`warning`) en la variante de disponibilidad. |
| Remedio | «Elegir un `fs` aportado por el kernel seleccionado, o cambiar de kernel». Compatibilidad concreta: no verificado — fuente primaria pendiente (SRC-002). |
| Referencia | `capabilityId` (`common#/$defs/capabilityId`). |
| Resolución / Objetivo / Procedencia | Etapa 9; sin objetivo; canónico. |

### `count_gt` (legacy)

| Campo | Valor |
|---|---|
| Entradas | Un `entityId` y un `integer ≥ 0`. |
| Condición | `count_gt(Ref, n) ≡ gt(count(Ref), n)`. Operador **heredado**. |
| Efecto | El runtime nuevo **no lo ejecuta**: la migración v5.1 lo **reescribe** a la forma normalizada `count` con umbral y luego evalúa la forma normalizada. |
| Mensaje | Tras la reescritura, el de la regla contenedora (p. ej. `AM-RULE-001`). |
| Remedio | El de la regla contenedora. |
| Referencia / Resolución / Objetivo / Procedencia | `entityId`; se parsea (no se ejecuta) en la migración; sin objetivo; **legacy**. |

### `target_is` (diferido a v1)

| Campo | Valor |
|---|---|
| Entradas | Un `targetId` (un segmento, DEC-001). |
| Condición | Verdadero sii el target de exportación/ejecución activo coincide. |
| Efecto | Ninguno en el MVP; fuera del alcance MVP. |
| Mensaje | El de la regla contenedora; fase v1 (`VAL-TARGET`). |
| Remedio | El de la regla contenedora. |
| Referencia / Resolución / Objetivo / Procedencia | `$defs.targetId`; etapa 12 (v1); sin objetivo; **proposed/v1**. El schema lo mantiene representable para no romper documentos v1, pero ninguna regla MVP lo usa. |

## Operador desconocido o no soportado

- **Sintaxis:** cualquier clave de operador fuera de `$defs.operator`.
- **Condición:** no existe; no se evalúa.
- **Efecto:** se rechaza el documento (`condition` es una unión cerrada) y el motor emite un
  diagnóstico. En runtime, el diagnóstico es `AM-RULE-009` (`error`, **bloqueante**),
  determinista e independiente del valor de la referencia; nunca se evalúa como `false`
  silenciosamente.
- **Mensaje:** `AM-RULE-009`.
- **Remedio:** usar un operador del vocabulario normativo; si la intención no es expresable con
  ninguno, abrir decisión (FR/NFR + ADR) antes de ampliar el vocabulario.
- **Pruebas:** `CORPUS-UNKNOWN-ADV-1` (`count_gte`) y `CORPUS-UNKNOWN-ADV-2` (clave arbitraria) en
  `contracts/json-schema/rule-corpus/operators.corpus.json`.
