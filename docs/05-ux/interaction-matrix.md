---
id: DOC-UX-INT-001
phase: MVP
priority: P1
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - ux
reviewers:
  - independent-reviewer
---

# Matriz de interacción

| Journey | Ruta | Pantalla | Componentes | Acción/CorePort | Validación | Prueba |
|---|---|---|---|---|---|---|
| JNY-001 | `/` | Inicio | RecentDrafts, NewDraft, Import | `productInfo` | Capabilities | E2E-HOME |
| JNY-001 | `/new` | Nuevo | TargetPicker, PresetPicker | `createDraft` | Draft schema | E2E-NEW |
| JNY-001 | `/configure/:step` | Configurar | StepRail, OptionCard, Inspector | `resolveDraft` | Domain/rules | E2E-CONF |
| JNY-001 | `/review` | Revisar | ManifestSummary, Diff, Diagnostics | `validateDraft`, `buildManifest` | Full pipeline | E2E-REV |
| JNY-001 | `/export` | Exportar | TargetPicker, ArtifactCard | `checkTarget`, `exportArtifact` | Target | E2E-EXP |
| JNY-002 | `/import` | Importar | FileDrop, MigrationReport | `importDraft`, migration | Limits/migration | E2E-IMP |
| JNY-003 | `/runner/plan` | Plan v1 | PlanTree, RiskSummary | Runner protocol | Plan/privilege | E2E-RUN |
