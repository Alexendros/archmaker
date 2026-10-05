---
id: DOC-IF-WASM-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Adaptadores Tauri y WASM

## Tauri MVP

- Capabilities en `src-tauri/capabilities/` por ventana.
- Permisos explícitos para comandos; scope limitado para filesystem.
- File picker para importar/exportar.
- Escritura temporal + fsync + rename cuando lo soporte el FS.
- Sin shell, sidecar, root ni acceso global al home.
- CSP `default-src 'self'`; excepciones justificadas.

## WASM

- `CorePort` compatible.
- Web Worker para resolver, validar, migrar y exportar.
- No bloquear el hilo de UI.
- Límites equivalentes a desktop.
- IndexedDB solo tras ADR de persistencia; downloads para artifacts.

## Paridad

Un corpus compartido debe producir los mismos diagnostics, manifest bytes canónicos y digests en Rust nativo y WASM.
