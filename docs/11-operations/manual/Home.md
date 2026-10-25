---
id: DOC-OPS-MAN-HOME
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# ArchMaker

Texto canónico: [`docs/11-operations/manual/`](https://github.com/Alexendros/archmaker/blob/main/docs/11-operations/manual/). Esta wiki lo publica.

## Qué es

ArchMaker es un **configurador local** para diseñar, validar y exportar un perfil. Rust es la autoridad semántica; la interfaz es React (Tauri en escritorio, WASM en navegador).

## Estado (tres capas)

| Capa                 | Estado       | Lectura                                                                 |
| -------------------- | ------------ | ----------------------------------------------------------------------- |
| Baseline de código   | passed       | Tag `implementation-baseline-mvp5`; cierre F0–F8 en `main`              |
| Aprobación funcional | pending      | Limitaciones F7; sin `RC_SHA_FUNCIONAL`                                 |
| Release binaria      | no publicada | Tags `v0.1.0`/`v0.2.0` sin GitHub Release ni AppImage; próxima `v0.3.0` |

Detalle de gobierno: [`docs/10-delivery/closure/f8-governance.md`](https://github.com/Alexendros/archmaker/blob/main/docs/10-delivery/closure/f8-governance.md).

Veredicto vigente: **CONDITIONAL-CLEAR** hasta MVP-5 (sin runner). Walking skeleton MVP-0 y slices MVP-1..MVP-5.

## Qué no hace

- No instala Arch Linux ni ningún sistema.
- No ejecuta un plan privilegiado ni abre un shell.
- No requiere red para el flujo MVP (catálogo y drafts locales).
- No anuncia `.deb`/`.rpm`/AUR/ISO como Done; el canal publicado es solo AppImage Forge.

## Lectura

1. [Instalación de Forge](Instalacion-Forge.md)
2. [Guía de usuario](Guia-de-usuario.md)
3. [Manual de uso](Manual-de-uso.md)
4. [Límites](Limites.md)

Repositorio: <https://github.com/Alexendros/archmaker>
