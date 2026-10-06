---
id: DOC-IF-DTO-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-IF-CORE-001
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
  - id: ADR-0007
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
---

# DTO de CorePort v0

Catálogo de los DTO que cruzan `CorePort v0` (`core-port.md`, DOC-IF-CORE-001) y de los tipos de
valor compartidos. La **forma canónica** de los contratos persistidos está fijada en los JSON Schema
2020-12 **v0** de `contracts/json-schema/` (ADR-0005); este documento describe su semántica y los
DTO de operación sin schema propio, y **no duplica reglas**: las referencia por `$id` y `$defs`. La
autoridad de operaciones, pureza, idempotencia, límites y errores es `core-port.md` y
`errors-events.md`; aquí solo se fijan formas.

Cada contrato se versiona con `schemaVersion`, independiente de `documentVersion`,
`catalogRef.version`, `targetRef.version`, `producer.version` y `protocolVersion` (ADR-0006). Los
contratos persistidos requieren versión y *migration note* (`docs/03-data/versioning-migrations.md`).

## Convenciones

- **Dirección**: `→` (entrada a `CorePort`) o `←` (salida de `CorePort`).
- **Obligatoriedad**: `Sí`, `No` o `Cond.` (condicional, descrito en notas).
- **Persistible**: si el DTO puede almacenarse como verdad. `Resolution` y los resultados derivados
  **nunca** se persisten (glosario, `DM-RESOLUTION`).
- **Tipos**: `uuid`, `string`, `boolean`, `number`, `int`, `Digest`, `Ref`, `array<T>`,
  `map<string,T>`, `enum`.
- **Resolución de referencias**: por `$id`, nunca por ruta de fichero.

## Correspondencia con JSON Schema

Los DTO con schema canónico se validan contra él; los DTO de operación se derivan de las firmas de
`core-port.md` y no tienen schema propio.

| DTO | Schema (`$id`) | `$defs` | Estado |
|---|---|---|---|
| `Draft` | `https://archmaker.dev/schemas/draft.schema.json` | — | canónico v0 |
| `SaveDraftInput` | `https://archmaker.dev/schemas/draft.schema.json` | `#/$defs/saveDraftRequest` | canónico v0 |
| `Catalog` | `https://archmaker.dev/schemas/catalog.schema.json` | — | canónico v0 |
| `Manifest` | `https://archmaker.dev/schemas/manifest.schema.json` | — | canónico v0 |
| `Artifact` | `https://archmaker.dev/schemas/artifact.schema.json` | `#/$defs/artifactRef`, `#/$defs/manifestRef` | canónico v0 |
| `CoreError` | `https://archmaker.dev/schemas/core-error.schema.json` | `#/$defs/code`, `#/$defs/family`, `#/$defs/category`, `#/$defs/remedy` | canónico v0 |
| `Diagnostic` | `https://archmaker.dev/schemas/diagnostic.schema.json` | `#/$defs/diagnosticList` | canónico v0 |
| Tipos compartidos | `https://archmaker.dev/schemas/common.schema.json` | ver tabla siguiente | canónico v0 |

> Los DTO de operación (`*Input` y los resultados efímeros) no tienen schema persistido; su forma
> es la de esta tabla y su comportamiento el de `core-port.md`. No se persisten como verdad.

## Tipos de valor compartidos

Referenciados desde `common.schema.json` (`https://archmaker.dev/schemas/common.schema.json`):

