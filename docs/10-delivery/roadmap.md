---
id: DOC-DEL-RMAP-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: complete
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# Roadmap

## Predesarrollo

PD-0 fuentes; PD-1 producto; PD-2 arquitectura; PD-3 datos; PD-4 interfaces; PD-5 UX/design; PD-6 validation/security; PD-7 quality/backlog; PD-8 review y tag `planning-v1`.

## MVP

- MVP-0.1 walking skeleton stabilizado (baseline `implementation-baseline-mvp0.1`): CorePort, adapters Tauri/WASM, canonicalización, reglas, resolución, manifest, export, security, CI, supply chain mínimo. Sin runner ni privilegios.
- MVP-1 domain/schema/catalog/migrations — baseline `implementation-baseline-mvp1` ([`mvp1-scope.md`](mvp1-scope.md)).
- MVP-2 rules/resolver/validation — alcance en [`mvp2-scope.md`](mvp2-scope.md); hacia el tag `implementation-baseline-mvp2`.
- MVP-3 manifest/exporters.
- MVP-4 React + MockCorePort + design system.
- MVP-5 Tauri/WASM/persistencia/paridad.
- MVP-6 packaging/hardening/beta/stable.

## v1

- v1-0 protocolo y runner read-only.
- v1-1 plan/dry-run/confirmación/journal.
- v1-2 adapter/ejecución/verificación/recovery.

## Enterprise

- ENT-0 identidad/tenant/audit.
- ENT-1 catálogos/policies/profiles/approvals.
- ENT-2 runners/campaigns/rollout/operations.
