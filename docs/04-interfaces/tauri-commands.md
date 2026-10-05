# Comandos Tauri del MVP

- ID: DOC-IF-TAURI-001
- Estado: draft
- Propietario: Arquitectura de interfaces / Seguridad
- Última revisión: 2026-10-05
- Requisitos relacionados: NFR-SEC-001, NFR-OFF-001, FR-DRAFT-001, FR-CAT-001, FR-IMPORT-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001, FR-EXPORT-001
- Sustituye / sustituido por: —

## Alcance

Superficie de invocación entre el WebView y el host Tauri en el MVP. Los tipos de entrada y salida
se enumeran en `dto.md` (`DOC-IF-DTO-001`). La política general está en `tauri-policy.md` y el
modelo de privilegios en `privilege-model.md`. Este documento es `draft`; nada aquí es `accepted`.

> **Nota de verificación.** Los identificadores de permisos, capabilities, plugins y directivas
> exactas de Tauri 2 **no están confirmados** por fuente primaria capturada: **no verificado —
> fuente primaria pendiente (SRC-004)**. Los nombres de permiso de esta tabla son **lógicos
> internos** de ArchMaker y deben mapearse a permissions Tauri tras capturar `SRC-004`. Las rutas
> de scope son propuestas sujetas a ratificación.

## Comandos del MVP

El conjunto se deriva de las acciones `CorePort` de las rutas MVP de `interaction-matrix.md`
(`/`, `/new`, `/configure/:step`, `/review`, `/export`, `/import`); `/runner/plan` es v1 y queda
fuera. Nomenclatura de comando propuesta en `snake_case`.

| Comando | Propósito | Método `CorePort` | Permiso requerido | Entradas (DTO) | Salidas (DTO) | Efectos de filesystem | Red |
|---|---|---|---|---|---|---|---|
| `product_info` | Metadatos de producto y versiones soportadas. | `productInfo` | `PERM-CMD-PRODUCT-INFO` | — | `ProductInfo` | Ninguno | No |
| `create_draft` | Crear un Draft nuevo ligado a catálogo y target. | `createDraft` | `PERM-CMD-CREATE-DRAFT` | `CreateDraftInput` | `Draft` | Escritura en el directorio de drafts (atómica) si persiste | No |
| `import_draft` | Importar un documento local; planificar y aplicar la migración. | `importDraft` (auxiliares `planMigration`, `applyMigration`) | `PERM-CMD-IMPORT-DRAFT` | `ImportDraftInput` | `ImportResult` | Lectura del fichero elegido en el picker; el original no se sobrescribe | No |
| `resolve_draft` | Resolver capacidades y conflictos. | `resolveDraft` | `PERM-CMD-RESOLVE-DRAFT` | `ResolveDraftInput` | `ResolveResult` | Ninguno | No |
| `validate_draft` | Ejecutar el pipeline de validación. | `validateDraft` | `PERM-CMD-VALIDATE-DRAFT` | `ValidateDraftInput` | `ValidationResult` | Ninguno | No |
| `build_manifest` | Construir el manifest canónico. | `buildManifest` | `PERM-CMD-BUILD-MANIFEST` | `BuildManifestInput` | `ManifestResult` | Ninguno (persistencia opcional en directorio de la app) | No |
| `check_target` | Comprobar compatibilidad con un target. | `checkTarget` | `PERM-CMD-CHECK-TARGET` | `CheckTargetInput` | `TargetCompatibility` | Ninguno | No |
| `export_artifact` | Exportar un artifact a un destino elegido. | `exportArtifact` | `PERM-CMD-EXPORT-ARTIFACT` | `ExportArtifactInput` | `ExportResult` | Escritura en destino del picker (temporal + `fsync` + `rename`) | No |

**Red**: todos `No`. El MVP debe ser usable sin red (`NFR-OFF-001`, `DEC-009`, `privilege-model.md`).

### Cobertura de `CorePort` (resolución propuesta)

Métodos sin comando propio: `capabilities`, `loadCatalog`, `applyPreset`, `compareDrafts`,
`planMigration`, `applyMigration`, `listExportTargets`. **Resolución propuesta** (sujeta a
ratificación humana; no es contrato `accepted`):

| Método `CorePort` | Resolución propuesta | Motivo |
|---|---|---|
| `capabilities` | Expuesto en `product_info` | Metadatos de capacidades del core junto a versión. |
| `loadCatalog` | Llamada interna de `create_draft` / `resolve_draft` | El catálogo embebido se carga antes de resolver; no requiere comando de ventana propio en MVP. |
| `applyPreset` | Llamada interna de `resolve_draft` | Aplicar un preset es precondición de resolución, no comando de ventana. |
| `compareDrafts` | Llamada interna de `import_draft` y del revisor | Se usa para mostrar el ChangeSet; no cruza la frontera como comando propio en MVP. |
| `planMigration` | Llamada interna de `import_draft` | La UI de importación muestra el plan antes de aplicar. |
| `applyMigration` | Llamada interna de `import_draft` | La migración se aplica dentro del flujo de importación. |
| `listExportTargets` | Expuesto en `export_artifact` (paso previo) | La lista de targets se obtiene al abrir el exportador; sin comando separado. |

Con esta agrupación el MVP expone **8 comandos** y cubre los 15 métodos `CorePort` (7 como llamadas
internas). La alternativa «un comando por método» queda registrada como opción no elegida.

