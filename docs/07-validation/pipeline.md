---
id: DOC-VAL-PIPE-001
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

# Pipeline de validación

ID `DOC-VAL-PIPE-001` · Estado `in-review` · Propietario Calidad. Define las etapas, la severidad
canónica, el orden estable de diagnósticos, la localización de mensajes y la reproducibilidad de la
validación. Autoridad de códigos: `docs/04-interfaces/errors-events.md` (catálogo canónico, fuente
única). Este documento no redefine códigos.

## Etapas canónicas

1. Límite de bytes, nesting, collections y strings.
2. Encoding/parse.
3. Detección de versión.
4. Meta-schema.
5. Schema estructural.
6. IDs únicos e integridad referencial.
7. Invariantes de dominio.
8. Tipado/evaluación de reglas.
9. Resolución de capabilities y conflictos.
10. Compatibilidad de arquitectura/hardware.
11. Procedencia, digest, firma y freshness.
12. Target/exporter compatibility.
13. Canonicalización y digest.
14. Revalidación del resultado.
15. v1: environment/preflight/plan.

La evaluación de reglas (etapa 8) consume el AST declarativo de `contracts/json-schema/rule.schema.json`
y su vocabulario cerrado de operadores (`$defs.operator`), especificado en
`docs/07-validation/operator-spec.md`. La resolución de `provides` (etapa 9) es la que decide el
conflicto de capabilities.

## Severidad y bloqueo canónicos

- Vocabulario cerrado de severidad: `error`, `warning`, `info` (`common#/$defs/severity`); `fatal` y
  `debug` están **reservados** y no los usa el MVP.
- Invariante de forma: `severity: error` ⇒ `blocking: true`; `warning`/`info` ⇒ `blocking: false`.
  La severidad tipada de `contracts/json-schema/diagnostic.schema.json` lo impone.
- Una regla con `action: block` emite severidad `error` bloqueante. `suggest`, `derive`, `change` y
  `lock` **no** bloquean por sí mismos: bloquean solo si la regla lo declara explícitamente.
- Un operador fuera del vocabulario (`$defs.operator`) no se ignora ni se evalúa como `false`:
  produce `AM-RULE-009` (`error`, bloqueante), determinista e independiente del valor de la
  referencia.

## Orden estable de diagnósticos

- La lista de diagnósticos se ordena por la clave canónica `(path, code, source)` antes de emitirse
  (`diagnostic.schema.json#/$defs/diagnosticList`). El orden es total y estable: no depende del orden
  de recorrido del motor.
- `path` es un JSON Pointer RFC 6901 (`common#/$defs/jsonPointer`); `code` es un `AM-*`;
  `source` identifica la regla o el componente emisor.
- No se ordena por marca temporal ni por campos efímeros (`createdAt`, `diagnostics`, …): esos campos
  se eliminan del payload canónico (perfil de canonicalización, decisión 8).

## Localización estable de mensajes

- El mensaje de un diagnóstico se identifica por `messageKey` (clave estable, localizable;
  `common#/$defs/messageKey`) más `params` escalares para interpolación. No hay strings libres como
  contrato público.
- La localización no incluye rutas absolutas ni rutas de fichero del entorno; usa el `path` (JSON
  Pointer) referido al documento canónico.

## Validación reproducible

- La misma entrada canónica produce siempre la misma salida y el mismo `contentDigest`; dos
  ejecuciones en entornos distintos coinciden (NFR-DET-001).
- El motor evalúa sobre el dominio resuelto y nunca sobre estado de UI ni campos efímeros.
- El vocabulario de operadores es cerrado; un cambio de vocabulario exige subir `schemaVersion` del
  contrato y registrar *migration note*.

## Corpus positivo y negativo en CI

El pipeline de CI valida el corpus **positivo y negativo** de los contratos:

- `contracts/json-schema/examples/` aporta los pares `*.valid.json` / `*.invalid.json` por schema
  (y las fixtures de migración v5.1), validados por el job `corpus-validate`.
- `contracts/json-schema/rule-corpus/operators.corpus.json` aporta el corpus por operador con casos
  `positive`, `negative`, `edge` y `adversarial` (campo `expect`: `valid`/`invalid`). El job
  `rule-corpus-validate` de `.github/workflows/json-schema.yml` valida cada `instance` contra
  `rule.schema.json` y exige el resultado declarado; además comprueba que **todo** operador del
  vocabulario (`$defs.operator`) tiene al menos un caso.
- El gate falla si un caso positivo es rechazado o si un caso negativo es aceptado.

## Referencias

- `docs/07-validation/operator-spec.md` — especificación por operador.
- `docs/07-validation/operator-corpus.md` — corpus por operador.
- `docs/07-validation/rule-inventory.md` — inventario de reglas.
- `docs/04-interfaces/errors-events.md` — catálogo canónico de códigos.
- `contracts/json-schema/rule.schema.json`, `contracts/json-schema/diagnostic.schema.json`.
