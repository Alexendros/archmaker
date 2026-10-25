---
id: DOC-DEL-ISO-001
phase: v1
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: complete
verificationStatus: passed
releaseStatus: ineligible
evidence:
  - .github/workflows/iso.yml (ISO de humo + QEMU UEFI, run 40160364257, 5m25s)
  - scripts/build-iso.sh + scripts/boot-test.sh (variante A, ISO 483733504 B, arranque UEFI 50 s)
  - archinstall --dry-run --silent (config nativa 2.8.6, exit 0)
owners:
  - architecture
reviewers:
  - independent-reviewer
dependsOn:
  - id: ADR-0010
  - id: ADR-0003
  - id: ADR-0020
  - id: ADR-0021
---

# Alcance de construcción — perfil archiso e ISO en Docker

- Documento: DOC-DEL-ISO-001
- Estado: accepted
- Propietario: Arquitectura
- Fecha: 2026-10-25
- Aprobación: operador, 2026-10-25 (reorientación)
- Requisitos: FR-EXPORT-001, NFR-SEC-001, NFR-DET-001

## Objetivo

Forge valida la selección y produce dos artefactos consumibles por las
herramientas oficiales: un perfil archiso completo (overlay sobre `releng` o
`baseline`) y un `user_configuration.json` nativo de archinstall 4.5. La ISO se
construye con `mkarchiso` dentro de un contenedor Docker fijado por digest y debe
arrancar en UEFI (QEMU + OVMF) en CI.

La instalación del SSD la ejecuta archinstall en la sesión viva. Forge no escribe
discos, no usa root, no lanza un runner y no acepta shell.

Fuera de alcance: escritura de discos, runner de instalación, aarch64, Enterprise,
actualizaciones firmadas, UKI/ESP, A6 y el contrato de aplicación.

## Cadena

1. Manifiesto sellado (dominio Rust, sha256 canónico).
2. Overlay archiso: `profiledef.sh`, `packages.x86_64`, `pacman.conf` y
   `airootfs/root/user_configuration.json`.
3. Configuración archinstall nativa (esquema 2.8.6; `disk_config` ausente para que
   archinstall guíe el disco).
4. `mkarchiso` en Docker (`archlinux@sha256:…`) con privilegios mínimos
   (`SYS_ADMIN`, apparmor y seccomp sin confinar) y overlay en solo lectura.
5. ISO + `SHA256SUMS`; arranque UEFI verificado con QEMU + OVMF.

## Requisitos

- FR-EXPORT-001: exportación determinista; perfil y configuración derivados del
  manifiesto, sin duplicar reglas en TypeScript.
- NFR-SEC-001: sin root, sin shell y sin discos; contenedor efímero, overlay en
  solo lectura y salida acotada.
- NFR-DET-001: mismos bytes para el mismo manifiesto; `SHA256SUMS` por ISO.

## Evidencia

- `crates/archmaker-core/fixtures/archinstall-user-configuration-4.5.json`
  validado con `archinstall --config … --dry-run` (Arch, archinstall 4.5).
- `scripts/build-iso.sh` y `docker/archiso.Dockerfile`: ISO de humo de
  483.733.504 bytes, doble El Torito (BIOS + UEFI), 93 s.
- `scripts/boot-test.sh`: arranque UEFI hasta `archlinux login:` en 50 s.
- CI: job «ISO de humo + arranque QEMU UEFI (G9)» en verde (5m25s).

## Exclusiones

`pacstrap` como constructor, `mkarchiso` en el host, `sudo`/`pkexec`, escritura de
discos, aarch64, Enterprise y runner de instalación.
