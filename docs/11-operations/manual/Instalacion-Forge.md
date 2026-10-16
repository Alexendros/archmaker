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

## Canal publicado (`v0.1.0`)

1. Descarga el AppImage desde
   [releases/tag/v0.1.0](https://github.com/Alexendros/archmaker/releases/tag/v0.1.0).
2. El nombre del artefacto histórico es `Neubat_0.1.0_amd64.AppImage` (ADR-0010).
   El nombre visible vigente es **Forge** (ADR-0012); no se reescribe el asset.
3. Verifica `SHA256SUMS` (`sha256sum --check`).
4. Marca ejecutable y arranca: `chmod +x … && ./Neubat_0.1.0_amd64.AppImage`.

Limitaciones de esa release: verificación parcial (sin cosign local obligatorio en este
manual; sin tres instalaciones GUI independientes documentadas aquí). Detalle:
[`docs/10-delivery/closure/f9-release.md`](../../10-delivery/closure/f9-release.md).

## Tip de `main` (desarrollo)

- Escritorio: `cargo build -p archmaker` / flujo Tauri del repo (capabilities deny-by-default).
- Navegador: build WASM (`tools/build-wasm.sh`) + `apps/web` (Mock solo con `?mock=1`).
- El gate desktop/offline del tip puede estar **blocked** sin binario de producción del
  `SOURCE_SHA`: ver [`p6-desktop-offline.md`](../../10-delivery/closure/p6-desktop-offline.md).

## Qué no hace

No abre runner, root, discos ni ISO. No sustituye el manual de uso del recorrido Forge.
