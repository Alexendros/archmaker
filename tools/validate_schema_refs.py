#!/usr/bin/env python3
"""Validador de referencias a contratos JSON Schema ($id / $defs).

Uso:
    python3 tools/validate_schema_refs.py

Escanea los documentos gobernados (docs/**/*.md y contracts/**/*.md) y comprueba que toda
referencia del tipo:
  - `https://archmaker.dev/schemas/<nombre>.schema.json`
  - `<nombre>.schema.json#/$defs/<def>`
  - `<nombre>#/$defs/<def>`
resuelve a un schema existente y, cuando lleva `$defs`, a una definición existente.

Salida distinta de cero si alguna referencia no resuelve. No modifica ficheros.
"""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
GOVERNED_GLOBS = ("docs/**/*.md", "contracts/**/*.md")
SCHEMA_GLOB = "contracts/**/*.schema.json"

# A: URL completa, con o sin $defs.
RE_URL = re.compile(
    r"https://archmaker\.dev/schemas/([a-z0-9-]+)\.schema\.json"
    r"(?:#/\$defs/([A-Za-z0-9._-]+))?"
)
# B: nombre de fichero con extensión, no precedido por ruta.
RE_FILE = re.compile(
    r"(?<![/\w-])([a-z0-9-]+)\.schema\.json(?:#/\$defs/([A-Za-z0-9._-]+))?"
)
# C: nombre corto sin extensión seguido de #/$defs/<def>.
RE_DEFS = re.compile(
    r"(?<![./\w-])([a-z0-9-]+)#/\$defs/([A-Za-z0-9._-]+)"
)


def load_schemas() -> tuple[dict[str, dict], dict[str, dict]]:
    """Devuelve (por_nombre, por_id)."""
    by_name: dict[str, dict] = {}
    by_id: dict[str, dict] = {}
    for path in sorted(ROOT.glob(SCHEMA_GLOB)):
        try:
            schema = json.loads(path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            print(f"ERROR: schema inválido {path.relative_to(ROOT).as_posix()}: {exc}")
            continue
        by_name[path.name] = schema
        schema_id = schema.get("$id")
        if isinstance(schema_id, str):
            by_id[schema_id] = schema
    return by_name, by_id


def defs_of(schema: dict) -> set[str]:
    defs = schema.get("$defs")
    return set(defs) if isinstance(defs, dict) else set()


def resolve(name: str, by_name: dict[str, dict], by_id: dict[str, dict]) -> dict | None:
    if name in by_name:
        return by_name[name]
    filename = f"{name}.schema.json"
    if filename in by_name:
        return by_name[filename]
    for schema_id, schema in by_id.items():
        if schema_id.endswith(f"/{filename}") or schema_id.endswith(f"/{name}"):
            return schema
    return None


def main() -> int:
    by_name, by_id = load_schemas()
    if not by_name:
        print("ERROR: no se encontró ningún schema en contracts/**/*.schema.json")
        return 1

    errors: list[str] = []
    checked = 0
    seen: set[tuple[str, str, str]] = set()

    files: list[Path] = []
    for pattern in GOVERNED_GLOBS:
        files.extend(sorted(ROOT.glob(pattern)))

    for path in files:
        rel = path.relative_to(ROOT).as_posix()
        for lineno, line in enumerate(path.read_text(encoding="utf-8").splitlines(), 1):
            refs: list[tuple[str, str | None]] = []
            refs.extend((n, d or None) for n, d in RE_URL.findall(line))
            for name, def_name in RE_FILE.findall(line):
                if name + ".schema.json" in by_name or resolve(name, by_name, by_id):
                    refs.append((name, def_name or None))
            refs.extend((n, d or None) for n, d in RE_DEFS.findall(line))

            for name, def_name in refs:
                key = (rel, name, def_name or "")
                if key in seen:
                    continue
                seen.add(key)
                schema = resolve(name, by_name, by_id)
                if schema is None:
                    errors.append(f"{rel}:{lineno}: schema desconocido `{name}.schema.json`")
                    continue
                checked += 1
                if def_name is not None and def_name not in defs_of(schema):
                    errors.append(
                        f"{rel}:{lineno}: `{name}#/$defs/{def_name}` no existe en "
                        f"{schema.get('$id', name)}"
                    )

    print(f"Schemas cargados: {len(by_name)}")
    print(f"Referencias $id/$defs comprobadas: {checked}")
    for message in errors:
        print(f"ERROR: {message}")
    if errors:
        print(f"FALLO: {len(errors)} referencia(s) no resuelta(s).")
        return 1
    print("OK: todas las referencias $id/$defs resuelven a schemas y definiciones existentes.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
