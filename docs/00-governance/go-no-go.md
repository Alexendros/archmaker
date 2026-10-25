---
id: DOC-GOV-GNG-001
phase: planning
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - architecture
reviewers:
  - independent-reviewer
---

# Checkpoint de misión — planning-v1.1

- Documento: DOC-GOV-GNG-001 · Propietario: Arquitectura · Fecha: 2026-10-12
- Resultado: **Clear** (checkpoint superado: todos los gates G0–G10 están `complete`; el hilo de la misión puede continuar).
- Veredicto de gobierno (R13): **CONDITIONAL-CLEAR — walking skeleton MVP-0**.
- Reevaluación slice MVP-1 (MVP-1-08): **CONDITIONAL-CLEAR — walking skeleton MVP-0 + slice MVP-1**
  (dominio, catálogo local, `importDraft` / MIG-5.1). Baseline emitida:
  `implementation-baseline-mvp1` (el tag `implementation-baseline-mvp0.1` no se movió).
- Reevaluación slice MVP-2 (MVP-2-07): **CONDITIONAL-CLEAR — walking skeleton MVP-0 + slices MVP-1/MVP-2**
  (reglas de catálogo, resolución de dominio, validación que bloquea manifiesto). Baseline emitida:
  `implementation-baseline-mvp2` (tags previos intactos).
- Reevaluación slice MVP-3 (MVP-3-06): **CONDITIONAL-CLEAR — walking skeleton MVP-0 + slices MVP-1/MVP-2/MVP-3**
  (adapters de exportación versionados; ADR-0003 parcial). Baseline emitida:
  `implementation-baseline-mvp3` (tags previos intactos).
- Reevaluación slice MVP-4 (MVP-4-06): **CONDITIONAL-CLEAR — walking skeleton MVP-0 + slices MVP-1..MVP-4**
  (React + MockCorePort; design system sin rediseño). Baseline emitida:
  `implementation-baseline-mvp4` (tags previos intactos).
- Reevaluación alcance MVP-5 (MVP-5-01): **CONDITIONAL-CLEAR — walking skeleton MVP-0 + slices MVP-1..MVP-5**
  (alcance `mvp5-scope.md` accepted: catálogo de matriz, UI de cinco pasos, paridad Tauri/WASM).
- Reevaluación slice MVP-5 (MVP-5-05): **CONDITIONAL-CLEAR — walking skeleton MVP-0 + slices MVP-1..MVP-5**
  (remediación operativa + Prove-It). Baseline emitida: `implementation-baseline-mvp5`
  (tags previos intactos). Exclusiones del alcance vigentes hasta el slice ISO-01 de abajo.
- Post-slice cierre F0–F9 (código en `main` + release): **no** amplía el veredicto al
  runner/ISO. Capas separadas (DOC-DEL-CLO-F8-001 / DOC-DEL-CLO-F9-001): baseline de
  código **passed**; aprobación funcional **pending** (runner F7 estricto y P95 harness
  en árbol; sin `RC_SHA_FUNCIONAL` ni revisor independiente); release binaria
  **published** — AppImage Linux x86_64 tag `v0.1.0` (`RC_SHA` `9297ed7…`; artefacto histórico en `f9-release.md`),
  verificación parcial (checksum OK; sin `.sig`/`.pem` públicos; sin 3×
  install GUI). Ver `docs/10-delivery/closure/f8-governance.md` y `f9-release.md`.
- Post P0–P6 (cierre ArchMaker/Forge): nomenclatura ADR-0012, integridad de evidencia,
  P95 web, Archify AS_BUILT, aceptación web y desktop/offline **blocked** (P6) documentados
  en `docs/10-delivery/closure/`. Ese cierre no publica un tag nuevo ni abre el runner de instalación.
