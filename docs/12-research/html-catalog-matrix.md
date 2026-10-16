---
id: DOC-RES-HTML-001
phase: planning
priority: P0
documentStatus: in-review
approvalStatus: pending
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - data
reviewers:
  - independent-reviewer
---

# Matriz del canon visual v5.1

- ID: DOC-RES-HTML-001
- Estado: in-review
- Propietario: Datos
- Última revisión: 2026-10-14
- Requisitos relacionados: FR-CAT-001, FR-PRESET-001, CON-005, CON-011, CON-012
- Sustituye / sustituido por: —

Fuente de opciones: `reference/v5.1/index.html` (el operador lo nombra ArchMaker.html; no hay otro fichero). No se edita. El bloque entre marcadores lo regenera `tools/html_catalog_matrix.py`. La clase no afirma pertenencia a un repo oficial: eso espera captura primaria. «no verificada» cubre versiones y tamaños del HTML (CON-011).

Un id canónico «sí» en catálogo es el que ya está en `crates/archmaker-core/fixtures/local-catalog-mvp1.json` (MVP-5-02: las 54 filas de la matriz).

<!-- BEGIN HTML-MATRIX -->

| Paso        | Sección      | Id HTML                   | Paquete                   | Id canónico propuesto                   | En catálogo | Clase                           | Versión/tamaño |
| ----------- | ------------ | ------------------------- | ------------------------- | --------------------------------------- | ----------- | ------------------------------- | -------------- |
| base        | kernel       | `linux`                   | `linux`                   | `archmaker.kernel.linux`                | sí          | sin-clasificar                  | no verificada  |
| base        | kernel       | `linux-zen`               | `linux-zen`               | `archmaker.kernel.zen`                  | sí          | sin-clasificar                  | no verificada  |
| base        | kernel       | `linux-hardened`          | `linux-hardened`          | `archmaker.kernel.linux-hardened`       | sí          | sin-clasificar; cmd no migrable | no verificada  |
| base        | filesystem   | `btrfs-progs`             | `btrfs-progs`             | `archmaker.filesystem.btrfs`            | sí          | sin-clasificar                  | no verificada  |
| base        | filesystem   | `e2fsprogs`               | `e2fsprogs`               | `archmaker.filesystem.ext4`             | sí          | sin-clasificar                  | no verificada  |
| base        | base-tools   | `base-devel`              | `base-devel`              | `archmaker.pkg.base-devel`              | sí          | sin-clasificar                  | no verificada  |
| base        | boot         | `systemd-boot`            | `systemd-boot`            | `archmaker.boot.systemd-boot`           | sí          | sin-clasificar                  | no verificada  |
| base        | boot         | `grub`                    | `grub`                    | `archmaker.boot.grub`                   | sí          | sin-clasificar                  | no verificada  |
| base        | boot         | `limine`                  | `limine`                  | `archmaker.boot.limine`                 | sí          | sin-clasificar                  | no verificada  |
| environment | compositor   | `hyprland`                | `hyprland`                | `archmaker.compositor.hyprland`         | sí          | sin-clasificar                  | no verificada  |
| environment | compositor   | `gnome`                   | `gnome`                   | `archmaker.compositor.gnome`            | sí          | sin-clasificar                  | no verificada  |
| environment | compositor   | `sway`                    | `sway`                    | `archmaker.compositor.sway`             | sí          | sin-clasificar                  | no verificada  |
| environment | compositor   | `cosmic`                  | `cosmic-epoch`            | `archmaker.compositor.cosmic`           | sí          | sin-clasificar                  | no verificada  |
| environment | dm           | `gdm`                     | `gdm`                     | `archmaker.dm.gdm`                      | sí          | sin-clasificar                  | no verificada  |
| environment | dm           | `sddm`                    | `sddm`                    | `archmaker.dm.sddm`                     | sí          | sin-clasificar                  | no verificada  |
| environment | dm           | `greetd-tuigreet`         | `greetd-tuigreet`         | `archmaker.dm.greetd-tuigreet`          | sí          | sin-clasificar                  | no verificada  |
| libpacks    | office-pack  | `libreoffice-fresh`       | `libreoffice-fresh`       | `archmaker.pkg.libreoffice-fresh`       | sí          | sin-clasificar                  | no verificada  |
| libpacks    | office-pack  | `hunspell-es`             | `hunspell-es_es`          | `archmaker.pkg.hunspell-es`             | sí          | sin-clasificar                  | no verificada  |
| libpacks    | office-pack  | `poppler`                 | `poppler`                 | `archmaker.pkg.poppler`                 | sí          | sin-clasificar                  | no verificada  |
| libpacks    | office-pack  | `onlyoffice-bin`          | `onlyoffice-bin`          | `archmaker.pkg.onlyoffice-bin`          | sí          | no-redistribuible-candidato     | no verificada  |
| libpacks    | office-pack  | `pdfarranger`             | `pdfarranger`             | `archmaker.pkg.pdfarranger`             | sí          | sin-clasificar                  | no verificada  |
| libpacks    | office-pack  | `nextcloud-client`        | `nextcloud-client`        | `archmaker.pkg.nextcloud-client`        | sí          | sin-clasificar                  | no verificada  |
| libpacks    | dev-pack     | `git`                     | `git`                     | `archmaker.pkg.git`                     | sí          | sin-clasificar                  | no verificada  |
| libpacks    | dev-pack     | `python`                  | `python`                  | `archmaker.pkg.python`                  | sí          | sin-clasificar                  | no verificada  |
| libpacks    | dev-pack     | `nodejs`                  | `nodejs`                  | `archmaker.pkg.nodejs`                  | sí          | sin-clasificar                  | no verificada  |
| libpacks    | dev-pack     | `docker`                  | `docker`                  | `archmaker.pkg.docker`                  | sí          | sin-clasificar                  | no verificada  |
| libpacks    | dev-pack     | `podman`                  | `podman`                  | `archmaker.pkg.podman`                  | sí          | sin-clasificar                  | no verificada  |
| libpacks    | dev-pack     | `rustup`                  | `rustup`                  | `archmaker.pkg.rustup`                  | sí          | sin-clasificar                  | no verificada  |
| libpacks    | dev-pack     | `go`                      | `go`                      | `archmaker.pkg.go`                      | sí          | sin-clasificar                  | no verificada  |
| libpacks    | dev-pack     | `jdk-openjdk`             | `jdk-openjdk`             | `archmaker.pkg.jdk-openjdk`             | sí          | sin-clasificar                  | no verificada  |
| libpacks    | gaming-pack  | `vulkan-icd-loader`       | `vulkan-icd-loader`       | `archmaker.pkg.vulkan-icd-loader`       | sí          | sin-clasificar                  | no verificada  |
| libpacks    | gaming-pack  | `lib32-vulkan-icd-loader` | `lib32-vulkan-icd-loader` | `archmaker.pkg.lib32-vulkan-icd-loader` | sí          | sin-clasificar                  | no verificada  |
| libpacks    | gaming-pack  | `gamemode`                | `gamemode`                | `archmaker.pkg.gamemode`                | sí          | sin-clasificar                  | no verificada  |
| libpacks    | gaming-pack  | `mangohud`                | `mangohud`                | `archmaker.pkg.mangohud`                | sí          | sin-clasificar                  | no verificada  |
| libpacks    | gaming-pack  | `gamescope`               | `gamescope`               | `archmaker.pkg.gamescope`               | sí          | sin-clasificar                  | no verificada  |
| libpacks    | gaming-pack  | `protonup-qt`             | `protonup-qt`             | `archmaker.pkg.protonup-qt`             | sí          | sin-clasificar                  | no verificada  |
| libpacks    | gaming-pack  | `lutris`                  | `lutris`                  | `archmaker.pkg.lutris`                  | sí          | sin-clasificar                  | no verificada  |
| apps        | browser      | `firefox`                 | `firefox`                 | `archmaker.browser.firefox`             | sí          | sin-clasificar                  | no verificada  |
| apps        | browser      | `vivaldi`                 | `vivaldi`                 | `archmaker.browser.vivaldi`             | sí          | sin-clasificar                  | no verificada  |
| apps        | browser      | `brave`                   | `brave-bin`               | `archmaker.browser.brave`               | sí          | no-redistribuible-candidato     | no verificada  |
| apps        | comm         | `discord`                 | `discord`                 | `archmaker.app.discord`                 | sí          | sin-clasificar                  | no verificada  |
| apps        | comm         | `telegram`                | `telegram-desktop`        | `archmaker.app.telegram`                | sí          | sin-clasificar                  | no verificada  |
| apps        | term         | `ghostty`                 | `ghostty`                 | `archmaker.app.ghostty`                 | sí          | sin-clasificar                  | no verificada  |
| apps        | term         | `kitty`                   | `kitty`                   | `archmaker.app.kitty`                   | sí          | sin-clasificar                  | no verificada  |
| apps        | term         | `micro`                   | `micro`                   | `archmaker.app.micro`                   | sí          | sin-clasificar                  | no verificada  |
| apps        | term         | `cursor`                  | `cursor-bin`              | `archmaker.app.cursor`                  | sí          | no-redistribuible-candidato     | no verificada  |
| apps        | term         | `btop`                    | `btop`                    | `archmaker.app.btop`                    | sí          | sin-clasificar                  | no verificada  |
| apps        | media        | `vlc`                     | `vlc`                     | `archmaker.app.vlc`                     | sí          | sin-clasificar                  | no verificada  |
| apps        | media        | `gimp`                    | `gimp`                    | `archmaker.app.gimp`                    | sí          | sin-clasificar                  | no verificada  |
| apps        | productivity | `obsidian`                | `obsidian`                | `archmaker.app.obsidian`                | sí          | sin-clasificar                  | no verificada  |
| apps        | productivity | `onlyoffice-standalone`   | `onlyoffice-bin`          | `archmaker.app.onlyoffice-standalone`   | sí          | no-redistribuible-candidato     | no verificada  |
| apps        | gaming       | `steam`                   | `steam`                   | `archmaker.app.steam`                   | sí          | sin-clasificar                  | no verificada  |
| build       | output       | `zstd`                    | `zstd`                    | `archmaker.build.zstd`                  | sí          | sin-clasificar                  | no verificada  |
| build       | output       | `xz`                      | `xz`                      | `archmaker.build.xz`                    | sí          | sin-clasificar                  | no verificada  |