| Tipo | `$defs` | Notas |
|---|---|---|
| `namespace` | `#/$defs/namespace` | `vendor.kind` (DEC-002). |
| `shortId` | `#/$defs/shortId` | Identificador de un segmento. |
| `entityId` | `#/$defs/entityId` | `vendor.kind.id` (DEC-002). |
| `capabilityId` | `#/$defs/capabilityId` | `fs.btrfs`, `boot.uefi`, … |
| `uuid` | `#/$defs/uuid` | Identidad y correlación. |
| `semver` | `#/$defs/semver` | `catalogRef.version`, `presetRef.version`. |
| `version` | `#/$defs/version` | Cadena no vacía (ejes restantes, ADR-0006). |
| `schemaVersion` | `#/$defs/schemaVersion` | Eje de versión de contrato. |
| `documentVersion` | `#/$defs/documentVersion` | Eje de versión de documento. |
| `revision` | `#/$defs/revision` | Entero monotónico ≥ 0 (ETag). |
| `binary64` | `#/$defs/binary64` | Número finito acotado; se rechaza `1e400`. |
| `unitCode` | `#/$defs/unitCode` | Vocabulario cerrado, sensible a mayúsculas. |
| `quantity` | `#/$defs/quantity` | `{ value, unit }`; no se convierte al canonicalizar. |
| `digest` | `#/$defs/digest` | `{ algorithm: "sha256", value, kind? }`. |
| `contentDigest` | `#/$defs/contentDigest` | Semántico; `kind` (si aparece) = `content`. |
| `binaryDigest` | `#/$defs/binaryDigest` | Bytes exactos; `kind` = `binary`. |
| `utcTimestamp` | `#/$defs/utcTimestamp` | ISO-8601 UTC con `Z`; efímero. |
| `secretRef` | `#/$defs/secretRef` | URI opaca; el valor del secreto no es representable. |
| `jsonPointer` | `#/$defs/jsonPointer` | RFC 6901. |
| `messageKey` | `#/$defs/messageKey` | Clave estable, no texto libre. |
| `severity` | `#/$defs/severity` | `error \| warning \| info`. |
| `limits` | `#/$defs/limits` | `maxBytes`, `maxDepth`, `maxStringLength`, `maxArrayItems`. |
| `catalogRef` | `#/$defs/catalogRef` | `namespace`, `id`, `version`, `digest`. |
| `targetRef` | `#/$defs/targetRef` | `id`, `version`. |
| `presetRef` | `#/$defs/presetRef` | `namespace`, `id`, `version`. |
| `producer` | `#/$defs/producer` | `id`, `version`. |
| `selectionValue` | `#/$defs/selectionValue` | Unión `oneOf`: `single`, `multiple`, `boolean`, `number`, `text`, `secretRef`. |
| `selection` | `#/$defs/selection` | `{ stepId, value }`; el valor vive solo en `value`. |
| `resolvedSelection` | `#/$defs/resolvedSelection` | `{ stepId, optionId, value, origin, locked }`. |
| `capabilityRef` | `#/$defs/capabilityRef` | `{ id, title? }`. |
| `conflict` | `#/$defs/conflict` | `{ severity, capabilityId?, path, messageKey, suggestions }`. |

### `Result<T, CoreError>`

Resultado de toda operación. Unión discriminada por `ok` (DTO de operación, no persistible):

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `ok` | boolean | Sí | `true` → `value`; `false` → `error`. |
| `value` | T | Cond. | Presente si `ok = true`. |
| `error` | `CoreError` | Cond. | Presente si `ok = false`. |

### `SelectionValue`

Unión discriminada fijada en `common.schema.json#/$defs/selectionValue` (variante exclusiva por
`oneOf` sobre `kind`). Persistible salvo que el contenido sea un secreto: `secretRef` **nunca**
serializa el secreto (`THR-SEC-001`).

| Variante | Campo(s) | `$defs` | Notas |
|---|---|---|---|
| `single` | `optionId` | `#/$defs/entityId` | Cardinalidad uno. |
| `multiple` | `optionIds` | array de `#/$defs/entityId` | Ordenado por ID al canonicalizar (perfil v1, decisión 5); `uniqueItems`. |
| `boolean` | `value` | boolean | — |
| `number` | `value`, `unit?` | `#/$defs/binary64`, `#/$defs/unitCode` | Finito; se rechaza el desbordamiento. |
| `text` | `value` | string | Longitud máxima 4096. |
| `secretRef` | `ref` | `#/$defs/secretReference` | Referencia opaca; el valor no es representable. |

