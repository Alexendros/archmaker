---
id: DOC-IF-CORE-001
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
  - id: ADR-0005
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
  - id: ADR-0007
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
  - id: DOC-DATA-CANON-001
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
  - id: DOC-DATA-DM-001
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
---

# CorePort v0

Autoridad única de la frontera implementable UI↔dominio (fase R7, issue `AUD-016`). Congela la
superficie `CorePort` **v0** con exactamente las siete operaciones del esqueleto caminante. Toda
otra operación citada en otros documentos (`importDraft`, `applyPreset`, `planMigration`,
`applyMigration`, `compareDrafts`, `checkTarget`, `listExportTargets`, `productInfo`,
`capabilities`) queda **fuera de `CorePort v0`** y se reintroduce en fases posteriores; su presencia
en requisitos, ciclos de vida o UX describe comportamiento futuro, no la versión congelada.

La forma canónica de los contratos persistidos vive en los JSON Schema 2020-12 **v0** de
`contracts/json-schema/` (ADR-0005); este documento no la duplica: la referencia por `$id` y
`$defs`. La semántica de cada operación es de **Rust** (ADR-0001); TypeScript solo declara tipos.

## Reglas duras

1. **TypeScript no implementa reglas de dominio.** El tipo `CorePort` de TypeScript es una
   declaración de frontera; cada regla la evalúa el core Rust. Ninguna regla se reimplementa en UI.
2. **Tauri y WASM son solo adaptadores.** Ambos implementan la **misma** interfaz `CorePort` sin
   semántica divergente (`tauri-commands.md`, `tauri-wasm.md`).
3. **Los errores públicos son tipados.** Toda falla se expresa como `CoreError`
   (`core-error.schema.json`); nunca un string libre. Los códigos `AM-*` provienen del catálogo
   único de `errors-events.md`.
4. **`saveDraft` exige precondición de revisión.** Persiste solo si la revisión almacenada coincide
   con `expectedRevision`; un desajuste es un conflicto tipado, nunca un sobrescritura silenciosa
   (`draft.schema.json#/$defs/saveDraftRequest`).
5. **Las operaciones puras no acceden a filesystem ni red.** `createDraft`, `resolveDraft`,
   `validateDraft` y `buildManifest` son puras. `loadCatalog`, `saveDraft` y `exportArtifact`
   realizan I/O acotada, siempre mediada por el adaptador.
6. **Ninguna operación acepta shell, comandos, hooks, `pacstrap` ni rutas arbitrarias.** Las
   referencias externas son mangos opacos (`destinationHandle`) emitidos por el picker.
7. **La `Resolution` es efímera.** Nunca se persiste como verdad; solo se materializa en el
   `Manifest`.

## Superficie congelada

```ts
// Declaración de frontera. Rust es la autoridad semántica (ADR-0001).
export interface CorePort {
  createDraft(input: CreateDraftInput): Promise<Result<Draft, CoreError>>;
  loadCatalog(input: LoadCatalogInput): Promise<Result<Catalog, CoreError>>;
  saveDraft(input: SaveDraftInput): Promise<Result<Draft, CoreError>>;
  resolveDraft(input: ResolveDraftInput): Promise<Result<ResolveResult, CoreError>>;
  validateDraft(input: ValidateDraftInput): Promise<Result<ValidationResult, CoreError>>;
  buildManifest(input: BuildManifestInput): Promise<Result<Manifest, CoreError>>;
  exportArtifact(input: ExportArtifactInput): Promise<Result<Artifact, CoreError>>;
}
```

Los DTO de entrada y salida se detallan en `dto.md` (DOC-IF-DTO-001) y se derivan de los schemas
v0. La operación `loadCatalog` devuelve el `Catalog` canónico (`catalog.schema.json`); cualquier
proyección de UI sobre él es derivada y no un contrato.

## Contrato común

