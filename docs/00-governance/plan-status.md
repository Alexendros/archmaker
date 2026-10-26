---
id: DOC-GOV-STA-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: partial
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
---

# Estado de planificación

- Estado: in-review
- Gate global: CONDITIONAL-CLEAR — walking skeleton MVP-0 + slices MVP-1..MVP-5, más el slice ISO-01 de imagen viva (sin runner de instalación)
- Baseline vigente: `implementation-baseline-mvp5` (predecesores mvp0.1..mvp4 intactos)
- Alcance MVP-5 `accepted` y cerrado (MVP-5-01..05): `docs/10-delivery/mvp5-scope.md`. Hueco residual: `docs/10-delivery/mvp5-gap-spec.md` (draft).
- Remediación operativa cerrada (transportes reales, ACL, ciclo de edición, exportación, Prove-It): `docs/10-delivery/remediation/`; notas en `acceptance-notes.md`.
- Cierre de producto F0–F8: código en `main`. Gobierno de canal F8:
  `docs/10-delivery/closure/f8-governance.md` (**accepted**).
- Canal de distribución: AppImage Linux x86_64; **release `v0.3.0` publicada**
  (<https://github.com/Soluciones-Alexendros/archmaker/releases/tag/v0.3.0>). Los tags históricos
  `v0.1.0` y `v0.2.0` no tuvieron GitHub Release (`release.yml` terminó _cancelled_ para
  `v0.1.0` y _success_ sin publicar para `v0.2.0`). A partir de ADR-0022 las releases usan
  `publish.yml` y release-please. Nombre visible vigente **Forge** (ADR-0012). ISO-01
  (`build-scope.md`, ADR-0019 / DEC-013) está `accepted` solo para imagen viva x86_64 UEFI;
  el target `archmaker-live-profile` está en el árbol. La cara no invoca `mkarchiso`. El runner
  de instalación y Enterprise siguen sin `accepted`. Evidencia y `APPROVAL_RECORD` fuera del
  árbol.
- Limitaciones: cosign local verificado parcialmente; 3× install GUI pendiente; F7 sin
  `RC_SHA_FUNCIONAL` independiente. R2: `releaseStatus` del doc F9 = `released`.
- Reorientación aprobada por el operador el 2026-10-25: Forge pasa a ser una GUI fina sin
  root que genera perfil `archiso` completo + `user_configuration.json` real de `archinstall`
  y construye la ISO con `mkarchiso` dentro de Docker (ADR-0020/ADR-0021 en preparación).
  El estado anterior queda congelado en el tag `pre-reorientacion`.
- Fecha de corte: 2026-10-25

## Tres capas (no equiparar)

| Capa                 | Estado       | Nota                                                                                                              |
| -------------------- | ------------ | ----------------------------------------------------------------------------------------------------------------- |
| Baseline de código   | passed       | Tag MVP-5 + F0–F8 en `main`                                                                                       |
| Aprobación funcional | pending      | F7 estricto + P95 web en árbol; desktop/offline **blocked** (P6); sin `RC_SHA_FUNCIONAL` ni revisor independiente |
| Release binaria      | no publicada | Tags `v0.1.0`/`v0.2.0` sin GitHub Release ni AppImage; próxima release `v0.3.0`; ver `f9-release.md`              |

Detalle y residuales: `docs/10-delivery/closure/f8-governance.md`.
Candidato tip post-P0–P7 (sin publicar): `docs/10-delivery/closure/release-candidate.md`
(`59246f08…`); handoff: `closure-handoff.md`.

## Bloqueos P0

Los hallazgos P0 de integración (mock silencioso, ACL ambigua, WASM sin sesión, manifiesto
obsoleto, token de export fijo) están clasificados en `docs/10-delivery/remediation/findings-r0.md`
y se cierran con la remediación; no reabren runner/ISO. El estado de `DEC-001..DEC-011` se lee en
`docs/00-governance/decision-register.md`.

La aprobación funcional del cierre y la release binaria **no** son bloqueos P0 del slice MVP-5;
siguen `pending` / `blocked` según la tabla de capas.

## Regla de avance

Se autorizan el walking skeleton MVP-0 y los slices MVP-1 (`mvp1-scope.md`), MVP-2
(`mvp2-scope.md`), MVP-3 (`mvp3-scope.md`), MVP-4 (`mvp4-scope.md`, React + MockCorePort) y
MVP-5 (`mvp5-scope.md`: catálogo de matriz, UI de cinco pasos, paridad Tauri/WASM; exclusiones
del propio alcance). El runner v1 y los privilegios se autorizan únicamente cuando G0–G10 tengan
evidencia específica de esas fases, no existan riesgos P0 sin tratamiento y todos los contratos
P0 posean fixtures válidos e inválidos.

F9 **no publicó** ningún AppImage: los tags `v0.1.0`/`v0.2.0` carecen de GitHub Release
(corregido el 2026-10-25). La primera release real será `v0.3.0`. La aprobación funcional F7
permanece `pending` como capa distinta; no reabre runner/ISO ni anuncia `.deb`/`.rpm`/AUR.

## Referencias

- Veredicto y condiciones: `docs/00-governance/final-review-planning-v1.1.md` (DOC-GOV-REV-001)
  → **CONDITIONAL-GO — walking skeleton MVP-0**.
- Resultado motivado: `docs/00-governance/go-no-go.md` (DOC-GOV-GNG-001).
- Gobierno del cierre: `docs/10-delivery/closure/f8-governance.md` (DOC-DEL-CLO-F8-001).
- Cobertura de gates: `docs/10-delivery/gates.md`.
- Registro de decisiones: `docs/00-governance/decision-register.md` (DOC-GOV-DEC-001).
- Próximos pasos ordenados: `docs/10-delivery/next-issues.md` (ISSUE-001..010).
- Índice documental: `docs/index.md`.
- Desktop/offline P6: `docs/10-delivery/closure/p6-desktop-offline.md`.
- Candidato sin publicar (P8–P9): `docs/10-delivery/closure/release-candidate.md` / `closure-handoff.md`.