### `CoreError` y `Diagnostic`

Se definen en `errors-events.md` y tienen schema canónico en
`https://archmaker.dev/schemas/core-error.schema.json` y
`https://archmaker.dev/schemas/diagnostic.schema.json`; se referencian, no se redefinen. `CoreError`
no admite strings libres como contrato público: `code`, `family`, `category`, `severity`,
`messageKey`, `source` son estables. `severity` usa `error | warning | info`; `blocking` es
canónico (`error` ⇒ `true`). Las colecciones de diagnósticos se emiten ordenadas por
`(path, code, source)`.

## DTO de entrada

Todos no persistibles (son argumentos). El detalle de precondiciones, pureza, límites y errores por
operación está en `core-port.md`.

| DTO | Propósito | Operación | Schema |
|---|---|---|---|
| `CreateDraftInput` | Crear un Draft vacío ligado a catálogo y target. | `createDraft` | operación |
| `LoadCatalogInput` | Cargar un catálogo versionado y acotar su lectura. | `loadCatalog` | operación |
| `SaveDraftInput` | Persistir un Draft con precondición de revisión. | `saveDraft` | `draft.schema.json#/$defs/saveDraftRequest` |
| `ResolveDraftInput` | Resolver un Draft contra un catálogo. | `resolveDraft` | operación |
| `ValidateDraftInput` | Ejecutar el pipeline de validación. | `validateDraft` | operación |
| `BuildManifestInput` | Construir el manifest canónico. | `buildManifest` | operación |
| `ExportArtifactInput` | Exportar un artifact a un destino elegido. | `exportArtifact` | operación |

### `CreateDraftInput`

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `draftId` | uuid | No | `common#/$defs/uuid` | Si se aporta, la operación es idempotente; si se omite, el core acuña identidad. |
| `catalogRef` | CatalogRef | Sí | `common#/$defs/catalogRef` | Catálogo base. |
| `targetRef` | TargetRef | Sí | `common#/$defs/targetRef` | Objetivo (DEC-008). |
| `presetRef` | PresetRef | No | `common#/$defs/presetRef` | Preset inicial opcional. |
| `requestId` | uuid | No | `common#/$defs/uuid` | Correlación. |

### `LoadCatalogInput`

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `source` | enum(`embedded`, `file`) | Sí | — | `registry` fuera de MVP. |
| `catalogRef` | CatalogRef | Cond. | `common#/$defs/catalogRef` | Requerido si `source = file`. |
| `expectedDigest` | ContentDigest | No | `common#/$defs/contentDigest` | Integridad/anti-manipulación (`THR-CAT-001`). |
| `limits` | limits | No | `common#/$defs/limits` | Acota bytes/profundidad/string/array antes y durante el parseo. |
| `requestId` | uuid | No | `common#/$defs/uuid` | Correlación. |

### `SaveDraftInput`

Schema `draft.schema.json#/$defs/saveDraftRequest`. Persistencia con concurrencia optimista: si la
revisión almacenada difiere de `expectedRevision`, falla con conflicto tipado (`AM-DOC-002`), nunca
sobrescribe en silencio.

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `draft` | Draft | Sí | `draft.schema.json` | Su `revision` es la nueva revisión. |
| `expectedRevision` | revision | Sí | `common#/$defs/revision` | Revisión observada por el llamante. |
| `requestId` | uuid | No | `common#/$defs/uuid` | Correlación. |

### `ResolveDraftInput`

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `draft` | Draft | Sí | `draft.schema.json` | Intención manual y refs; sin estado derivado. |
| `catalogRef` | CatalogRef | Sí | `common#/$defs/catalogRef` | Catálogo de definiciones. |
| `requestId` | uuid | No | `common#/$defs/uuid` | Correlación. |

