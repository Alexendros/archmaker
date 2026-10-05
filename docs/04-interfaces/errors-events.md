---
id: DOC-IF-ERR-001
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
---

# Errores, diagnósticos y eventos

Catálogo único de errores tipados y eventos de `CorePort v0` (DOC-IF-CORE-001). Los códigos `AM-*`
y las claves `messageKey` son **estables**: no se renombran ni se reutilizan; un código retirado se
marca `deprecated`. Los códigos usados por las siete operaciones del esqueleto quedan **congelados**
en v0; la ampliación futura es aditiva.

## CoreError

Forma canónica en `https://archmaker.dev/schemas/core-error.schema.json`. Campos obligatorios:
`code`, `family`, `category`, `severity`, `messageKey`, `source`. El error público **nunca** es un
string libre.

| Campo | Tipo | `$defs` | Notas |
|---|---|---|---|
| `schemaVersion` | schemaVersion | `common#/$defs/schemaVersion` | Eje de contrato. |
| `code` | string | `core-error#/$defs/code` | `AM-<FAMILIA>-<NNN>`. |
| `family` | enum | `core-error#/$defs/family` | Prefijo de familia. |
| `category` | enum | `core-error#/$defs/category` | Categoría gruesa. |
| `severity` | enum | `common#/$defs/severity` | `error \| warning \| info`. |
| `retryable` | boolean | — | Reintentable sin cambio. |
| `messageKey` | messageKey | `common#/$defs/messageKey` | Clave estable, localizable. |
| `params` | map<string,scalar> | — | Mapa abierto documentado; solo escalares. |
| `detail` | Detail | `core-error#/$defs/detail` | Detalle estructurado; sin prosa libre. |
| `cause` | CoreError \| null | recursivo | Cadena de causas. |
| `causeCode` | code \| null | `core-error#/$defs/code` | Causa inmediata sin objeto. |
| `remedy` | Remedy | `core-error#/$defs/remedy` | `kind` + `messageKey`. |
| `source` | string | — | Componente emisor. |
| `correlationId` | uuid | `common#/$defs/uuid` | Observabilidad. |
| `occurredAt` | utcTimestamp | `common#/$defs/utcTimestamp` | Efímero; excluido del hash. |

Ejemplo:

```json
{
  "code": "AM-DOC-002",
  "family": "AM-DOC",
  "category": "conflict",
  "severity": "error",
  "retryable": true,
  "messageKey": "error.draft.revisionConflict",
  "source": "archmaker-store",
  "causeCode": null,
  "remedy": { "kind": "retry", "messageKey": "remedy.refreshRevision" }
}
```

## Diagnostic

Forma canónica en `https://archmaker.dev/schemas/diagnostic.schema.json`. Campos obligatorios:
`code`, `severity`, `blocking`, `messageKey`, `source`. `blocking` es **canónico**: `error` ⇒ `true`;
`warning`/`info` ⇒ `false`.

Ejemplo:

```json
{
  "code": "AM-RES-004",
  "severity": "error",
  "blocking": true,
  "path": "/selections/environment.desktop",
  "messageKey": "diagnostic.capabilityMissing",
  "params": {},
  "source": "archmaker-resolver",
  "suggestions": []
}
```

### Severidad de diagnóstico

| Valor | Uso | MVP |
|---|---|---|
| `error` | fallo que impide continuar | sí |
| `warning` | problema no bloqueante | sí |
| `info` | informativo / cambio aplicado | sí |
| `fatal` | fallo irrecuperable de proceso | reservado |
| `debug` | traza de desarrollo | reservado |

`fatal` y `debug` quedan **reservados** (no los emite el MVP).

## Familias

`AM-DOC`, `AM-SCHEMA`, `AM-CAT`, `AM-MIG`, `AM-RULE`, `AM-RES`, `AM-TGT`, `AM-IO`, `AM-PROTO`,
`AM-RUN`, `AM-POL`, `AM-AUTH` (enum de `core-error#/$defs/family`).

