# Contratos JSON Schema

Contratos del modelo canónico en **JSON Schema Draft 2020-12**. Todos los documentos están en estado **`draft`** (no `accepted`). No contienen campos ejecutables (`cmd`, hooks, shell, `pacstrap`): la ejecución es un plan tipado v1 y nunca un script.

## Schemas

`$id` canónico: `https://archmaker.dev/schemas/<nombre>.schema.json`.

| Archivo | Agregado | Ejes de versión | Estado |
|---|---|---|---|
| `draft.schema.json` | DM-DRAFT | `documentVersion`, `schemaVersion`, `catalogRef.version`, `targetRef.version` | draft |
| `catalog.schema.json` | DM-CATALOG | `documentVersion`, `schemaVersion`, `catalogRef.version` | draft |
| `rule.schema.json` | RULE-* | `documentVersion`, `schemaVersion` | draft |
| `diagnostic.schema.json` | Diagnostic (AM-*) | `schemaVersion` | draft |
| `manifest.schema.json` | DM-MANIFEST | `documentVersion`, `schemaVersion`, `catalogRef.version`, `targetRef.version`, `producer.version` | draft |
| `artifact.schema.json` | DM-ARTIFACT | `documentVersion`, `schemaVersion`, `targetRef.version`, `producer.version` | draft |
| `installation-plan.schema.json` | DM-PLAN (v1) | `documentVersion`, `schemaVersion`, `protocolVersion` | draft |
| `runner-envelope.schema.json` | DM-SESSION (protocolo v1) | `protocolVersion`, `schemaVersion` | draft |

Los ejes son independientes (ADR-0006). El formato exacto de cada versión sigue pendiente de ADR; aquí solo se exige que sea una cadena no vacía.

## Cierre de esquema

Todos los schemas nacen con **`additionalProperties: false`** en la raíz y en cada objeto definido (ADR-0005): lo no declarado se rechaza en el borde. Las únicas excepciones son mapas abiertos **semánticamente necesarios**, documentados en su `description` y acotados en lo posible:

| Mapa abierto | Schema | Motivo |
|---|---|---|
| `metadata` | `catalog.schema.json` | Metadatos libres del catálogo; valores `string`. |
| `params` | `diagnostic.schema.json` | Interpolación de mensajes; solo escalares (`string`/`number`/`boolean`). |
| `content` | `artifact.schema.json` | Payload definido por el adapter del target (ADR-0003), versionado por `targetRef.version`. |
| `payload` | `runner-envelope.schema.json` | Payload por mensaje del protocolo runner v1; solo operaciones tipadas. |

## Corpus

`examples/` contiene, por schema, un ejemplo `NN-nombre.valid.json` y uno `NN-nombre.invalid.json`. El inválido viola una restricción concreta:

| Ejemplo inválido | Restricción violada |
|---|---|
| `01-draft.invalid.json` | `additionalProperties: false` en la raíz (`derivedSelections`). |
| `02-catalog.invalid.json` | `enum` en `steps[].mode` (`grid` no permitido). |
| `03-rule.invalid.json` | `oneOf` de condición: operador desconocido `count_gte`. |
| `04-diagnostic.invalid.json` | `pattern` en `code` (`AM-BROKEN` no es `AM-<FAMILIA>-<NNN>`). |
| `05-manifest.invalid.json` | `enum` en `effectiveSelections[].origin` (`auto`) y `const` de `domainLabel`. |
| `06-artifact.invalid.json` | `enum` en `targetRef.id` (`iso`). |
| `07-installation-plan.invalid.json` | `enum` en `operations[].kind` (`shell.run` no allowlisted). |
| `08-runner-envelope.invalid.json` | `pattern` en `protocolVersion` (`1.0` no es `v1`). |

### Migración v5.1 → vNext

`examples/migration/` aporta `v5.1-instance.sample.json` (evidencia saneada, sin campos ejecutables) y `vnext-draft.expected.json` (destino esperado). Cubre los casos obligatorios `mode: single`, `count_gt`, arrays en presets y `base-devel` duplicado; ver `examples/migration/README.md`.

## Validación en CI

El gate `json-schema` (`.github/workflows/json-schema.yml`, gate G4/G5) instala `jsonschema>=4` y ejecuta `Draft202012Validator.check_schema` sobre cada `*.json` de `contracts/json-schema/` excluyendo `fixtures/`. Cuando se añada la validación de corpus, cada `*.valid.json` debe validar y cada `*.invalid.json` debe fallar contra su schema. El gate `docs-validate` (G5) escanea los contratos no-Markdown y rechaza cualquier construcción de shell, `pacstrap` o campos `cmd`/`hooks`.

## Reglas de mantenimiento

- Estado inicial de todo schema nuevo: `draft`.
- Cada cambio de forma incrementa `schemaVersion` y exige *migration note* (`document-control.md`).
- IDs de entidad con patrón global `vendor.kind.id` (DEC-002); las referencias a `catalogRef`/`presetRef` usan `namespace` + `id` + `version`.
- Sin secretos: solo `secretRef` (`THR-SEC-001`).