### `ValidateDraftInput`

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `draft` | Draft | Sí | `draft.schema.json` | — |
| `catalogRef` | CatalogRef | Sí | `common#/$defs/catalogRef` | — |
| `targetRef` | TargetRef | No | `common#/$defs/targetRef` | Habilita etapas de target. |
| `validatorIds` | array<string> | No | — | Subconjunto del pipeline; por defecto, todas. |
| `requestId` | uuid | No | `common#/$defs/uuid` | Correlación. |

### `BuildManifestInput`

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `draft` | Draft | Sí | `draft.schema.json` | — |
| `catalogRef` | CatalogRef | Sí | `common#/$defs/catalogRef` | — |
| `targetRef` | TargetRef | Sí | `common#/$defs/targetRef` | — |
| `expectedResolutionDigest` | ContentDigest | No | `common#/$defs/contentDigest` | Evita construir desde otra Resolution. |
| `requestId` | uuid | No | `common#/$defs/uuid` | Correlación. |

### `ExportArtifactInput`

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `manifestRef` | ManifestRef | Sí | `artifact.schema.json#/$defs/manifestRef` | Entrada canónica del exporter. |
| `targetRef` | TargetRef | Sí | `common#/$defs/targetRef` | Debe estar en el conjunto soportado (DEC-001/DEC-008). |
| `destinationHandle` | string | Sí | — | Mango opaco del picker; nunca una ruta arbitraria (`THR-FS-001`). |
| `requestId` | uuid | No | `common#/$defs/uuid` | Correlación. |

## DTO de salida

| DTO | Propósito | Operación | Persistible | Schema |
|---|---|---|---|---|
| `Draft` | Agregado de intención editable (`DM-DRAFT`). | `createDraft`, `saveDraft` | **Sí** | `draft.schema.json` |
| `Catalog` | Catálogo versionado e inmutable (`DM-CATALOG`). | `loadCatalog` | Sí (el catálogo) | `catalog.schema.json` |
| `ResolveResult` | Resolution determinista y efímera (`DM-RESOLUTION`). | `resolveDraft` | **No** | efímero |
| `ValidationResult` | Diagnósticos del pipeline. | `validateDraft` | No | efímero |
| `Manifest` | Manifest canónico (`DM-MANIFEST`). | `buildManifest` | Sí (el manifest) | `manifest.schema.json` |
| `Artifact` | Artifact exportado (`DM-ARTIFACT`). | `exportArtifact` | Sí (el artifact) | `artifact.schema.json` |

### `Draft`

Schema `https://archmaker.dev/schemas/draft.schema.json`. Solo intención manual y refs; las
selecciones derivadas no se almacenan.

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `id` | uuid | Sí | — | Identidad `DM-DRAFT`. |
| `documentVersion` | documentVersion | Sí | — | Eje de documento. |
| `schemaVersion` | schemaVersion | Sí | — | Eje de schema. |
| `revision` | revision | Sí | — | Token de concurrencia (ETag); 0 al crear. |
| `contentDigest` | ContentDigest | No | `common#/$defs/contentDigest` | Ausente hasta el primer guardado. |
| `catalogRef` | CatalogRef | Sí | `common#/$defs/catalogRef` | — |
| `targetRef` | TargetRef | Sí | `common#/$defs/targetRef` | — |
| `selections` | array<Selection> | Sí | `common#/$defs/selection` | ≤ 4096. |
| `presetRefs` | array<PresetRef> | No | `common#/$defs/presetRef` | ≤ 256, únicos. |
| `createdAt` | utcTimestamp | No | — | Efímero; excluido del hash. |
| `updatedAt` | utcTimestamp | No | — | Efímero; excluido del hash. |

### `Catalog`

