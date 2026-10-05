---
id: DOC-VAL-CORPUS-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - quality
reviewers:
  - independent-reviewer
---

# Corpus de operadores

ID `DOC-VAL-CORPUS-001` · Estado `draft` · Propietario Calidad · Última revisión 2026-10-06.
Requisitos relacionados: FR-VALIDATE-001, FR-RESOLVE-001, FR-CAT-001.

> Documento `draft`. La gramática de referencia es `docs/07-validation/operator-spec.md`. Los códigos
> `AM-*` provienen del catálogo canónico de `docs/04-interfaces/errors-events.md`; este documento no
> los redefine.

## Convención

- Un operador no soportado o desconocido **produce diagnóstico bloqueante**; nunca se ignora ni se
  evalúa como `false` silenciosamente.
- El diagnóstico de un combinador booleano (`all`, `any`, `not`) corresponde a la regla que lo
  contiene.
- Severidades: `error` (bloqueante) / `warning` / `info`.
- Clases de caso: `positive` (debe aceptarse), `negative` (debe rechazarse por una violación
  concreta), `edge` (borde válido) y `adversarial` (entrada hostil o malformada).

## Fixture ejecutable

El corpus ejecutable vive en `contracts/json-schema/rule-corpus/operators.corpus.json`. Cada caso
declara `operator`, `kind` (`positive`/`negative`/`edge`/`adversarial`), `expect` (`valid`/`invalid`)
y una `instance` `Rule` completa. El job **`rule-corpus-validate`** de
`.github/workflows/json-schema.yml` valida cada `instance` contra
`contracts/json-schema/rule.schema.json` y exige el resultado declarado; además comprueba que todo
operador de `$defs.operator` tiene al menos un caso (cobertura del vocabulario). El gate falla si un
caso positivo es rechazado o un negativo es aceptado.

Total: **43 casos** (17 `valid`, 26 `invalid`) que cubren los **12** operadores del vocabulario.

## Vocabulario y estado

| Operador | Forma | Resultado | Estado (operator-spec) | Código típico |
|---|---|---|---|---|
| `all` | `List<Expr>` | bool | canónico | el de la regla contenedora |
| `any` | `List<Expr>` | bool | canónico | el de la regla contenedora |
| `not` | `Expr` | bool | canónico | el de la regla contenedora |
| `eq` | `(T,T)` | bool | canónico | el de la regla contenedora |
| `ne` | `(T,T)` | bool | canónico | el de la regla contenedora |
| `in` | `(T,List<T>)` | bool | canónico | el de la regla contenedora |
| `required` | `ValueRef` | bool | canónico | `AM-RULE-004` / `AM-RULE-005` |
| `selected` | `Id` | bool | canónico | `AM-RULE-006` |
| `count` | `ValueRef` | integer | canónico (término) | `AM-RULE-001` |
| `provides` | `Capability` | bool | canónico | `AM-RULE-007` |
| `count_gt` | `(ValueRef,integer)` | bool | **legacy (solo migración)** | `AM-RULE-001` |
| `target_is` | `TargetId` | bool | **proposed (diferido a v1)** | `AM-RULE-003` |
| *(desconocido)* | — | — | no soportado | `AM-RULE-009` |

## Casos por operador

### `all`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `all` con dos hojas verdaderas | positive | `CORPUS-ALL-POS` | `all([eq(gpu,nvidia), eq(compositor,sway)])` | `valid` |
| `all` con una sola hoja | edge | `CORPUS-ALL-EDGE` | `all([required(kernel)])` | `valid` |
| `all` con lista vacía | negative | `CORPUS-ALL-NEG` | `all([])` | `invalid` (`minItems:1`) |
| `all` con objeto en vez de lista | adversarial | `CORPUS-ALL-ADV` | `all({required:kernel})` | `invalid` |

### `any`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `any` con dos hojas | positive | `CORPUS-ANY-POS` | `any([eq(gpu,nvidia), eq(gpu,amd)])` | `valid` |
| `any` con una sola hoja | edge | `CORPUS-ANY-EDGE` | `any([selected(pkg.firefox)])` | `valid` |
| `any` con lista vacía | negative | `CORPUS-ANY-NEG` | `any([])` | `invalid` |

### `not`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `not` sobre `required` | positive | `CORPUS-NOT-POS` | `not(required(kernel))` | `valid` |
| `not` con lista en vez de expresión | negative | `CORPUS-NOT-NEG` | `not([required(kernel)])` | `invalid` |
| `not` con operador interno desconocido | adversarial | `CORPUS-NOT-ADV` | `not({unknown_op:kernel})` | `invalid` |

### `eq`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `eq` de dos `entityId` | positive | `CORPUS-EQ-POS` | `eq(compositor,gnome)` | `valid` |
| `eq` de dos booleanos | edge | `CORPUS-EQ-EDGE` | `eq(true,false)` | `valid` |
| `eq` con un solo operando | negative | `CORPUS-EQ-NEG` | `eq([compositor])` | `invalid` (`minItems:2`) |
| `eq` con literal sin tipar | adversarial | `CORPUS-EQ-ADV` | `eq(compositor,"gnome")` | `invalid` (no es `entityId`) |

