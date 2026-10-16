---
id: DOC-DEL-MVP5-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-MVP4-001
    expectedDocumentStatus: accepted
  - id: DOC-RES-HTML-001
  - id: DOC-GOV-GNG-001
---

# Alcance MVP-5 — paridad de adaptadores y UI de cinco pasos

- Documento: DOC-DEL-MVP5-001 · Estado: accepted · Propietario: Arquitectura · Fecha: 2026-10-16
- Entrada: baseline `implementation-baseline-mvp4` + gates G0–G10 `complete` (slice).
- Alcance **accepted** (MVP-5-01). Catálogo de matriz, UI de cinco pasos y paridad Tauri/WASM
  autorizados bajo las exclusiones de este documento; runner, ISO y privilegios siguen fuera.
- El hueco detallado vive en [`mvp5-gap-spec.md`](mvp5-gap-spec.md) (DOC-DEL-MVP5-GAP-001).

## Objetivo

Recorrer en la UI de producto los cinco pasos del canon visual
(`reference/v5.1/index.html`, matriz en `docs/12-research/html-catalog-matrix.md`) contra el
catálogo local, y acercar Tauri y WASM a la misma frontera CorePort. La semántica sigue en
Rust. No se construye ISO, no se descarga el espejo y no se llama a pacman.

## Incluido

| Área     | Contenido                                                                                                          | Anclas                                |
| -------- | ------------------------------------------------------------------------------------------------------------------ | ------------------------------------- |
| Catálogo | Cada opción de la matriz entra como definición sin `cmd`, sin versión inventada y sin tamaño presentado como hecho | DOC-RES-HTML-001, FR-CAT-001, CON-011 |
| UI       | Cinco pasos: base, environment, libpacks, apps, build. Tokens y contratos ya escritos                              | DOC-UX-COMP-001, DEC-009              |
| Paridad  | Las operaciones ya existentes del CorePort devuelven el mismo resultado en Tauri y WASM                            | NFR-DET-001, NFR-PORT-001             |
| Reglas   | Las cuatro reglas del HTML, reescritas al AST aceptado                                                             | DOC-VAL-OP-001                        |

## Excluido

- Runner, root, discos, shell, `pacstrap`, `mkarchiso`, catálogo remoto, firma de catálogo en carga.
- Presets (`FR-PRESET-001` sigue deferred). La UI no finge aplicarlos.
- Cifras de versión o de megabytes del HTML.
- Nombre público de tienda o de imagen (ADR-0010).
- Rediseño visual. La pantalla de laboratorio de seis pasos puede quedar como prueba.

## Issues

| #        | Issue                                                                    | Depende de | Gate    | Owner        | Criterio Done                                              | Estado |
| -------- | ------------------------------------------------------------------------ | ---------- | ------- | ------------ | ---------------------------------------------------------- | ------ |
| MVP-5-01 | Aceptar este alcance                                                     | —          | G2, G10 | product      | Documento `accepted`                                       | closed |
| MVP-5-02 | Ampliar el catálogo local con la matriz, sin afirmaciones no verificadas | 01         | G4      | data         | Cada id HTML está en catálogo o sigue como hueco explícito | closed |
| MVP-5-03 | UI de cinco pasos con los contratos existentes                           | 02         | G6      | ux           | Seleccionar, resolver, validar y exportar sin escribir ISO | closed |
| MVP-5-04 | Paridad Tauri/WASM del mismo draft                                       | 02         | G5, G9  | architecture | Mismos bytes de manifiesto                                 | closed |
| MVP-5-05 | Tag de baseline solo tras 01–04 y CI verde                               | 04         | G10     | release      | Tag nuevo; los anteriores no se mueven                     | closed |

## Trazabilidad

- Requisitos: FR-CAT-001, FR-DRAFT-001, FR-RESOLVE-001, FR-VALIDATE-001, FR-MANIFEST-001, FR-EXPORT-001, NFR-DET-001, NFR-ACC-001, NFR-OFF-001, NFR-PORT-001.
- Riesgos: RSK-004, RSK-009.
- Amenazas: THR-IMP-001.
- Pruebas: las de paridad y a11y ya existentes, más la matriz regenerada en CI.

## Criterio de salida

Los issues 01–05 cerrados. Tag `implementation-baseline-mvp5` emitido (MVP-5-05); tags
MVP-0.1..MVP-4 intactos. Runner e ISO siguen fuera del veredicto.

## Post-slice (cierre F0–F7 / gobierno F8)

El trabajo de cierre de producto (identidad, bytes, fidelidad, importación, escritorio, UX,
candidato) vive en `docs/10-delivery/closure/` y **no** reabre este alcance ni mueve el tag
MVP-5. Capas tras F8 (`closure/f8-governance.md`):

- Baseline de código: **passed** (este tag + commits F0–F7 en `main`).
- Aprobación funcional: **pending** (limitaciones F7; sin aceptación independiente).
- Release binaria: **blocked** (MVP-6 / ADR-0010 sin `accepted`; F9 no abierto).

Nombre público de tienda o imagen (ADR-0010) sigue excluido de MVP-5.
