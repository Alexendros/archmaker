---
id: DOC-QLT-CI-001
phase: MVP
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - quality
reviewers:
  - independent-reviewer
---

# CI/CD

- Estado: draft
- Propietario: Calidad
- Última revisión: 2026-10-26
- Requisitos relacionados: NFR-DET-001, NFR-OBS-001, NFR-SEC-001, NFR-PORT-001
- Sustituye / sustituido por: —
- Decisiones aplicables: DEC-005, DEC-006, DEC-009, DEC-016 (ver `../00-governance/decision-register.md`)

## Alcance

Pipeline de integración y entrega para ArchMaker. La validación vive en
`.github/workflows/ci.yml` y se agrupa bajo el job `ci-gate`. Los artefactos de release se
construyen y publican con `.github/workflows/publish.yml`, y los release PRs se generan con
`.github/workflows/release-please.yml`. Todo el contenido sigue siendo `draft` hasta su revisión
formal.

## Estrategia de gates

| Disparador          | Workflows                                           | Gates       | Naturaleza |
| ------------------- | --------------------------------------------------- | ----------- | ---------- |
| PR / push a `main`  | `.github/workflows/ci.yml` (`ci-gate`)              | G0–G10      | Bloqueante |
| Push de tag `v*`    | `.github/workflows/publish.yml`                     | G8, G9, G10 | Bloqueante |
| Push a `main`       | `.github/workflows/release-please.yml`              | —           | Automático |
| `workflow_call` tag | `.github/workflows/publish.yml` (provisional, D-E4) | G8, G9, G10 | Bloqueante |

- **PR / push a `main`**: `ci-gate` falla si cualquier job previo no termina en `success`.
- **Release**: release-please crea un release PR; al fusionarlo, se genera el tag y el borrador de
  release; `publish.yml` sube los artefactos al borrador.
- **Protección de `main`**: `ci-gate` como único check obligatorio, solo squash, historial lineal y
  commits firmados; sin bypass para nadie (AL-3).

## Supply chain

### Acciones fijadas por SHA

Toda acción de terceros se referencia por **commit SHA** (no por tag flotante) para que el
contenido ejecutado sea inmutable. Los SHA ya están fijados en los workflows:
`actions/checkout@3d3c42e5aac5ba805825da76410c181273ba90b1 # v7.0.1`,
`actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020 # v4.4.0`,
`actions/setup-python@5fda3b95a4ea91299a34e894583c3862153e4b97 # v7.0.0`,
`actions/upload-artifact@ea165f8d65b6e75b540449e92b4886f43607fa02 # v4.6.2`,
`actions/download-artifact@95815c38cf2ff2164869cbab79da8d1f422bc89e # v4.2.1`,
`actions/attest-build-provenance@4d101475d8b20a2381f78447822ac1eab6504dd8 # v2.2.3` y
`DavidAnson/markdownlint-cli2-action@992badcdf24e3b8eb7e87ff9287fe931bcb00c6e # v20.0.0`.
No deben introducirse hashes no verificados contra la fuente.

### Least privilege

- `permissions: contents: read` por defecto en todos los workflows.
- Los permisos elevados (`id-token: write`, `attestations: write`, `contents: write`) se conceden
  **solo** en el job que atesta o publica, nunca a nivel de workflow.
- Sin secretos de larga duración si puede usarse OIDC federado.

### Concurrencia y timeouts

- `concurrency` por workflow y ref con `cancel-in-progress: false` para evitar cancelaciones entre
  eventos del PR.
- `timeout-minutes` en cada job para acotar coste y ejecuciones colgadas.

### Análisis de seguridad en CI

- **Secretos**: `gitleaks` en el job `secrets` de `ci.yml`.
- **SCA**: `cargo audit`, `pnpm audit --audit-level high` y `osv-scanner` en el job `sca`.
- **Workflow lint**: `actionlint` 1.7.12 y `zizmor` 1.31.1 `--min-severity=medium` en el job
  `workflow-lint`.

## Artefactos: attestation, SBOM y firma

La política de firma queda fijada por DEC-006 (Sigstore para releases) y DEC-016 (CI al canon de
flota).

| Elemento            | Mecanismo                                               | Estándar        | Estado             |
| ------------------- | ------------------------------------------------------- | --------------- | ------------------ |
| Provenance          | `actions/attest-build-provenance` en `publish.yml` (F9) | SLSA / Sigstore | F9 AppImage        |
| SBOM                | CycloneDX desde `Cargo.lock` en `publish.yml`           | CycloneDX       | F9 AppImage        |
| Firma de artefactos | Sigstore (keyless, ligada a OIDC)                       | Sigstore        | Fijado por DEC-006 |
| Checksums           | `SHA256SUMS` + verificación `sha256sum --check`         | SHA-256         | F9 AppImage        |

- **Provenance**: se atesta cada artefacto de build para ligar binario, commit y workflow.
- **SBOM**: se publica CycloneDX junto al release para inventario de dependencias.
- **Firma**: cosign keyless para AppImage, SBOM y SHA256SUMS; `gh attestation verify` valida la
  provenance en el job `verify-release`.
- Toda atestación requiere `id-token: write` y `attestations: write` en el job correspondiente.

## Promoción sin rebuild

Los artefactos se construyen una sola vez en `publish.yml` para un tag dado. La pipeline
`package → sign-verify → attest → upload` mantiene los mismos digests entre jobs mediante
artefactos de GitHub Actions. Cualquier rebuild invalida la atestación y exige re-firmar. Detalle
de canales y rollback en [packaging-release.md](../10-delivery/packaging-release.md).

## Decisiones aplicables

Las decisiones que rigen esta área y su estado se leen en `decision-register.md` (DEC-005,
DEC-006, DEC-009, DEC-016).

- **DEC-009** confirma la prohibición de CDN en runtime; los recursos se empaquetan o se toman del
  sistema.
- **DEC-016** consolida CI bajo `ci-gate`, introduce release-please y `publish.yml`, y fija Node 24
  / pnpm 10.34.8.

## Referencias

- Workflow principal: `.github/workflows/ci.yml`.
- Workflow de release: `.github/workflows/publish.yml`.
- Release please: `.github/workflows/release-please.yml`.
- Gates G0–G10: [gates.md](../10-delivery/gates.md).
- Artefactos y canales: [packaging-release.md](../10-delivery/packaging-release.md).
- Decisiones: [decision-register.md](../00-governance/decision-register.md).
- Política supply-chain: [supply-chain-policy.md](../08-security/supply-chain-policy.md).
- Política Tauri (CSP sin CDN en producción): [tauri-policy.md](../08-security/tauri-policy.md).