| Elemento | Definición |
|---|---|
| Resultado | `Result<T> = { ok: true, value: T } \| { ok: false, error: CoreError }` (unión discriminada; no persistible). |
| Versionado | Cada payload declara `schemaVersion` (eje `schemaVersion`, ADR-0006) según su schema v0; ninguna operación introduce un eje propio. |
| Correlación | `requestId` (`common.schema.json#/$defs/uuid`) opcional en toda operación; correlaciona eventos y errores. |
| Límites | `limits` (`common.schema.json#/$defs/limits`): `maxBytes`, `maxDepth`, `maxStringLength`, `maxArrayItems`, todos enteros ≥ 0. |
| Errores | `CoreError` (`core-error.schema.json`): `code`, `family`, `category`, `severity`, `messageKey`, `source` obligatorios; `cause` es cadena recursiva. |
| Diagnósticos | `Diagnostic` (`diagnostic.schema.json`); las colecciones se emiten ordenadas por `(path, code, source)` (`#/$defs/diagnosticList`). |
| Eventos | Efímeros, excluidos del payload canónico, nunca persistidos; catálogo en `errors-events.md`. |
| Redacción | Nunca cruzan la frontera secretos (solo `secretRef`), rutas absolutas ni handles de sistema. |

### Límites por defecto

Los valores por defecto son objetivo de contrato (pendientes de implementación, no verificados) y
se acotan siempre por las cotas del schema correspondiente:

| Dimensión (`limits`) | Por defecto propuesto | Cota de schema |
|---|---|---|
| `maxBytes` | 16777216 (16 MiB) | — |
| `maxDepth` | 128 | — |
| `maxStringLength` | 65536 | `text` 4096; títulos 256; `metadata` 1024 |
| `maxArrayItems` | 4096 | `draft.selections` 4096; `manifest.effectiveSelections` 4096; `catalog.steps` 1024; `catalog` opciones 4096 |

Fuera de estos límites la operación falla con `AM-DOC-001` (límites/formato) o `AM-SCHEMA-001`
(schema) según el eje violado.

### Perfil de I/O

| Operación | Clase | Acceso |
|---|---|---|
| `createDraft` | pura | Ninguno |
| `loadCatalog` | mixta | `embedded`: pura. `file`: una lectura acotada de un handle del picker. Sin red. |
| `saveDraft` | I/O | Escritura atómica (`temp` + `fsync` + `rename`) en el directorio de drafts. Sin red. |
| `resolveDraft` | pura | Ninguno |
| `validateDraft` | pura | Ninguno |
| `buildManifest` | pura | Ninguno |
| `exportArtifact` | I/O | Escritura atómica en el destino del picker. Sin red. |

Los adaptadores nunca convierten una operación pura en I/O; Tauri y WASM respetan esta tabla.

## Operaciones

### `createDraft`

| Aspecto | Especificación |
|---|---|
| Request | `CreateDraftInput` (DTO de operación; campos en `dto.md`). |
| Response | `Draft` — `draft.schema.json`. |
| Precondiciones | `catalogRef` (`common.schema.json#/$defs/catalogRef`) y `targetRef` (`common.schema.json#/$defs/targetRef`) válidos; `targetRef.id` en el conjunto soportado (DEC-008). |
| Postcondiciones | `Draft` con `revision: 0`, `selections: []`, sin `contentDigest`; sin estado derivado; no se persiste. |
| Pureza/IO | Pura. Sin filesystem ni red. |
| Idempotencia | Determinista respecto del input; **idempotente si se aporta `draftId`** (mismo input ⇒ mismo `Draft`). Sin `draftId`, el core acuña identidad no determinista y la operación no es retry-idempotente. |
| Cancelación y tiempo límite | Instantánea; cancelación no aplicable. Tiempo límite objetivo 1 s. |
| Límites | `draft.selections` ≤ 4096; `draft.presetRefs` ≤ 256; `maxStringLength` según schema. |
| Errores | `AM-CAT-001` (catálogo no resoluble), `AM-SCHEMA-001`, `AM-DOC-001`, `AM-TGT-001`. |
| Eventos | `core.operation.started`, `core.operation.completed`; `draft.changed` cuando el adaptador lo ofrece. |
| Redacción | Sin rutas; `name` de UI no forma parte del `Draft` persistido (no representable en el schema). |
| Vectores | `contracts/json-schema/examples/01-draft.valid.json`, `01-draft.invalid.json`, `14-draft-adversarial.valid.json`; `TST-DRAFT-001`. |

### `loadCatalog`

