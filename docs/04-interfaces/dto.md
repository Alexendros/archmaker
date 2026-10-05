# DTO de CorePort

- ID: DOC-IF-DTO-001
- Estado: draft
- Propietario: Arquitectura de interfaces
- Última revisión: 2026-10-05
- Requisitos relacionados: FR-DRAFT-001, FR-IMPORT-001, FR-CAT-001, FR-PRESET-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001, FR-EXPORT-001, NFR-DET-001, NFR-OBS-001, NFR-PORT-001
- Sustituye / sustituido por: —

## Alcance

Este documento enumera los DTO que cruzan `CorePort` (ver `core-port.md`), los tipos de valor
compartidos y las reglas de paridad entre Rust, TypeScript y WASM. **No define schemas
definitivos**: la forma exacta se fijará en JSON Schema 2020-12 (`SRC-006`, pendiente de captura)
y en los correspondientes ADR de contrato. Los ejemplos JSON son ilustrativos.

Cada contrato se versiona con `schemaVersion`, independiente de `documentVersion`,
`catalogRef.version`, `targetRef.version`, `producer.version` y `protocolVersion`
(ver `versioning-migrations.md`). Los contratos persistidos requieren versión y *migration note*
(ver `document-control.md`).

## Convenciones

- **Dirección**: `→` (entrada a `CorePort`) o `←` (salida de `CorePort`).
- **Obligatoriedad**: `Sí`, `No` o `Cond.` (condicional, descrito en notas).
- **Persistible**: si el DTO puede almacenarse como verdad. `Resolution` y los planes derivados
  **nunca** se persisten como verdad (glosario, `DM-RESOLUTION`).
- **Discriminantes**: uniones cerradas por un campo discriminador; valor exacto pendiente de
  schema cuando no esté fijado por el glosario.
- **Tipos**: `uuid`, `string`, `boolean`, `number`, `int`, `Digest`, `Ref`, `array<T>`,
  `map<string,T>`, `enum`.

## Tipos de valor compartidos

### `Result<T, CoreError>`

Resultado de toda operación de `CorePort`. Forma propuesta (unión discriminada por `ok`):

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `ok` | boolean | Sí | `true` → `value`; `false` → `error`. |
| `value` | T | Cond. | Presente si `ok = true`. |
| `error` | CoreError | Cond. | Presente si `ok = false`. |

No persistible.

### `SelectionValue`

Unión discriminada fijada por el glosario (`DOC-GOV-001`): `single`, `multiple`, `boolean`,
`number`, `text`, `secretRef`. Discriminador propuesto: `kind`. Persistible salvo que el
contenido sea un secreto: `secretRef` **nunca** serializa el secreto (`THR-SEC-001`).

| Variante | Campo(s) | Tipo | Oblig. | Notas |
|---|---|---|---|---|
| `single` | `optionId` | string | Sí | Cardinalidad uno. |
| `multiple` | `optionIds` | array<string> | Sí | El orden se fija por ID al canonicalizar. |
| `boolean` | `value` | boolean | Sí | — |
| `number` | `value` | number | Sí | `unit` opcional; schema restringe uso de floats. |
| `number` | `unit` | string | No | — |
| `text` | `value` | string | Sí | Con límite de longitud por schema. |
| `secretRef` | `ref` | string | Sí | Referencia opaca; sin secreto en claro. |

### Referencias e identificadores

| Tipo | Campos | Oblig. | Notas |
|---|---|---|---|
| `CatalogRef` | `namespace`, `id`, `version`, `digest` | Sí / Sí / Sí / Sí | `namespace` depende de **DEC-002**. `digest` tipo `Digest`. |
| `TargetRef` | `id`, `version` | Sí / Sí | Conjunto exacto depende de **DEC-008**. |
| `DraftRef` | `id` (uuid), `documentVersion`, `digest` | Sí / No / No | `digest` solo si está guardado. |
| `PresetRef` | `namespace`, `id`, `version` | Sí | Patch inmutable (`DM-PRESET`). |
| `ManifestRef` | `digest` | Sí | Materialización cerrada (`DM-MANIFEST`). |
| `ArtifactRef` | `digest`, `mediaType` | Sí | `DM-ARTIFACT`. |
| `Digest` | `algorithm` (`sha256`), `value` | Sí | Etiqueta de dominio en el payload canónico (`canonicalization.md`). |
| `RequestId` | uuid | Sí | Correlación de operaciones largas. |
| `CorrelationId` | uuid | Sí | Correlación de errores/eventos. |

