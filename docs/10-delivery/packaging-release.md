---
id: DOC-DEL-PKG-001
phase: MVP
priority: P1
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
releaseStatus: eligible
evidence:
  - docs/10-delivery/iso05-scope.md (ISO-05, 2026-10-25)
owners:
  - release
reviewers:
  - independent-reviewer
---

# Packaging y release

- Estado: accepted (canal F8 accepted; F9 sin release binaria publicada hasta `v0.3.0`)
- Propietario: Release
- Última revisión: 2026-10-25
- Requisitos relacionados: FR-EXPORT-001, NFR-OBS-001, NFR-SEC-001, NFR-PORT-001, NFR-MARK-001, NFR-COST-001
- Sustituye / sustituido por: —
- Decisiones aplicables: DEC-001, DEC-005, DEC-006, DEC-007, DEC-009, DEC-011; ADR-0010 (accepted, opción 3)

## Dos actos de release

Hay dos actos distintos. Este documento los separa. La release de paquete AppImage Forge
se autoriza por MVP-6/`accepted` (solo esa plataforma) y su primera publicación real es
`v0.3.0`; ver `closure/f9-release.md` para la historia del intento `v0.1.0`.

### Release de plan

Se asigna a un hito de GitHub (`planning-v*`, `MVP-*`, `v1`). Se publica cuando:

1. Existe un tag `implementation-baseline-*` o `planning-v*` apuntando a un commit de `main`.
2. El CI de ese commit está en verde.
3. El cuerpo de la GitHub Release lista las issues del hito asociado.

Artefactos: el código del tag que GitHub adjunta y las notas. Sin binarios, sin firmas y sin paquetes. El workflow `.github/workflows/plan-release.yml` crea la release de notas al pushear esos tags; no llama a `release.yml`.

| Hito                | Tag                                       | Estado del hito    |
| ------------------- | ----------------------------------------- | ------------------ |
| `planning-v1`       | `planning-v1`                             | cerrado (historia) |
| `planning-v1.1`     | `planning-v1.1`                           | cerrado (historia) |
| `MVP-0.1` … `MVP-5` | `implementation-baseline-mvp0.1` … `mvp5` | cerrado (historia) |
| `MVP-6`, `v1`       | (cuando exista el tag)                    | abierto            |

### Release de paquete

Sigue la política de artefactos, canales y firma de este documento. **Canal autorizado (F8 /
DEC-011 / MVP-6 accepted):**

| Campo            | Valor                                                  |
| ---------------- | ------------------------------------------------------ |
| Plataforma       | AppImage Linux x86_64                                  |
| Nombre visible   | Forge (ADR-0012; `v0.1.0` histórico Neubat / ADR-0010) |
| Hosting          | GitHub Release del tag `v*` (`release.yml`)            |
| Familias no Done | `.deb`, `.rpm`, OBS, AUR, Flatpak → deferred/blocked   |

F9 intentó construir desde `RC_SHA` `9297ed7…`, pero el workflow terminó `cancelled` y no
publicó nada. Los tags `v0.1.0` y `v0.2.0` carecen de GitHub Release (`/releases` = `[]`;
`/releases/tags/*` = 404), por lo que `v0.1.0` se declara **no publicado** (histórico,
nombre anterior). Residual DEC-006 / cosign local y 3× install GUI siguen abiertos.

Desde `0.2.0`, un tag `v*` anota la versión de código y no publica el paquete. El job de
AppImage en `release.yml` solo corre con `workflow_dispatch` y `publish_package=true`, cuando
el AppImage esté funcional y validado con tests. La primera release binaria real es
`v0.3.0` (2026-10-25), verificada en ISO-05 (`docs/10-delivery/iso05-scope.md`).

Canales `nightly` / `beta` / `stable` son política de promoción del paquete. No son tags que
`plan-release.yml` dispare. El AUR, si se reabre, es PKGBUILD fuente, no repositorio binario
de pacman.

## MVP Linux

- **AppImage (autorizado):** portátil Linux x86_64; nombre visible Forge; verificar sandbox /
  desktop integration y update policy en F9.
- `.deb`: deferred (Debian/Ubuntu; scripts mínimos y uninstall limpio).
- `.rpm`: deferred (Fedora/openSUSE; spec reproducible).
- Web: assets estáticos versionados, CSP y Subresource-free bundle (no es el canal F9).
- Futuro: Flatpak tras evaluar portals/capabilities; AUR mediante PKGBUILD mantenido aparte
  (nunca como repo binario en este alcance).

## Release artifacts

Binarios/paquetes, checksums, firmas, SBOM CycloneDX/SPDX, provenance, source tarball, changelog y compatibility matrix.

