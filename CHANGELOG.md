# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.3.0] - 2026-10-25

Primera release real del proyecto: GitHub Release `v0.3.0` con AppImage de Forge, `SHA256SUMS`,
firma cosign keyless y SBOM. Cierra y verifica la reorientación (ISO-05, ADR-0020/ADR-0021).

### Added

- Verificación ISO-05 de la reorientación: `cargo test --workspace --all-targets`, build
  TypeScript/Vite/WASM, GUI EN SERIE (e2e WASM ×3, aceptación P5, axe, a11y estático y de flujo,
  regresión visual), CI `iso.yml` (G9: ISO de humo + arranque QEMU UEFI) y
  `tools/check_product_naming.py`. Evidencia en `docs/10-delivery/iso05-scope.md`.
- `docs/10-delivery/iso05-scope.md` (DOC-DEL-ISO05-001) y scaffold Autoworkspace
  `.autoworkspace/`.

### Fixed

- Estado de release corregido (2026-10-25): no existe GitHub Release ni AppImage publicado.
  Los tags `v0.1.0` y `v0.2.0` carecen de Release (`/releases` = `[]`; `/releases/tags/*` = 404).
  `v0.1.0` se declara no publicado; la primera release real es `v0.3.0`.
- Pipeline `release.yml`: instala `wasm-bindgen-cli 0.2.129` antes del build web y verifica las
  firmas Sigstore con pinning de identidad e issuer contra la instancia pública (`--trusted-root`
  exige firmas en formato bundle, diferido como residual de DEC-006).

### Changed

- Reorientación aprobada (2026-10-25): Forge será una GUI fina sin root que genera un perfil
  `archiso` completo y un `user_configuration.json` real de `archinstall`, y construye la ISO
  con `mkarchiso` dentro de Docker. Fuera de alcance: escritura de discos, `aarch64`,
  Enterprise y runner de instalación. Estado anterior congelado en el tag `pre-reorientacion`.
- ADR-0020 y ADR-0021 pasan a `verified`; `roadmap.md` a `verified` con la aprobación funcional
  F7 aprobada.
- Versión de workspace, Tauri y paquetes web a `0.3.0`.

## [0.2.0] - 2026-10-18

Versión de código de `main` desde `v0.1.0`, incluido el perfil vivo. No hay GitHub Release ni AppImage de esta versión. El contrato de productor (`PRODUCER_VERSION` y `PIPELINE_VERSION`) sigue en `0.1.0`. El tag `v0.1.0` no se mueve.

### Added

- Perfil vivo `archmaker-live-profile` y petición tipada `constitute_image` (botón «Pedir imagen»). La cara no ejecuta `mkarchiso` ni un shell. La ISO `Forge-aaa81a961b0a-x86_64.iso` se generó una vez en una VM Arch y no está en el repositorio. El arranque UEFI de esa ISO en VM cubre LiveISO. La instalación en SSD sigue `not_run`.
- Trazabilidad de las pruebas de constitución e ISO-02, y hashes as-built alineados con el árbol.
- Evolución Forge verificable (plan consolidado): contratos draft/manifiesto v2,
  experiencia única (sin Sencillo/Avanzado), catálogo Zen/ONLYOFFICE/Kitty/Ghostty,
  sonda Linux de solo lectura + plan UEFI/GPT/UKI, migración `legacyMetadata`,
  matriz T01–T44 / HA01–HA25 (`docs/10-delivery/closure/forge-acceptance-matrix.md`).
  **A6 (escritura en discos) fuera de alcance**; ApexShot CAT01 abierto; sin publicación.
- ADR-0013…0018 (UKI en ESP, estados ortogonales, handles, digests v2, navegadores/
  ONLYOFFICE, alcance declarativo con A6 excluida).

- Probe P6 residuales automatizables (`tools/run_p6_residual_probe.py`): 3× install limpia
  (FUSE) y offline (`bwrap --unshare-net` sobre AppRun extraído; evita `/tmp` noexec).
  Portal E2E rfd sigue blocked; gate P6 global blocked.
- Prove-It jsonschema: `validate_closure_evidence` avisa si la versión ≠ 4.23.0 y exige
  concordancia Draft202012 (`tools/test_validate_closure_jsonschema_pin.py`).

- Candidato y handoff P8–P9: `docs/10-delivery/closure/release-candidate.md` y
  `closure-handoff.md` identifican el tip post-P0–P7 sin publicar, firmar ni mover
  `v0.1.0`; firma/SBOM/revisión independiente y distribución quedan pending.
