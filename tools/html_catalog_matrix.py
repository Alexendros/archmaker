#!/usr/bin/env python3
"""Matriz de solo lectura: reference/v5.1/index.html frente al catálogo local.

No ejecuta cmd, hooks ni pacstrap. No clasifica un paquete como repo oficial:
esa captura sigue pendiente (CON-011). --check falla si el bloque generado del
documento gobernado no coincide con el HTML.
"""

from __future__ import annotations

import argparse
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
HTML = ROOT / "reference" / "v5.1" / "index.html"
CATALOG = ROOT / "crates" / "archmaker-core" / "fixtures" / "local-catalog-mvp1.json"
DOC = ROOT / "docs" / "12-research" / "html-catalog-matrix.md"
BEGIN = "<!-- BEGIN HTML-MATRIX -->"
END = "<!-- END HTML-MATRIX -->"

# Ids ya presentes en el catálogo MVP-1. El resto es propuesta, no un hecho de runtime.
CANONICAL = {
    "linux": "archmaker.kernel.linux",
    "linux-zen": "archmaker.kernel.zen",
    "btrfs-progs": "archmaker.filesystem.btrfs",
    "e2fsprogs": "archmaker.filesystem.ext4",
    "systemd-boot": "archmaker.boot.systemd-boot",
    "grub": "archmaker.boot.grub",
    "base-devel": "archmaker.pkg.base-devel",
    "gnome": "archmaker.compositor.gnome",
    "hyprland": "archmaker.compositor.hyprland",
    "sway": "archmaker.compositor.sway",
    "gdm": "archmaker.dm.gdm",
    "sddm": "archmaker.dm.sddm",
    "firefox": "archmaker.browser.firefox",
    "vivaldi": "archmaker.browser.vivaldi",
    "steam": "archmaker.app.steam",
    "vlc": "archmaker.app.vlc",
    "gimp": "archmaker.app.gimp",
}

KIND_BY_SECTION = {
    "kernel": "kernel",
    "filesystem": "filesystem",
    "base-tools": "pkg",
    "boot": "boot",
    "compositor": "compositor",
    "dm": "dm",
    "office-pack": "pkg",
    "dev-pack": "pkg",
    "gaming-pack": "pkg",
    "browser": "browser",
    "comm": "app",
    "term": "app",
    "media": "app",
    "productivity": "app",
    "gaming": "app",
    "output": "build",
}


def propose_id(section_id: str, option_id: str) -> str:
    if option_id in CANONICAL:
        return CANONICAL[option_id]
    kind = KIND_BY_SECTION.get(section_id, "option")
    slug = option_id.replace("_", "-")
    return f"archmaker.{kind}.{slug}"


def catalog_ids() -> set[str]:
    data = json.loads(CATALOG.read_text(encoding="utf-8"))
    found: set[str] = set()

    def walk(node: object) -> None:
        if isinstance(node, dict):
            value = node.get("id")
            if isinstance(value, str):
                found.add(value)
            for child in node.values():
                walk(child)
        elif isinstance(node, list):
            for child in node:
                walk(child)

    walk(data)
    return found


