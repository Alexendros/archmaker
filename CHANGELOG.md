# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Runner F7 estricto (`--strict`): estados por comando/test, `technicalResult` separado de
  `result=pending`, pkg WASM completo, harness P95 n=30 (`tools/run_f7_p95_harness.py`).
- Casos F7 atómicos (motor/export WASM, offline web/desktop, P95).
- Suite de diagramas Archify D01–D24 bajo `docs/02-architecture/diagrams/` (renderer fijado;
  regenerar con `tools/scaffold_archify_diagrams.py` + `tools/run_archify_diagrams.py`).

### Changed

- Nombre visible de la GUI: **Forge** (interfaz gráfica de ArchMaker); el motor y los
  ids técnicos `archmaker*` no cambian (ADR-0012 / DEC-012). La release `[0.1.0]`
  permanece con el nombre público histórico **Neubat**.
- Contrato de evidencia: campos opcionales `harness`, `technicalResult`, `commands`, `tests`,
  `approval` (`approvalRecordSha256` vs `approvalRecordCommit`).
- Documentación de cierre: código de `main`, AppImage `v0.1.0` y aceptación funcional siguen
  como capas distintas; el tag `v0.1.0` no se mueve.

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