- Gobernanza documental P7: manual de instalación de Forge, capas en plan-status/go-no-go,
  reglas de nombre/evidencia en AGENTS.md, enlaces README/índice; historia `v0.1.0` intacta.
- Gate desktop/offline P6: el verificador admite `sourceSha` retrasado respecto de HEAD cuando
  el estado es `blocked`/`failed` (solo `passed` exige igualdad).

- Gate desktop/offline P6: estado **blocked** explícito con evidencia ausente nombrada
  (`docs/10-delivery/closure/p6-desktop-offline.md`, `tools/check_desktop_offline_gate.py`);
  humo local AppImage Forge + launch + IPC ACL en verde; residuales portal E2E / offline /
  3× install; Mock y AppImage `v0.1.0` histórico no cierran el gate.
- Ajuste P7: `check_desktop_offline_gate` permite `sourceSha` ≠ HEAD si status es
  `blocked`/`failed`; solo `passed` exige igualdad con HEAD.

- Aceptación de producto P5: corpus de entrega ≥20 fixtures/target + 4 rechazos,
  tres invocaciones Playwright separadas (`--workers=1 --retries=0`, no `--repeat-each`),
  T-MIG 20 ciclos / cuota / sesión previa `archmaker-profiles`, teclado en recorrido esencial,
  instancias de perfil sin colisión cruzada y guardas de generación stale/reentrada.
- Harness P95 de producto (`tools/run_product_p95_harness.py`): web-ready / web-prepare
  con WASM real (n≥30), informe `docs/10-delivery/closure/product-p95-report.json`;
  desktop blocked con evidencia ausente nombrada; `--allow-pending` no cierra aceptación.
- Integridad de evidencia P2: `technicalResult` concordante con `jsonschema==4.23.0`, 20 escenarios adversarios (`tools/test_evidence_adversarial.py`), import de evidencia desktop sin bypass, `tools/check_product_closure.py` de solo lectura.
- Runner F7 estricto (`--strict`): estados por comando/test, `technicalResult` separado de
  `result=pending`, pkg WASM completo, microbenchmark P95 nativo n=30
  (`tools/run_f7_p95_harness.py`, no certifica producto).
- Casos F7 atómicos (motor/export WASM, offline web/desktop, P95).
- Suite de diagramas Archify D01–D24 bajo `docs/02-architecture/diagrams/` (renderer fijado;
  regenerar con `tools/scaffold_archify_diagrams.py` + `tools/run_archify_diagrams.py`).

### Changed

- Pipeline Archify P4: `AS_BUILT_SOURCE_SHA` + manifiesto de rutas/hashes (ya no exige
  `sourceSha == HEAD`); `--check` de solo lectura; verificación del checkout del renderer;
  codeRefs verificables D01–D24; D01 sin descarga de distribución como llamada runtime;
  corrección viewport D17–D22; títulos Forge/ArchMaker (ADR-0012); pruebas negativas en
  `tools/test_run_archify_diagrams.py`.
- Nombre visible de la GUI: **Forge** (interfaz gráfica de ArchMaker); el motor y los
  ids técnicos `archmaker*` no cambian (ADR-0012 / DEC-012). La release `[0.1.0]`
  permanece con el nombre público histórico **Neubat**.
- `archify_ok` exige exit 0 y recibo `ok:true` (un `ok:true` no salva exit ≠ 0).
- Contrato de evidencia: campos opcionales `harness`, `technicalResult`, `commands`, `tests`,
  `approval` (`approvalRecordSha256` vs `approvalRecordCommit`); aprobación exige
  `technicalResult=passed` (null u omitido no aprueban).
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

| Campo             | Valor                                                              |
| ----------------- | ------------------------------------------------------------------ |
| Tag               | `v0.1.0`                                                           |
| `RC_SHA`          | `9297ed7cc47f42a9611aed0093b5ab040daca512`                         |
| `ARTIFACT_SHA256` | `6567464f5e2f21398fd3eef35592f0e9d377fd2cc7887c1e46df092b5a8e6d81` |

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

[Unreleased]: https://github.com/Alexendros/archmaker/compare/v0.2.0...HEAD
[0.2.0]: https://github.com/Alexendros/archmaker/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/Alexendros/archmaker/releases/tag/v0.1.0
