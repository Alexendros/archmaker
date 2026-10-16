# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-10-16

Primera release pública del canal autorizado F8/F9: AppImage Linux x86_64 con nombre
público **Neubat** (DEC-011 / ADR-0010 opción 3 / MVP-6 accepted solo para esa plataforma).

### Added

- AppImage `Neubat_0.1.0_amd64.AppImage` (Linux x86_64) en
  [GitHub Release v0.1.0](https://github.com/Alexendros/archmaker/releases/tag/v0.1.0).
- Binario nativo `archmaker` (smoke CLI) adjunto a la misma release.
- `SHA256SUMS`, SBOM CycloneDX (`sbom.cdx.json`) y provenance SLSA v1
  (`slsa-provenance.json`).
- Cierre de producto F0–F8 en `main` (identidad, bytes, fidelidad, importación, escritorio,
  cara honesta, matriz candidata, gobierno de canal) sobre la baseline
  `implementation-baseline-mvp5`.

### Identidades de la release

| Campo            | Valor                                                              |
| ---------------- | ------------------------------------------------------------------ |
| Tag              | `v0.1.0`                                                           |
| `RC_SHA`         | `9297ed7cc47f42a9611aed0093b5ab040daca512`                         |
| `ARTIFACT_SHA256`| `6567464f5e2f21398fd3eef35592f0e9d377fd2cc7887c1e46df092b5a8e6d81` |

### Known limitations

- Firma cosign keyless: diferida / no verificada en el host local de evidencia
  (`cosign: deferred-ci-oidc-or-unavailable-local`).
- Instalación GUI: no se ejecutaron tres instalaciones GUI de aceptación
  (`install_gui: not-verified`).
- Aprobación funcional F7: sin `RC_SHA_FUNCIONAL` ni aceptación independiente firmada
  (`f7-candidate.md` permanece `pending`).
- Sin imagen de instalación Arch, runner, root, discos, shell, Enterprise, AUR binario,
  `.deb`/`.rpm`.
- Front matter R2: `releaseStatus` del doc F9 permanece `ineligible` mientras
  `implementationStatus` ≠ `complete` o `verificationStatus` ≠ `passed`.

[Unreleased]: https://github.com/Alexendros/archmaker/compare/v0.1.0...HEAD
[0.1.0]: https://github.com/Alexendros/archmaker/releases/tag/v0.1.0