`documentVersion` y `schemaVersion` son cadenas; su formato exacto depende de un ADR pendiente.

### `CoreError` y `Diagnostic`

Se definen en `errors-events.md` y se referencian, no se redefinen aquí:

- `CoreError`: `code`, `category`, `messageKey`, `params`, `retryable`, `correlationId`, `causeCode`.
- `Diagnostic`: `code`, `severity`, `blocking`, `path`, `messageKey`, `params`, `source`, `ruleId`, `suggestions`.

## DTO de entrada

Índice (todos no persistibles: son argumentos, no estado):

| DTO | Propósito | Método `CorePort` | Versión |
|---|---|---|---|
| `LoadCatalogInput` | Cargar un catálogo versionado y acotar su lectura. | `loadCatalog` | `schemaVersion 1` (propuesta) |
| `CreateDraftInput` | Crear un Draft vacío ligado a catálogo y target. | `createDraft` | ídem |
| `ImportDraftInput` | Importar un documento local y acotar el parseo. | `importDraft` | ídem |
| `PlanMigrationInput` | Planificar la migración detectada sin aplicarla. | `planMigration` | ídem |
| `ApplyMigrationInput` | Aplicar el plan con las decisiones del usuario. | `applyMigration` | ídem |
| `ApplyPresetInput` | Aplicar un preset como patch sobre un Draft. | `applyPreset` | ídem |
| `ResolveDraftInput` | Resolver un Draft contra un catálogo. | `resolveDraft` | ídem |
| `ValidateDraftInput` | Ejecutar el pipeline de validación. | `validateDraft` | ídem |
| `BuildManifestInput` | Construir el manifest canónico. | `buildManifest` | ídem |
| `CompareDraftsInput` | Comparar dos Drafts y producir un ChangeSet. | `compareDrafts` | ídem |
| `CheckTargetInput` | Comprobar compatibilidad con un target. | `checkTarget` | ídem |
| `ExportArtifactInput` | Exportar un artifact a un destino elegido. | `exportArtifact` | ídem |

### `LoadCatalogInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `catalogRef` | CatalogRef | Cond. | Requerido si `source = registry`; omitido para catálogo embebido. |
| `source` | enum(`embedded`, `file`, `registry`) | Sí | MVP: `embedded` o `file` (picker). `registry` no verificado en MVP. |
| `expectedDigest` | Digest | No | Verificación de integridad/anti-manipulación (`THR-CAT-001`). |
| `limits` | `{ maxBytes, maxDepth }` | No | Límites antes y durante parse (`THR-IMP-001`). |

### `CreateDraftInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `catalogRef` | CatalogRef | Sí | Catálogo base. |
| `targetRef` | TargetRef | Sí | UC-001: seleccionar objetivo. |
| `presetRef` | PresetRef | No | Preset inicial opcional. |
| `name` | string | No | Etiqueta local de UI (`FR-DRAFT-001`). |

### `ImportDraftInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `content` | string | Sí | Documento local acotado; original intacto (`FR-IMPORT-001`). |
| `mediaType` | string | Sí | P. ej. `application/json`. |
| `sourceFormat` | enum(`v5.1`) | Sí | Detección de versión (`versioning-migrations.md`). |
| `targetCatalogRef` | CatalogRef | No | Catálogo destino si difiere del activo. |
| `limits` | `{ maxBytes, maxDepth, maxItems }` | Sí | Obligatorio por `THR-IMP-001`. |

### `PlanMigrationInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `content` | string | Sí | Mismo documento validado estructuralmente. |
| `sourceFormat` | enum(`v5.1`) | Sí | — |
| `targetCatalogRef` | CatalogRef | Sí | Catálogo destino. |

