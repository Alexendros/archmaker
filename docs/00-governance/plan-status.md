---
id: DOC-GOV-STA-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - governance
reviewers:
  - independent-reviewer
---

# Estado de planificación

- Estado: in-review
- Gate global: CONDITIONAL-CLEAR — walking skeleton MVP-0 + slices MVP-1..MVP-5 (sin runner)
- Baseline vigente: `implementation-baseline-mvp5` (predecesores mvp0.1..mvp4 intactos)
- Alcance MVP-5 `accepted` y cerrado (MVP-5-01..05): `docs/10-delivery/mvp5-scope.md`. Hueco residual: `docs/10-delivery/mvp5-gap-spec.md` (draft).
- Remediación operativa cerrada (transportes reales, ACL, ciclo de edición, exportación, Prove-It): `docs/10-delivery/remediation/`; notas en `acceptance-notes.md`.
- Cierre de producto F0–F7: código en `main` (PRs de cierre + CI #112). Gobierno documental: `docs/10-delivery/closure/f8-governance.md`.
- Borradores posteriores, sin ampliar el veredicto al runner/ISO: `docs/10-delivery/mvp6-scope.md` (`in-review`), `docs/10-delivery/iso-scope.md`; ADR-0010 `proposed`. F9 **blocked**.
- Fecha de corte: 2026-10-16

## Tres capas (no equiparar)

| Capa                 | Estado  | Nota                                                        |
| -------------------- | ------- | ----------------------------------------------------------- |
| Baseline de código   | passed  | Tag MVP-5 + F0–F7 en `main`                                 |
| Aprobación funcional | pending | Limitaciones F7 (`f7-candidate.md`); sin `RC_SHA_FUNCIONAL` |
| Release binaria      | blocked | Sin MVP-6/`accepted`, sin ADR-0010/`accepted`, sin F9       |

Detalle y residuales: `docs/10-delivery/closure/f8-governance.md`.

## Bloqueos P0

Los hallazgos P0 de integración (mock silencioso, ACL ambigua, WASM sin sesión, manifiesto
obsoleto, token de export fijo) están clasificados en `docs/10-delivery/remediation/findings-r0.md`
y se cierran con la remediación; no reabren runner/ISO. El estado de `DEC-001..DEC-010` se lee en
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

F9 (empaquetado/firma/publicación) no arranca hasta aceptación funcional formal y autorización
explícita de gobierno de canal (ADR-0010 + MVP-6 `accepted` + plataforma única). Ese paso **no**
está concedido en F8.

## Referencias

- Veredicto y condiciones: `docs/00-governance/final-review-planning-v1.1.md` (DOC-GOV-REV-001)
  → **CONDITIONAL-GO — walking skeleton MVP-0**.
- Resultado motivado: `docs/00-governance/go-no-go.md` (DOC-GOV-GNG-001).
- Gobierno del cierre: `docs/10-delivery/closure/f8-governance.md` (DOC-DEL-CLO-F8-001).
- Cobertura de gates: `docs/10-delivery/gates.md`.
- Registro de decisiones: `docs/00-governance/decision-register.md` (DOC-GOV-DEC-001).
- Próximos pasos ordenados: `docs/10-delivery/next-issues.md` (ISSUE-001..010).
- Índice documental: `docs/index.md`.
