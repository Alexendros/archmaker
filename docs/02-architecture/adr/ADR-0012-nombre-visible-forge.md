---
id: ADR-0012
phase: planning
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - product
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0010
    expectedDocumentStatus: accepted
---

# ADR-0012: Nombre visible Forge (GUI de ArchMaker)

- Estado: accepted
- Fecha: 2026-10-17
- Aceptación: 2026-10-17
- Propietario: Producto
- Requisitos: NFR-MARK-001
- Decisiones: DEC-012

## Contexto

ADR-0010 / DEC-011 fijaron el nombre público **Neubat** para la primera release
verificable (`v0.1.0`, AppImage Linux x86_64). Ese artefacto ya está publicado y no se
reescribe. La GUI en desarrollo y el empaquetado futuro necesitan un nombre visible que no
entre en conflicto con el prefijo ARCH de los ids técnicos y que distinga interfaz y motor.

## Drivers

- Separar nombre de GUI y nombre de proyecto/motor sin renombrar crates, DB ni identificador
  Tauri.
- No mover tags ni sustituir assets de `v0.1.0`.
- Conservar ADR-0010 / DEC-011 como historia de la release publicada.

## Opciones

1. Seguir usando Neubat en toda superficie activa.
2. Renombrar también la identidad técnica (`archmaker*` → otro prefijo).
3. Sustituir solo el nombre visible de lo no publicado: GUI **Forge**, motor **ArchMaker**.

## Decisión

Se adopta la **opción 3**:

| Ámbito                                                         | Valor                                                                 |
| -------------------------------------------------------------- | --------------------------------------------------------------------- |
| Nombre visible de la GUI                                       | **Forge**                                                             |
| Proyecto / motor                                               | **ArchMaker**                                                         |
| Fórmula de entrada                                             | «Forge, la interfaz gráfica de ArchMaker»                             |
| Ids técnicos                                                   | Intactos (`dev.archmaker.app`, IndexedDB `archmaker-profiles`, …)     |
| Historia `v0.1.0`                                              | Sigue siendo Neubat (ADR-0010 / DEC-011); no se reescribe `[0.1.0]` |

## Consecuencias

- `productName`, títulos, diálogos y textos de UI activos dicen Forge.
- El módulo de presentación pasa a `forge-face.ts`.
- El workflow de release futuro exige `Forge_*.AppImage` (no se reejecuta sobre `v0.1.0`).
- `tools/check_product_naming.py` (T-NAME / T-MIG) vigila regresiones y la identidad técnica.

## Riesgos

- Confusión temporal entre el AppImage publicado (Neubat) y el nombre visible vigente (Forge):
  mitigado en README, CHANGELOG Unreleased y docs de cierre.

## Verificación

- T-NAME-01..04 y T-MIG-01..04 en `tools/check_product_naming.py` (+ Playwright de persistencia).
- Diff de superficies activas sin «Neubat» fuera de la allowlist.

## Sustituye

Solo el **nombre visible** de artefactos y UI no publicados que ADR-0010 fijó como Neubat.
No sustituye la decisión histórica de `v0.1.0` ni los ids técnicos.

## Sustituido por

—
