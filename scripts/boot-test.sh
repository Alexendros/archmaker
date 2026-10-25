#!/usr/bin/env bash
set -euo pipefail

iso=${1:?uso: scripts/boot-test.sh <iso> [segundos]}
timeout_s=${2:-900}

[[ -f $iso ]] || {
  echo "ISO no encontrada: $iso" >&2
  exit 2
}
command -v qemu-system-x86_64 >/dev/null || {
  echo "qemu-system-x86_64 no disponible" >&2
  exit 1
}

firmware=()
vars_copy=""
qemu_pid=""
log=$(mktemp)

cleanup() {
  if [[ -n $qemu_pid ]]; then
    kill "$qemu_pid" 2>/dev/null || true
  fi
  if [[ -n $vars_copy ]]; then
    rm -f "$vars_copy"
  fi
  rm -f "$log"
}
trap cleanup EXIT

if [[ -f /usr/share/ovmf/OVMF.fd ]]; then
  firmware=(-bios /usr/share/ovmf/OVMF.fd)
elif [[ -f /usr/share/OVMF/OVMF_CODE.fd && -f /usr/share/OVMF/OVMF_VARS.fd ]]; then
  vars_copy=$(mktemp)
  cp /usr/share/OVMF/OVMF_VARS.fd "$vars_copy"
  firmware=(
    -drive if=pflash,format=raw,readonly=on,file=/usr/share/OVMF/OVMF_CODE.fd
    -drive if=pflash,format=raw,file="$vars_copy"
  )
else
  echo "OVMF no encontrado; instala el paquete ovmf" >&2
  exit 1
fi

qemu-system-x86_64 \
  -machine q35 \
  -m 3072 \
  -smp 2 \
  "${firmware[@]}" \
  -cdrom "$iso" \
  -boot d \
  -nographic \
  -monitor none \
  -no-reboot \
  >"$log" 2>&1 &
qemu_pid=$!

deadline=$((SECONDS + timeout_s))
while ((SECONDS < deadline)); do
  if grep -qE 'login:|root@archiso|Reached target Multi-User System|Linux version' "$log"; then
    echo "OK: la ISO arrancó en QEMU UEFI"
    tail -n 20 "$log"
    exit 0
  fi
  if ! kill -0 "$qemu_pid" 2>/dev/null; then
    echo "FAIL: QEMU terminó sin arrancar la ISO" >&2
    tail -n 80 "$log" >&2
    exit 1
  fi
  sleep 5
done

echo "FAIL: timeout de ${timeout_s}s esperando el arranque" >&2
tail -n 80 "$log" >&2
exit 1
