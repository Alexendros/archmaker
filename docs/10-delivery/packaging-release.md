---
id: DOC-DEL-PKG-001
phase: MVP
priority: P1
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
---

# Packaging y release

- Estado: draft
- Propietario: Release
- Última revisión: 2026-10-05
- Requisitos relacionados: FR-EXPORT-001, NFR-OBS-001, NFR-SEC-001, NFR-PORT-001
- Sustituye / sustituido por: —
- Decisiones aplicables: DEC-001, DEC-005, DEC-006, DEC-007 (aceptadas 2026-10-05); DEC-009 (propuesta)

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
| Firmas | Firma de artefactos y de `SHA256SUMS` con Sigstore (+ offline evaluada). | Resuelto por DEC-006 |
| SBOM | CycloneDX y/o SPDX con el inventario de dependencias. | draft |
| Provenance | Atestación SLSA v1.2/in-toto ligada a commit y workflow. | draft |
| Source tarball | Código fuente reproducible del commit taggeado. | draft |
| Changelog | Formato Keep a Changelog; cambios por versión. | draft |
| Compatibilidad | Matriz target/arquitectura/dependencias por release. | draft |

- **Checksums**: el digest es la identidad del artefacto y se reutiliza en la promoción (sin rebuild).
- **Firmas**: la firma se aplica una vez; la verificación es requisito de instalación del updater.
- **SBOM y provenance**: se generan en el mismo job de build/attest (ver `docs/09-quality/ci-cd.md`).
- **Source tarball**: garantiza reproducibilidad y auditoría independiente del binario.
- **Compatibilidad**: matriz mínima Arch Linux **x86_64 únicamente** (DEC-008, aceptada 2026-10-05); aarch64 fuera de alcance.
- Pendiente de implementación: el pipeline de build/attest/SBOM y la firma real (DEC-005 y DEC-006 aceptadas; falta la Etapa C).

## Canales

`nightly`, `beta`, `stable`; promoción, no rebuild. Rollback documentado. El updater de Tauri se habilita en MVP (DEC-007): canal firmado, verificación de firma y fail-open offline (ADR-0009).

| Canal | Origen | Estabilidad | Promoción |
|---|---|---|---|
| `nightly` | build de `main` | inestable | no promueve directamente |
| `beta` | nightly promovido | candidata | promoción sin rebuild |
| `stable` | beta promovido | publicada | promoción sin rebuild |

- **Promoción sin rebuild**: el mismo artefacto (mismos digests y atestación) se etiqueta de canal;
  recompilar invalida provenance y firma.
- **Rollback**: revertir a la release anterior por digest, documentando el canal afectado.
- **Updater**: **en alcance MVP por DEC-007**. Canal firmado (DEC-006), endpoint por allowlist
  con TLS, verificación de firma, rollback y **fail-open offline** (la app funciona sin red).
  Ver `ADR-0009-actualizacion-firmada-mvp.md` y `docs/09-quality/ci-cd.md`.

## Apéndice I11 — Canal, versión mínima, rollback y verificación offline (T-I11-02/T-I11-03)

Fase I11 (2026-10-06). Decisiones: DEC-006 (Sigstore keyless + verificación offline),
DEC-007 (updater en MVP), ADR-0008 (catálogos firmados), ADR-0009 (actualización firmada).
Pipeline: `.github/workflows/release.yml`; raíces: `trusted_root.json` (raíz del repo).

| Canal | Artefacto | Firma | Requisito de instalación |
|---|---|---|---|
| `nightly` | build de `main` (mismos digests) | Sigstore keyless (Fulcio/Rekor) | firma válida o rechazo |
| `beta` | nightly promovido, sin rebuild | la del nightly promovido | firma válida + versión ≥ mínima |
| `stable` | beta promovido, sin rebuild | la del beta promovido | firma válida + versión ≥ mínima |

- **Versión mínima**: cada canal declara `minVersion`; el updater rechaza
  `versión < minVersion` (anti-downgrade) y rechaza artefactos con firma
  inválida o ausente (TST-SUP-001). Sin red, la app arranca y opera con
  normalidad (fail-open offline, `NFR-OFF-001`); la comprobación solo ocurre
  con conectividad y nunca bloquea el uso.
- **Rollback**: revertir al digest anterior publicado del mismo canal
  (promoción inversa, sin rebuild), documentando canal afectado y motivo;
  el digest restaurado ya está firmado y atestado, no se re-firma.
- **Verificación offline**: `cosign verify-blob --trusted-root trusted_root.json`
  (sin red). Estado actual: `trusted_root.json` es **placeholder documentado**
  (T-I11-02); `release.yml` hace SKIP —nunca verde fingido— hasta materializar
  las raíces reales (procedimiento dentro del propio fichero).
- **Provenance**: predicado SLSA v1 (`slsa-provenance.json`) generado en
  `release.yml`; la atestación GitHub (`actions/attest-*`) queda pendiente de
  pin SHA verificado (Etapa C, ver `docs/09-quality/ci-cd.md`).
