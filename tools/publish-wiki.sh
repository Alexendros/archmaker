#!/usr/bin/env bash
# Publica docs/11-operations/manual/ en la wiki de GitHub.
# GITHUB_TOKEN de Actions no puede pushear a la wiki; usar gh auth o un PAT local.
set -euo pipefail

ROOT="$(cd "$(dirname "$0")/.." && pwd)"
SRC="$ROOT/docs/11-operations/manual"
REPO="${GITHUB_REPOSITORY:-Alexendros/archmaker}"
WIKI_URL="https://github.com/${REPO}.wiki.git"
TMP="$(mktemp -d)"
trap 'rm -rf "$TMP"' EXIT

if [ ! -d "$SRC" ]; then
  echo "No existe $SRC" >&2
  exit 1
fi

# Quita front matter YAML y adapta enlaces .md a nombres de página wiki.
strip_for_wiki() {
  local src="$1" dst="$2"
  python3 - "$src" "$dst" <<'PY'
import re, sys
from pathlib import Path
src, dst = Path(sys.argv[1]), Path(sys.argv[2])
text = src.read_text(encoding="utf-8")
if text.startswith("---\n"):
    end = text.find("\n---\n", 4)
    if end != -1:
        text = text[end + 5 :]
# Enlaces relativos del repo → páginas wiki (sin .md; títulos con acento).
repl = {
    "Home.md": "Home",
    "Guia-de-usuario.md": "Guía-de-usuario",
    "Manual-de-uso.md": "Manual-de-uso",
    "Limites.md": "Límites",
}
for a, b in repl.items():
    text = text.replace(f"]({a})", f"]({b})")
dst.write_text(text, encoding="utf-8")
PY
}

echo "Clonando wiki $WIKI_URL ..."
if ! git -c credential.helper='!gh auth git-credential' clone --depth 1 "$WIKI_URL" "$TMP/wiki"; then
  echo "No se pudo clonar la wiki. Crea Home una vez en GitHub (wiki/_new) y vuelve a ejecutar." >&2
  exit 1
fi

strip_for_wiki "$SRC/Home.md" "$TMP/wiki/Home.md"
strip_for_wiki "$SRC/Guia-de-usuario.md" "$TMP/wiki/Guía-de-usuario.md"
strip_for_wiki "$SRC/Manual-de-uso.md" "$TMP/wiki/Manual-de-uso.md"
strip_for_wiki "$SRC/Limites.md" "$TMP/wiki/Límites.md"
strip_for_wiki "$SRC/_Sidebar.md" "$TMP/wiki/_Sidebar.md"

cd "$TMP/wiki"
git add -A
if git diff --cached --quiet; then
  echo "Wiki ya actualizada; sin cambios."
  exit 0
fi
git -c user.email="noreply@github.com" -c user.name="archmaker-wiki" commit -m "docs(wiki): sync from docs/11-operations/manual"
git -c credential.helper='!gh auth git-credential' push origin HEAD:master
echo "Wiki publicada."
