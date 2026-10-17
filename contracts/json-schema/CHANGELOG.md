---
id: DOC-CON-CHANGELOG-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - data
reviewers:
  - independent-reviewer
---

# Changelog de contratos JSON Schema

Registro por versión de contrato (`schemaVersion` del documento) de los cambios de forma. Estado
inicial de todo schema: `draft`. La fase R6 publica la versión de contrato **v0** (los documentos
declaran `schemaVersion: "1"`; la etiqueta `v0` es la revisión del contrato, no el eje
`schemaVersion`). Cambiar la forma de un contrato exige incrementar `schemaVersion` y una
_migration note_ (`docs/03-data/versioning-migrations.md`).

## v0.3 — Draft/manifiesto schemaVersion 2 (ADR-0016)

### Añadidos

- `draft-v2-defs.schema.json`: `storagePlan`, capas ESP/sistema, `encryptionPolicy`,
  `browserPolicy`, `verificationStatuses`, `safeByteCount`, `uki-on-esp`.
- Campos opcionales v2 en `draft.schema.json` / `manifest.schema.json`.
- `domainLabel` admite `archmaker:manifest:v2`.
- Corpus `32-draft-v2.*`, `33-manifest-v2.*`, `34-draft-future.invalid.json`.

### Notas

- `schemaVersion` de draft/manifiesto restringido a `"1"` | `"2"` (rechazo de futuras).
- Migración v1→v2 en Rust: idempotente, sin inventar hardware ni sustituir apps (ADR-0016).
- A6 / escritura en discos fuera de alcance (ADR-0018).

## v0.2 — Contrato de aplicación (ADR-0011)

### Añadidos

- `application-contract.schema.json`: contrato generado desde la selección resuelta, base
  Arch Linux x86_64 y formato DEC-001.
- `clause-base.schema.json` … `clause-boot.schema.json`: siete cláusulas con `$id` propio.
- Corpus `24-application-contract.*` y `25-clause-base.*` … `31-clause-boot.*`.

### Notas

- No se amplía el enum de targets de `artifact.schema.json` (DEC-001 intacto).
- `CLAUSE-BOOT-001` no admite `status: satisfied` mientras el formato de imagen esté cerrado.

## v0.1 — MVP-1 (domain / catalog / migration)

### Modificados

- `common.schema.json`: formato congelado ADR-0006 — `documentVersion` (`vNext`/`vNext.N`),
  `schemaVersion`/`protocolVersion` (entero positivo), `version` alias de SemVer para
  `targetRef`/`producer`.
- `catalog.schema.json`: `$defs.provenance` (sourceId/path/capturedAt) para catálogos locales.
- CorePort: operación aditiva `importDraft` (fuera del congelado estricto de 7 ops; semántica v0
  de las siete intacta).

### Corpus

- `22-common-documentversion.*`, `23-common-schemaversion.*`.
- Fixture `crates/archmaker-core/fixtures/local-catalog-mvp1.json` (multi-entity + procedencia).
- VAL-MIG (`tools/validate_mig.py`) + tests `import_draft_migrates_v51_sample`.

## v0 — fase R6 (AUD-011…AUD-015)

### Añadidos

- `common.schema.json`: `$defs` compartidos (identificadores DEC-002, ejes de versión, `unitCode` y
  `quantity`, `digest`/`contentDigest`/`artifactDigest`/`binaryDigest`, `utcTimestamp`, denylist de
  campos efímeros, `secretRef` tipado, `limits`, política de campos desconocidos, `catalogRef`,
  `targetRef`, `presetRef`, `producer`, `selectionValue`, `selection`, `resolvedSelection`,
  `capabilityRef`, `conflict`).
- `core-error.schema.json`: catálogo de errores tipados de CorePort (código estable, familia,
  categoría, severidad, `messageKey`, detalle estructurado, causa/cadena recursiva, remedio). Sin
  strings libres como contrato público (R7).
- `changeset.schema.json`: cambios discretos aplicables a un Draft (operación tipada, ruta/selector,
  valor anterior/nuevo, id de operación), compatible con migración y con la semántica de digest.

### Modificados

