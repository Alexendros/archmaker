---
id: DOC-DEL-CLO-F8-001
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: partial
releaseStatus: eligible
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
  - id: DOC-DEL-MVP6-001
    expectedDocumentStatus: accepted
  - id: ADR-0010
    expectedDocumentStatus: accepted
  - id: DEC-011
---

# F8 — Gobierno de canal (distribución autorizada, una plataforma)

- Documento: DOC-DEL-CLO-F8-001 · Estado: **accepted** · Propietario: Gobierno · Fecha: 2026-10-16
- Autorización de canal: **sí** — plataforma única **AppImage Linux x86_64**, nombre público
  **Neubat** (ADR-0010 opción 3, DEC-011, MVP-6 accepted solo para esa plataforma).
- F9 queda **authorized** para construir y publicar esos bytes verificables. No reabre runner,
  ISO, root, discos, shell, Enterprise ni AUR binario.

## Tres capas (no se confunden)

| Capa                     | Qué demuestra                                                                                              | Estado en F8                                                                                      | Anclas                                                                                                       |
| ------------------------ | ---------------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------------ |
| **Baseline de código**   | Tag de slice + commits de cierre en `main`                                                                 | **passed** para el slice MVP-5; código F0–F7 **en `main`**                                        | Tag `implementation-baseline-mvp5` → `b13c200a8747290e03383c6b76cbe31752741a31`                              |
| **Aprobación funcional** | Matriz F7 sobre un `RC_SHA` de `main`, sin P0/P1 de recorrido, firma independiente o limitación registrada | **pending** (residuales F7; no bloquea el canal autorizado por este F8 bajo mandato del operador) | `f7-candidate.md`: `result=pending`; sin `RC_SHA_FUNCIONAL` ni `APPROVAL_RECORD_SHA` de aceptación funcional |
| **Release binaria**      | Paquete, SBOM, firma, descarga verificada (F9)                                                             | **authorized** — AppImage Linux x86_64 / Neubat / GitHub Release                                  | MVP-6 `accepted` (solo AppImage); ADR-0010 `accepted` (opción 3); F9 abierto                                 |

Un cambio documental produce otro SHA. F9 construye desde el `RC_SHA` de `main` post-merge de
este gobierno (y cualquier fix de pipeline necesario).

## Puertas F0–F9

Estados permitidos: `passed` | `failed` | `pending` | `blocked` | `not-applicable` | `authorized`.

| Puerta                    | Estado     | Justificación                                                                                                |
| ------------------------- | ---------- | ------------------------------------------------------------------------------------------------------------ |
| F0 Evidencia / inventario | passed     | Inventario, schema y validador en árbol                                                                      |
| F1 Identidad              | passed     | Entropía WASM de producción + create sin sobrescritura                                                       |
| F2 Bytes                  | passed     | Bytes del renderizador + digest de archivo                                                                   |
| F3 Fidelidad              | passed     | Matriz de campos, hostname/cifrado como intención                                                            |
| F4 Importación            | passed     | Consentimiento, `file.size`, persistencia observable                                                         |
| F5 Escritorio             | passed     | Guardado nativo acotado + ACL                                                                                |
| F6 Cara honesta           | passed     | Mensajes, rail de seis pasos                                                                                 |
| F7 Candidato              | pending    | Matriz ejecutable; E2E WASM×3 / offline / P95 y aceptación independiente **no** cerrados (`f7-candidate.md`) |
| F8 Gobierno (este doc)    | passed     | Canal autorizado: AppImage x86_64 + Neubat; resto deferred/blocked                                           |
| F9 Publicación            | authorized | Depende de este F8; ejecución en rama/PR F9 sobre `RC_SHA` de `main`                                         |

## Plataforma única — decisión trazable

| Campo                | Valor                                                               |
| -------------------- | ------------------------------------------------------------------- |
| Plataforma           | AppImage Linux x86_64                                               |
| Nombre público       | Neubat                                                              |
| Canal                | GitHub Release (`v*`, `release.yml`)                                |
| ADR / DEC            | ADR-0010 accepted (opción 3); DEC-011                               |
| MVP-6                | accepted **solo** para AppImage; `.deb`/`.rpm`/OBS/AUR = deferred   |
| Exclusiones vigentes | runner, ISO, root, discos, shell, Enterprise, AUR como repo binario |

### Por qué AppImage

Alineado con `packaging-release.md` (primer formato MVP Linux) y con la fila «sin tienda» del
borrador MVP-6; no exige OBS ni ficha AUR; el hosting ya está cableado en `release.yml`
(SBOM, cosign, provenance). Autorizar varias familias violaría el requisito de plataforma única.

## Residuales no esenciales

| Id            | Residual                                                                         | Impacto                                      | Responsable                       | Fecha objetivo |
| ------------- | -------------------------------------------------------------------------------- | -------------------------------------------- | --------------------------------- | -------------- |
| RES-F7-E2E    | Tres E2E WASM consecutivos + offline web/escritorio en evidencia fuera del árbol | Aprobación funcional sigue pending           | Architecture                      | 2026-11-15     |
| RES-F7-P95    | P95 F6 sin medir                                                                 | No se etiqueta rendimiento como passed       | Architecture                      | 2026-11-15     |
| RES-F7-IND    | Revisor independiente o limitación firmada                                       | Sin `APPROVAL_RECORD_SHA` funcional          | Governance + independent-reviewer | 2026-11-30     |
| RES-F5-PORTAL | Humo GUI F5 no es E2E de `xdg-desktop-portal`                                    | Cobertura de diálogo nativo incompleta en CI | Architecture                      | 2026-12-15     |
| RES-F0-CORPUS | Corpus de entrega densificado                                                    | Densidad de evidencia                        | Architecture                      | 2026-12-15     |
| RES-GAP-MVP5  | `mvp5-gap-spec.md` draft                                                         | Documentación de hueco                       | Architecture                      | 2026-11-30     |
| RES-DEC-006   | Pipeline real SBOM/firma Etapa C                                                 | Cierre en F9                                 | Release                           | F9             |

Cerrados por este F8: `RES-ADR-0010`, `RES-MVP6` (canal único).

## Qué F8 autoriza / prohíbe

**Autoriza:** F9 sobre AppImage Linux x86_64 llamado Neubat; GitHub Release verificable;
checksums, SBOM, firma/atestación según `release.yml` / `packaging-release.md`.

**Prohíbe sin nuevo `accepted`:** anunciar `.deb`/`.rpm`/OBS/AUR/Flatpak como Done; runner;
ISO; root; discos; shell; Enterprise; AUR como repo binario; mover tags históricos; cambiar
bytes tras el digest publicado.

## Referencias

- F7: [`f7-candidate.md`](f7-candidate.md)
- Empaquetado: [`../packaging-release.md`](../packaging-release.md)
- MVP-6: [`../mvp6-scope.md`](../mvp6-scope.md)
- ADR-0010: [`../../02-architecture/adr/ADR-0010-marca-y-nombre-de-artefacto.md`](../../02-architecture/adr/ADR-0010-marca-y-nombre-de-artefacto.md)
- Decisiones: [`../../00-governance/decision-register.md`](../../00-governance/decision-register.md) (DEC-011)
- Veredicto: [`../../00-governance/go-no-go.md`](../../00-governance/go-no-go.md)
- Plan: [`../../00-governance/plan-status.md`](../../00-governance/plan-status.md)
