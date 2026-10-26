---
id: ADR-0022
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
releaseStatus: ineligible
evidence:
  - .github/workflows/ci.yml (ci-gate verde en main, 2026-10-26)
  - .github/workflows/publish.yml (actionlint + zizmor 0 findings)
  - .github/workflows/release-please.yml (actionlint + zizmor 0 findings)
owners:
  - architecture
  - release
reviewers:
  - independent-reviewer
---

# ADR-0022: CI al canon de flota

- Estado: accepted
- Fecha: 2026-10-26
- Propietario: Arquitectura y Release
- Aprobación: Arquitecto, 2026-10-26
- Requisitos: NFR-SEC-001, NFR-DET-001, FR-EXPORT-001
- Verificación: ADR-E (E1–E6) fusionados en `plan/adr-e-ci-estandar`; `ci-gate` verde en cada PR.

## Contexto

Tras la transferencia del repositorio a `Soluciones-Alexendros` y la publicación de `v0.3.0`, la CI de ArchMaker seguía con workflows locales heredados (`release.yml`, `plan-release.yml`, `assign-milestone.yml`), pnpm 11.21.0, y un proceso de release manual con `workflow_dispatch` y `publish_package`. El canon de flota de `Soluciones-Alexendros/autoworkspace` v6.0.0 define reusable-static para gobernanza, release-please para versionado, y un conjunto de controles de seguridad (zizmor, actionlint, gitleaks, OSV) que deben aplicarse de forma homogénea.

## Decisión

1. Adoptar `Soluciones-Alexendros/autoworkspace/.github/workflows/reusable-static.yml@v6.0.0` como base de gobernanza en CI, con excepción documentada para los jobs específicos de Rust/Tauri/AppImage que no cubre la reusable.
2. Consolidar todos los checks obligatorios bajo un job `ci-gate` que falle si cualquier job previo no termina en `success`.
3. Fijar toolchain: Node 24 y pnpm 10.34.8 con corepack (`packageManager` con hash en `package.json`).
4. Añadir análisis de secretos (`gitleaks`) y análisis de composición de software (`cargo audit`, `pnpm audit high`, `osv-scanner`) como jobs obligatorios de `ci-gate`.
5. Aceptar provisionalmente 6 avisos OSV transitivos de Tauri/GTK/`rfd` (D-E1), con revisión obligatoria el 2027-01-25; registrarlos en `.github/policy-exceptions.md`.
6. Reemplazar `release.yml` por `publish.yml`: pipeline `package → sign-verify → attest → upload` con `environment: production`, firmas cosign keyless, provenance mediante `actions/attest-build-provenance`, y subida al borrador del release con `gh release upload --clobber`.
7. Introducir `release-please.yml` para generar release PRs con estrategia `simple` sobre `version.txt`. Como solución provisional (D-E4), `publish.yml` acepta `workflow_call` desde `release-please.yml` hasta que exista la GitHub App `soluciones-alexendros-ci`.
8. Retirar `plan-release.yml` y `assign-milestone.yml` (D-E3); el primero es sustituido por release-please, el segundo queda fuera del canon.
9. Incluir `workflow-lint` con actionlint 1.7.12 y zizmor 1.31.1 `--min-severity=medium` en `ci-gate`.
10. Mantener CSP sin `'unsafe-inline'`; el script de arranque del tema se autoriza por hash `'sha256-d4so4ONh+2suxlZa3JYcnVbsdxy6c2SDs3/wbJ2VTSU='`.
11. Requerir commits firmados solo en `main`; T0.2 verifica `.commit.verification.verified` en el último squash.
12. Actualizar `AGENTS.md` con el canon de flota (C2–C6) y `SECURITY.md` con la cadena de suministro.

## Consecuencias

- La CI es más homogénea con el resto de la flota y auditable por `reusable-static`.
- Los releases pasan a ser semi-automáticos: release-please crea el PR, la fusión genera el tag y el borrador, y `publish.yml` sube los artefactos.
- El entorno `production` no lleva revisor ni bypass de administradores; solo despliegan tags `v*` (AL-2).
- Los rulesets de `main` y `plan/*` no tienen bypass, tampoco para el dueño (AL-3).
- La creación de tags `v*` quedará restringida a la App CI una vez se implemente AL-1/AL-4b.
- Riesgo residual: mientras dura D-E4, `workflow_call` entre `release-please.yml` y `publish.yml` depende del token por defecto; la migración a la App CI mitigará este riesgo.

## Excepciones

- D-E1: 6 avisos OSV transitivos aceptados temporalmente; revisión el 2027-01-25.
- D-E3: retiro de `plan-release.yml` y `assign-milestone.yml`.
- D-E4: `workflow_call` provisional de release-please a publish.yml hasta la App CI.
- `version.txt` añadido como fuente de verdad de release-please, pese a no estar en el alcance literal original de ADR-E.
