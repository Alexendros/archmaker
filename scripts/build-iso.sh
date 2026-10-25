#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Uso: scripts/build-iso.sh <bundle.json> [opciones]

Opciones:
  --out DIR              Directorio de salida (por defecto: target/iso)
  --base releng|baseline Perfil base de archiso (por defecto: releng)
  --serial-console       Añade console=ttyS0,115200 al arranque (pruebas de CI)
  --image NOMBRE         Imagen Docker del constructor (por defecto: archmaker-archiso:local)
  -h, --help             Muestra esta ayuda
USAGE
}

bundle=""
out_dir="target/iso"
base="releng"
serial_console=0
image="archmaker-archiso:local"

while (($#)); do
  case "$1" in
    --out)
      out_dir=${2:?falta valor para --out}
      shift 2
      ;;
    --base)
      base=${2:?falta valor para --base}
      shift 2
      ;;
    --serial-console)
      serial_console=1
      shift
      ;;
    --image)
      image=${2:?falta valor para --image}
      shift 2
      ;;
    -h | --help)
      usage
      exit 0
      ;;
    -*)
      echo "opción desconocida: $1" >&2
      usage >&2
      exit 2
      ;;
    *)
      bundle=$1
      shift
      ;;
  esac
done

[[ -n $bundle ]] || {
  usage >&2
  exit 2
}
[[ -f $bundle ]] || {
  echo "bundle no encontrado: $bundle" >&2
  exit 2
}
case "$base" in
  releng | baseline) ;;
  *)
    echo "base inválida: $base" >&2
    exit 2
    ;;
esac

repo_root=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
command -v docker >/dev/null || {
  echo "docker no disponible" >&2
  exit 1
}
command -v jq >/dev/null || {
  echo "jq no disponible" >&2
  exit 1
}
docker info >/dev/null 2>&1 || {
  echo "el daemon de Docker no responde" >&2
  exit 1
}

allowed=(
  "profiledef.sh"
  "packages.x86_64"
  "pacman.conf"
  "airootfs/root/user_configuration.json"
)

jq -e '.files | type == "object"' "$bundle" >/dev/null || {
  echo "el bundle no contiene un objeto files" >&2
  exit 1
}

stage=$(mktemp -d)
trap 'rm -rf "$stage"' EXIT
mkdir -p "$stage/overlay" "$out_dir"
out_abs=$(cd "$out_dir" && pwd)

while IFS= read -r rel; do
  permitted=0
  for candidate in "${allowed[@]}"; do
    if [[ $rel == "$candidate" ]]; then
      permitted=1
      break
    fi
  done
  if ((permitted == 0)); then
    echo "ruta de overlay no permitida: $rel" >&2
    exit 1
  fi
  case "$rel" in
    /* | .. | ../* | */.. | */../*)
      echo "ruta de overlay insegura: $rel" >&2
      exit 1
      ;;
  esac
  mkdir -p "$stage/overlay/$(dirname "$rel")"
  jq -r --arg key "$rel" '.files[$key]' "$bundle" >"$stage/overlay/$rel"
done < <(jq -r '.files | keys[]' "$bundle")

if ! docker image inspect "$image" >/dev/null 2>&1; then
  docker build -t "$image" -f "$repo_root/docker/archiso.Dockerfile" "$repo_root/docker"
fi

docker run --rm \
  --cap-add SYS_ADMIN \
  --security-opt apparmor=unconfined \
  --security-opt seccomp=unconfined \
  -e "BASE=$base" \
  -e "SERIAL_CONSOLE=$serial_console" \
  -v "$stage/overlay:/overlay:ro" \
  -v "$out_abs:/out" \
  --mount type=volume,dst=/work \
  "$image" bash -euo pipefail -c '
    cp -a "/usr/share/archiso/configs/$BASE" /tmp/profile
    if [ -d /overlay/airootfs ]; then
      cp -a /overlay/airootfs/. /tmp/profile/airootfs/
    fi
    cp /overlay/profiledef.sh /tmp/profile/profiledef.sh
    cp /overlay/pacman.conf /tmp/profile/pacman.conf
    while IFS= read -r pkg; do
      [ -n "$pkg" ] || continue
      grep -qxF "$pkg" /tmp/profile/packages.x86_64 \
        || printf "%s\n" "$pkg" >> /tmp/profile/packages.x86_64
    done < /overlay/packages.x86_64
    if [ "$SERIAL_CONSOLE" = "1" ]; then
      for f in /tmp/profile/efiboot/loader/entries/*.conf; do
        sed -i "s/^options \(.*\)$/options \1 console=ttyS0,115200/" "$f"
      done
      for f in /tmp/profile/syslinux/archiso_sys.cfg /tmp/profile/syslinux/archiso_pxe.cfg; do
        if [ -f "$f" ]; then
          sed -i "s/^APPEND \(.*\)$/APPEND \1 console=ttyS0,115200/" "$f"
        fi
      done
    fi
    mkarchiso -v -w /work -o /out /tmp/profile
  '

iso=$(find "$out_abs" -maxdepth 1 -name '*.iso' -printf '%f\n' | head -n 1)
if [[ -z $iso ]]; then
  echo "mkarchiso no generó ninguna ISO" >&2
  exit 1
fi
(
  cd "$out_abs"
  sha256sum "$iso" >SHA256SUMS
)
echo "ISO: $out_abs/$iso"
echo "SHA256SUMS: $out_abs/SHA256SUMS"
