---
id: DOC-CON-JSONSCHEMA-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - data
reviewers:
  - independent-reviewer
---

# Contratos JSON Schema

Contratos del modelo canónico en **JSON Schema Draft 2020-12**. Los documentos están en la versión
de contrato **v0** (fase R6); cada schema conserva su `$comment` y el estado documental sigue en
`draft`. No contienen campos ejecutables (`cmd`, hooks, shell, `pacstrap`): la ejecución es un plan
tipado v1 y nunca un script.

## Schemas

`$id` canónico y estable: `https://archmaker.dev/schemas/<nombre>.schema.json`. La resolución de
`$ref` es **por `$id`**, no por ruta de fichero (registro de `$id` en el gate). Los `$defs`
compartidos viven una sola vez en `common.schema.json`.

| Archivo | Agregado | Ejes de versión | Estado |
|---|---|---|---|
| `common.schema.json` | `$defs` compartidos | — | v0 |
| `draft.schema.json` | DM-DRAFT | `documentVersion`, `schemaVersion`, `revision`, `catalogRef.version`, `targetRef.version` | v0 |
| `catalog.schema.json` | DM-CATALOG | `documentVersion`, `schemaVersion`, `catalogRef.version` | v0 |
| `rule.schema.json` | RULE-* | `documentVersion`, `schemaVersion` | draft (AUD-017) |
| `diagnostic.schema.json` | Diagnostic (AM-*) | `schemaVersion` | v0 |
| `core-error.schema.json` | CoreError (AM-*) | `schemaVersion` | v0 |
| `changeset.schema.json` | ChangeSet | `schemaVersion`, `baseRevision` | v0 |
| `manifest.schema.json` | DM-MANIFEST | `documentVersion`, `schemaVersion`, `catalogRef.version`, `targetRef.version`, `producer.version` | v0 |
| `artifact.schema.json` | DM-ARTIFACT | `documentVersion`, `schemaVersion`, `targetRef.version`, `producer.version` | v0 |
| `installation-plan.schema.json` | DM-PLAN (v1) | `documentVersion`, `schemaVersion`, `protocolVersion` | v0 |
| `runner-envelope.schema.json` | DM-SESSION (protocolo v1) | `protocolVersion`, `schemaVersion` | v0 |

Los ejes son independientes (ADR-0006). El formato exacto de cada versión sigue pendiente de ADR;
`catalogRef.version`/`presetRef.version` usan semver, el resto exige cadena no vacía.

## Identidad, revisión y digests

- **Identificadores** (`common#/$defs`): `namespace` (`vendor.kind`), `entityId` (`vendor.kind.id`,
  DEC-002), `shortId`, `capabilityId`, `uuid`.
- **Revisión**: `revision` es un entero monotónico ≥ 0 y actúa como token de concurrencia optimista
  (ETag). `saveDraft` exige `expectedRevision` (`draft.schema.json#/$defs/saveDraftRequest`).
- **Digests** (ADR-0007, decisión 11): `contentDigest` es el digest **semántico** del payload
  canónico; `artifactDigest`/`binaryDigest` es el digest de los **bytes** exactos. Nunca se comparan.
  El nombre del campo porta el rol; `kind` (`content`/`artifact`/`binary`), cuando aparece, debe
  coincidir.
- **Unidades**: vocabulario cerrado `unitCode` (`common#/$defs/unitCode`); cantidad canónica
  `{value, unit}` (`common#/$defs/quantity`). La canonicalización no convierte unidades.
- **Secretos**: `secretRef` es una URI opaca (`secret://…`); el valor del secreto no es
  representable (`THR-SEC-001`).
- **Números**: binary64 finito acotado (`common#/$defs/binary64`); se rechazan valores fuera de rango
  (p. ej. `1e400`).

## Cierre de esquema

Todos los schemas nacen con **`additionalProperties: false`** en la raíz y en cada objeto definido
(ADR-0005): lo no declarado se rechaza en el borde. Las únicas excepciones son mapas abiertos
**semánticamente necesarios**, documentados en su `description` y acotados en lo posible:

| Mapa abierto | Schema | Motivo |
|---|---|---|
| `metadata` | `catalog.schema.json` | Metadatos libres del catálogo; valores `string`. |
| `params` | `diagnostic.schema.json` | Interpolación de mensajes; solo escalares. |
| `params` | `core-error.schema.json` | Interpolación de mensajes; solo escalares. |
| `content` | `artifact.schema.json` | Payload definido por el adapter del target (ADR-0003). |
| `payload` | `runner-envelope.schema.json` | Payload por mensaje del protocolo runner v1. |

Estos mapas se mantienen **abiertos y documentados** de forma deliberada y su cierre se **difiere a
v1**: cerrar `artifact.content` por `oneOf` exige fijar el profile `archinstall`, hoy no verificado
(SRC-003). No aceptan comandos, scripts ni shell.