## Capabilities por ventana

- MVP: una sola ventana `main` con los ocho comandos de la tabla.
- **Deny-by-default**: ninguna ventana recibe permisos implícitos; `main` declara solo los comandos
  que invoca (`tauri-policy.md`, `ADR-0002`).
- Ventanas futuras (no MVP) tendrán su propia capability separada; p. ej. una ventana de
  plan/runner (v1) que **no** incluya `export_artifact` ni escritura fuera de su scope.
- Sin `shell:*`, sidecars, root ni discos en el MVP (`AGENTS.md`, `tauri-policy.md`).
- Una capability por ventana y plataforma; los ficheros de capability propuestos viven en
  `src-tauri/capabilities/` (`tauri-wasm.md`). Estructura exacta del fichero: **no verificado —
  fuente primaria pendiente (SRC-004)**.

## Modelo de permisos allow/deny

- Cada comando expone un permiso custom con `allow`/`deny` explícitos; el `deny` prevalece.
- Un permiso se concede únicamente al comando concreto, nunca de forma global por plugin.
- Scopes mínimos por permiso; cada comando recibe el scope más estrecho que necesite.
- Los identificadores `PERM-CMD-*` son lógicos. El nombre real de la permission/capability Tauri es
  **no verificado — fuente primaria pendiente (SRC-004)**.

### Scopes propuestos

| Ámbito | Acceso | Uso |
|---|---|---|
| Directorio de drafts de la app | lectura/escritura | `create_draft`, persistencia de borradores. |
| Fichero o directorio elegido en el picker | lectura/escritura | `import_draft` (lectura), `export_artifact` (escritura). |
| Directorio temporal de la app | lectura/escritura | Escritura atómica previa al `rename`. |

### Rutas sensibles denegadas (mínimo)

`$HOME` global, `~/.ssh/**`, `~/.gnupg/**`, `~/.config/**` (salvo el directorio de la app),
`/etc/**`, `/root/**`, `/proc/**`, `/sys/**`, `/dev/**`, dispositivos de bloque, llaveros de
claves y perfiles de navegador. Sin acceso global al home (`tauri-wasm.md`) ni a discos
(`privilege-model.md`). Lista sujeta a ratificación.

## Efectos de filesystem y atomicidad

- Lecturas solo de catálogo embebido, del picker o del directorio de la app.
- Escrituras siempre temporales + `fsync` + `rename` cuando el FS lo soporte (`tauri-wasm.md`).
- `destinationHandle` es un mango opaco del picker; nunca se acepta una ruta arbitraria
  (`dto.md`, `THR-FS-001`).
- Un overwrite solo ocurre si el usuario lo selecciona explícitamente (`THR-FS-001`).
- `import_draft` conserva el original intacto (`FR-IMPORT-001`).

## Pruebas: test negativo obligatorio

Toda capability y comando tiene test negativo (`tauri-policy.md`, `ADR-0002`, `NFR-SEC-001`).
Pruebas específicas por comando:

| Comando | Test negativo obligatorio |
|---|---|
| `product_info` | Una ventana sin `PERM-CMD-PRODUCT-INFO` no puede invocarlo. |
| `create_draft` | Escritura fuera del scope de drafts denegada; sin rutas arbitrarias. |
| `import_draft` | Scope deniega rutas no elegidas; `..` y symlinks no escapan (`THR-FS-001`); límites de tamaño/profundidad (`THR-IMP-001`). |
| `resolve_draft` | Invocación sin permiso denegada; verificar que no toca FS ni red. |
| `validate_draft` | Invocación sin permiso denegada; sin FS ni red. |
| `build_manifest` | Invocación sin permiso denegada; sin escritura fuera del directorio de la app. |
| `check_target` | Invocación sin permiso denegada; sin FS ni red. |
| `export_artifact` | Escritura fuera del destino elegido denegada; `..`/symlink no escapan (`THR-FS-001`); fallo de `rename` no deja artefacto parcial. |

Complementos transversales: ventana no autorizada no invoca ningún comando; carga de recursos
remotos bloqueada por CSP (`default-src 'self'`); ausencia de red en todos los comandos. Se apoyan
en la fila `Tauri/WASM` de `test-matrix.md` (unit, property, golden, contract, E2E, fault).

## Trazabilidad

| Referencia | Relación |
|---|---|
| `THR-IPC-001` (invocación Tauri no autorizada) | Mitigado por capabilities, permissions y scopes. |
| `THR-FS-001` (traversal u overwrite) | Mitigado por picker, canonical path y escritura atómica. |
| `NFR-SEC-001` (deny-by-default y mínimo privilegio) | Evidencia: capabilities, scopes y tests negativos. |
| `THR-IMP-001` (bomb/nesting de import) | Límites en `import_draft` antes y durante parse. |
| `NFR-OFF-001` (MVP sin red) | Ningún comando usa red. |

## Decisiones pendientes (humanas)

1. Ratificar (o modificar) la **resolución propuesta** de cobertura `CorePort` de la sección
   anterior (`capabilities`, `loadCatalog`, `applyPreset`, `compareDrafts`, `planMigration`,
   `applyMigration`, `listExportTargets`).
2. Ratificar el esquema de identificadores de permiso y el mapeo a permissions Tauri tras capturar
   `SRC-004`.
3. Ratificar los ámbitos de scope y la lista de rutas sensibles denegadas.