### `ApplyMigrationInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `planId` | uuid | Sí | Plan mostrado al usuario. |
| `targetCatalogRef` | CatalogRef | Sí | Debe coincidir con el del plan. |
| `decisions` | array<`MigrationDecision`> | Cond. | Requerido para casos `requiresDecisions`. |
| `requestId` | RequestId | No | Operación larga (eventos correlacionados). |

`MigrationDecision`: `{ caseId: string, choice: string }`.

### `ApplyPresetInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `draft` | Draft | Sí | Destino del patch. |
| `presetRef` | PresetRef | Sí | Fuente inmutable. |
| `requestId` | RequestId | No | — |

### `ResolveDraftInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `draft` | Draft | Sí | Intención manual y refs; sin estado derivado. |
| `catalogRef` | CatalogRef | Sí | Catálogo de definiciones. |
| `policyBundleRef` | `{ namespace, id, version, signature }` | No | Solo Enterprise (`DM-POLICY`); no en MVP. |
| `requestId` | RequestId | No | — |

### `ValidateDraftInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `draft` | Draft | Sí | — |
| `catalogRef` | CatalogRef | Sí | — |
| `targetRef` | TargetRef | No | Habilita etapas de target. |
| `validatorIds` | array<string> | No | Subconjunto del pipeline (`VAL-*`); por defecto, todas. |
| `requestId` | RequestId | No | — |

### `BuildManifestInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `draft` | Draft | Sí | — |
| `catalogRef` | CatalogRef | Sí | — |
| `targetRef` | TargetRef | Sí | — |
| `expectedResolutionDigest` | Digest | No | Evita construir desde una Resolution distinta. |
| `requestId` | RequestId | No | — |

### `CompareDraftsInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `left` | Draft | Sí | — |
| `right` | Draft | Sí | — |
| `catalogRef` | CatalogRef | Sí | — |

### `CheckTargetInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `manifestRef` | ManifestRef | Sí | Entrada canónica de los exporters. |
| `targetRef` | TargetRef | Sí | — |

### `ExportArtifactInput`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `manifestRef` | ManifestRef | Sí | — |
| `targetRef` | TargetRef | Sí | Debe estar en `listExportTargets` y pasar `checkTarget`. |
| `destinationHandle` | string | Sí | Mango opaco del picker; **nunca** una ruta arbitraria (`THR-FS-001`). |
| `requestId` | RequestId | No | — |

## DTO de salida

Índice:

| DTO | Propósito | Método(s) `CorePort` | Persistible | Versión |
|---|---|---|---|---|
| `ProductInfo` | Metadatos de producto y versiones soportadas. | `productInfo` | No | `schemaVersion 1` (propuesta) |
| `CapabilitySet` | Habilidades del runtime expuestas al adaptador. | `capabilities` | No | ídem |
| `CatalogView` | Proyección de un catálogo cargado. | `loadCatalog` | No (derivada) | ídem |
| `Draft` | Agregado de intención editable (`DM-DRAFT`). | `createDraft`, `importDraft`, `applyMigration` | **Sí** | ídem |
| `ImportResult` | Draft migrado más informe de importación. | `importDraft` | No | ídem |
| `MigrationPlan` | Casos de migración a confirmar. | `planMigration` | No (recalculable) | ídem |
| `MigrationResult` | Resultado de aplicar el plan. | `applyMigration` | No | ídem |
| `ResolveResult` | Resolution determinista y efímera (`DM-RESOLUTION`). | `applyPreset`, `resolveDraft` | **No** | ídem |
| `ValidationResult` | Diagnósticos del pipeline. | `validateDraft` | No | ídem |
| `ManifestResult` | Manifest canónico y su digest. | `buildManifest` | Sí (el manifest) | ídem |
| `DraftDiff` | ChangeSet entre dos Drafts. | `compareDrafts` | No | ídem |
| `ExportTarget` | Destino de exportación disponible. | `listExportTargets` | Sí (referencia) | ídem |
| `TargetCompatibility` | Compatibilidad con un target. | `checkTarget` | No | ídem |
| `ExportResult` | Artifact exportado y digests. | `exportArtifact` | Sí (el artifact) | ídem |