| Aspecto | Especificación |
|---|---|
| Request | `LoadCatalogInput` (DTO de operación; campos en `dto.md`). |
| Response | `Catalog` — `catalog.schema.json`. |
| Precondiciones | `source = file` exige un handle del picker y `catalogRef`; `source = embedded` exige catálogo empaquetado. Si hay `expectedDigest`, debe coincidir con el `contentDigest` del catálogo. |
| Postcondiciones | `Catalog` válido contra `catalog.schema.json`; digest verificado; sin estado efímero persistido. |
| Pureza/IO | `embedded`: pura. `file`: una lectura acotada, sin red. |
| Idempotencia | Idempotente para la misma fuente y digest; si `expectedDigest` difiere, falla en lugar de degradar. |
| Cancelación y tiempo límite | Cancelable durante la lectura; objetivos 1 s (`embedded`) y 10 s (`file`). |
| Límites | `limits` de entrada acotan bytes/profundidad/array/string antes y durante el parseo. |
| Errores | `AM-CAT-001`, `AM-CAT-002` (digest no coincide), `AM-CAT-003` (firma), `AM-SCHEMA-001`, `AM-DOC-001`, `AM-DOC-003`, `AM-IO-001`. |
| Eventos | `core.operation.started`, `catalog.loaded`, `core.operation.completed`. |
| Redacción | Rutas locales excluidas (`sourcePath`, `localPath`); solo `catalogRef` y digests cruzan. |
| Vectores | `contracts/json-schema/examples/02-catalog.valid.json`, `02-catalog.invalid.json`, `15-catalog-adversarial.valid.json`, `19-common-limits.valid.json`; `TST-CAT-001`, `TST-OFF-001`. |

### `saveDraft`

| Aspecto | Especificación |
|---|---|
| Request | `SaveDraftInput` — `draft.schema.json#/$defs/saveDraftRequest`. |
| Response | `Draft` persistido — `draft.schema.json`. |
| Precondiciones | El `draft` valida el schema; la revisión almacenada coincide con `expectedRevision`. |
| Postcondiciones | Nueva `revision` = previa + 1; `contentDigest` recalculado (perfil v1, ADR-0007); escritura atómica. |
| Pureza/IO | I/O de escritura acotada; sin red. |
| Idempotencia | **No idempotente.** Tras un guardado correcto, reintentar con el mismo `expectedRevision` falla con `AM-DOC-002` (conflicto). El reintento correcto exige la revisión actualizada. |
| Cancelación y tiempo límite | Cancelable hasta antes del `fsync`; un fallo de `rename` no deja artefacto parcial. Tiempo límite objetivo 5 s. |
| Límites | Cotas de `draft.schema.json` (`selections` ≤ 4096, `presetRefs` ≤ 256). |
| Errores | `AM-DOC-002` (conflicto de revisión), `AM-DOC-001`, `AM-DOC-003`, `AM-SCHEMA-001`, `AM-IO-001`. |
| Eventos | `core.operation.started`, `draft.changed`, `core.operation.completed`. |
| Redacción | Nunca se serializa el valor de un secreto; solo `secretRef`. Sin rutas en la respuesta. |
| Vectores | `contracts/json-schema/examples/13-draft-savedraft.valid.json`, `13-draft-savedraft.invalid.json`, `01-draft.valid.json`; `TST-DRAFT-001`, `TST-NEG-001`. |

### `resolveDraft`

| Aspecto | Especificación |
|---|---|
| Request | `ResolveDraftInput` (DTO de operación; campos en `dto.md`). |
| Response | `ResolveResult` (efímero, `DM-RESOLUTION`, sin schema persistido). |
| Precondiciones | `draft.catalogRef` coincide con el catálogo cargado; catálogo y draft válidos. |
| Postcondiciones | Selección efectiva (manual + derivada), `providedCapabilities`, `requiredCapabilities`, `conflicts`, `diagnostics`; `resolutionDigest` recalculable. Nada se persiste. |
| Pureza/IO | Pura. Sin filesystem ni red. |
| Idempotencia | Idempotente y determinista (NFR-DET-001): el mismo input produce el mismo `resolutionDigest`. |
| Cancelación y tiempo límite | Instantánea; cancelación no aplicable. Tiempo límite objetivo 5 s. |
| Límites | `effectiveSelections` ≤ 4096; `conflicts` ≤ 4096; profundidad de resolución acotada por `maxDepth`. |
| Errores | `AM-RES-001` (referencia de valor no resoluble), `AM-RES-004` (capability ausente), `AM-RULE-001`…`AM-RULE-009`, `AM-CAT-001`, `AM-DOC-001`. |
| Eventos | `core.operation.started`, `resolution.changed`, `core.operation.completed`. |
| Redacción | La `Resolution` no se persiste ni contiene secretos; `secretRef` se conserva como referencia. |
| Vectores | `contracts/test-vectors/canonicalization/manifest.json` (`a-order-1`/`a-order-2`), `contracts/json-schema/examples/01-draft.valid.json`; `TST-RES-001`. |

