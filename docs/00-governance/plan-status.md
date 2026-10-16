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
- Baseline vigente: `implementation-baseline-mvp4` (predecesores mvp0.1..mvp3 intactos; tag MVP-5 pendiente de MVP-5-05)
- Alcance MVP-5 `accepted` (MVP-5-01): `docs/10-delivery/mvp5-scope.md`. Catálogo (02), UI (03) y paridad (04) cerrados. Tag pendiente (MVP-5-05). Hueco: `docs/10-delivery/mvp5-gap-spec.md` (draft).
- Borradores posteriores, sin ampliar el veredicto al runner/ISO: `docs/10-delivery/mvp6-scope.md`, `docs/10-delivery/iso-scope.md`
- Fecha de corte: 2026-10-16

## Bloqueos P0

Sin bloqueos P0 activos. El estado, la autoridad, la fecha y las consecuencias de las decisiones
`DEC-001..DEC-010` se leen en `docs/00-governance/decision-register.md` (autoridad única); este
documento no los repite.

## Regla de avance

Se autorizan el walking skeleton MVP-0 y los slices MVP-1 (`mvp1-scope.md`), MVP-2
(`mvp2-scope.md`), MVP-3 (`mvp3-scope.md`), MVP-4 (`mvp4-scope.md`, React + MockCorePort) y
MVP-5 (`mvp5-scope.md`: catálogo de matriz, UI de cinco pasos, paridad Tauri/WASM; exclusiones
del propio alcance). El runner v1 y los privilegios se autorizan únicamente cuando G0–G10 tengan
evidencia específica de esas fases, no existan riesgos P0 sin tratamiento y todos los contratos
P0 posean fixtures válidos e inválidos.

## Referencias

- Veredicto y condiciones: `docs/00-governance/final-review-planning-v1.1.md` (DOC-GOV-REV-001)
  → **CONDITIONAL-GO — walking skeleton MVP-0**.
- Resultado motivado: `docs/00-governance/go-no-go.md` (DOC-GOV-GNG-001).
- Cobertura de gates: `docs/10-delivery/gates.md`.
- Registro de decisiones: `docs/00-governance/decision-register.md` (DOC-GOV-DEC-001).
- Próximos pasos ordenados: `docs/10-delivery/next-issues.md` (ISSUE-001..010).
- Índice documental: `docs/index.md`.
