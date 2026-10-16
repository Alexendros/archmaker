---
id: DOC-DEL-CLO-F9-001
phase: MVP
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
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
  - id: DOC-GOV-DEC-001
---

# F9 — Release verificable (AppImage Forge)

- Documento: DOC-DEL-CLO-F9-001 · Propietario: Release · Fecha: 2026-10-16
- Canal: AppImage Linux x86_64, nombre visible **Forge** (DEC-011 / F8).
- Hosting: GitHub Release del tag `v*`; workflow `.github/workflows/release.yml`.
- Estado de publicación: **published** — tag
  [`v0.1.0`](https://github.com/Alexendros/archmaker/releases/tag/v0.1.0).
- Evidencia de digests y `APPROVAL_RECORD` viven **fuera del árbol** de código
  (p. ej. `archmaker-closure-evidence/<rc-sha>/<run-id>/` hermano del repo). Este
  documento no embebe digests como única verdad; la identidad es el
  `ARTIFACT_SHA256` del AppImage publicado y el `manifest.json.sha256` externo.
- Front matter R2: `releaseStatus=ineligible` porque la verificación es **parcial**
  (no `complete` + `passed`). No se declara `eligible` hasta cosign verificable y
  humo GUI exigido.

## Publicación v0.1.0 (hechos)

| Campo             | Valor                                                              |
| ----------------- | ------------------------------------------------------------------ |
| Tag               | `v0.1.0` (no se mueve; tip de `main` puede avanzar con docs)       |
| `RC_SHA`          | `9297ed7cc47f42a9611aed0093b5ab040daca512`                         |
| `ARTIFACT_SHA256` | `6567464f5e2f21398fd3eef35592f0e9d377fd2cc7887c1e46df092b5a8e6d81` |
| Artefacto         | `Neubat_0.1.0_amd64.AppImage`                                      |
| Adjuntos          | `archmaker`, `SHA256SUMS`, `sbom.cdx.json`, `slsa-provenance.json` |
| Evidencia externa | `archmaker-closure-evidence/9297ed7…/f9-20261016T202654Z/`         |

Verificación registrada en el manifiesto externo:

| Comprobación     | Resultado                                         |
| ---------------- | ------------------------------------------------- |
| CLI / checksums  | verified                                          |
| checksum AppImage | verified (`ARTIFACT_SHA256` coincide al descargar)              |
| cosign local       | deferred: assets `.sig`/`.pem` no listados en la release pública |
| install_gui (×3)   | not-verified                                                    |
| `result`           | published                                                       |

## Identidades

| Id                         | Uso                                                                  |
| -------------------------- | -------------------------------------------------------------------- |
| `RC_SHA`                   | Commit de `main` desde el que se construye                           |
| `ARTIFACT_SHA256`          | SHA-256 del AppImage publicado (identidad del paquete)               |
| `EVIDENCE_SHA256`          | SHA-256 del manifiesto de evidencia (externo; no dentro de sí mismo) |
| `approvalRecordSha256`     | SHA-256 del fichero de registro de aprobación                        |
| `approvalRecordCommit`     | Commit git del registro si está versionado (identidad distinta)      |

No reutilizar el nombre histórico `APPROVAL_RECORD_SHA` para un commit: el valor de
aprobación es el digest del fichero. El tip de `main` posterior a `v0.1.0` no altera este tag.

## Procedimiento (reproducible)

1. `git fetch origin main` y fijar `RC_SHA` del candidato (para v0.1.0 fue el de arriba).
2. Árbol limpio de cambios versionados; lockfiles (`Cargo.lock`, `pnpm-lock.yaml`)
   y `rust-toolchain.toml` registrados.
3. `pnpm --dir apps/web build` y `pnpm tauri build --bundles appimage`.
4. Generar `SHA256SUMS`, SBOM CycloneDX (Cargo.lock / `cargo-sbom`), provenance
   SLSA v1 y firmas cosign keyless según `release.yml`.
5. Publicar en GitHub Release **sin** modificar bytes tras el digest.
6. Verificar: `sha256sum --check` del AppImage descargado == `ARTIFACT_SHA256`.
7. Smoke: binario nativo CLI y/o AppImage si hay display/portal; si no, checksums +
   smoke CLI documentados en la evidencia externa.

## Residuales abiertos (honestos)

| Id              | Residual                                                         | Bloquea `releaseStatus=eligible` |
| --------------- | ---------------------------------------------------------------- | -------------------------------- |
| RES-F9-COSIGN   | cosign keyless no verificado en host local de evidencia          | sí (verification parcial)        |
| RES-F9-GUI      | Tres instalaciones GUI de aceptación no ejecutadas               | sí (verification parcial)        |
| RES-F7-FUNC     | Sin `RC_SHA_FUNCIONAL` / aceptación independiente (capa F7)      | no (capa distinta)               |
| RES-ISO         | Sin imagen de instalación Arch                                   | n/a (fuera de alcance)           |

## Retirada

Si un digest publicado resulta incorrecto: no reescribir el asset; publicar
nota de retirada y, si procede, un tag nuevo con rebuild completo (nuevos digests).
El tag `v0.1.0` **no** se mueve.

## Exclusiones

Runner, ISO, root, discos, shell, Enterprise, AUR binario, `.deb`/`.rpm` como Done.
