---
id: DOC-IF-WASM-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
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

# Adaptadores Tauri y WASM

Ambos adaptadores implementan la **misma** interfaz `CorePort v0` (DOC-IF-CORE-001) sin semántica
divergente. El core (Rust) es la autoridad; los adaptadores solo median I/O, capacidades de host y
transporte de eventos. La paridad se verifica contra el corpus compartido.

## Contrato común de adaptador

- **Superficie**: las siete operaciones de `CorePort v0`, sin añadir, ocultar ni reordenar
  semántica.
- **DTO**: los de `dto.md` (DOC-IF-DTO-001), con los mismos nombres y reglas de unión.
- **Errores**: `CoreError` tipado (`errors-events.md`); nunca strings libres.
- **Eventos**: el catálogo de `errors-events.md`; efímeros y redactados.
- **Límites**: los mismos `limits` (`common.schema.json#/$defs/limits`) y las mismas cotas de
  schema en ambos adaptadores.
- **Redacción**: idéntica (secretos solo como `secretRef`; sin rutas absolutas).

Una operación pura (`createDraft`, `resolveDraft`, `validateDraft`, `buildManifest`) permanece pura
en ambos adaptadores; ninguno la convierte en I/O.

## Tauri (host nativo)

- Capabilities en `src-tauri/capabilities/` por ventana; deny-by-default (ADR-0002).
- Permisos explícitos por comando; scope mínimo de filesystem (`tauri-commands.md`).
- File picker para `load_catalog` y `export_artifact`; `destinationHandle` opaco.
- Escritura temporal + `fsync` + `rename` cuando el FS lo soporte.
- Sin shell, sidecar, root ni acceso global al home.
- CSP `default-src 'self'`; sin excepciones no justificadas (DEC-009).

## WASM (navegador)

- `CorePort` compatible, ejecutado en un Web Worker; no bloquea el hilo de UI.
- Resolver, validar y construir manifest corren íntegramente en WASM; paridad de bytes y digests
  con Rust nativo.
- I/O de `load_catalog` (fichero) y `export_artifact` (descarga) mediada por la plataforma del
  navegador; sin acceso arbitrario al sistema de ficheros.
- Persistencia de drafts: **en memoria** en v0. IndexedDB solo tras un ADR de persistencia; hasta
  entonces `saveDraft` no persiste más allá de la sesión.
- Límites equivalentes a escritorio; sin red en runtime.

## Paridad

Un corpus compartido debe producir los mismos `Diagnostic`, los mismos bytes canónicos de manifest y
los mismos digests en Rust nativo y WASM (NFR-DET-001, NFR-PORT-001, `ADR-0007`). El corpus es
`contracts/json-schema/examples/` más `contracts/test-vectors/canonicalization/`.

## Verificación

- Contract tests por operación contra el corpus compartido (`TST-PORT-001`).
- Los tests negativos de `tauri-commands.md` (permisos, scopes, traversal) aplican solo al host
  nativo; en WASM el equivalente es la ausencia de acceso a filesystem y red.
- Estado: **contract tests definidos, no materializados** (fase R12).

## Trazabilidad

- Autoridad: `core-port.md` (DOC-IF-CORE-001).
- Adaptador nativo: `tauri-commands.md` (DOC-IF-TAURI-001).
- Requisitos: NFR-PORT-001, NFR-DET-001, NFR-OFF-001, NFR-SEC-001.
- ADR: ADR-0001 (autoridad Rust), ADR-0002 (seguridad Tauri), ADR-0007 (canonicalización).
