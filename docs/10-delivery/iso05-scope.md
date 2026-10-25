---
id: DOC-DEL-ISO05-001
phase: v1
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
releaseStatus: released
evidence:
  - cargo test --workspace --all-targets (exit 0)
  - pnpm --filter @archmaker/web build (exit 0; wasm 1677080 B, css 30530 B)
  - Playwright en serie - e2e WASM x3, P5 27, axe 12, a11y estatico, flow 74, visual 4
  - .github/workflows/iso.yml G9 (ISO de humo + QEMU UEFI) en main
  - tools/check_product_naming.py (T-NAME-01..04 + T-MIG-01..04 OK)
  - https://github.com/Alexendros/archmaker/releases/tag/v0.3.0 (publicada, 2026-10-25)
  - .github/workflows/release.yml run 40184194831 (success; AppImage + firmas + SBOM + provenance)
owners:
  - release
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0020
  - id: ADR-0021
  - id: DOC-DEL-ISO-001
---

# Alcance ISO-05 — cierre y verificación de la reorientación

- Documento: DOC-DEL-ISO05-001
- Estado: accepted
- Propietario: Release
- Fecha: 2026-10-25
- Aprobación: operador, 2026-10-25 (plan ISO-05)
- Requisitos: FR-EXPORT-001, NFR-SEC-001, NFR-DET-001

## Objetivo

Cerrar la reorientación aprobada el 2026-10-25 (DEC-014): la GUI fina sin root
valida la selección y produce un perfil archiso completo más un
`user_configuration.json` real de archinstall; `mkarchiso` construye la ISO
únicamente dentro de Docker. El código ya está en `main` (ISO-01..ISO-04); ISO-05
verifica sus gates y publica el primer release real bajo el nombre Forge.

## Alcance

Dentro:

- Verificar y sellar ADR-0020 y ADR-0021 (`verificationStatus: verified`).
- Verificar `roadmap.md` y cerrar la aprobación funcional F7.
- Ejecutar la suite completa (Rust, TypeScript/Vite, Playwright, CI `iso.yml`
  con ISO de humo y arranque QEMU UEFI, comprobación de nomenclatura).
- Publicar `v0.3.0` (GitHub Release + AppImage) y actualizar CHANGELOG y
  `packaging-release.md`.

Fuera de alcance (no se reabre el veredicto): runner de instalación, escritura de
discos (A6), root, shell arbitraria, aarch64, Enterprise, UKI/ESP, contrato JSON
de aplicación y digest v2. La instalación del SSD sigue siendo responsabilidad de
archinstall en la sesión viva.

## Criterios de aceptación

- [x] ADR-0020 `verificationStatus: verified`: la cara valida las reglas y genera
      perfil + `user_configuration.json` sin root, shell ni acceso a discos.
- [x] ADR-0021 `verificationStatus: verified`: `mkarchiso` solo en Docker con
      overlay de solo lectura y `scripts/build-iso.sh` como único entrypoint;
      ISO de humo y arranque UEFI reproducidos.
- [x] `roadmap.md` `verificationStatus: verified` y F7 aprobada.
- [x] `cargo test` del workspace en verde.
- [x] Build TypeScript/Vite en verde.
- [x] Playwright (3 configuraciones) en verde.
- [x] CI `iso.yml` (ISO de humo + arranque QEMU UEFI, G9) en verde.
- [x] `tools/check_product_naming.py` en verde (nombre visible Forge).
- [x] Release `v0.3.0` publicada con AppImage; CHANGELOG y
      `packaging-release.md` coherentes.

## Plan de verificación

1. `cargo test --workspace --all-targets` (dominio, reglas, adapters, constitute).
2. `pnpm --filter @archmaker/web build` (WASM release, `tsc --noEmit`, Vite).
3. Playwright en serie: `test-f7:e2e-wasm-x3`, `test-p5:acceptance`, `test-a11y`,
   `test-a11y:axe`, `test-a11y:flow`, `test-a11y:visual`. Nunca en paralelo: los
   suites comparten el puerto 5173.
4. `bash scripts/build-iso.sh` (ISO de humo) y `bash scripts/boot-test.sh`
   (arranque UEFI), o su equivalente en CI `iso.yml`.
5. `python3 tools/check_product_naming.py`.
6. Publicación: tag `v0.3.0`, workflow de release con `publish_package=true`,
   verificación de assets (AppImage).

## Evidencia de verificación (2026-10-25)

| Gate                 | Comando                                            | Resultado                              |
| -------------------- | -------------------------------------------------- | -------------------------------------- |
| Rust workspace       | `cargo test --workspace --all-targets`             | exit 0                                 |
| Build GUI            | `pnpm --filter @archmaker/web build`               | exit 0; wasm 1 677,08 kB, css 30,53 kB |
| GUI ×3 (WASM real)   | `pnpm --filter @archmaker/web test-f7:e2e-wasm-x3` | 3 × 4 passed                           |
| Aceptación P5        | `pnpm --filter @archmaker/web test-p5:acceptance`  | 27 passed                              |
| Axe/teclado/viewport | `pnpm --filter @archmaker/web test-a11y:axe`       | 12 passed                              |
| A11y estático        | `pnpm test-a11y`                                   | 0 fallos                               |
| Flujo mock           | `pnpm test-a11y:flow`                              | 74 passed                              |
| Regresión visual     | `pnpm test-a11y:visual`                            | 4 passed                               |
| Nomenclatura         | `python3 tools/check_product_naming.py`            | PASS T-NAME-01..04, T-MIG-01..04       |
| ISO + arranque UEFI  | CI `iso.yml` (G9)                                  | verde en `main`                        |
| Release `v0.3.0`     | `release.yml` con `publish_package=true`           | publicada; run 40184194831 success     |

## Residuales declarados

- Firma cosign local diferida (MVP-6-02); no bloquea el release AppImage.
- La verificación de firmas del release se hace en CI contra la instancia pública
  de Sigstore con pinning de identidad e issuer; la verificación **estrictamente
  offline** (`--trusted-root`) requiere firmas en formato bundle
  (`--new-bundle-format`) y sigue diferida (DEC-006, T-I11-02).
- Verificación de la GUI en las tres configuraciones de release queda registrada
  con su evidencia; un fallo abre incidencia, no amplía el alcance.
- Cualquier desviación se documenta en `docs/10-delivery/closure/` sin reabrir
  los elementos fuera de alcance.

## Trazabilidad

- Reorientación: DEC-014; ADR-0020 (GUI fina), ADR-0021 (ISO en Docker).
- Construcción: DOC-DEL-ISO-001 (`build-scope.md`).
- Roadmap: DOC-DEL-RMAP-001; release: DOC-DEL-PKG-001.