def parse_instance(text: str) -> dict:
    start = text.find("const INSTANCE = {")
    end = text.find("\n};\n", start)
    if start < 0 or end < 0:
        raise SystemExit("No se encontró el objeto INSTANCE en el HTML")
    body = text[start:end]
    steps_at = body.find("steps:")
    validation_at = body.find("validation:")
    if steps_at < 0 or validation_at < 0:
        raise SystemExit("INSTANCE sin steps o validation")
    steps_src = body[steps_at:validation_at]
    tail = body[validation_at:]

    step_id = None
    section_id = None
    in_packages = False
    options: list[dict] = []
    for raw in steps_src.splitlines():
        line = raw.strip().rstrip(",")
        step_match = re.fullmatch(r'id:\s*"([^"]+)"', line)
        if step_match and not in_packages and "packages:" not in line:
            # El id de paso va solo en su línea; el de sección comparte línea con name.
            if raw.startswith("        id:"):
                step_id = step_match.group(1)
                section_id = None
            continue
        section_match = re.match(
            r'id:\s*"([^"]+)",\s*name:\s*"([^"]+)".*packages:\s*\[',
            line,
        )
        if section_match:
            section_id = section_match.group(1)
            in_packages = True
            continue
        if in_packages and line == "]":
            in_packages = False
            continue
        pkg = re.match(r'\{ id:\s*"([^"]+)",\s*name:\s*"([^"]+)"(.*)\}', line)
        if pkg and in_packages and step_id and section_id:
            rest = pkg.group(3)
            pkg_name = re.search(r'pkg:\s*"([^"]+)"', rest)
            options.append(
                {
                    "step": step_id,
                    "section": section_id,
                    "id": pkg.group(1),
                    "name": pkg.group(2),
                    "pkg": pkg_name.group(1) if pkg_name else "",
                    "fixed": "fixed: true" in rest,
                    "cmd": bool(re.search(r'\bcmd:\s*"', rest)),
                }
            )

    rules = re.findall(r'\{\s*id:\s*"([^"]+)",\s*type:\s*"([^"]+)"', tail)
    presets = re.findall(r'\{\s*id:\s*"([^"]+)",\s*name:\s*"([^"]+)"', tail)
    pipeline = re.search(r'pipeline:\s*\[([^\]]+)\]', tail)
    pipe = []
    if pipeline:
        pipe = re.findall(r'"([^"]+)"', pipeline.group(1))
    fixed = re.findall(r'const FIXED_IDS = \[(.*?)\];', text, re.S)
    fixed_ids = re.findall(r'"([^"]+)"', fixed[0]) if fixed else []
    return {
        "options": options,
        "rules": rules,
        "presets": presets,
        "pipeline": pipe,
        "fixed_ids": fixed_ids,
    }


def classify(row: dict) -> str:
    pkg = row["pkg"]
    if pkg.endswith("-bin") or row["id"].endswith("-bin"):
        base = "no-redistribuible-candidato"
    else:
        base = "sin-clasificar"
    if row["cmd"]:
        return base + "; cmd no migrable"
    return base


def render(parsed: dict, present: set[str]) -> str:
    lines = [
        "| Paso | Sección | Id HTML | Paquete | Id canónico propuesto | En catálogo | Clase | Versión/tamaño |",
        "| --- | --- | --- | --- | --- | --- | --- | --- |",
    ]
    for row in parsed["options"]:
        canon = propose_id(row["section"], row["id"])
        lines.append(
            "| {step} | {section} | `{id}` | `{pkg}` | `{canon}` | {here} | {klass} | no verificada |".format(
                step=row["step"],
                section=row["section"],
                id=row["id"],
                pkg=row["pkg"],
                canon=canon,
                here="sí" if canon in present else "no",
                klass=classify(row),
            )
        )
    lines.append("")
    lines.append("Fijos declarados en el HTML: " + ", ".join(f"`{i}`" for i in parsed["fixed_ids"]) + ".")
    lines.append("")
    lines.append("| Regla HTML | Tipo en el prototipo | Migración |")
    lines.append("| --- | --- | --- |")
    for rule_id, kind in parsed["rules"]:
        lines.append(f"| `{rule_id}` | {kind} | reescribir al AST aceptado; no copiar `count_gt` |")
    lines.append("")
    lines.append("| Preset | Nombre | Estado de requisito |")
    lines.append("| --- | --- | --- |")
    for preset_id, name in parsed["presets"]:
        lines.append(f"| `{preset_id}` | {name} | FR-PRESET-001 deferred |")
    lines.append("")
    pipe = ", ".join(f"`{p}`" for p in parsed["pipeline"])
    lines.append(f"Pipeline heredado: {pipe}. `pacstrap` no migra a runtime.")
    lines.append("")
    cmd_rows = [row["id"] for row in parsed["options"] if row["cmd"]]
    lines.append(
        "Opciones con campo `cmd`: "
        + (", ".join(f"`{i}`" for i in cmd_rows) if cmd_rows else "ninguna")
        + ". El campo no entra al catálogo."
    )
    return "\n".join(lines) + "\n"