Cada release publica, como mínimo:

| Artefacto         | Descripción                                                              | Estado                            |
| ----------------- | ------------------------------------------------------------------------ | --------------------------------- |
| Binarios/paquetes | **AppImage Linux x86_64 (Forge)**; `.deb`/`.rpm` deferred.               | `v0.3.0` (primera real)           |
| Checksums         | `SHA256SUMS` por release; verificación `sha256sum --check`.              | `v0.3.0`                          |
| Firmas            | Firma de artefactos y de `SHA256SUMS` con Sigstore (+ offline evaluada). | `v0.3.0`; cosign local deferred   |
| SBOM              | CycloneDX y/o SPDX con el inventario de dependencias.                    | `v0.3.0` (`sbom.cdx.json`)        |
| Provenance        | Atestación SLSA v1 / in-toto ligada a commit y workflow.                 | `v0.3.0` (`slsa-provenance.json`) |
| Source tarball    | Código fuente reproducible del commit taggeado.                          | via GitHub Release source         |
| Changelog         | Formato Keep a Changelog; cambios por versión.                           | `[0.3.0]`                         |
| Compatibilidad    | Matriz: Forge AppImage / Linux x86_64 (DEC-008).                         | publicado (canal único)           |

- **Checksums**: el digest es la identidad del artefacto y se reutiliza en la promoción (sin rebuild).
- **Firmas**: la firma se aplica una vez; la verificación es requisito de instalación del updater.
- **SBOM y provenance**: se generan en el mismo job de build/attest (ver `docs/09-quality/ci-cd.md`).
- **Source tarball**: garantiza reproducibilidad y auditoría independiente del binario.
- **Compatibilidad**: matriz mínima Arch Linux **x86_64 únicamente** (DEC-008, aceptada 2026-10-05); aarch64 fuera de alcance.
- Etapa C en `v0.3.0`: SBOM + provenance + checksums publicados; cosign local y
  verificación estrictamente offline (firmas bundle) siguen pendientes (DEC-006).

## Canales

`nightly`, `beta`, `stable`; promoción, no rebuild. Rollback documentado. El updater de Tauri se habilita en MVP (DEC-007): canal firmado, verificación de firma y fail-open offline (ADR-0009).

| Canal     | Origen            | Estabilidad | Promoción                |
| --------- | ----------------- | ----------- | ------------------------ |
| `nightly` | build de `main`   | inestable   | no promueve directamente |
| `beta`    | nightly promovido | candidata   | promoción sin rebuild    |
| `stable`  | beta promovido    | publicada   | promoción sin rebuild    |

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

| Canal     | Artefacto                        | Firma                           | Requisito de instalación        |
| --------- | -------------------------------- | ------------------------------- | ------------------------------- |
| `nightly` | build de `main` (mismos digests) | Sigstore keyless (Fulcio/Rekor) | firma válida o rechazo          |
| `beta`    | nightly promovido, sin rebuild   | la del nightly promovido        | firma válida + versión ≥ mínima |
| `stable`  | beta promovido, sin rebuild      | la del beta promovido           | firma válida + versión ≥ mínima |

- **Versión mínima**: cada canal declara `minVersion`; el updater rechaza
  `versión < minVersion` (anti-downgrade) y rechaza artefactos con firma
  inválida o ausente (TST-SUP-001). Sin red, la app arranca y opera con
  normalidad (fail-open offline, `NFR-OFF-001`); la comprobación solo ocurre
  con conectividad y nunca bloquea el uso.
- **Rollback**: revertir al digest anterior publicado del mismo canal
  (promoción inversa, sin rebuild), documentando canal afectado y motivo;
  el digest restaurado ya está firmado y atestado, no se re-firma.
- **Verificación de firmas**: `cosign verify-blob` con pinning de identidad
  (`^https://github.com/Alexendros/archmaker/`) e issuer
  (`https://token.actions.githubusercontent.com`), ejecutada en CI contra la
  instancia pública de Sigstore. `trusted_root.json` se materializa en CI
  (`cosign trusted-root create`) y se publica junto a las firmas como
  evidencia. La verificación **estrictamente offline** (`--trusted-root`)
  requiere firmas en formato bundle (`--new-bundle-format`) y queda diferida
  como residual de DEC-006 (T-I11-02).
- **Provenance**: predicado SLSA v1 (`slsa-provenance.json`) generado en
  `release.yml`; la atestación GitHub (`actions/attest-*`) queda pendiente de
  pin SHA verificado (Etapa C, ver `docs/09-quality/ci-cd.md`).
