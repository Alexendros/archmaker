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

echo "Clonando wiki $WIKI_URL ..."
# La wiki remota solo existe tras la primera página (Home). Usar credenciales de gh.
if ! git -c credential.helper='!gh auth git-credential' clone --depth 1 "$WIKI_URL" "$TMP/wiki"; then
  echo "No se pudo clonar la wiki. Crea Home una vez en GitHub (wiki/_new) y vuelve a ejecutar." >&2
  exit 1
fi

# Mapear nombres de fichero a páginas wiki (Home.md, etc.)
cp -f "$SRC/Home.md" "$TMP/wiki/Home.md"
cp -f "$SRC/Guia-de-usuario.md" "$TMP/wiki/Guía-de-usuario.md"
cp -f "$SRC/Manual-de-uso.md" "$TMP/wiki/Manual-de-uso.md"
cp -f "$SRC/Limites.md" "$TMP/wiki/Límites.md"
cp -f "$SRC/_Sidebar.md" "$TMP/wiki/_Sidebar.md"

cd "$TMP/wiki"
git add -A
if git diff --cached --quiet; then
  echo "Wiki ya actualizada; sin cambios."
  exit 0
fi
git -c user.email="noreply@github.com" -c user.name="archmaker-wiki" commit -m "docs(wiki): sync from docs/11-operations/manual"
git push -u origin HEAD:master
echo "Wiki publicada."
