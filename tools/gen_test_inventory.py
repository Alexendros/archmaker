#!/usr/bin/env python3
"""
Generador de inventario de pruebas (TST/VAL) para matriz de trazabilidad.

Escanea:
- crates/*/tests/*.rs (tests de integracion/unidad)
- tools/*.py (validadores)
- apps/web/**/*.test.ts (tests E2E/unit web)

Emite tests-inventory.json con estructura:
{
  "tests": [...],
  "validators": [...]
}
"""

import json
import re
import subprocess
from pathlib import Path
from typing import Dict, List, Any, Optional

REPO_ROOT = Path(__file__).parent.parent

TEST_ID_PATTERN = re.compile(r'#\[test\]\s*(?:fn\s+)?(\w+)')

RUST_TEST_TYPES = {
    'capabilities_negative': 'negative',
    'negative_capabilities': 'negative',
    'core_contract': 'integration',
    'parity': 'integration',
    'invoke_negative': 'negative',
    'vectors': 'unit',
}

PYTHON_VALIDATORS = {
    'validate_front_matter.py': {'id': 'VAL-FRONT-MATTER', 'covers': [], 'type': 'doc', 'transversal': True},
    'validate_traceability.py': {'id': 'VAL-TRACEABILITY', 'covers': [], 'type': 'doc', 'transversal': True},
    'gates.py': {'id': 'VAL-GATES', 'covers': [], 'type': 'gate', 'transversal': True},
    'validate_schema_refs.py': {'id': 'VAL-SCHEMA-REFS', 'covers': ['FR-DRAFT-001', 'FR-CAT-001', 'FR-MANIFEST-001'], 'type': 'schema', 'transversal': False},
    'canonicalize.py': {'id': 'VAL-CANONICALIZE', 'covers': ['NFR-DET-001', 'FR-MANIFEST-001'], 'type': 'canonicalize', 'transversal': False},
    'check_negative_capabilities.py': {'id': 'VAL-NEG-CAPABILITIES', 'covers': ['NFR-SEC-001'], 'type': 'negative', 'transversal': False},
}

WEB_TEST_TYPES = {
    'a11y': 'a11y',
    'visual': 'visual',
    'e2e': 'e2e',
    'unit': 'unit',
}


def scan_rust_tests() -> List[Dict[str, Any]]:
    tests = []
    crates_dir = REPO_ROOT / "crates"
    if not crates_dir.exists():
        return tests

    for crate_dir in crates_dir.iterdir():
        if not crate_dir.is_dir():
            continue
        tests_dir = crate_dir / "tests"
        if not tests_dir.exists():
            continue

        crate_name = crate_dir.name
        for test_file in tests_dir.glob("*.rs"):
            test_name = test_file.stem
            test_type = RUST_TEST_TYPES.get(test_name, 'unit')

            content = test_file.read_text(encoding='utf-8')
            test_functions = TEST_ID_PATTERN.findall(content)

            for fn in test_functions:
                test_id = f"TST-{crate_name.upper()}-{test_name.upper()}-{fn.upper()}"
                covers = infer_covers(crate_name, test_name, fn)

                tests.append({
                    "id": test_id,
                    "type": test_type,
                    "location": str(test_file.relative_to(REPO_ROOT)),
                    "command": f"cargo test -p {crate_name} --test {test_name} {fn}",
                    "workflow": "ci",
                    "covers": covers,
                    "description": f"{test_name}::{fn}"
                })

            if not test_functions:
                test_id = f"TST-{crate_name.upper()}-{test_name.upper()}"
                covers = infer_covers(crate_name, test_name, test_name)
                tests.append({
                    "id": test_id,
                    "type": test_type,
                    "location": str(test_file.relative_to(REPO_ROOT)),
                    "command": f"cargo test -p {crate_name} --test {test_name}",
                    "workflow": "ci",
                    "covers": covers,
                    "description": f"Suite {test_name}"
                })

    return tests


def infer_covers(crate: str, test_file: str, test_fn: str) -> List[str]:
    covers = []
    combined = f"{crate} {test_file} {test_fn}".lower()

    keyword_map = {
        'draft': ['FR-DRAFT-001'],
        'catalog': ['FR-CAT-001'],
        'resolve': ['FR-RESOLVE-001'],
        'validat': ['FR-VALIDATE-001'],
        'manifest': ['FR-MANIFEST-001'],
        'export': ['FR-EXPORT-001'],
        'import': ['FR-IMPORT-001'],
        'preset': ['FR-PRESET-001'],
        'negativ': ['NFR-SEC-001'],
        'capabilit': ['NFR-SEC-001'],
        'parity': ['NFR-DET-001', 'NFR-PORT-001'],
        'canonical': ['NFR-DET-001'],
        'depth': ['NFR-DET-001'],
        'offline': ['NFR-OFF-001'],
        'migrat': ['NFR-MIG-001'],
        'json': ['NFR-DET-001'],
        'visual': ['NFR-ACC-001', 'NFR-OFF-001'],
        'a11y': ['NFR-ACC-001'],
        'e2e': ['FR-DRAFT-001', 'FR-CAT-001', 'FR-RESOLVE-001', 'FR-VALIDATE-001', 'FR-MANIFEST-001', 'FR-EXPORT-001', 'NFR-OFF-001'],
    }

    for keyword, reqs in keyword_map.items():
        if keyword in combined:
            covers.extend(reqs)

    # Add negative coverage for contract requirements when test is negative type
    if 'negative' in combined or 'capabilit' in combined:
        for req in ['FR-DRAFT-001', 'FR-CAT-001', 'FR-MANIFEST-001']:
            if req not in covers:
                covers.append(req)

    return list(dict.fromkeys(covers))


