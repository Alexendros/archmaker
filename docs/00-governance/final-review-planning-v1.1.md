---
id: DOC-GOV-REV-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - independent-reviewer
reviewers:
  - governance
---

# Revisión independiente — planning-v1.1 (R13)

- Documento: DOC-GOV-REV-001 · Propietario: Revisor independiente · Fecha: 2026-10-06
- Alcance: revisión independiente (AUD-024) del baseline `planning-v1` remediado por las fases
  R0–R12 y recálculo honesto de los gates G0–G10 antes de publicar `planning-v1.1` (AUD-025).
- Autoría: sin autoría directa en ninguno de los artefactos revisados (regla R8 de
  `docs/00-governance/status-model.md`).

## Metodología

La revisión **recalcula** en lugar de confiar en declaraciones: ejecuta los validadores reales
del repositorio, contrasta cada criterio `complete` con evidencia material (commit existente o
run de CI en verde) y reproduce localmente la meta-validación de JSON Schema del workflow
`json-schema`. La evidencia de CI se verificó contra la API de GitHub.

Validadores ejecutados y resultado (exit code):

| Validador | Resultado | Exit |
|---|---|---|
| `python3 tools/validate_front_matter.py` | front matter válido, 0 inconsistencias cruzadas | 0 |
| `python3 tools/validate_front_matter.py --strict` | igual, 0 inconsistencias | 0 |
| `python3 tools/validate_traceability.py` | IDs/estados/referencias/gates coherentes, 8 advertencias | 0 |
| `python3 tools/validate_traceability.py --strict` | 8 advertencias promovidas a fallo | 1 (esperado) |
| `python3 tools/gates.py` | 11 manifiestos válidos y coherentes con `gates.md` y `go-no-go.md` | 0 |
| `python3 tools/validate_schema_refs.py` | 13 schemas, 121 referencias `$id`/`$defs` resueltas | 0 |
| `python3 tools/canonicalize.py` | 34 vectores, 33 con paridad Python/JS | 0 |
| Meta-validación JSON Schema (local, réplica del job CI) | 11 schemas meta-válidos (Draft 2020-12) | 0 |
| `npx --no-install markdownlint-cli2 "docs/**/*.md" "*.md"` | 0 incidencias | 0 |
| `sha256sum --check --strict reference/v5.1/SHA256SUMS` | 11/11 OK | 0 |

Evidencia de CI (API de GitHub, repositorio `Alexendros/archmaker`):