Schema `https://archmaker.dev/schemas/catalog.schema.json`. Declara qué se puede elegir y con qué
consecuencias; nunca comandos, hooks ni shell. `metadata` es el único mapa abierto documentado.

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `namespace` | namespace | Sí | `common#/$defs/namespace` | DEC-002. |
| `id` | shortId | Sí | `common#/$defs/shortId` | — |
| `version` | semver | Sí | `common#/$defs/semver` | Eje `catalogRef.version`. |
| `revision` | revision | No | `common#/$defs/revision` | Si el store publica sucesivas. |
| `documentVersion` | documentVersion | Sí | — | — |
| `schemaVersion` | schemaVersion | Sí | — | — |
| `contentDigest` | ContentDigest | No | `common#/$defs/contentDigest` | Calculado externamente. |
| `metadata` | map<string,string> | No | — | Mapa abierto documentado. |
| `steps` | array<Step> | Sí | `catalog.schema.json#/$defs/step` | ≤ 1024. |
| `libpacks` | array<Libpack> | No | `catalog.schema.json#/$defs/libpack` | ≤ 1024. |
| `capabilities` | array<Capability> | No | `catalog.schema.json#/$defs/capability` | ≤ 4096. |
| `rules` | array<RuleRef> | No | `catalog.schema.json#/$defs/ruleRef` | ≤ 4096. |

### `ResolveResult`

DTO efímero (`DM-RESOLUTION`), no persistible. El mismo input produce el mismo `resolutionDigest`
(NFR-DET-001).

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `resolutionDigest` | ContentDigest | Sí | `common#/$defs/contentDigest` | Digest del input; recalculable. |
| `effectiveSelections` | array<ResolvedSelection> | Sí | `common#/$defs/resolvedSelection` | Manual + derivado. |
| `providedCapabilities` | array<CapabilityRef> | Sí | `common#/$defs/capabilityRef` | Cierre computado. |
| `requiredCapabilities` | array<CapabilityRef> | Sí | `common#/$defs/capabilityRef` | — |
| `conflicts` | array<Conflict> | Sí | `common#/$defs/conflict` | Vacío si no hay. |
| `diagnostics` | array<Diagnostic> | No | `diagnostic.schema.json#/$defs/diagnosticList` | Orden estable. |

### `ValidationResult`

DTO efímero, no persistible.

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `diagnostics` | array<Diagnostic> | Sí | `diagnostic.schema.json#/$defs/diagnosticList` | Tipados, localizados y estables. |
| `blocking` | boolean | Sí | — | `true` si algún diagnóstico es `error`. |
| `pipelineVersion` | string | Sí | — | Versión de `archmaker-validation`. |

### `Manifest`

Schema `https://archmaker.dev/schemas/manifest.schema.json`. Documento canónico, cerrado, ordenado
y hasheado con `domainLabel: archmaker:manifest:v1` (ADR-0007).

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `documentVersion` | documentVersion | Sí | — | — |
| `schemaVersion` | schemaVersion | Sí | — | — |
| `domainLabel` | const | Sí | — | `archmaker:manifest:v1`. |
| `catalogRef` | CatalogRef | Sí | `common#/$defs/catalogRef` | Versión exacta consumida. |
| `targetRef` | TargetRef | Sí | `common#/$defs/targetRef` | — |
| `producer` | Producer | Sí | `common#/$defs/producer` | — |
| `resolutionDigest` | ContentDigest | Sí | `common#/$defs/contentDigest` | Trazabilidad a la Resolution. |
| `effectiveSelections` | array<ResolvedSelection> | Sí | `common#/$defs/resolvedSelection` | ≥ 1, ≤ 4096. |
| `providedCapabilities` | array<CapabilityRef> | Sí | `common#/$defs/capabilityRef` | ≤ 4096. |
| `requiredCapabilities` | array<CapabilityRef> | Sí | `common#/$defs/capabilityRef` | ≤ 4096. |
| `conflicts` | array<Conflict> | Sí | `common#/$defs/conflict` | Vacío para un manifest sellado. |
| `contentDigest` | ContentDigest | Sí | `common#/$defs/contentDigest` | Dominio `archmaker:manifest:v1`. |

### `Artifact`

