---
id: DOC-IF-TAURI-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
  - security
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-IF-CORE-001
    expectedDocumentStatus: accepted
    expectedApprovalStatus: approved
---

# Comandos Tauri del MVP

Adaptador Tauri de `CorePort v0` (DOC-IF-CORE-001). Superficie de invocación entre el WebView y el
host Tauri. Los DTO están en `dto.md` (DOC-IF-DTO-001); la política general en
`docs/08-security/tauri-policy.md` y el modelo de privilegios en
`docs/08-security/privilege-model.md`. Este documento es `in-review`: nada aquí es `accepted`.

> **Nota de verificación.** Los identificadores de permisos, capabilities, plugins y directivas
> exactas de Tauri 2 **no están confirmados** por fuente primaria capturada: **no verificado —
> fuente primaria pendiente (SRC-004)**. Los nombres `PERM-CMD-*` son **lógicos internos** y deben
> mapearse a permissions Tauri tras capturar `SRC-004`. Los scopes son propuestas sujetas a
> ratificación.

## Comandos v0

El conjunto se corresponde **1:1** con las siete operaciones de `CorePort v0`; no hay comandos
ocultos ni métodos internos. Nomenclatura de comando en `snake_case`.

| Comando | Operación `CorePort` | Permiso requerido | Entrada (DTO) | Salida (DTO) | I/O | Red |
|---|---|---|---|---|---|---|
| `create_draft` | `createDraft` | `PERM-CMD-CREATE-DRAFT` | `CreateDraftInput` | `Draft` | Ninguna | No |
| `load_catalog` | `loadCatalog` | `PERM-CMD-LOAD-CATALOG` | `LoadCatalogInput` | `Catalog` | Lectura del catálogo empaquetado o del fichero elegido en el picker | No |
| `save_draft` | `saveDraft` | `PERM-CMD-SAVE-DRAFT` | `SaveDraftInput` | `Draft` | Escritura atómica en el directorio de drafts | No |
| `resolve_draft` | `resolveDraft` | `PERM-CMD-RESOLVE-DRAFT` | `ResolveDraftInput` | `ResolveResult` | Ninguna | No |
| `validate_draft` | `validateDraft` | `PERM-CMD-VALIDATE-DRAFT` | `ValidateDraftInput` | `ValidationResult` | Ninguna | No |
| `build_manifest` | `buildManifest` | `PERM-CMD-BUILD-MANIFEST` | `BuildManifestInput` | `Manifest` | Ninguna | No |
| `export_artifact` | `exportArtifact` | `PERM-CMD-EXPORT-ARTIFACT` | `ExportArtifactInput` | `Artifact` | Escritura atómica en el destino del picker | No |

**Red**: todos `No`. El MVP es usable sin red (NFR-OFF-001, DEC-009).

**Fuera de v0**: `import_draft`, `product_info` y `check_target` no forman parte de `CorePort v0` y
se reintroducen en fases posteriores. La versión congelada no expone métodos internos que cubran
operaciones ausentes.

## Capabilities por ventana

- MVP: una sola ventana `main` con los siete comandos de la tabla.
- **Deny-by-default**: ninguna ventana recibe permisos implícitos; `main` declara solo los comandos
  que invoca (ADR-0002).
- Ventanas futuras (no MVP) tendrán su propia capability separada; la ventana de plan/runner (v1)
  **no** incluirá `export_artifact` ni escritura fuera de su scope.
- Sin `shell:*`, sidecars, root ni discos en el MVP.
- Una capability por ventana y plataforma; los ficheros propuestos viven en
  `src-tauri/capabilities/`. Estructura exacta del fichero: **no verificado (SRC-004)**.

## Modelo de permisos allow/deny

- Cada comando expone un permiso custom con `allow`/`deny` explícitos; el `deny` prevalece.
- Un permiso se concede únicamente al comando concreto, nunca de forma global por plugin.
- Scopes mínimos por permiso; cada comando recibe el scope más estrecho que necesite.
- Los identificadores `PERM-CMD-*` son lógicos; el nombre real de la permission/capability Tauri es
  **no verificado (SRC-004)**.