### `ProductInfo`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `productName` | string | Sí | — |
| `productVersion` | string | Sí | — |
| `apiVersion` | string | Sí | Versión del contrato `CorePort`. |
| `supportedDocumentVersions` | array<string> | Sí | P. ej. `v5.1`, `vNext`. |
| `supportedProtocolVersions` | array<string> | No | v1 (`runner-protocol.md`). |

### `CapabilitySet`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `features` | array<string> | Sí | Habilidades del runtime. |
| `exportTargets` | array<ExportTarget> | No | Atajo de `listExportTargets`. |
| `adapters` | array<`{ id, version }`> | No | Adapters disponibles. |

> Homónimo: no confundir `CapabilitySet` (habilidades del runtime) con la `Capability` de dominio
> del glosario (`CAP-*`, `provides`/`requires`). Si la ambigüedad se confirma, abrir entrada en
> `contradiction-register.md`.

### `CatalogView`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `catalogRef` | CatalogRef | Sí | Incluye `digest`. |
| `steps` | array<`StepView`> | Sí | — |
| `capabilities` | array<Capability> | No | Términos `CAP-*` del catálogo. |
| `metadata` | map<string,string> | No | — |
| `diagnostics` | array<Diagnostic> | No | Carga degradada o avisos. |
| `loadedAt` | timestamp | No | Efímero; excluido del hash (`canonicalization.md`). |

`StepView`: `{ stepId: string, title: string, cardinality: enum(`one`,`many`) }`.

### `Draft`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `id` | uuid | Sí | Identidad `DM-DRAFT`. |
| `documentVersion` | string | Sí | Versión del documento. |
| `schemaVersion` | string | Sí | Versión del schema. |
| `catalogRef` | CatalogRef | Sí | — |
| `targetRef` | TargetRef | Sí | — |
| `selections` | array<`Selection`> | Sí | Solo intención manual y refs. |
| `presetRefs` | array<PresetRef> | No | Presets aplicados; las selecciones derivadas se recalculan. |
| `createdAt` | timestamp | No | Efímero; excluido del hash. |
| `updatedAt` | timestamp | No | Efímero; excluido del hash. |

`Selection`: `{ stepId: string, optionId?: string, value: SelectionValue }`. Las selecciones
derivadas **no** se almacenan en el Draft: las produce el resolver a partir de `presetRefs`.

### `ImportResult`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `draft` | Draft | Sí | Nuevo documento; el original se conserva. |
| `report` | `MigrationReport` | Sí | `preserved` / `transformed` / `dropped` / `rejected`. |
| `sourceDigest` | Digest | Sí | Digest del documento original. |
| `diagnostics` | array<Diagnostic> | No | Pérdidas explícitas. |

### `MigrationPlan`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `planId` | uuid | Sí | — |
| `sourceFormat` | enum(`v5.1`) | Sí | — |
| `targetCatalogRef` | CatalogRef | Sí | — |
| `cases` | array<`MigrationCase`> | Sí | — |
| `requiresDecisions` | boolean | Sí | Si `true`, `applyMigration` exige `decisions`. |
| `diagnostics` | array<Diagnostic> | No | — |

`MigrationCase`: `{ caseId: string, kind: enum(`preserved`,`transformed`,`dropped`,`rejected`), path: string, reason: string }`.

### `MigrationResult`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `draft` | Draft | Sí | Extracto migrado. |
| `appliedCases` | array<MigrationCase> | Sí | — |
| `report` | `MigrationReport` | Sí | — |
| `diagnostics` | array<Diagnostic> | No | — |

### `ResolveResult`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `resolutionDigest` | Digest | Sí | Digest del input; recalculable. |
| `effectiveSelections` | array<`ResolvedSelection`> | Sí | Manual + derivado. |
| `providedCapabilities` | array<Capability> | Sí | Cierre computado. |
| `requiredCapabilities` | array<Capability> | Sí | — |
| `conflicts` | array<`Conflict`> | Sí | Vacío si no hay. |
| `changeSet` | DraftDiff | No | Presente si deriva de `applyPreset`. |
| `diagnostics` | array<Diagnostic> | No | — |