Fijos declarados en el HTML: `btrfs-progs`, `base-devel`, `libreoffice-fresh`, `hunspell-es`, `poppler`, `git`, `python`, `nodejs`, `vulkan-icd-loader`, `lib32-vulkan-icd-loader`, `gamemode`.

| Regla HTML          | Tipo en el prototipo | Migración                                        |
| ------------------- | -------------------- | ------------------------------------------------ |
| `single-compositor` | error                | reescribir al AST aceptado; no copiar `count_gt` |
| `kernel-required`   | error                | reescribir al AST aceptado; no copiar `count_gt` |
| `browser-required`  | info                 | reescribir al AST aceptado; no copiar `count_gt` |
| `gdm-auto`          | info                 | reescribir al AST aceptado; no copiar `count_gt` |

| Preset             | Nombre           | Estado de requisito    |
| ------------------ | ---------------- | ---------------------- |
| `minimal-dev`      | Minimal Dev      | FR-PRESET-001 deferred |
| `full-workstation` | Full Workstation | FR-PRESET-001 deferred |
| `gaming`           | Gaming Station   | FR-PRESET-001 deferred |

Pipeline heredado: `validate`, `resolve`, `derive_palette`, `pacstrap`, `assemble`, `checksum`. `pacstrap` no migra a runtime.

Opciones con campo `cmd`: `linux-hardened`. El campo no entra al catálogo.
<!-- END HTML-MATRIX -->

## Lectura

- `sin-clasificar`: el HTML nombra un paquete y este repo aún no ha capturado si está en los repos oficiales de Arch.
- `no-redistribuible-candidato`: el nombre termina en `-bin` o el paquete declarado también. No se copia el binario a una imagen. Solo se conserva el nombre hasta leer la licencia de upstream.
- `cmd no migrable`: la opción puede existir en el catálogo; el campo `cmd` no.
- `pacstrap` aparece en el pipeline heredado y no es constructor de este producto.