### `validateDraft`

| Aspecto | Especificación |
|---|---|
| Request | `ValidateDraftInput` (DTO de operación; campos en `dto.md`). |
| Response | `ValidationResult` (efímero, sin schema persistido). |
| Precondiciones | `draft` y `catalogRef` válidos; `targetRef` opcional habilita etapas de target. |
| Postcondiciones | `diagnostics` tipados y estables, ordenados por `(path, code, source)`; `blocking: true` si hay algún `error`. |
| Pureza/IO | Pura. Sin filesystem ni red. |
| Idempotencia | Idempotente y determinista; el mismo input produce la misma lista ordenada. |
| Cancelación y tiempo límite | Instantánea; cancelación no aplicable. Tiempo límite objetivo 10 s. |
| Límites | `diagnostics` ≤ 4096 (`diagnostic.schema.json#/$defs/diagnosticList`); `suggestions` ≤ 64. |
| Errores | `AM-RULE-*`, `AM-SCHEMA-001`, `AM-CAT-001`, `AM-TGT-001`, `AM-DOC-001`. |
| Eventos | `core.operation.started`, `validation.completed`, `core.operation.completed`. |
| Redacción | Los diagnósticos no incluyen secretos ni rutas absolutas; `path` es un JSON Pointer. |
| Vectores | `contracts/json-schema/examples/04-diagnostic.valid.json`, `16-diagnostic-adversarial.valid.json`, `20-core-error-adversarial.invalid.json`; `TST-VAL-001`. |

### `buildManifest`

| Aspecto | Especificación |
|---|---|
| Request | `BuildManifestInput` (DTO de operación; campos en `dto.md`). |
| Response | `Manifest` — `manifest.schema.json`. |
| Precondiciones | Resolución sin `conflicts` bloqueantes; `catalogRef` y `targetRef` válidos. |
| Postcondiciones | `Manifest` cerrado y ordenado; `domainLabel: archmaker:manifest:v1`; `contentDigest` semántico (ADR-0007). |
| Pureza/IO | Pura. Sin filesystem ni red. |
| Idempotencia | Idempotente: el mismo input produce los mismos bytes canónicos y el mismo `contentDigest`. |
| Cancelación y tiempo límite | Instantánea; cancelación no aplicable. Tiempo límite objetivo 5 s. |
| Límites | `effectiveSelections` ≤ 4096; `providedCapabilities`/`requiredCapabilities` ≤ 4096; `conflicts` ≤ 4096. |
| Errores | `AM-RES-002` (resolución con bloqueos), `AM-RES-001`, `AM-DOC-003`, `AM-TGT-001`. |
| Eventos | `core.operation.started`, `core.operation.completed`. |
| Redacción | Sin secretos ni rutas; digests semánticos únicamente. |
| Vectores | `contracts/json-schema/examples/05-manifest.valid.json`, `17-manifest-adversarial.valid.json`; `contracts/test-vectors/canonicalization/manifest.json`; `TST-MAN-001`. |

### `exportArtifact`