`ResolvedSelection`: `{ stepId, optionId, value: SelectionValue, origin: enum(`manual`,`derived`), locked: boolean }`.
`Conflict`: `{ severity, capabilityId?, path, messageKey, suggestions: array<string> }`.

No persistible: tratar una Resolution como persistible está prohibido por el glosario.

### `ValidationResult`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `diagnostics` | array<Diagnostic> | Sí | Tipados, localizados y estables (`FR-VALIDATE-001`). |
| `blocking` | boolean | Sí | `true` si algún diagnóstico bloqueante. |
| `pipelineVersion` | string | Sí | Versión de `archmaker-validation`. |
| `validatedAt` | timestamp | No | Efímero; excluido del hash. |

### `ManifestResult`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `manifest` | `Manifest` | Sí | Documento canónico, cerrado y ordenado. |
| `manifestRef` | ManifestRef | Sí | — |
| `manifestDigest` | Digest | Sí | Digest canónico. |
| `producer` | `{ id, version }` | Sí | `archmaker-export`/productor. |
| `resolutionDigest` | Digest | Sí | Trazabilidad hacia la Resolution. |
| `diagnostics` | array<Diagnostic> | No | — |

### `DraftDiff`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `entries` | array<`DiffEntry`> | Sí | — |
| `summary` | `{ manual: int, derived: int }` | Sí | `FR-PRESET-001`. |

`DiffEntry`: `{ path: string, before: SelectionValue|null, after: SelectionValue|null, origin: enum(`manual`,`derived`), kind: enum(`added`,`removed`,`changed`) }`.

### `ExportTarget`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `id` | enum(`archmaker-profile`, `archinstall-profile`, `report`) | Sí | Conjunto exacto depende de **DEC-001**. |
| `version` | string | Sí | — |
| `mediaType` | string | Sí | — |
| `description` | string | No | — |
| `experimental` | boolean | Sí | `archinstall-profile` marcado experimental. |

### `TargetCompatibility`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `targetRef` | TargetRef | Sí | — |
| `compatible` | boolean | Sí | — |
| `supported` | boolean | Sí | Soporte oficial (**DEC-008**). |
| `reasons` | array<string> | No | — |
| `diagnostics` | array<Diagnostic> | No | — |

### `ExportResult`

| Campo | Tipo | Oblig. | Notas |
|---|---|---|---|
| `artifactRef` | ArtifactRef | Sí | — |
| `targetRef` | TargetRef | Sí | — |
| `mediaType` | string | Sí | — |
| `digest` | Digest | Sí | — |
| `producer` | `{ id, version }` | Sí | — |
| `destinationHandle` | string | No | Mango opaco; sin ruta absoluta. |
| `report` | `MigrationReport` | No | Solo para `target.id = report`. |
| `diagnostics` | array<Diagnostic> | No | — |

## Regla de paridad TS/Rust/WASM

1. **Mismos campos y semántica** en Rust nativo, TypeScript (`CorePort`) y WASM: mismos nombres
   serializados, mismos tipos, misma obligatoriedad y mismas reglas de unión.
2. **Fuente única de schema**: un solo JSON Schema 2020-12 (`SRC-006`) genera/valida los tres
   consumidores. Rust es la autoridad semántica (`ADR-0001`, `DOC-GOV-001` regla 5); TypeScript no
   duplica reglas.
3. **Paridad verificable**: un corpus compartido debe producir los mismos diagnósticos, bytes de
   manifest canónicos y digests en Rust nativo y WASM (`tauri-wasm.md`, `NFR-DET-001`).
4. **Contratos**: cada DTO con fixtures válidos e inválidos y *contract tests* (`NFR-OBS-001`,
   `test-matrix.md` fila `Tauri/WASM`).
5. **Versionado**: cambios de forma incrementan `schemaVersion`; los DTO persistidos requieren
   versión y *migration note*.

## Regla de no exponer infraestructura