### `ne`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `ne` de dos `entityId` | positive | `CORPUS-NE-POS` | `ne(compositor,gnome)` | `valid` |
| `ne` con un solo operando | negative | `CORPUS-NE-NEG` | `ne([1])` | `invalid` |
| `ne` con objeto en vez de lista | adversarial | `CORPUS-NE-ADV` | `ne({compositor:gnome})` | `invalid` |

### `in`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `in` con lista de dos | positive | `CORPUS-IN-POS` | `in(compositor,[gnome,cosmic])` | `valid` |
| `in` con lista de uno | edge | `CORPUS-IN-EDGE` | `in(compositor,[gnome])` | `valid` |
| `in` con lista vacía | negative | `CORPUS-IN-NEG` | `in(compositor,[])` | `invalid` |
| `in` con segundo argumento no lista | adversarial | `CORPUS-IN-ADV` | `in(compositor,gnome)` | `invalid` |

### `required`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `required(kernel)` | positive | `CORPUS-REQUIRED-POS` | `required(archmaker.environment.kernel)` | `valid` |
| `required` con id de un segmento | negative | `CORPUS-REQUIRED-NEG` | `required(kernel)` | `invalid` (no es `entityId`) |
| `required` con array | adversarial | `CORPUS-REQUIRED-ADV` | `required([kernel])` | `invalid` |

### `selected`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `selected(pkg.firefox)` | positive | `CORPUS-SELECTED-POS` | `selected(archmaker.environment.pkg.firefox)` | `valid` |
| `selected` con string vacío | negative | `CORPUS-SELECTED-NEG` | `selected("")` | `invalid` |

### `count`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `count` como operando de `eq` | positive | `CORPUS-COUNT-POS` | `eq(count(compositor), 0)` | `valid` |
| `count` con id de un segmento | negative | `CORPUS-COUNT-NEG` | `eq(count("compositor"), 0)` | `invalid` |
| `count` con array | adversarial | `CORPUS-COUNT-ADV` | `eq(count([compositor]), 0)` | `invalid` |

### `provides`

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `provides(fs.btrfs)` | positive | `CORPUS-PROVIDES-POS` | `provides(fs.btrfs)` | `valid` |
| `provides` con id de un segmento | negative | `CORPUS-PROVIDES-NEG` | `provides(fs)` | `invalid` (no es `capabilityId`) |
| `provides` con entero | adversarial | `CORPUS-PROVIDES-ADV` | `provides(123)` | `invalid` |

### `count_gt` (legacy)

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `count_gt(compositor,1)` heredado | positive | `CORPUS-COUNTGT-POS` | `count_gt(compositor,1)` | `valid` (se parsea para migrar) |
| `count_gt(kernel,0)` umbral cero | edge | `CORPUS-COUNTGT-EDGE` | `count_gt(kernel,0)` | `valid` |
| `count_gt` con umbral negativo | negative | `CORPUS-COUNTGT-NEG` | `count_gt(compositor,-1)` | `invalid` (`minimum:0`) |

### `target_is` (diferido a v1)

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `target_is` con target válido | positive | `CORPUS-TARGETIS-POS` | `target_is(archinstall)` | `valid` (representable, fuera de MVP) |
| `target_is` con string vacío | negative | `CORPUS-TARGETIS-NEG` | `target_is("")` | `invalid` |
| `target_is` con mayúsculas | adversarial | `CORPUS-TARGETIS-ADV` | `target_is("ArchInstall")` | `invalid` |

### Operador desconocido

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| operador `count_gte` no declarado | adversarial | `CORPUS-UNKNOWN-ADV-1` | `when: {count_gte: [compositor, 1]}` | `invalid` (`AM-RULE-009` en runtime) |
| clave de operador arbitraria | adversarial | `CORPUS-UNKNOWN-ADV-2` | `when: {foo: "bar"}` | `invalid` (`AM-RULE-009` en runtime) |

### Raíz de la regla

| Caso | Clase | ID de fixture | Entrada | Esperado |
|---|---|---|---|---|
| `action` fuera del enum (`execute`) | adversarial | `CORPUS-ROOT-ADV-1` | `action: "execute"` | `invalid` |
| campo raíz desconocido (`operators`) | adversarial | `CORPUS-ROOT-ADV-2` | `additionalProperties:false` | `invalid` |
| falta `when` obligatorio | negative | `CORPUS-ROOT-NEG` | sin `when` | `invalid` |

## Correspondencia con reglas

| Regla | Operadores del corpus que la ejercitan | Casos |
|---|---|---|
| RULE-COMP-001 | `count_gt` (legacy) | `CORPUS-COUNTGT-POS`, `CORPUS-COUNTGT-EDGE` |
| RULE-DM-001 | `eq` | `CORPUS-EQ-POS` |
| RULE-GPU-001 | `all`, `eq` | `CORPUS-ALL-POS` |
| RULE-KERNEL-001 | `not`, `required` | `CORPUS-NOT-POS`, `CORPUS-REQUIRED-POS` |
| RULE-BROWSER-001 | `eq`, `count` | `CORPUS-COUNT-POS` |
| RULE-PKG-001 | `selected`, `not`, `provides` | `CORPUS-SELECTED-POS`, `CORPUS-PROVIDES-POS` |
| RULE-KERNEL-002 | `selected`, `not`, `provides` | `CORPUS-PROVIDES-POS` |
| RULE-DUP-001 | `count_gt` (legacy) | `CORPUS-COUNTGT-POS` |
