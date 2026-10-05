# Packaging y release

## MVP Linux

- AppImage: portátil; verificar sandbox/desktop integration y update policy.
- `.deb`: Debian/Ubuntu; scripts mínimos y uninstall limpio.
- `.rpm`: Fedora/openSUSE; spec reproducible.
- Web: assets estáticos versionados, CSP y Subresource-free bundle.
- Futuro: Flatpak tras evaluar portals/capabilities; AUR mediante PKGBUILD mantenido aparte.

## Release artifacts

Binarios/paquetes, checksums, firmas, SBOM CycloneDX/SPDX, provenance, source tarball, changelog y compatibility matrix.

## Canales

`nightly`, `beta`, `stable`; promoción, no rebuild. Rollback documentado. El updater de Tauri no se habilita hasta aceptar DEC-007 y verificar firmas/permisos.
