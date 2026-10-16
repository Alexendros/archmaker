---
id: DOC-DEL-CLO-F8-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-CLO-F7-001
    expectedDocumentStatus: draft
  - id: DOC-GOV-GNG-001
  - id: DOC-DEL-MVP5-001
    expectedDocumentStatus: accepted
---

# F8 — Gobierno documental del cierre (sin distribución)

- Documento: DOC-DEL-CLO-F8-001 · Estado: in-review · Propietario: Gobierno · Fecha: 2026-10-16
- Este documento **alinea** el gobierno con F0–F7 ya fusionados en `main`. **No** autoriza
  empaquetado, tags de producto, releases binarias ni canales de tienda.
- ADR-0010 permanece `proposed` / `in-review`. MVP-6 permanece `in-review` (no `accepted`).
- F9 permanece **blocked** hasta aceptación funcional formal y autorización explícita de canal.

## Tres capas (no se confunden)

| Capa                     | Qué demuestra                                                                                              | Estado en F8                                                               | Anclas                                                                                                           |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- | -------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------- |
| **Baseline de código**   | Tag de slice + commits de cierre en `main`                                                                 | **passed** para el slice MVP-5; código F0–F7 **en `main`** (PRs #105–#112) | Tag `implementation-baseline-mvp5` → `b13c200a8747290e03383c6b76cbe31752741a31`; `main` post-#112 incluye CI fix |
| **Aprobación funcional** | Matriz F7 sobre un `RC_SHA` de `main`, sin P0/P1 de recorrido, firma independiente o limitación registrada | **pending**                                                                | `f7-candidate.md`: `result=pending`; no hay `RC_SHA_FUNCIONAL` ni `APPROVAL_RECORD_SHA`                          |
| **Release binaria**      | Paquete, SBOM, firma, descarga verificada (F9)                                                             | **blocked**                                                                | MVP-6 no `accepted`; ADR-0010 no `accepted`; sin plataforma única autorizada                                     |

Un cambio documental produce otro SHA; el candidato funcional se revalida si se pretende cerrar la capa de aprobación.

## Puertas F0–F9

Estados permitidos: `passed` | `failed` | `pending` | `blocked` | `not-applicable`.
Justificación breve en cada fila. Ninguna puerta de distribución se marca `passed` aquí.

| Puerta                    | Estado  | Justificación                                                                                                                                                            |
| ------------------------- | ------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| F0 Evidencia / inventario | passed  | Inventario, schema y validador en árbol; C-01..C-07 clasificados (`inventory.md`, `evidence-contract.md`)                                                                |
| F1 Identidad              | passed  | Entropía WASM de producción + create sin sobrescritura fusionados (#107)                                                                                                 |
| F2 Bytes                  | passed  | Bytes del renderizador + digest de archivo fusionados (#106)                                                                                                             |
| F3 Fidelidad              | passed  | Matriz de campos, hostname/cifrado como intención (#105)                                                                                                                 |
| F4 Importación            | passed  | Consentimiento, `file.size`, persistencia observable (#108)                                                                                                              |
| F5 Escritorio             | passed  | Guardado nativo acotado + ACL (#110); CI post-F5 (#112)                                                                                                                  |
| F6 Cara honesta           | passed  | Mensajes, rail de seis pasos, sin base 650 / Sway inventado (#109)                                                                                                       |
| F7 Candidato              | pending | Matriz ejecutable en árbol (#111); manifiesto de evidencia `result=pending`; E2E WASM×3 / offline / P95 y aceptación independiente **no** cerrados (`f7-candidate.md`)   |
| F8 Gobierno (este doc)    | pending | Alineación documental **hecha** en este PR; autorización de distribución (ADR-0010 / MVP-6 / plataforma) **no** concedida → la puerta de gobierno de canal sigue pending |
| F9 Publicación            | blocked | Depende de F7 accepted + F8 con canal autorizado. Motivo: sin `accepted` de MVP-6 ni ADR-0010; prohibido anunciar familias de paquete o abrir F9                         |

## Residuales no esenciales

Impacto ≠ bloqueo de baseline de código. No reabren runner/ISO ni el veredicto CONDITIONAL-CLEAR hasta MVP-5.

| Id            | Residual                                                                            | Impacto                                               | Responsable                       | Fecha objetivo                      |
| ------------- | ----------------------------------------------------------------------------------- | ----------------------------------------------------- | --------------------------------- | ----------------------------------- |
| RES-F7-E2E    | Tres E2E WASM consecutivos + offline web/escritorio en evidencia fuera del árbol    | Aprobación funcional sigue pending                    | Architecture                      | 2026-11-15                          |
| RES-F7-P95    | P95 F6 (init ≤ 3 000 ms; resolve+validate+manifiesto ≤ 1 000 ms; n=30) sin medir    | No se etiqueta rendimiento como passed                | Architecture                      | 2026-11-15                          |
| RES-F7-IND    | Revisor independiente o limitación firmada sin etiqueta «independiente»             | Sin `APPROVAL_RECORD_SHA`                             | Governance + independent-reviewer | 2026-11-30                          |
| RES-F5-PORTAL | Humo GUI F5 no es E2E de `xdg-desktop-portal` (picker simulado)                     | Cobertura de diálogo nativo incompleta en CI          | Architecture                      | 2026-12-15                          |
| RES-F0-CORPUS | Corpus de entrega: 20 casos/target propuesto; F0 versiona 3 fixtures de clase       | Densidad de evidencia, no regresión de código         | Architecture                      | 2026-12-15                          |
| RES-GAP-MVP5  | `mvp5-gap-spec.md` sigue draft (hueco residual del slice, no del cierre F0–F7)      | Documentación de hueco; no bloquea baseline de código | Architecture                      | 2026-11-30                          |
| RES-ADR-0010  | Nombre público / marca sin cerrar                                                   | Bloquea release binaria y ficha de tienda             | Product                           | Hasta autorización explícita        |
| RES-MVP6      | Alcance MVP-6 `in-review`; cinco familias del borrador **no** anunciadas como canal | Bloquea F9                                            | Release                           | Hasta `accepted` + plataforma única |
| RES-DEC-006   | Pipeline real SBOM/firma (Etapa C) pendiente                                        | Bloquea release de paquete aunque MVP-6 se acepte     | Release                           | Tras F8 de canal                    |

## Contradicciones de baseline / alcance (objetivo: cero abiertas)

| Antes de F8                                                                                                                                      | Después de F8                                                                    |
| ------------------------------------------------------------------------------------------------------------------------------------------------ | -------------------------------------------------------------------------------- |
| README decía baseline `mvp4` y «tag MVP-5 pendiente» mientras `plan-status` / `go-no-go` / `mvp5-scope` ya tenían `implementation-baseline-mvp5` | README alineado a `mvp5`; tag emitido en `b13c200…`                              |
| Mezcla implícita de «código en main» con «aceptación funcional» y «release»                                                                      | Tres capas explícitas en este documento y en `plan-status` / `go-no-go` / manual |
| Riesgo de leer MVP-6 / ADR-0010 como autorizados por el cierre F0–F7                                                                             | Estados `in-review` / `proposed` reiterados; F9 `blocked`                        |

No se añaden contradicciones nuevas CON-*: la de README queda cerrada por el texto canónico, no por un tag movido.

## Qué F8 no hace (prohibido sin autorización explícita)

- Poner ADR-0010 o MVP-6 en `accepted`.
- Elegir o anunciar plataforma de paquete (AppImage, `.deb`, `.rpm`, AUR, etc.).
- Abrir F9, crear tags de producto, GitHub Releases binarias o presentar AUR como repo binario de pacman.
- Ampliar el veredicto a runner, root, discos, shell, ISO o Enterprise.

## Referencias

- F7: [`f7-candidate.md`](f7-candidate.md)
- Inventario F0: [`inventory.md`](inventory.md)
- Remediación OP: [`../remediation/acceptance-notes.md`](../remediation/acceptance-notes.md)
- Veredicto: [`../../00-governance/go-no-go.md`](../../00-governance/go-no-go.md)
- Plan: [`../../00-governance/plan-status.md`](../../00-governance/plan-status.md)
- Empaquetado (política, no ejecución): [`../packaging-release.md`](../packaging-release.md)
- MVP-6 (borrador): [`../mvp6-scope.md`](../mvp6-scope.md)
- ADR-0010: [`../../02-architecture/adr/ADR-0010-marca-y-nombre-de-artefacto.md`](../../02-architecture/adr/ADR-0010-marca-y-nombre-de-artefacto.md)