Schema `https://archmaker.dev/schemas/artifact.schema.json`. Declara target, productor, versiones,
`mediaType` y digests (DEC-001, ADR-0003).

| Campo | Tipo | Oblig. | `$defs` | Notas |
|---|---|---|---|---|
| `documentVersion` | documentVersion | Sí | — | — |
| `schemaVersion` | schemaVersion | Sí | — | — |
| `artifactRef` | ArtifactRef | Sí | `artifact.schema.json#/$defs/artifactRef` | `{ binaryDigest, mediaType }`. |
| `targetRef` | TargetRef | Sí | — | `id` ∈ `archmaker-profile`, `archinstall-profile`, `report`. |
| `producer` | Producer | Sí | `common#/$defs/producer` | — |
| `mediaType` | string | Sí | — | IANA. |
| `binaryDigest` | BinaryDigest | Sí | `common#/$defs/binaryDigest` | Bytes exactos. |
| `manifestRef` | ManifestRef | Sí | `artifact.schema.json#/$defs/manifestRef` | `{ contentDigest }` semántico. |
| `experimental` | boolean | No | — | `true` para `archinstall-profile`. |
| `content` | object \| array \| string | No | — | Mapa abierto definido por el adapter (SRC-003). |

## Regla de paridad Rust/TypeScript/WASM

1. **Mismos campos y semántica** en Rust nativo, TypeScript (`CorePort`) y WASM: mismos nombres
   serializados, mismos tipos, misma obligatoriedad y mismas reglas de unión.
2. **Fuente única de schema**: un solo JSON Schema 2020-12 (`SRC-006`) genera/valida los tres
   consumidores. Rust es la autoridad semántica (ADR-0001); TypeScript no duplica reglas.
3. **Paridad verificable**: un corpus compartido produce los mismos diagnósticos, bytes de manifest
   canónicos y digests en Rust nativo y WASM (`tauri-wasm.md`, NFR-DET-001).
4. **Contratos**: cada DTO con fixtures válidos e inválidos y *contract tests* (NFR-OBS-001).
5. **Versionado**: cambios de forma incrementan `schemaVersion`; los DTO persistidos requieren
   versión y *migration note*.

## Regla de no exponer infraestructura

Los DTO **no** pueden contener, ni directa ni indirectamente: rutas absolutas, descriptores de
fichero, handles de socket o de sistema operativo; conexiones de base de datos o dependencias de
red; cadenas de shell, comandos, hooks o `pacstrap`; tipos del runtime (DOM, WebView) o del sistema
de ficheros; ni el contenido de un secreto (solo `secretRef`, `THR-SEC-001`). Las referencias a
recursos externos son mangos opacos (`destinationHandle`) emitidos por el picker. El core permanece
puro y sin privilegios (NFR-PORT-001).

## Fuera de CorePort v0

Los siguientes DTO no forman parte de la versión congelada y se reintroducen en fases posteriores:
`ImportDraftInput`/`ImportResult`, `MigrationPlan`/`MigrationResult` (`FR-IMPORT-001`),
`ApplyPresetInput`, `CompareDraftsInput`/`DraftDiff` (`FR-PRESET-001`), `CheckTargetInput`/
`TargetCompatibility`, `ExportTarget`, `ProductInfo` y `CapabilitySet` (v1/Enterprise). No se
congelan en v0.

## Trazabilidad

- Autoridad de operaciones y reglas: `core-port.md` (DOC-IF-CORE-001).
- Errores y eventos: `errors-events.md` (DOC-IF-ERR-001).
- Adaptadores: `tauri-commands.md`, `tauri-wasm.md`.
- Schemas: `contracts/json-schema/` (11 schemas Draft 2020-12 v0); corpus en
  `contracts/json-schema/examples/`; vectores en `contracts/test-vectors/canonicalization/`.
- Requisitos: FR-DRAFT-001, FR-CAT-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001,
  FR-EXPORT-001, NFR-DET-001, NFR-OBS-001, NFR-PORT-001.
