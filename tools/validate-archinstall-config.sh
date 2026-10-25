#!/usr/bin/env bash
set -euo pipefail

if [[ $# -lt 1 ]]; then
  echo "usage: $0 <user_configuration.json>..." >&2
  exit 2
fi

repo_root="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
image="archlinux:latest"
status=0

for config in "$@"; do
  if [[ "$config" == "$repo_root"/* ]]; then
    rel="${config#"$repo_root"/}"
  else
    rel="$config"
  fi

  if [[ ! -f "$repo_root/$rel" ]]; then
    echo "missing config: $config" >&2
    status=1
    continue
  fi

  echo "== archinstall contract: $rel"
  if docker run --rm \
    -v "$repo_root:/src:ro" \
    -e CONFIG="/src/$rel" \
    "$image" \
    bash -lc 'pacman -Sy --noconfirm --needed archinstall >/dev/null && archinstall --config "$CONFIG" --dry-run --silent --skip-wifi-check --skip-version-check --skip-ntp'; then
    echo "OK: $rel"
  else
    echo "FAIL: $rel" >&2
    status=1
  fi
done

exit "$status"