Los DTO **no** pueden contener, ni directa ni indirectamente:

- rutas absolutas, descriptores de fichero, handles de socket o de sistema operativo;
- conexiones de base de datos, clientes HTTP o cualquier dependencia de red;
- cadenas de shell, comandos, hooks o `pacstrap` (glosario, anti-patrones);
- tipos del runtime (DOM, React), del sistema de ficheros o del WebView;
- el contenido de un secreto: solo `secretRef` (`THR-SEC-001`).

Las referencias a recursos externos son mangos opacos (`destinationHandle`) emitidos por el
picker del adaptador. El core permanece puro y sin privilegios (`NFR-PORT-001`).

## Relación con familias de error

Asociación indicativa (las familias están definidas en `errors-events.md`):

| Método | Familias principales |
|---|---|
| `productInfo`, `capabilities` | `AM-PROTO`, `AM-DOC` |
| `loadCatalog` | `AM-CAT`, `AM-SCHEMA`, `AM-IO` |
| `createDraft` | `AM-DOC`, `AM-SCHEMA` |
| `importDraft`, `planMigration`, `applyMigration` | `AM-MIG`, `AM-IO`, `AM-DOC` |
| `applyPreset`, `resolveDraft` | `AM-RES`, `AM-CAT` |
| `validateDraft` | `AM-RULE`, `AM-SCHEMA`, `AM-CAT`, `AM-TGT` |
| `buildManifest` | `AM-RES` |
| `compareDrafts` | `AM-RES` |
| `listExportTargets`, `checkTarget`, `exportArtifact` | `AM-TGT`, `AM-IO` |

## Ejemplos JSON ilustrativos

> Solo ilustrativos. No son los schemas definitivos.

**Entrada — `CreateDraftInput`:**

```json
{
  "catalogRef": {
    "namespace": "archmaker.core",
    "id": "desktop",
    "version": "1.0.0",
    "digest": { "algorithm": "sha256", "value": "9f2c...ab10" }
  },
  "targetRef": { "id": "arch-x86_64", "version": "2026.10" },
  "presetRef": { "namespace": "archmaker.presets", "id": "minimal-uefi", "version": "1.0.0" },
  "name": "Mi escritorio"
}
```

**Salida — `ResolveResult`:**

```json
{
  "resolutionDigest": { "algorithm": "sha256", "value": "1c77...90ff" },
  "effectiveSelections": [
    {
      "stepId": "boot.loader",
      "optionId": "grub",
      "value": { "kind": "single", "optionId": "grub" },
      "origin": "manual",
      "locked": false
    },
    {
      "stepId": "fs.root",
      "optionId": "btrfs",
      "value": { "kind": "single", "optionId": "btrfs" },
      "origin": "derived",
      "locked": true
    }
  ],
  "providedCapabilities": [{ "id": "fs.btrfs" }],
  "requiredCapabilities": [{ "id": "boot.uefi" }],
  "conflicts": [],
  "diagnostics": [
    {
      "code": "AM-RES-004",
      "severity": "warning",
      "blocking": false,
      "path": "/selections/environment.desktop",
      "messageKey": "diagnostic.capabilityMissing",
      "params": {},
      "source": "archmaker-resolver",
      "ruleId": "rule-cap-fs",
      "suggestions": []
    }
  ]
}
```

## Trazabilidad y dependencias

- `core-port.md`: firmas de los 15 métodos.
- `errors-events.md`: `CoreError`, `Diagnostic`, familias `AM-*`.
- `domain-model.md`: agregados (`DM-*`) y `SelectionValue`.
- `versioning-migrations.md` y `canonicalization.md`: versiones, campos efímeros y hashing.
- `tauri-wasm.md`: paridad y adaptadores.
- `glossary.md`: términos canónicos y anti-patrones.
- Decisiones abiertas: **DEC-001** (`ExportTarget`), **DEC-002** (`CatalogRef.namespace`),
  **DEC-008** (`TargetRef`).
- Fuente pendiente: **SRC-006** (JSON Schema 2020-12) para cerrar los schemas definitivos.
