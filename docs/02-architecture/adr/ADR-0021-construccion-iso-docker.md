---
id: ADR-0021
phase: MVP
priority: P0
documentStatus: accepted
approvalStatus: approved
implementationStatus: partial
verificationStatus: not-verified
releaseStatus: ineligible
owners:
  - security
reviewers:
  - independent-reviewer
---

# ADR-0021: Construcción de la ISO en contenedor Docker

- Estado: accepted
- Fecha: 2026-10-25
- Propietario: Seguridad
- Aprobación: operador, 2026-10-25 (reorientación)
- Requisitos: FR-EXPORT-001, NFR-SEC-001, NFR-DET-001
- Supersede a: ADR-0004

## Contexto

Forge genera un perfil archiso completo (ADR-0020) y la imagen se construye con
`mkarchiso`. `mkarchiso` monta sistemas de archivos, crea dispositivos de bucle y
ejecuta `pacstrap`; no puede correr sin privilegios ni dentro del WebView.

El criterio de producto mantiene a Forge sin root: no hay runner privilegiado, no
hay `pkexec` y no hay `mkarchiso` en el host. La construcción ocurre en un
contenedor efímero con una imagen fijada por digest.

## Decisión

1. La imagen del constructor es `archlinux` fijada por digest
   (`docker/archiso.Dockerfile`) e instala `archiso`. Renovate actualiza el digest;
   no se usa `latest` sin digest.
2. `scripts/build-iso.sh` es el único punto de entrada. Copia el perfil base
   (`releng` o `baseline`) desde `/usr/share/archiso/configs`, aplica el overlay
   validado por allowlist y ejecuta `mkarchiso` dentro del contenedor.
3. El contenedor se ejecuta con `--rm`, `--cap-add SYS_ADMIN`,
   `--security-opt apparmor=unconfined` y `--security-opt seccomp=unconfined`.
   Si el montaje de bucle falla, la alternativa documentada es `--privileged`;
   se prefiere la variante mínima y cualquier cambio exige medirlo en CI.
4. Montajes: overlay en solo lectura, directorio de salida explícito y volumen
   anónimo para `/work`. No se monta el repositorio completo ni el home.
5. `console=ttyS0,115200` solo se inyecta con `--serial-console`, para la prueba
   de arranque de CI; la ISO publicada no lo lleva.
6. La verificación de arranque usa QEMU con OVMF (UEFI) y falla si no se observa
   el arranque en el plazo configurado.

## Evidencia medida (2026-10-25)

- La variante mínima (`SYS_ADMIN` + `apparmor=unconfined` + `seccomp=unconfined`) construyó la ISO de humo sin `--privileged` ni `--device /dev/loop*`.
- Construcción con `scripts/build-iso.sh … --base baseline --serial-console`: 93 s; ISO de 483.733.504 bytes.
- El ISO declara dos imágenes El Torito: BIOS (`/boot/syslinux/isolinux.bin`) y UEFI (32768 bloques, 64 MiB).
- Arranque: QEMU con OVMF alcanzó `archlinux login:` en 50 s mediante `scripts/boot-test.sh`.
- Orden obligatorio en `profiledef.sh`: `bootmodes` lista `bios.syslinux` antes de `uefi.systemd-boot`. `mkarchiso` recorre los modos y añade `-eltorito-alt-boot` para UEFI; si UEFI va primero, la entrada UEFI se pierde y el ISO solo arranca por BIOS.
- La ISO completa con `releng` queda para `workflow_dispatch`; el job de PR usa `baseline` para contener el coste.

## Consecuencias

- La cadena es reproducible: digest fijado, perfil base copiado en tiempo de
  construcción y overlay determinista.
- Pertenecer al grupo `docker` equivale a root en el host. La mitigación es un
  contenedor efímero, comandos fijos sin interpolación de cadenas del usuario,
  overlay de solo lectura y salida acotada.
- La ISO de humo usa el perfil `baseline` para mantener el coste de CI bajo; la
  ISO completa con `releng` se construye bajo demanda.
- ADR-0004 (runner privilegiado) queda superseded: ya no hay runner ni elevación
  en el WebView.