- Alcance autorizado: `docs/10-delivery/walking-skeleton.md` (DOC-DEL-WSK-001),
  `docs/10-delivery/mvp1-scope.md` (DOC-DEL-MVP1-001), `docs/10-delivery/mvp2-scope.md`
  (DOC-DEL-MVP2-001), `docs/10-delivery/mvp3-scope.md` (DOC-DEL-MVP3-001),
  `docs/10-delivery/mvp4-scope.md` (DOC-DEL-MVP4-001), `docs/10-delivery/mvp5-scope.md`
  (DOC-DEL-MVP5-001) y empaquetado AppImage de `docs/10-delivery/mvp6-scope.md`
  (DOC-DEL-MVP6-001, solo esa plataforma). Slice **ISO-01** (2026-10-18, ADR-0019 / DEC-013,
  `build-scope.md` accepted): perfil archiso desde el manifiesto sellado, una corrida
  `mkarchiso` en proceso separado de constitución, y arranque vivo UEFI x86_64. La
  instalación en SSD es prueba de archinstall en esa sesión, no escritura de Forge.
  Bloqueados: runner de instalación (ADR-0004), root, escritura en discos (A6), shell
  arbitraria, catálogos remotos, firma criptográfica de catálogo en carga, Enterprise,
  aarch64, `.deb`/`.rpm`/OBS/AUR binario como Done de MVP-6.

## Motivo del resultado mecánico

El resultado `Clear` es la lectura literal de la cobertura de gates: G0–G10 están todos
`complete` (bloque derivado más abajo). El dictamen de gobierno permanece condicionado
(**CONDITIONAL-CLEAR**) al walking skeleton MVP-0 y al slice MVP-1 de datos/migración;
no autoriza runner ni privilegios.

## Veredicto y condiciones

El informe `docs/00-governance/final-review-planning-v1.1.md` (DOC-GOV-REV-001) emitió
**CONDITIONAL-CLEAR** para el walking skeleton MVP-0. La reevaluación MVP-1-08 amplía ese
veredicto al slice MVP-1 (`mvp1-scope.md`) sin abrir runner ni privilegios, y sin P0 abiertos
sin tratamiento. Las condiciones fechadas (`C1`–`C8`) se leen en ese informe y no se repiten aquí.

## Por qué el Clear es condicionado al walking skeleton

1. El Clear mecánico refleja que todos los gates G0–G10 están `complete` para el walking skeleton MVP-0.
2. El SBOM, las firmas reales Sigstore y la provenance dependen de artefactos de producto
   (`G9-C03`, `DEC-006`) y se materializan en la fase de release (I11).
3. La paridad Tauri/WASM (`G5-C03`), la accesibilidad (`G6-C03`) y la seguridad (`G8-C04`,
   con cierre de `RSK-001/002/008`) están verificadas para el slice técnico; su extensión
   al MVP completo requiere trabajo adicional.
4. `THR-UPD-001` está modelado y el canal de actualización está firmado (`DEC-007`);
   el cierre formal del threat model de actualización es requisito previo al tag MVP-0.

El estado de las decisiones (`DEC-*`), los ADR, los riesgos (`RSK-*`) y los gates no se repite
aquí: se lee en `docs/00-governance/decision-register.md`, `docs/02-architecture/adr/`,
`docs/00-governance/risk-register.md` y los manifiestos `contracts/governance/gates/`. Este
documento conserva únicamente el resultado motivado y deriva la cobertura de gates de los
manifiestos.

<!-- BEGIN GATES-DERIVED -->
## Cobertura de gates (derivada)

Bloque generado desde `contracts/governance/gates/G*.json` por `tools/gates.py`; no editar a mano.

| Gate | Estado | Fases (doc/diseño/impl/verif/release) | Falta para `complete` |
|---|---|---|---|
| G0 Fuentes controladas | complete | complete/n/a/complete/complete/n/a | — |
| G1 Problema y usuarios | complete | complete/n/a/n/a/complete/n/a | — |
| G2 Alcance y requisitos | complete | complete/n/a/n/a/complete/n/a | — |
| G3 Arquitectura | complete | complete/complete/n/a/complete/n/a | — |
| G4 Datos y contratos | complete | complete/complete/complete/complete/n/a | — |
| G5 Interfaces y CorePort | complete | complete/complete/complete/complete/n/a | — |
| G6 UX y design system | complete | complete/complete/complete/complete/n/a | — |
| G7 Validación y corpus | complete | complete/complete/complete/complete/n/a | — |
| G8 Seguridad | complete | complete/complete/complete/complete/n/a | — |
| G9 Calidad y CI | complete | complete/complete/complete/complete/n/a | — |
| G10 Delivery y walking skeleton | complete | complete/complete/complete/complete/n/a | — |