## Política de campos desconocidos y compatibilidad

- Política declarada: **reject** (`common#/$defs/unknownFieldPolicy`).
- **Backward**: los documentos v0 previos siguen validando salvo los renombrados explícitos
  (`digest`→`contentDigest`, `manifestDigest`→`contentDigest`, `digest`→`binaryDigest` y la
  eliminación de `selection.optionId`). Ver `CHANGELOG.md`.
- **Forward**: la compatibilidad es solo aditiva; cualquier campo nuevo exige subir `schemaVersion`.

## Corpus

`examples/` contiene, por schema, pares `NN-<schema>[-descriptor].valid.json` /
`.invalid.json`. El inválido viola una restricción concreta; los casos adversariales cubren bordes
(precisión numérica, severidad/blocking, secretos, límites, operaciones de changeset).

| Ejemplo | Restricción ejercitada |
|---|---|
| `01-draft.invalid.json` | `additionalProperties: false` en la raíz (`derivedSelections`). |
| `02-catalog.invalid.json` | `enum` en `steps[].mode` (`grid`). |
| `03-rule.invalid.json` | `oneOf` de condición: operador desconocido `count_gte`. |
| `04-diagnostic.invalid.json` | `pattern` en `code` (`AM-BROKEN`). |
| `05-manifest.invalid.json` | `const` de `domainLabel` y `enum` en `origin`. |
| `06-artifact.invalid.json` | `enum` en `targetRef.id` (`iso`). |
| `07-installation-plan.invalid.json` | `enum` en `operations[].kind` (`shell.run`). |
| `08-runner-envelope.invalid.json` | `pattern` en `protocolVersion` (`1.0`). |
| `09-common-quantity.*` | `enum` cerrado de `unitCode` (`gib` no permitido). |
| `10-common-contentdigest.*` | `pattern` de digest en minúsculas. |
| `11-core-error.*` | R7: campo libre `message` rechazado. |
| `12-changeset.*` | `if/then`: `replace` exige `before` y `after`. |
| `13-draft-savedraft.*` | `saveDraftRequest` exige `expectedRevision`. |
| `14-draft-adversarial.*` | binary64: `1e400` fuera de rango. |
| `15-catalog-adversarial.*` | `revision` negativo. |
| `16-diagnostic-adversarial.*` | consistencia `error` ⇒ `blocking:true`. |
| `17-manifest-adversarial.*` | `contentDigest` en minúsculas. |
| `18-common-secretref.*` | `secretRef` sin valor de secreto. |
| `19-common-limits.*` | límites no negativos. |
| `20-core-error-adversarial.*` | severidad canónica (`fatal` reservado). |
| `21-changeset-adversarial.*` | `if/then`: `move` exige `from`. |

### Migración v5.1 → vNext

`examples/migration/` aporta `v5.1-instance.sample.json` (evidencia saneada, sin campos ejecutables)
y `vnext-draft.expected.json` (destino esperado). El gate valida que el destino **sí** cumple
`draft.schema.json` y que el sample heredado **no** valida como draft vNext. Cubre `mode: single`,
`count_gt`, arrays en presets y `base-devel` duplicado; ver `examples/migration/README.md`.

### Ejemplos no normativos

`examples/non-normative/` agrupa ejemplos ilustrativos **excluidos** del corpus y del chequeo de
límites (p. ej. el anti-patrón de almacenar `derivedSelections`).

## Validación en CI

El gate `json-schema` (`.github/workflows/json-schema.yml`, G4/G5) tiene tres jobs:

1. **meta-validate**: `Draft202012Validator.check_schema` sobre cada `contracts/json-schema/**/*.schema.json`.
2. **corpus-validate**: construye un registro `$id → schema` (resolución de `$ref` externos),
   valida cada `*.valid.json`/`*.invalid.json`, resuelve tokens con prefijo (`common-*`, `draft-savedraft`)
   y comprueba las fixtures de migración.
3. **limits-validate**: comprueba bytes, profundidad, tamaño de array y longitud de string de cada
   fixture normativo contra los límites declarados.

El gate `docs-validate` (G5) escanea los contratos no-Markdown y rechaza cualquier construcción de
shell, `pacstrap` o campos `cmd`/`hooks`.

## Reglas de mantenimiento

- Estado inicial de todo schema nuevo: `draft`; la versión de contrato se registra en `CHANGELOG.md`.
- Cada cambio de forma incrementa `schemaVersion` y exige *migration note* (`document-control.md`).
- IDs de entidad con patrón global `vendor.kind.id` (DEC-002); las referencias a `catalogRef`/`presetRef`
  usan `namespace` + `id` + `version`.
- Sin secretos: solo `secretRef` (`THR-SEC-001`).
- No duplicar reglas semánticas: los `$defs` compartidos viven en `common.schema.json` y se
  referencian por `$id`.