## Catálogo canónico de códigos v0 (fuente única)

Convención `AM-<FAMILIA>-<NNN>`. La asignación es estable y aditiva. Los códigos de la columna
`Operaciones` son la primera asignación congelada por `CorePort v0`; los de la columna `Fase` son
reservados para fases posteriores y se ampliarán de forma aditiva.

| Código | Familia | Categoría | Severidad | `messageKey` | Bloqueante | Reintentable | Remedy | Operaciones / Fase |
|---|---|---|---|---|---|---|---|---|
| `AM-DOC-001` | AM-DOC | validation | error | `error.document.limitExceeded` | sí | no | fix-input | límites/parseo (`loadCatalog`, `createDraft`, `saveDraft`, `validateDraft`, `exportArtifact`) |
| `AM-DOC-002` | AM-DOC | conflict | error | `error.draft.revisionConflict` | sí | sí | retry | `saveDraft` |
| `AM-DOC-003` | AM-DOC | canonicalization | error | `error.canonicalization.rejected` | sí | no | fix-input | `loadCatalog`, `saveDraft`, `buildManifest` |
| `AM-SCHEMA-001` | AM-SCHEMA | validation | error | `error.schema.invalidDocument` | sí | no | fix-input | todas |
| `AM-CAT-001` | AM-CAT | catalog | error | `error.catalog.notFound` | sí | no | reload-catalog | `loadCatalog`, `createDraft`, `resolveDraft`, `validateDraft` |
| `AM-CAT-002` | AM-CAT | catalog | error | `error.catalog.digestMismatch` | sí | no | reload-catalog | `loadCatalog` |
| `AM-CAT-003` | AM-CAT | catalog | error | `error.catalog.signatureInvalid` | sí | no | report | reservado (firma, no v0) |
| `AM-RES-001` | AM-RES | resolution | error | `error.resolution.valueUnresolved` | sí | no | fix-input | `resolveDraft`, `buildManifest` |
| `AM-RES-002` | AM-RES | resolution | error | `error.resolution.blocked` | sí | no | fix-input | `buildManifest` |
| `AM-RES-004` | AM-RES | resolution | error | `diagnostic.capabilityMissing` | sí | no | fix-input | `resolveDraft` |
| `AM-RULE-001` | AM-RULE | rule | error | `diagnostic.compositorCardinality` | sí | no | fix-input | RULE-COMP-001 |
| `AM-RULE-002` | AM-RULE | rule | info | `diagnostic.dmDerived` | no | no | report | RULE-DM-001 |
| `AM-RULE-003` | AM-RULE | rule | error | `diagnostic.gpuCompositorConflict` | sí | no | fix-input | RULE-GPU-001 |
| `AM-RULE-004` | AM-RULE | rule | error | `diagnostic.kernelCardinality` | sí | no | fix-input | RULE-KERNEL-001 |
| `AM-RULE-005` | AM-RULE | rule | warning | `diagnostic.browserMissing` | no | no | report | RULE-BROWSER-001 |
| `AM-RULE-006` | AM-RULE | rule | warning | `diagnostic.packageUnverified` | no | no | report | RULE-PKG-001 |
| `AM-RULE-007` | AM-RULE | rule | error | `diagnostic.kernelFilesystemConflict` | sí | no | fix-input | RULE-KERNEL-002 |
| `AM-RULE-008` | AM-RULE | rule | error | `diagnostic.duplicateIdentity` | sí | no | fix-input | RULE-DUP-001 |
| `AM-RULE-009` | AM-RULE | rule | error | `diagnostic.operatorUnsupported` | sí | no | report | operator-corpus |
| `AM-TGT-001` | AM-TGT | target | error | `error.target.unsupported` | sí | no | fix-input | `createDraft`, `buildManifest`, `exportArtifact` |
| `AM-IO-001` | AM-IO | io | error | `error.io.failed` | sí | sí | retry | `loadCatalog`, `saveDraft`, `exportArtifact` |
| `AM-PROTO-001` | AM-PROTO | protocol | error | `error.protocol.invalidMessage` | sí | no | report | adaptadores (`tauri-commands.md`, `tauri-wasm.md`) |
| `AM-PROTO-002` | AM-PROTO | protocol | error | `error.protocol.unsupportedOperation` | sí | no | report | adaptadores |
| `AM-MIG-001` | AM-MIG | migration | error | `error.migration.unsupported` | sí | no | migrate | fase posterior (`importDraft`) |
| `AM-RUN-001` | AM-RUN | protocol | error | `error.run.rejected` | sí | no | report | v1 runner |
| `AM-POL-001` | AM-POL | policy | error | `error.policy.violation` | sí | no | contact-support | Enterprise |
| `AM-AUTH-001` | AM-AUTH | auth | error | `error.auth.failed` | sí | no | contact-support | Enterprise/runner |

