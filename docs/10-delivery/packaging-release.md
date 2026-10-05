# Packaging y release

- Estado: draft
- Propietario: Release
- Última revisión: 2026-10-05
- Requisitos relacionados: FR-EXPORT-001, NFR-OBS-001, NFR-SEC-001, NFR-PORT-001
- Sustituye / sustituido por: —
- Sujeto a: DEC-001, DEC-005, DEC-006, DEC-007, DEC-009

## MVP Linux

- AppImage: portátil; verificar sandbox/desktop integration y update policy.
- `.deb`: Debian/Ubuntu; scripts mínimos y uninstall limpio.
- `.rpm`: Fedora/openSUSE; spec reproducible.
- Web: assets estáticos versionados, CSP y Subresource-free bundle.
- Futuro: Flatpak tras evaluar portals/capabilities; AUR mediante PKGBUILD mantenido aparte.

## Release artifacts

Binarios/paquetes, checksums, firmas, SBOM CycloneDX/SPDX, provenance, source tarball, changelog y compatibility matrix.

Cada release publica, como mínimo:

| Artefacto | Descripción | Estado |
|---|---|---|
| Binarios/paquetes | AppImage, `.deb`, `.rpm` y bundle web (ver MVP Linux). | draft |
| Checksums | `SHA256SUMS` por release; verificación `sha256sum --check`. | draft |
| Firmas | Firma de artefactos y de `SHA256SUMS`. Mecanismo PENDIENTE (DEC-006). | PENDIENTE DEC-006 |
| SBOM | CycloneDX y/o SPDX con el inventario de dependencias. | draft |
| Provenance | Atestación SLSA/in-toto ligada a commit y workflow. | draft |
| Source tarball | Código fuente reproducible del commit taggeado. | draft |
| Changelog | Formato Keep a Changelog; cambios por versión. | draft |
| Compatibilidad | Matriz target/arquitectura/dependencias por release. | draft |

- **Checksums**: el digest es la identidad del artefacto y se reutiliza en la promoción (sin rebuild).
- **Firmas**: la firma se aplica una vez; la verificación es requisito de instalación del updater.
- **SBOM y provenance**: se generan en el mismo job de build/attest (ver `docs/09-quality/ci-cd.md`).
- **Source tarball**: garantiza reproducibilidad y auditoría independiente del binario.
- **Compatibilidad**: matriz mínima x86_64 (oficial) y aarch64 (experimental), sujeta a DEC-008.
- PENDIENTE: el pipeline de build/attest/SBOM y los términos de licencia dependen de DEC-005 y DEC-006.

## Canales

`nightly`, `beta`, `stable`; promoción, no rebuild. Rollback documentado. El updater de Tauri no se habilita hasta aceptar DEC-007 y verificar firmas/permisos.

| Canal | Origen | Estabilidad | Promoción |
|---|---|---|---|
| `nightly` | build de `main` | inestable | no promueve directamente |
| `beta` | nightly promovido | candidata | promoción sin rebuild |
| `stable` | beta promovido | publicada | promoción sin rebuild |

- **Promoción sin rebuild**: el mismo artefacto (mismos digests y atestación) se etiqueta de canal;
  recompilar invalida provenance y firma.
- **Rollback**: revertir a la release anterior por digest, documentando el canal afectado.
- **Updater**: **supeditado a DEC-007**. No se habilita hasta aceptar DEC-007 y verificar
  firmas/permisos contra la política vigente (ver `docs/09-quality/ci-cd.md`).