| Aspecto | Especificación |
|---|---|
| Request | `ExportArtifactInput` (DTO de operación; campos en `dto.md`). |
| Response | `Artifact` — `artifact.schema.json`. |
| Precondiciones | `manifestRef` resuelve a un manifest sellado; `targetRef.id` en `archmaker-profile`, `archinstall-profile`, `report` (DEC-001); `destinationHandle` válido del picker. |
| Postcondiciones | Bytes escritos de forma atómica; `binaryDigest` de los bytes exactos; `manifestRef.contentDigest` liga al manifest de origen. |
| Pureza/IO | I/O de escritura acotada; sin red. |
| Idempotencia | El mismo manifest y destino producen los mismos bytes y `binaryDigest` (byte-idempotente). El efecto de sobrescritura no es idempotente si el destino lo permite. |
| Cancelación y tiempo límite | Cancelable hasta antes del `fsync`; un fallo de `rename` no deja artefacto parcial. Tiempo límite objetivo 30 s. |
| Límites | Payload `artifact.content` acotado por el adaptador de target; cierre del mapa diferido (SRC-003). |
| Errores | `AM-TGT-001`, `AM-IO-001`, `AM-DOC-001`, `AM-SCHEMA-001`. |
| Eventos | `core.operation.started`, `artifact.created`, `core.operation.completed`. |
| Redacción | `destinationHandle` es un mango opaco; nunca una ruta absoluta. Sin secretos. |
| Vectores | `contracts/json-schema/examples/06-artifact.valid.json`, `06-artifact.invalid.json`; `TST-EXP-001`. |

## Matriz resumen

| Operación | Request (schema) | Response (schema) | Pura | Idempotente | I/O |
|---|---|---|---|---|---|
| `createDraft` | DTO operación | `draft.schema.json` | sí | con `draftId` | no |
| `loadCatalog` | DTO operación | `catalog.schema.json` | si `embedded` | sí | lectura `file` |
| `saveDraft` | `draft.schema.json#/$defs/saveDraftRequest` | `draft.schema.json` | no | no (conflicto) | escritura |
| `resolveDraft` | DTO operación | efímero | sí | sí | no |
| `validateDraft` | DTO operación | efímero | sí | sí | no |
| `buildManifest` | DTO operación | `manifest.schema.json` | sí | sí | no |
| `exportArtifact` | DTO operación | `artifact.schema.json` | no | byte-idempotente | escritura |

## Adaptadores

Tauri (`tauri-commands.md`, DOC-IF-TAURI-001) y WASM (`tauri-wasm.md`, DOC-IF-WASM-001) exponen la
misma interfaz. La paridad se verifica contra el corpus compartido: mismos diagnósticos, mismos
bytes de manifest y mismos digests en Rust nativo y WASM (NFR-PORT-001, NFR-DET-001).

## Trazabilidad

- Requisitos: FR-DRAFT-001, FR-CAT-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001,
  FR-EXPORT-001, NFR-DET-001, NFR-OFF-001, NFR-PORT-001, NFR-OBS-001.
- Fuera de v0: FR-IMPORT-001 y FR-PRESET-001 (`importDraft`/`applyPreset`) se reintroducen en fases
  posteriores; su interfaz no forma parte de esta congelación.
- Decisiones: DEC-001 (targets), DEC-002 (identificadores), DEC-003 (dominio agnóstico del target),
  DEC-008 (Arch x86_64).
- ADR: ADR-0001 (autoridad Rust), ADR-0002 (seguridad Tauri), ADR-0005 (JSON Schema), ADR-0006
  (ejes de versión), ADR-0007 (canonicalización).
- Amenazas: THR-SEC-001 (secretos), THR-FS-001 (traversal/overwrite), THR-IPC-001 (invocación no
  autorizada), THR-IMP-001 (bomb/nesting de import).
- Validadores y pruebas: `VAL-DOC`, `VAL-CAT`, `VAL-REF`, `VAL-RULE`, `VAL-DET`, `VAL-TARGET`;
  `TST-DRAFT-001`, `TST-CAT-001`, `TST-RES-001`, `TST-VAL-001`, `TST-MAN-001`, `TST-EXP-001`,
  `TST-PORT-001`, `TST-SEC-001`.
- Dependencias: `dto.md`, `errors-events.md`, `tauri-commands.md`, `tauri-wasm.md`, y el corpus
  `contracts/json-schema/examples/` + `contracts/test-vectors/canonicalization/`.

## Límites declarados

- Los tiempos límite y los límites por defecto son **objetivo de contrato**: no hay implementación
  del esqueleto que los materialice todavía (fase R12).
- La operación no verifica firmas de catálogo en v0; `AM-CAT-003` queda reservado.
- `artifact.content` y el cierre de su schema quedan diferidos a SRC-003.
- El corpus de contract tests por operación está **definido**, no materializado.