### Scopes propuestos

| Ámbito | Acceso | Uso |
|---|---|---|
| Directorio de drafts de la app | lectura/escritura | `save_draft`. |
| Fichero o directorio elegido en el picker | lectura/escritura | `load_catalog` (lectura), `export_artifact` (escritura). |
| Directorio temporal de la app | lectura/escritura | Escritura atómica previa al `rename`. |

### Rutas sensibles denegadas (mínimo)

`$HOME` global, `~/.ssh/**`, `~/.gnupg/**`, `~/.config/**` (salvo el directorio de la app),
`/etc/**`, `/root/**`, `/proc/**`, `/sys/**`, `/dev/**`, dispositivos de bloque, llaveros de claves
y perfiles de navegador. Sin acceso global al home ni a discos. Lista sujeta a ratificación.

## Efectos de filesystem y atomicidad

- Lecturas solo del catálogo empaquetado, del picker o del directorio de la app.
- Escrituras siempre temporales + `fsync` + `rename` cuando el FS lo soporte.
- `destinationHandle` es un mango opaco del picker; nunca se acepta una ruta arbitraria
  (`dto.md`, `THR-FS-001`).
- Un overwrite solo ocurre si el usuario lo selecciona explícitamente (`THR-FS-001`).
- Las operaciones puras (`create_draft`, `resolve_draft`, `validate_draft`, `build_manifest`) no
  tocan el filesystem ni la red.

## Paridad con WASM

`resolve_draft`, `validate_draft`, `build_manifest` y `create_draft` producen los mismos
diagnósticos, bytes de manifest y digests que el adaptador WASM (`tauri-wasm.md`, NFR-PORT-001). El
corpus compartido es la evidencia de paridad.

## Pruebas: test negativo obligatorio

Toda capability y comando tiene test negativo (ADR-0002, NFR-SEC-001).

| Comando | Test negativo obligatorio |
|---|---|
| `create_draft` | Invocación sin permiso denegada; sin FS ni red. |
| `load_catalog` | Scope deniega rutas no elegidas; `..` y symlinks no escapan (`THR-FS-001`); límites de tamaño/profundidad (`THR-IMP-001`). |
| `save_draft` | Escritura fuera del scope de drafts denegada; conflicto de revisión ⇒ `AM-DOC-002`, sin sobrescritura silenciosa. |
| `resolve_draft` | Invocación sin permiso denegada; verificar que no toca FS ni red. |
| `validate_draft` | Invocación sin permiso denegada; sin FS ni red. |
| `build_manifest` | Invocación sin permiso denegada; sin FS ni red. |
| `export_artifact` | Escritura fuera del destino elegido denegada; `..`/symlink no escapan (`THR-FS-001`); fallo de `rename` no deja artefacto parcial. |

Complementos transversales: ventana no autorizada no invoca ningún comando; carga de recursos
remotos bloqueada por CSP (`default-src 'self'`); ausencia de red en todos los comandos; errores
públicos tipados (`AM-PROTO-001`/`AM-PROTO-002` para invocaciones no soportadas).

## Trazabilidad

| Referencia | Relación |
|---|---|
| `THR-IPC-001` (invocación Tauri no autorizada) | Mitigado por capabilities, permissions y scopes. |
| `THR-FS-001` (traversal u overwrite) | Mitigado por picker, canonical path y escritura atómica. |
| `NFR-SEC-001` (deny-by-default y mínimo privilegio) | Evidencia: capabilities, scopes y tests negativos. |
| `THR-IMP-001` (bomb/nesting de import) | Límites de `load_catalog` antes y durante el parseo. |
| `NFR-OFF-001` (MVP sin red) | Ningún comando usa red. |
| `TST-SEC-001`, `TST-SEC-002`, `TST-PORT-001` | Pruebas de seguridad y paridad de adaptadores. |

## Decisiones pendientes (humanas)

1. Ratificar el esquema de identificadores de permiso y el mapeo a permissions Tauri tras capturar
   `SRC-004`.
2. Ratificar los ámbitos de scope y la lista de rutas sensibles denegadas.