- `draft.schema.json`
  - Añadido `revision` (entero monotónico, obligatorio) como token de concurrencia optimista (ETag).
  - Añadido `contentDigest` (opcional; digest semántico, ADR-0007).
  - Eliminada la duplicación `selection.optionId` / `single.optionId`: el valor vive solo en
    `selection.value`.
  - Exclusividad de la unión `SelectionValue` por `oneOf` sobre `kind`.
  - Números finitos acotados a binary64 (`common#/$defs/binary64`); unidades por `unitCode` cerrado.
  - `secretRef` tipado (`secretReference` URI opaca; `additionalProperties:false` impide el valor).
  - `$defs` extraídos a `common`; `$id` estable por `$id` (resolución por identificador, no por ruta).
  - Añadido `$defs.saveDraftRequest` con `expectedRevision` (precondición de guardado).
- `catalog.schema.json`: `$defs` compartidos a `common`; `digest` renombrado a `contentDigest`;
  añadido `revision` opcional; cotas de tamaño.
- `diagnostic.schema.json`: `$defs` a `common`; severidad/blocking canónicos y consistentes
  (`error` ⇒ `blocking:true`; `warning`/`info` ⇒ `blocking:false`); `$defs.diagnosticList` con orden
  estable por `(path, code, source)`.
- `manifest.schema.json`: `$defs` a `common`; `manifestDigest` renombrado a `contentDigest`
  (conforme al perfil canónico v1, dominio `archmaker:manifest:v1`).
- `artifact.schema.json`: `$defs` a `common`; `digest` → `binaryDigest` (bytes) y
  `manifestRef.digest` → `manifestRef.contentDigest` (semántico).
- `installation-plan.schema.json`: `$defs` a `common`; `planHash`/`manifestHash` tipados como
  `contentDigest`; `parameters.checksum` como `binaryDigest`.
- `rule.schema.json` (fase R8, AUD-017): vocabulario **cerrado** de operadores (`$defs.operator`,
  enum normativo) y `propertyNames` sobre cada expresión; `$defs.entityId` y `$defs.capabilityId`
  **extraídos** a `common` y referenciados por `$id`; `severity` referenciado a `common`; `$defs`
  locales de identificadores eliminados. `count_gt` queda marcado **legacy** (solo migración) y
  `target_is` **diferido a v1**; el resto son canónicos.
- `README.md`: tabla de schemas (11), política de cierre, corpus y CI.
- `docs/04-interfaces/dto.md`: DTO verificados contra los contratos v0.

### Corpus

- Pares válidos/inválidos/adversariales para `common` (`quantity`, `contentDigest`, `secretRef`,
  `limits`), `core-error`, `changeset` y casos nuevos de `draft` (`saveDraft`/precisión),
  `catalog` (`revision`), `diagnostic` (severidad/blocking) y `manifest` (`contentDigest`).
- Fixtures de migración v5.1 en `examples/migration/` (el destino esperado valida; el sample
  heredado no).
- Ejemplos no normativos separados en `examples/non-normative/`.
- `rule-corpus/operators.corpus.json` (fase R8, AUD-017): corpus **ejecutable** por operador
  (positivo/negativo/borde/adversarial) que cubre los 12 tokens del vocabulario; validado por el job
  `rule-corpus-validate` de `.github/workflows/json-schema.yml` (positivo y negativo + cobertura del
  enum). Ver `docs/07-validation/operator-corpus.md`.

### Compatibilidad y política de campos

- **Backward**: los documentos v0 previos siguen validando salvo los renombrados explícitos
  (`digest`→`contentDigest`, `manifestDigest`→`contentDigest`, `digest`→`binaryDigest` y la
  eliminación de `selection.optionId`), documentados aquí.
- **Forward**: todo objeto es `additionalProperties:false`; un campo desconocido se **rechaza**
  (`common#/$defs/unknownFieldPolicy`). La compatibilidad es solo aditiva y exige subir
  `schemaVersion`.

### Pendiente (no cerrado en v0)

- Formato exacto de los ejes de versión (ADR-0006).
- Cierre de los mapas abiertos `artifact.content` y `runner-envelope.payload` (SRC-003).
