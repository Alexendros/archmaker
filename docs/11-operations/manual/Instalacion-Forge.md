---
id: DOC-OPS-MAN-INSTALL-FORGE
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# Instalación de Forge (GUI de ArchMaker)

Forge es la interfaz gráfica de ArchMaker. Este manual describe cómo obtener y abrir
el configurador; **no** instala Arch Linux.

## Canal publicado: ninguno todavía

No hay GitHub Release ni AppImage descargable. Los tags `v0.1.0` y `v0.2.0` existen,
pero `/releases` de la API devuelve `[]` (corregido el 2026-10-25). El intento de
`v0.1.0` terminó _cancelled_ y el de `v0.2.0` terminó _success_ sin publicar. `v0.1.0`
se declara **no publicado** y no se intenta recuperar; la primera release real será
`v0.3.0`. Detalle:
[`docs/10-delivery/closure/f9-release.md`](../../10-delivery/closure/f9-release.md).

## Tip de `main` (desarrollo)

- Escritorio: `cargo build -p archmaker` / flujo Tauri del repo (capabilities deny-by-default).
- Navegador: build WASM (`tools/build-wasm.sh`) + `apps/web` (Mock solo con `?mock=1`).
- El gate desktop/offline del tip puede estar **blocked** sin binario de producción del
  `SOURCE_SHA`: ver [`p6-desktop-offline.md`](../../10-delivery/closure/p6-desktop-offline.md).

## Qué no hace

No abre runner, root, discos ni ISO. No sustituye el manual de uso del recorrido Forge.