Ninguna familia queda sin al menos un código asignado. Los códigos reservados (`AM-CAT-003`,
`AM-MIG-001`, `AM-RUN-001`, `AM-POL-001`, `AM-AUTH-001`) no los emite `CorePort v0`.

## Eventos

Eventos **efímeros**: se excluyen del payload canónico, nunca se persisten como verdad y se
**redactan**. Se emiten por el adaptador (eventos Tauri o mensajes del Web Worker WASM). Todo evento
lleva `correlationId` (uuid) y, cuando aplica, `requestId`.

| Evento | Cuándo | Campos estables |
|---|---|---|
| `core.operation.started` | al iniciar una operación | `operation`, `requestId?` |
| `core.operation.progress` | avance de operación larga (I/O) | `operation`, `requestId?`, `step` |
| `core.operation.completed` | al terminar con éxito | `operation`, `requestId?` |
| `catalog.loaded` | catálogo cargado y verificado | `catalogRef` |
| `draft.changed` | draft creado o guardado | `draftId`, `revision` |
| `resolution.changed` | resolución recalculada | `draftId`, `resolutionDigest` |
| `validation.completed` | pipeline de validación terminado | `draftId`, `blocking`, `diagnosticCount` |
| `artifact.created` | artifact exportado | `artifactRef`, `targetRef` |

v1 añade `runner.preflight.*`, `plan.*`, `execution.*`, `journal.*`; quedan fuera de `CorePort v0`.

## Redacción

- **Secretos**: solo cruza `secretRef` (`common#/$defs/secretRef`); el valor del secreto nunca se
  serializa ni aparece en errores, diagnósticos o eventos (`THR-SEC-001`).
- **Rutas**: nunca rutas absolutas, descriptores de fichero ni handles de sistema. Los campos
  efímeros (`sourcePath`, `localPath`, `absolutePath`, `workingDirectory`) se excluyen del payload
  canónico (`common#/$defs/ephemeralFieldName`) y no cruzan la frontera. Las referencias externas
  son mangos opacos (`destinationHandle`).
- **Forma**: `params` admite solo escalares (`string`, `number`, `boolean`); `messageKey` es estable;
  no se admiten campos de prosa libre (`detail` es estructurado). Esto sostiene NFR-OBS-001.
- **Correlación**: `correlationId` y `requestId` son uuids; no transportan datos del usuario.

## Trazabilidad

- Autoridad de operaciones: `core-port.md` (DOC-IF-CORE-001); formas DTO en `dto.md`
  (DOC-IF-DTO-001).
- Schemas: `https://archmaker.dev/schemas/core-error.schema.json` y
  `https://archmaker.dev/schemas/diagnostic.schema.json`.
- Requisitos: NFR-OBS-001, NFR-DET-001, FR-VALIDATE-001.
- Amenazas: THR-SEC-001 (secretos). Decisiones: DEC-010 (sin telemetría; solo eventos locales
  redactados).
