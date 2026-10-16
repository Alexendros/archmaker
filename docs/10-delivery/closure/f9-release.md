---
id: DOC-DEL-CLO-F9-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: eligible
owners:
  - release
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-CLO-F8-001
    expectedDocumentStatus: accepted
  - id: DOC-DEL-MVP6-001
    expectedDocumentStatus: accepted
  - id: ADR-0010
    expectedDocumentStatus: accepted
  - id: DEC-011
---

# F9 — Release verificable (AppImage Neubat)

- Documento: DOC-DEL-CLO-F9-001 · Propietario: Release · Fecha: 2026-10-16
- Canal: AppImage Linux x86_64, nombre público **Neubat** (DEC-011 / F8).
- Hosting: GitHub Release del tag `v*`; workflow `.github/workflows/release.yml`.
- Evidencia de digests y `APPROVAL_RECORD` viven **fuera del árbol** de código
  (p. ej. `archmaker-closure-evidence/<rc-sha>/<run-id>/` hermano del repo). Este
  documento no embebe digests como única verdad; la identidad es el
  `ARTIFACT_SHA256` del AppImage publicado y el `manifest.json.sha256` externo.

## Identidades

| Id                    | Uso                                                                  |
| --------------------- | -------------------------------------------------------------------- |
| `RC_SHA`              | Commit de `main` desde el que se construye                           |
| `ARTIFACT_SHA256`     | SHA-256 del AppImage publicado (identidad del paquete)               |
| `EVIDENCE_SHA256`     | SHA-256 del manifiesto de evidencia (externo; no dentro de sí mismo) |
| `APPROVAL_RECORD_SHA` | SHA-256 del registro de aprobación (fichero separado)                |

## Procedimiento

1. `git fetch origin main` y fijar `RC_SHA=$(git rev-parse origin/main)`.
2. Árbol limpio de cambios versionados; lockfiles (`Cargo.lock`, `pnpm-lock.yaml`)
   y `rust-toolchain.toml` registrados.
3. `pnpm --dir apps/web build` y `pnpm tauri build --bundles appimage`.
4. Generar `SHA256SUMS`, SBOM CycloneDX (Cargo.lock / `cargo-sbom`), provenance
   SLSA v1 y firmas cosign keyless según `release.yml`.
5. Publicar en GitHub Release **sin** modificar bytes tras el digest.
6. Verificar: `sha256sum --check` del AppImage descargado == `ARTIFACT_SHA256`.
7. Smoke: binario nativo CLI y/o AppImage si hay display/portal; si no, checksums
   - smoke CLI documentados en la evidencia externa.

## Retirada

Si un digest publicado resulta incorrecto: no reescribir el asset; publicar
nota de retirada y, si procede, un tag nuevo con rebuild completo (nuevos digests).

## Exclusiones

Runner, ISO, root, discos, shell, Enterprise, AUR binario, `.deb`/`.rpm` como Done.
