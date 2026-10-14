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

Baseline vigente: `implementation-baseline-mvp4` (walking skeleton MVP-0 y slices MVP-1..MVP-4).

## Qué no hace

- No instala Arch Linux ni ningún sistema.
- No ejecuta un plan privilegiado ni abre un shell.
- No requiere red para el flujo MVP (catálogo y drafts locales).
- No publica paquetes de tienda mientras MVP-6 no esté `accepted`.

## Lectura

1. [Guía de usuario](Guia-de-usuario.md)
2. [Manual de uso](Manual-de-uso.md)
3. [Límites](Limites.md)

Repositorio: <https://github.com/Alexendros/archmaker>
