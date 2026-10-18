---
id: DOC-DEL-CLO-ISO04-001
phase: v1
priority: P0
documentStatus: draft
approvalStatus: pending
implementationStatus: partial
verificationStatus: partial
releaseStatus: ineligible
owners:
  - release
reviewers:
  - independent-reviewer
dependsOn:
  - id: DOC-DEL-ISO-001
    expectedDocumentStatus: accepted
---

# ISO-04 — corrida en VM y arranque UEFI

## Resultado

La corrida de `mkarchiso` se ejecutó una vez, dentro de una máquina virtual Arch x86_64, porque el host es Ubuntu 26.04 y no tiene `mkarchiso` ni `pacman`. No es un fichero de sustitución. P6 y F7 no cambian. `technicalResult` de cierre de producto no se emite. El arranque UEFI de esta ISO cubre el criterio LiveISO. SSD sigue `not_run`.

| Dato          | Valor                                                              |
| ------------- | ------------------------------------------------------------------ |
| Imagen        | `Forge-aaa81a961b0a-x86_64.iso` (1 085 308 928 bytes)              |
| SHA-256       | `82f5afb1d80b682952ac616e4d52803a8730b6b85ab26a2dfd8c0ea2223d0745` |
| Manifiesto    | `aaa81a961b0aac65d58bfad91f59f8755935872b5f2996e4cd04e92dbcdca1f0` |
| Ruta local    | `/home/alexendros/archmaker-iso-build/out/` (fuera del repo)       |
| `mkarchiso`   | 2026-10-18 20:15:14Z → 20:19:19Z (4 min 5 s), archiso 91           |
| Arranque UEFI | OVMF, menú `Forge (x86_64, UEFI)`, getty y shell de root           |
| Tarifa        | 0 USD. Corrida local, no Actions                                   |

`sha256sum -c SHA256SUMS` en el host respondió OK. El nombre no contiene `Arch` y no empieza por `ARCH`. La aplicación de la ISO es «derivada de Arch Linux». xorriso avisó que el volid `FORGE_AAA81A961B0A` pasa de 16 caracteres en Joliet; la imagen se escribió y arrancó.

## Cómo se construyó

VM de construcción: imagen oficial `Arch-Linux-x86_64-basic.qcow2` (SHA-256 `bb888a084568f4e68b1260ffa614ebf806c79668fa20f946c691973538f4d33c`), KVM, 3 GiB, disco virtual de 40 GiB. El perfil sellado del borrador de escritorio aporta `gnome-shell` y `linux`, `iso_name=Forge`, `iso_version=aaa81a961b0a` e `install_dir=forge`. El directorio que recibió `mkarchiso` parte de `/usr/share/archiso/configs/baseline` (modos `bios.syslinux` y `uefi.grub`, que el perfil sellado no trae) y añade `gnome-shell`. La compresión erofs del template bajó de lzma 109 a 6 para caber en la RAM de la VM. El argv fue `mkarchiso -v -w /var/tmp/archiso-work -o /var/tmp/archiso-out /var/tmp/archiso-profile`.

El perfil no contiene `pacstrap` ni `cmd`. El hostname de la sesión viva sigue siendo `archlinux`: lo deja el template baseline. El menú y `/etc/issue` dicen Forge y «derivada de Arch Linux».

## Arranque UEFI

Segunda VM, firmware `/usr/share/OVMF/OVMF_CODE_4M.fd`, la ISO como CD-ROM, serie. El menú resalta `Forge (x86_64, UEFI)`. Tras el arranque, el getty muestra:

```text
Forge
derivada de Arch Linux
archlinux login:
```

Un segundo arranque entró como root (contraseña vacía del template), ejecutó `echo LIVE_OK` y `cat /etc/issue`, y apagó a los 13 s de kernel (`reboot: Power down`, 2026-10-18 20:22:09Z). No se particionó ningún disco del host.

## Estado tras la corrida

| Caso                    | Estado    | Motivo                                                                                                                                                       |
| ----------------------- | --------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| Botón de Revisión       | cableado  | «Pedir imagen» llama a `requestLiveImage`. En el mock el aviso es `constitution-mkarchiso-absent` (Playwright, 20 pruebas). No arranca la VM ni `mkarchiso`. |
| LiveISO                 | cubierto  | Orden del operador (2026-10-18): si arranca en VM UEFI, el criterio queda cubierto. Evidencia: OVMF, `LIVE_OK`, digest de esta ISO. No se escribió un USB.   |
| SSD con archinstall     | `not_run` | El perfil no nombra `archinstall`. No hubo disco de prueba.                                                                                                  |
| AppImage, tag, `v0.1.0` | no hecho  | Siguen fuera de este plan.                                                                                                                                   |

La ISO no entra en el repositorio. No hay commit de este registro hasta una orden distinta.