Resultado derivado: **Clear** — todos los gates G0–G10 están `complete`.
<!-- END GATES-DERIVED -->

## Criterios de salida (para superar el checkpoint)

- G0–G10 en `complete` con evidencia ejecutable y sin P0 abiertos asociados.
- Contratos P0 con corpus válido **e** inválido validado en CI (job de corpus activo).
- CI en verde sobre el commit publicado (documental + meta-validación + corpus).
- `sha256sum --check --strict reference/v5.1/SHA256SUMS` en verde y sin regresiones de
  trazabilidad.
- Cierre de `RSK-001/002/008` con pruebas negativas y de `THR-UPD-001` antes de MVP-0.

## Consecuencia inmediata

Autorizado: el walking skeleton MVP-0 definido en `docs/10-delivery/walking-skeleton.md`,
los slices **MVP-1** (`mvp1-scope.md`), **MVP-2** (`mvp2-scope.md`), **MVP-3**
(`mvp3-scope.md`), **MVP-4** (React + MockCorePort, `mvp4-scope.md`), **MVP-5**
(catálogo de matriz, UI de cinco pasos, paridad Tauri/WASM; `mvp5-scope.md`) y el canal
**MVP-6 AppImage Linux x86_64** (`mvp6-scope.md`, ADR-0010 / DEC-011 para `v0.1.0`;
nombre visible vigente **Forge** por ADR-0012 / DEC-012). Alcance **adicional**
declarativo/sonda (ADR-0013…0018, `forge-declarative-scope.md`): planificación y
observación de solo lectura; **no** reescribe MVP-5 como permiso de ejecución.
**Autorizado además, solo ISO-01:** perfil `archmaker-live-profile`, una corrida
`mkarchiso` fuera de la cara y el arranque vivo UEFI x86_64 (ADR-0019 / DEC-013).
El botón de Revisión pide la imagen con una operación tipada: no ejecuta shell ni
`mkarchiso`. La ISO del 2026-10-18 se generó en una VM, no desde el WebView. Sin la
línea de este párrafo no había exporter ni `mkarchiso`.
**No** autorizado: runner de instalación, root, escritura en discos (A6), shell
arbitraria, catálogos remotos, firma criptográfica de catálogo en carga, Enterprise,
aarch64, ni `.deb`/`.rpm`/OBS/AUR binario como Done. `mvp5-scope.md` no se reabre.
`forge-declarative-scope.md` permanece con `executionStatus=not-executed`.

La presencia de F0–F8 en `main` y la publicación F9 (`v0.1.0`) **no** equivalen a
aprobación funcional firmada ni a `RC_SHA_FUNCIONAL`. El acto de gobierno F8/DEC-011
autorizó el canal; F9 publicó bytes con verificación parcial, capa distinta de F7.

## Decisiones de alcance (C3 — G1-C03)

**G1-C03 — Métricas cuantitativas de usuarios y objetivos verificadas con evidencia**:

- **Estado**: `eximido del walking skeleton MVP-0`.
- **Justificación**: El walking skeleton MVP-0 (ver `docs/10-delivery/walking-skeleton.md`) cubre la arquitectura técnica ejecutable (CorePort, adapters Tauri/WASM, canonicalización, reglas, resolución, manifest, export, security, CI, supply chain). Las métricas de usuario (adopción, usabilidad cuantitativa, satisfacción) pertenecen al ciclo de vida del producto post-MVP (fase v1+) y requieren instrumentación, telemetría (DEC-010: ninguna en MVP) y recolección de datos reales que están explícitamente fuera del alcance del slice técnico.
- **Decisión**: G1-C03 no bloquea el tag `implementation-baseline-mvp0`. G1-C01/C02/C03 están `complete` (C03 por exención documentada); G1 queda `complete` para el walking skeleton.
- **Evidencia**: `docs/01-product/objectives.md` declara "Métrica de éxito verificable" como "no verificado — fuente primaria pendiente"; `docs/01-product/requirements.md` NFR-ACC-001/NNF-OFF-001 definen criterios cualitativos sin métricas cuantitativas de usuarios.
