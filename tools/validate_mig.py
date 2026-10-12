#!/usr/bin/env python3
"""VAL-MIG: corpus de migración v5.1 → vNext (MVP-1).

Comprueba que el destino esperado valida contra draft.schema.json y que el
sample v5.1 no valida como draft. Complementa el job json-schema.yml.
"""
from __future__ import annotations

import json
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent


def main() -> int:
    try:
        from jsonschema import Draft202012Validator
        from referencing import Registry, Resource
        from referencing.jsonschema import DRAFT202012
    except ImportError:
        print("SKIP VAL-MIG: jsonschema/referencing no instalados (CI los aporta)")
        return 0

    base = ROOT / "contracts" / "json-schema"
    schemas_by_id = {}
    for path in base.rglob("*.schema.json"):
        schema = json.loads(path.read_text(encoding="utf-8"))
        schemas_by_id[schema["$id"]] = schema
    registry = Registry().with_resources(
        (sid, Resource.from_contents(s, default_specification=DRAFT202012))
        for sid, s in schemas_by_id.items()
    )
    draft = json.loads((base / "draft.schema.json").read_text(encoding="utf-8"))
    validator = Draft202012Validator(draft, registry=registry)

    migration = base / "examples" / "migration"
    expected = migration / "vnext-draft.expected.json"
    sample = migration / "v5.1-instance.sample.json"
    errors = 0

    if expected.exists():
        errs = list(validator.iter_errors(json.loads(expected.read_text(encoding="utf-8"))))
        if errs:
            print(f"FAIL {expected} debería validar como draft")
            errors += 1
        else:
            print(f"OK   {expected}")
    else:
        print(f"FAIL falta {expected}")
        errors += 1

    if sample.exists():
        if list(validator.iter_errors(json.loads(sample.read_text(encoding="utf-8")))):
            print(f"OK   {sample} no valida como draft")
        else:
            print(f"FAIL {sample} no debería validar como draft")
            errors += 1
    else:
        print(f"FAIL falta {sample}")
        errors += 1

    if errors:
        print(f"VAL-MIG: {errors} fallos")
        return 1
    print("VAL-MIG: corpus migración OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
