#!/usr/bin/env python3
"""Valida tauri.conf.json y capabilities contra schemas Tauri 2 (SRC-004 / #30 / #32).

- Rechaza app.security.devtools (fuera de SecurityConfig v2).
- Valida cada capability JSON contra src-tauri/gen/schemas/desktop-schema.json.
- Exige main.json (producto) y restricted.json (prueba ACL) en capabilities/.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
fails: list[str] = []


def fail(msg: str) -> None:
    print(f"FAIL {msg}")
    fails.append(msg)


def ok(msg: str) -> None:
    print(f"PASS {msg}")


def main() -> int:
    conf_path = ROOT / "src-tauri/tauri.conf.json"
    schema_path = ROOT / "src-tauri/gen/schemas/desktop-schema.json"
    caps_dir = ROOT / "src-tauri/capabilities"

    conf = json.loads(conf_path.read_text(encoding="utf-8"))
    ok(f"parse {conf_path.relative_to(ROOT)}")

    security = conf.get("app", {}).get("security", {})
    if "devtools" in security:
        fail("app.security.devtools presente (rechazado por schema Tauri 2)")
    else:
        ok("app.security sin devtools")

    windows = conf.get("app", {}).get("windows", [])
    labels = [w.get("label") for w in windows if isinstance(w, dict)]
    if labels != ["main"]:
        fail(f"ventanas esperadas ['main'], got {labels}")
    else:
        ok("tauri.conf.json solo ventana main")

    if not schema_path.is_file():
        fail(f"falta {schema_path.relative_to(ROOT)}; regenerar con cargo build -p archmaker")
        print(f"\n{len(fails)} fallos.")
        return 1

    try:
        from jsonschema import Draft7Validator
    except ImportError:
        fail("falta jsonschema (pip install jsonschema==4.23.0)")
        print(f"\n{len(fails)} fallos.")
        return 1

    desktop_schema = json.loads(schema_path.read_text(encoding="utf-8"))
    # CapabilityFile (raíz): anyOf acepta una sola Capability.
    validator = Draft7Validator(desktop_schema)

    cap_files = sorted(caps_dir.glob("*.json"))
    names = [p.name for p in cap_files]
    if names != ["main.json", "restricted.json"]:
        fail(f"capabilities debe ser main.json+restricted.json, got {names}")
    else:
        ok("capabilities/ main + restricted")

    for cap_path in cap_files:
        instance = json.loads(cap_path.read_text(encoding="utf-8"))
        # $schema es metadato del fichero; el schema CapabilityFile no lo declara.
        instance.pop("$schema", None)
        if "denied" in instance:
            fail(f"{cap_path.name}: campo raíz 'denied' no soportado por schema")
        errors = sorted(validator.iter_errors(instance), key=lambda e: list(e.path))
        if errors:
            for err in errors[:5]:
                fail(f"{cap_path.name}: {err.message}")
        else:
            ok(f"capability valida contra desktop-schema: {cap_path.name}")
        perms = instance.get("permissions")
        if cap_path.name == "restricted.json":
            if perms != []:
                fail(f"{cap_path.name}: permissions debe ser [] (ACL negativa)")
            else:
                ok(f"{cap_path.name} permissions vacía")
        elif cap_path.name == "main.json":
            if not isinstance(perms, list) or len(perms) == 0:
                fail(f"{cap_path.name}: debe conceder allow-* de CorePort")
            elif not all(isinstance(p, str) and p.startswith("allow-") for p in perms):
                fail(f"{cap_path.name}: permisos inesperados {perms}")
            else:
                ok(f"{cap_path.name} concede {len(perms)} allow-*")

    caps_gen = ROOT / "src-tauri/gen/schemas/capabilities.json"
    if caps_gen.is_file():
        gen = json.loads(caps_gen.read_text(encoding="utf-8"))
        keys = sorted(gen.keys())
        expected = ["main-capability", "restricted-capability"]
        if keys != expected:
            fail(f"gen/schemas/capabilities.json keys={keys}, esperado {expected}")
        else:
            ok("gen/schemas/capabilities.json main+restricted")

    print(f"\n{len(fails)} fallos.")
    return 1 if fails else 0


if __name__ == "__main__":
    sys.exit(main())
