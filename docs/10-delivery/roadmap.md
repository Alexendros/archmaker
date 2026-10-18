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
- MVP-2 rules/resolver/validation — baseline `implementation-baseline-mvp2` ([`mvp2-scope.md`](mvp2-scope.md)).
- MVP-3 manifest/exporters — baseline `implementation-baseline-mvp3` ([`mvp3-scope.md`](mvp3-scope.md)).
- MVP-4 React + MockCorePort + design system — baseline `implementation-baseline-mvp4` ([`mvp4-scope.md`](mvp4-scope.md)).
- MVP-5 catálogo de matriz, UI cinco pasos, paridad Tauri/WASM — baseline `implementation-baseline-mvp5` ([`mvp5-scope.md`](mvp5-scope.md)).
- Cierre F0–F9: gobierno F8 ([`closure/f8-governance.md`](closure/f8-governance.md)) **accepted**;
  F9 publicó Forge AppImage [`v0.1.0`](https://github.com/Alexendros/archmaker/releases/tag/v0.1.0)
  ([`closure/f9-release.md`](closure/f9-release.md)); aprobación funcional F7 **pending**.
- MVP-6 packaging — [`mvp6-scope.md`](mvp6-scope.md) **accepted** solo AppImage; ADR-0010
  **accepted** (opción 3). Residual: cosign local / 3× GUI; `.deb`/`.rpm`/AUR deferred.
- Imagen viva x86_64 UEFI — [`iso-scope.md`](iso-scope.md) **accepted** (ISO-01, 2026-10-18). El veredicto nombra perfil archiso, una corrida `mkarchiso` y arranque vivo. El SSD lo instala archinstall en la sesión. El target `archmaker-live-profile` y la descarga tipada están en el árbol. La corrida de 2026-10-18 produjo `Forge-aaa81a961b0a-x86_64.iso` en una VM y arrancó por UEFI; ese arranque cubre LiveISO. SSD sigue `not_run` ([`closure/iso04-mkarchiso-not-run.md`](closure/iso04-mkarchiso-not-run.md)).

## v1

- v1-0 protocolo y runner read-only.
- v1-1 plan/dry-run/confirmación/journal.
- v1-2 adapter/ejecución/verificación/recovery.

## Enterprise

- ENT-0 identidad/tenant/audit.
- ENT-1 catálogos/policies/profiles/approvals.
- ENT-2 runners/campaigns/rollout/operations.
