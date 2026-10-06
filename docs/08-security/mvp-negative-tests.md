---
id: DOC-SEC-NEG-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
---

# Controles Tauri y pruebas negativas del MVP

Fase R9 (issue `AUD-018`). Diseña los controles Tauri del MVP y las pruebas negativas que los
verifican. La política general se lee en `docs/08-security/tauri-policy.md`; los comandos y scopes
en `docs/04-interfaces/tauri-commands.md`. Amenazas asociadas: `THR-IPC-001` (invocación Tauri no
autorizada), `THR-FS-001` (traversal u overwrite) y `THR-IMP-001` (bomb/nesting de import).
**Diseño, no implementación**: los identificadores de permisos/capabilities de Tauri 2 son lógicos
internos y su mapeo a Tauri queda pendiente de fuente primaria (`SRC-004`).

## Controles del MVP

### Capability por ventana

- **Deny-by-default**: ninguna ventana recibe permisos implícitos.
- La ventana `main` declara solo los comandos que invoca; cada ventana futura tendrá su propia
  capability.
- Una capability por ventana y plataforma; sin capabilities globales por plugin.

### Scope de filesystem mínimo

- Lecturas: catálogo embebido, fichero del picker o directorio de la app.
- Escrituras: directorio de drafts, destino del picker y temporal de la app (temporal + `fsync` +
  `rename`).
- Rutas sensibles denegadas: `$HOME` global, `~/.ssh/**`, `~/.gnupg/**`, `~/.config/**` (salvo la
  app), `/etc/**`, `/root/**`, `/proc/**`, `/sys/**`, `/dev/**`, dispositivos de bloque, llaveros y
  perfiles de navegador.
- `destinationHandle` es un mango opaco; nunca se acepta una ruta arbitraria.

### Sin plugin shell en MVP

- Sin `shell:*`, sin sidecars, sin root y sin discos (`AGENTS.md`, `privilege-model.md`).
- Ninguna operación del MVP ejecuta un string heredado `cmd`, hooks, `pacstrap` ni shell.

### CSP restrictiva

- `default-src 'self'`; sin CDN en producción (`DEC-009`).
- Sin `unsafe-eval`/`unsafe-inline` innecesarios; recursos empaquetados o del sistema.
- Devtools desactivadas en release salvo build de diagnóstico controlada.

### Sin contenido remoto en WebView

- El WebView no carga contenido remoto; la navegación externa usa allowlist y confirmación.
- Un recurso ausente localmente no se descarga en silencio (`NFR-OFF-001`).

### Updater aislado por capability

- El updater tiene capability propia; ninguna otra ventana lo alcanza.
- La comprobación es una acción del host; el WebView no invoca el updater directamente.
- Ver `docs/08-security/updater-threat-model.md` (`DOC-SEC-UPD-001`).

## Pruebas negativas por comando

| Comando | Prueba negativa obligatoria |
|---|---|
| `product_info` | Ventana sin `PERM-CMD-PRODUCT-INFO` no puede invocarlo. |
| `create_draft` | Escritura fuera del scope de drafts denegada; sin rutas arbitrarias. |
| `import_draft` | Scope deniega rutas no elegidas; `..` y symlinks no escapan; límites de tamaño/profundidad. |
| `resolve_draft` | Invocación sin permiso denegada; sin FS ni red. |
| `validate_draft` | Invocación sin permiso denegada; sin FS ni red. |
| `build_manifest` | Invocación sin permiso denegada; sin escritura fuera del directorio de la app. |
| `check_target` | Invocación sin permiso denegada; sin FS ni red. |
| `export_artifact` | Escritura fuera del destino elegido denegada; `..`/symlink no escapan; `rename` fallido no deja artefacto parcial. |

## Pruebas negativas de rutas y comandos (diseño)

| ID | Given | When | Then |
|---|---|---|---|
| TST-NEG-001 | Una ventana no autorizada | Intenta invocar cualquier comando | Denegado (capability ausente). |
| TST-NEG-002 | Ruta con `..` | Se pasa al scope de FS | Rechazada; no escapa del scope. |
| TST-NEG-003 | Symlink que apunta fuera del scope | Se resuelve la ruta | Rechazada tras canonicalización. |
| TST-NEG-004 | Comando shell / string heredado | Se intenta ejecutar | No existe superficie; rechazado por diseño. |
| TST-NEG-005 | Recurso remoto en WebView | Se intenta cargar | Bloqueado por CSP. |
| TST-NEG-006 | Import con nesting/profundidad excesivos | Se parsea | Abortado por límites (`THR-IMP-001`). |
| TST-NEG-007 | Cualquier comando | Se ejecuta en entorno sin red | No hay llamada de red (`NFR-OFF-001`). |
| TST-NEG-008 | Escritura con `rename` fallido | Se exporta | No queda artefacto parcial. |

Estas pruebas se apoyan en la fila `Tauri/WASM` de `docs/09-quality/test-matrix.md` y quedan
**diseñadas**, no implementadas. Su evidencia de implementación es condición de G8.

## Trazabilidad

| Referencia | Relación |
|---|---|
| `THR-IPC-001` / `THR-FS-001` / `THR-IMP-001` | Amenazas mitigadas por estos controles. |
| `RSK-001` / `RSK-002` / `RSK-008` | Riesgos con verificación pendiente; ver `DOC-SEC-RISK-001`. |
| `DOC-SEC-TAURI-001` | Política Tauri general. |
| `DOC-IF-TAURI-001` | Comandos, permisos y scopes. |
| `DEC-009` | Sin CDN en runtime; CSP `self`. |
| `G8` | Gate de seguridad; exige pruebas negativas diseñadas. |

## Límites declarados

- **Diseño, no implementación**: no se escriben capabilities, `tauri.conf.json` ni comandos.
- Los identificadores de permisos son lógicos; el mapeo a Tauri 2 requiere `SRC-004`.
- No modifica `reference/**` ni los contratos de `ADR-0007`/R5.