def cells(line: str) -> list[str]:
    return [part.strip().strip("`") for part in line.strip().strip("|").split("|")]


def check_document(document: str, parsed: dict, present: set[str]) -> list[str]:
    if BEGIN not in document or END not in document:
        return ["faltan los marcadores de matriz"]
    block = document.split(BEGIN, 1)[1].split(END, 1)[0]
    problems: list[str] = []
    rows = []
    for line in block.splitlines():
        if not line.startswith("|") or line.startswith("| ---") or line.startswith("| --- |"):
            continue
        if set(line.replace("|", "").replace("-", "").replace(" ", "")) == set():
            continue
        row = cells(line)
        if row and row[0] == "Paso":
            continue
        if row and row[0] in {"Regla HTML", "Preset"}:
            continue
        if len(row) == 8:
            rows.append(row)
    expected = []
    for row in parsed["options"]:
        canon = propose_id(row["section"], row["id"])
        expected.append(
            [
                row["step"],
                row["section"],
                row["id"],
                row["pkg"],
                canon,
                "sí" if canon in present else "no",
                classify(row),
                "no verificada",
            ]
        )
    if rows != expected:
        problems.append(
            f"tabla de opciones: documento={len(rows)} html={len(expected)}"
        )
        for index, (got, want) in enumerate(zip(rows, expected), 1):
            if got != want:
                problems.append(f"fila {index}: {got} != {want}")
                break
        if len(rows) != len(expected) and not any(p.startswith("fila ") for p in problems):
            problems.append("el número de filas no coincide y no hay una fila común distinta")
    for rule_id, _kind in parsed["rules"]:
        if f"`{rule_id}`" not in block:
            problems.append(f"falta la regla {rule_id}")
    for preset_id, _name in parsed["presets"]:
        if f"`{preset_id}`" not in block:
            problems.append(f"falta el preset {preset_id}")
    for step in parsed["pipeline"]:
        if f"`{step}`" not in block:
            problems.append(f"falta el paso de pipeline {step}")
    for fixed in parsed["fixed_ids"]:
        if f"`{fixed}`" not in block:
            problems.append(f"falta el id fijo {fixed}")
    for row in parsed["options"]:
        if row["cmd"] and f"`{row['id']}`" not in block.split("Opciones con campo", 1)[-1]:
            problems.append(f"falta cmd de {row['id']}")
    if "pacstrap" not in block:
        problems.append("falta la marca de pacstrap")
    return problems


def splice(document: str, block: str) -> str:
    if BEGIN not in document or END not in document:
        raise SystemExit(f"{DOC} no tiene los marcadores de matriz")
    pre, rest = document.split(BEGIN, 1)
    _, post = rest.split(END, 1)
    return f"{pre}{BEGIN}\n{block}{END}{post}"


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--write", action="store_true")
    args = parser.parse_args()
    parsed = parse_instance(HTML.read_text(encoding="utf-8"))
    if not parsed["options"]:
        print("ERROR: el parser no extrajo opciones", file=sys.stderr)
        return 1
    block = render(parsed, catalog_ids())
    if not args.check and not args.write:
        sys.stdout.write(block)
        return 0
    current = DOC.read_text(encoding="utf-8")
    if args.check:
        problems = check_document(current, parsed, catalog_ids())
        if problems:
            print("ERROR: la matriz no coincide con reference/v5.1/index.html", file=sys.stderr)
            for problem in problems:
                print(problem, file=sys.stderr)
            return 1
        print(f"Matriz al día: {len(parsed['options'])} opciones")
        return 0
    updated = splice(current, block)
    if args.write:
        DOC.write_text(updated, encoding="utf-8")
        print(f"Escrito {DOC} ({len(parsed['options'])} opciones)")
        return 0
    sys.stdout.write(block)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