def scan_python_validators() -> List[Dict[str, Any]]:
    validators = []
    tools_dir = REPO_ROOT / "tools"
    if not tools_dir.exists():
        return validators

    for py_file in tools_dir.glob("*.py"):
        if py_file.name.startswith('_') or py_file.name == 'gen_test_inventory.py' or py_file.name == 'traceability_matrix.py':
            continue

        info = PYTHON_VALIDATORS.get(py_file.name, {})
        val_id = info.get('id', f"VAL-{py_file.stem.upper()}")
        covers = info.get('covers', infer_python_covers(py_file.name))
        val_type = info.get('type', 'doc')
        transversal = info.get('transversal', False)

        validators.append({
            "id": val_id,
            "type": val_type,
            "location": str(py_file.relative_to(REPO_ROOT)),
            "command": f"python3 {py_file.relative_to(REPO_ROOT)}",
            "workflow": "ci",
            "covers": covers,
            "description": f"Validador {py_file.stem}",
            "transversal": transversal
        })

    return validators


def infer_python_covers(filename: str) -> List[str]:
    covers = []
    name = filename.lower()
    if 'front_matter' in name:
        return []  # Transversal
    if 'traceability' in name:
        return []  # Transversal
    if 'gates' in name:
        return []  # Transversal
    if 'schema' in name:
        return ['FR-DRAFT-001', 'FR-CAT-001', 'FR-MANIFEST-001']
    if 'canonicalize' in name:
        return ['NFR-DET-001', 'FR-MANIFEST-001']
    if 'negative' in name and 'capabilit' in name:
        return ['NFR-SEC-001', 'FR-DRAFT-001', 'FR-CAT-001', 'FR-MANIFEST-001']
    return covers


def scan_web_tests() -> List[Dict[str, Any]]:
    tests = []
    web_dir = REPO_ROOT / "apps" / "web"
    if not web_dir.exists():
        return tests

    for test_file in web_dir.rglob("*.test.ts"):
        rel_path = test_file.relative_to(REPO_ROOT)
        test_name = test_file.stem.replace('.test', '')

        test_type = 'unit'
        for key, ttype in WEB_TEST_TYPES.items():
            if key in str(rel_path).lower():
                test_type = ttype
                break

        test_id = f"TST-WEB-{test_name.upper()}"
        covers = infer_web_covers(test_name, str(rel_path))

        tests.append({
            "id": test_id,
            "type": test_type,
            "location": str(rel_path),
            "command": f"cd apps/web && pnpm test {test_name}",
            "workflow": "ci",
            "covers": covers,
            "description": f"Web test {test_name}"
        })

    return tests


def infer_web_covers(test_name: str, path: str) -> List[str]:
    covers = []
    name = test_name.lower()
    path_lower = path.lower()
    if 'a11y' in name or 'accessibility' in name:
        covers.append('NFR-ACC-001')
    if 'visual' in name or 'regression' in name:
        covers.extend(['NFR-ACC-001', 'NFR-OFF-001'])
    if 'e2e' in name:
        covers.extend(['FR-DRAFT-001', 'FR-CAT-001', 'FR-RESOLVE-001', 'FR-VALIDATE-001', 'FR-MANIFEST-001', 'FR-EXPORT-001', 'NFR-OFF-001'])
    return list(dict.fromkeys(covers))


def main():
    print("Escaneando tests Rust...")
    rust_tests = scan_rust_tests()
    print(f"  Encontrados {len(rust_tests)} tests/suites Rust")

    print("Escaneando validadores Python...")
    python_validators = scan_python_validators()
    print(f"  Encontrados {len(python_validators)} validadores Python")

    print("Escaneando tests web...")
    web_tests = scan_web_tests()
    print(f"  Encontrados {len(web_tests)} tests web")

    inventory = {
        "generated_at": subprocess.check_output(["git", "log", "-1", "--format=%ct"], cwd=REPO_ROOT).decode().strip(),
        "git_sha": subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=REPO_ROOT).decode().strip(),
        "tests": rust_tests + web_tests,
        "validators": python_validators
    }

    output_path = REPO_ROOT / "docs" / "10-delivery" / "tests-inventory.json"
    output_path.parent.mkdir(parents=True, exist_ok=True)
    output_path.write_text(json.dumps(inventory, indent=2, ensure_ascii=False))

    print(f"Inventario escrito a {output_path}")
    print(f"Total tests: {len(rust_tests) + len(web_tests)}")
    print(f"Total validadores: {len(python_validators)}")


if __name__ == "__main__":
    main()