| Workflow | Run | Commit | Conclusión | Jobs |
|---|---|---:|---|---|
| `docs-validate` | [37390633111](https://github.com/Alexendros/archmaker/actions/runs/37390633111) | `3e112c0` | success | 7/7 en pass |
| `json-schema` | [37390632952](https://github.com/Alexendros/archmaker/actions/runs/37390632952) | `3e112c0` | success | 4/4 en pass |
| `canonicalization` | [37390633429](https://github.com/Alexendros/archmaker/actions/runs/37390633429) | `3e112c0` | success | 1/1 en pass |

El PR #29 (`remediation/planning-v1.1` → `main`) reporta 13 checks en verde (12 jobs de CI más
GitGuardian). Los commits de evidencia citados en los manifiestos (`611e1e7`, `0a0e98c`,
`3625889`, `88388ba`, `fbe2226`, `7e63142`) existen y su mensaje corresponde a la evidencia
declarada.

## Recálculo de gates G0–G10

Estado recalculado tras cerrar `G3-C04` y `G10-C03` por esta revisión. La tabla mecánica de
`docs/10-delivery/gates.md` se genera desde `contracts/governance/gates/G*.json` con
`tools/gates.py`.

| Gate | Estado | Fases (doc/diseño/impl/verif/release) | Criterios pendientes | Evidencia verificada |
|---|---|---|---|---|
| G0 | complete | complete/n-a/complete/complete/n-a | — | comandos reproducibles, `cmp` alias y commit `611e1e7`; SHA-256 11/11 |
| G1 | in-progress | complete/n-a/n-a/pending/n-a | G1-C03 | documentos aprobados; métricas cuantitativas sin evidencia |
| G2 | in-progress | complete/n-a/n-a/pending/n-a | G2-C04 | requisitos y DEC-001..010 resueltos; cobertura FR/NFR → prueba pendiente |
| G3 | in-progress | complete/complete/n-a/pending/n-a | — | C4 sin unidades desplegables, viewpoints 42010, ADR-0007 `accepted` |
| G4 | in-progress | in-progress/in-progress/pending/pending/n-a | — | schemas v0 y corpus en verde en CI (`3e112c0`) |
| G5 | in-progress | in-progress/pending/pending/pending/n-a | G5-C03 | CorePort v0 congelado; contract tests y paridad Tauri/WASM pendientes |
| G6 | in-progress | in-progress/in-progress/pending/pending/n-a | G6-C01, G6-C03 | baseline visual y tokens preservados; evidencia WCAG sin ejecutar |
| G7 | in-progress | in-progress/n-a/in-progress/pending/n-a | — | operadores normalizados y corpus en verde en CI |
| G8 | in-progress | in-progress/in-progress/pending/pending/n-a | G8-C01, G8-C04 | threat model, sign-off y riesgo residual; negativas pendientes |
| G9 | in-progress | in-progress/n-a/in-progress/pending/n-a | G9-C03 | CI en verde sobre `3e112c0`; SBOM, firmas y provenance pendientes |
| G10 | in-progress | in-progress/n-a/pending/pending/pending | — | roadmap, backlog, slice y owners; publicación de AUD-025 por el orquestador |

Resultado global mecánico derivado de la cobertura: **No-Go** — no todos los gates están
`complete` (faltan G1, G2, G3, G4, G5, G6, G7, G8, G9, G10). El dictamen de gobierno es
independiente de este resultado mecánico y se emite más abajo.

## Auditoría de trazabilidad

`tools/validate_traceability.py` evalúa 30 filas y reporta **8 advertencias de materialización**
que no se ocultan; son deuda de prueba preexistente y no un artefacto de esta revisión: siete
pruebas de reglas (`TST-RULE-*`) y una validación de observabilidad aparecen referenciadas en
`docs/00-governance/traceability-matrix.md` sin definición materializada (el listado exacto de
IDs lo emite el propio validador; no se transcribe aquí para no falsear el recuento mecánico).

Estas referencias se corresponden con `G2-C04` (cobertura FR/NFR → prueba materializada), que
permanece `pending`. Las cadenas OBJ/JNY/UC → FR/NFR → ADR/DM → interfaz → VAL/THR → TST →
release de la matriz resuelven sus IDs gobernados; la brecha es de materialización de pruebas,
no de resolución de referencias.

## Revisión de arquitectura

- El `Rust Core` **no** figura como unidad desplegable: en
  `docs/02-architecture/c4/component-views.md` (VW-03/VW-04/VW-05) es un componente enlazado y
  el inventario de `deployment-views.md` lo lista bajo «componentes enlazados (no desplegables)».
- El **Runner v1** se representa como **proceso separado** (VW-06 y VW-09), con elevación
  temporal fuera del WebView y host no privilegiado.
- `ADR-0007` está `accepted`/`approved` (front matter y línea narrativa), coherente con
  `G3-C03` y `G4-C01`.
- Los viewpoints (`viewpoints.md`) y los concerns (`stakeholders-concerns.md`) cubren todos los
  concerns P0 con al menos una view.

## Revisión de seguridad

- El modelo de riesgo residual (`docs/08-security/residual-risk-model.md`) acepta `RSK-003`,
  `RSK-004` y `RSK-007` para el slice con evidencia de diseño enlazada; `RSK-001`, `RSK-002` y
  `RSK-008` quedan `pendiente-de-verificacion`. Ningún P0 residual queda **aceptado sin
  autoridad**: cada fila declara autoridad competente (`architecture`, `data`, `security`).
- `THR-UPD-001` está modelado en `docs/08-security/updater-threat-model.md` con canal firmado
  (`DEC-006`/`ADR-0009`), anti-rollback y fail-open offline (`NFR-OFF-001`); **no se declara
  cerrado** y debe cerrarse antes de MVP-0 (`DEC-007`).
- El walking skeleton (`docs/10-delivery/walking-skeleton.md`) excluye runner, root, discos,
  shell arbitraria, catálogos remotos y updater activado por defecto. La superficie de red del
  updater es una desviación aceptada (`DEC-007`) que pertenece a MVP-0, no al slice.

## Revisión de accesibilidad del slice

- `docs/05-ux/accessibility-matrix.md` es una matriz WCAG 2.2 AA con **evidencia automatizada y
  manual separadas**, vocabulario de estados y combinaciones navegador/lector de pantalla. Su
  estado global es `not-verified`: define criterio, método y evidencia esperada, sin declarar
  conformidad (correcto conforme a R1).
- La evidencia de contraste de tokens abre correcciones trazables reales (texto de detalle en
  tema oscuro, `--color-warning` y `--color-border` como límites) enlazadas a `AUD-021`.
- `docs/05-ux/component-contracts.md` contrata los componentes interactivos `v0`;
  `docs/06-design-system/token-migration.md` preserva los diez tokens `--rh-*` como alias de
  compatibilidad y documenta la divergencia heredada.

## Hallazgos por severidad

### P0

Ninguno. No se detecta ningún criterio `complete` sin evidencia real, ningún P0 residual
aceptado sin autoridad, ni artefacto prohibido (root, discos, shell, red) introducido por el
walking skeleton.

### P1

- **P1-01 — CI no ejecutada sobre el commit local `a7b1414`.** El HEAD local
  (`a7b1414`, R11/R12: evidencia CI en gates + walking skeleton) está un commit por delante de
  `origin/remediation/planning-v1.1` (`3e112c0`) y no existe en GitHub; la evidencia de CI citada
  corresponde a `3e112c0`. Mitigado localmente (validadores y markdownlint en verde). Condición
  `C8`.
- **P1-02 — Cobertura de prueba no materializada.** Las 8 advertencias de materialización
  (siete `TST-RULE-*` y una validación de observabilidad) mantienen `G2-C04` `pending`.
  Condición `C4`.
- **P1-03 — Contract tests y paridad Tauri/WASM (`G5-C03`) pendientes.** Condición `C1`.
- **P1-04 — SBOM, firmas reales Sigstore y provenance (`G9-C03`, `DEC-006`) pendientes.**
  Condición `C2`.
- **P1-05 — Verificación de implementación de Seguridad (`G8-C04`) y cierre de
  RSK-001/002/008 pendientes.** Condición `C6`.
- **P1-06 — Evidencia de accesibilidad sin ejecutar (`G6-C03`) y contratos de design system
  (`G6-C01`) en curso.** Condición `C5`.
- **P1-07 — `THR-UPD-001` sin cerrar (obligatorio antes de MVP-0, `DEC-007`).** Condición `C7`.

### P2

- **P2-01 — `docs/05-ux/component-contracts.md` declara «las catorce facetas» pero enumera 15.**
  Corrección documental menor.
- **P2-02 — `docs/index.md` desactualizado**: conserva identificadores/estados heredados que ya
  no coinciden (p. ej. `DOC-UX-CMP-001` frente a `DOC-UX-COMP-001`). En esta revisión se
  corrigieron el baseline/gate y se registraron los documentos de R13; el resto del índice queda
  pendiente de refresco.
- **P2-03 — Front matter de `go-no-go.md` en `draft`** pese a portar el veredicto formal de R13.

## Veredicto

**CONDITIONAL-GO — walking skeleton MVP-0 únicamente.**

No existe ningún P0 abierto sin tratamiento. Se autoriza exclusivamente el vertical slice
definido y acotado en `docs/10-delivery/walking-skeleton.md`, sujeto a las condiciones fechadas
siguientes. El MVP completo, el runner v1, los privilegios (root, discos), la shell arbitraria y
Enterprise permanecen bloqueados.

## Condiciones fechadas

Fecha de reevaluación fijada: **2026-10-20**. Cada condición se cierra antes de la apertura de la
fase o gate que la gobierna.

| ID | Condición | Criterio | Cierre |
|---|---|---|---|
| C1 | Contract tests definidos con paridad Tauri/WASM sin semántica divergente (implementación de producto). | G5-C03 | antes de `implementation-ready` del slice |
| C2 | SBOM, firmas reales Sigstore (Fulcio/Rekor) y provenance de artefactos (DEC-006). | G9-C03 | antes de la release de MVP-0 |
| C3 | Métricas cuantitativas de usuarios y objetivos verificadas con evidencia. | G1-C03 | antes de G1 `complete` |
| C4 | Cobertura FR/NFR → prueba materializada; resolución de las 8 advertencias de materialización de pruebas. | G2-C04 | antes de G2 `complete` |
| C5 | Contratos de design system cerrados y evidencia WCAG 2.2 AA ejecutada (automatizada y manual). | G6-C01, G6-C03 | antes de G6 `complete` |
| C6 | Threat model aprobado, pruebas negativas de capabilities/elevación y cierre de RSK-001/002/008. | G8-C01, G8-C04 | antes de G8 `complete` |
| C7 | Cierre de `THR-UPD-001` con evidencia ejecutable. | DEC-007 / G8 | antes de MVP-0 |
| C8 | Re-ejecución de CI en verde sobre el commit publicado de `planning-v1.1`. | G9-C02 | al publicar (AUD-025) |

## Límites declarados

- Esta revisión es documental: no implementa producto ni ejecuta pruebas de producto; la
  verificación de implementación queda como condición.
- No modifica contratos congelados `v0` ni ADR aceptados; solo los enlaza.
- No ejecuta la publicación (tag/merge); AUD-025 la realiza el orquestador. `G10-C03` se marca
  `complete` por la parte de revisión independiente, no por la publicación.
- `tools/validate_traceability.py --strict` falla por las 8 advertencias preexistentes; la CI usa
  el modo no estricto. No se oculta la deuda.

## Trazabilidad

| Elemento | Referencia |
|---|---|
| Issue de origen | `AUD-024` (`docs/10-delivery/backlog.md`) |
| Publicación derivada | `AUD-025` (pendiente, orquestador) |
| Fases de remediación | R13 (`docs/00-governance/remediation-plan-v1.1.md`) |
| Gates | G3, G10 (`docs/10-delivery/gates.md`) |
| Resultado de gobierno | `docs/00-governance/go-no-go.md` (DOC-GOV-GNG-001) |
| Estado del plan | `docs/00-governance/plan-status.md` (DOC-GOV-STA-001) |
| Slice autorizado | `docs/10-delivery/walking-skeleton.md` (DOC-DEL-WSK-001) |
