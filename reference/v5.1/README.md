# ArchMaker Forge v5.1 - Bundle Completo

Este bundle contiene todos los archivos actuales del rediseño desde 0.

## Frontend
- `index.html` - Frontend final standalone (Red Hat Design + Babel + React). Abrir directo en navegador.
- `App.tsx` - Fuente React del artefacto v5.1 (1000+ líneas, single compositor, libpacks, hardened, limine, ghostty/kitty)
- `neubat_forge_v4_final.html` - Diseño original base (referencia)

## JSON Schema (esqueletos)
- `archmaker.schema.original.json` - Schema original que subiste (v10.4)
- `archmaker.schema.v5.1.json` - **Schema actualizado v5.1** con:
  - arch: x86_64/aarch64 con descripción compatibilidad
  - kernel: linux, linux-zen, linux-hardened (LTS eliminado)
  - filesystem: btrfs-progs fija + e2fsprogs
  - base-devel fijo
  - bootloader: systemd-boot, grub, limine (efibootmgr eliminado)
  - compositor single selection (hyprland, gnome, sway, cosmic - KDE eliminado)
  - dm auto-logic (GDM bloqueado con GNOME)
  - libpacks nueva capa
  - terminal: ghostty, kitty (alacritty eliminado)
  - apps: steam, vlc, gimp, obsidian nuevas

## Instancias / Esqueletos
- `archmaker.instance.v5.1.json` - Instancia completa que cumple schema v5.1, con todas las secciones pobladas
- `archmaker-v10.4-p3.yaml` - Esqueleto original YAML completo (particionado, sistema, hardware, entorno, capas, identidad)
- `libpacks.json` - Solo la nueva capa Packs de librerías (oficina, dev, gaming) con esenciales fijas y recomendadas
- `validation-rules.json` - Reglas: single-compositor, gdm-gnome auto, nvidia-sway
- `presets.json` - Presets minimal, desktop, dev actualizados a ghostty/steam
- `arch-info.json` - Info detallada x86_64 vs aarch64 para modal
- `i18n.es.json` - Catálogo i18n español

## Cambios aplicados v5.1 (según tu lista)
1. Linux LTS eliminado, Hardened agregado
2. Filesystem Btrfs fijo recomendado + base-devel fijo (sustituye concepto antiguo)
3. efibootmgr -> Limine (más actual)
4. 1 solo compositor (single), GDM auto-bloqueado con GNOME, KDE Plasma -> COSMIC
5. Info x86_64 vs aarch64 con modal tabla comparativa
6. Alacritty eliminado, Ghostty + Kitty agregados, Steam + VLC + GIMP + Obsidian
7. Nueva capa LibPacks entre Entorno y Apps (oficina/dev/gaming) con fijas y recomendables

## Uso
Abrir `index.html` o usar el artefacto en:
container:///mnt/data/archmaker_forge_v5_rediseno_agentic_artifact_2_e0956fae570c.html

Para validar:
ajv validate -s archmaker.schema.v5.1.json -d archmaker.instance.v5.1.json

