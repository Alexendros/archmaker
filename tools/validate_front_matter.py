#!/usr/bin/env python3
"""Validador de front matter y coherencia de estados gobernados (fase R1).

Uso:
    python3 tools/validate_front_matter.py [--strict]

Comprueba:
  (a) todo documento gobernado (docs/**/*.md y contracts/**/*.md) tiene front matter
      válido contra contracts/governance/document-metadata.schema.json;
  (b) las reglas de transición intra-documento R1, R2, R5, R6, R7 y R8 de
      docs/00-governance/status-model.md.

Las reglas cruzadas R3 (un ADR proposed no satisface una decisión accepted) y R4
(un gate complete no depende de ítems incompletos) se resuelven vía `dependsOn` y se
reportan como INCONSISTENCIA. Por defecto no fuerzan el fallo (las resuelven R3/R5);
con --strict también devuelven código distinto de cero.

Salida distinta de cero ante cualquier fallo.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

try:
    import yaml
except ImportError:  # pragma: no cover
    print("ERROR: falta PyYAML (python3 -m pip install pyyaml)")
    sys.exit(1)

try:
    from jsonschema import Draft202012Validator
except ImportError:  # pragma: no cover
    Draft202012Validator = None

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "contracts" / "governance" / "document-metadata.schema.json"
GOVERNED_GLOBS = ("docs/**/*.md", "contracts/**/*.md")

FM_OPEN = "---"
FM_CLOSE = "---"

errors: list[str] = []
inconsistencies: list[str] = []


def split_front_matter(text: str) -> tuple[str | None, str]:
    lines = text.splitlines()
    if not lines or lines[0].strip() != FM_OPEN:
        return None, text
    for idx in range(1, len(lines)):
        if lines[idx].strip() == FM_CLOSE:
            return "\n".join(lines[1:idx]), "\n".join(lines[idx + 1 :])
    return None, text


def collect_documents() -> list[Path]:
    files: list[Path] = []
    for pattern in GOVERNED_GLOBS:
        files.extend(sorted(ROOT.glob(pattern)))
    return files


def check_transitions(rel: str, meta: dict) -> None:
    owners = set(meta.get("owners", []))
    reviewers = set(meta.get("reviewers", []))
    overlap = owners & reviewers
    if overlap:
        errors.append(
            f"{rel}: R8 owners y reviewers se solapan: {sorted(overlap)}"
        )

    if meta.get("documentStatus") == "accepted" and meta.get("approvalStatus") != "approved":
        errors.append(
            f"{rel}: R6 documentStatus=accepted exige approvalStatus=approved"
        )

    if meta.get("approvalStatus") == "rejected" and meta.get("documentStatus") == "accepted":
        errors.append(
            f"{rel}: R7 approvalStatus=rejected es incompatible con documentStatus=accepted"
        )

    if meta.get("verificationStatus") == "passed" and not meta.get("evidence"):
        errors.append(
            f"{rel}: R1 verificationStatus=passed exige evidencia enlazada (campo evidence)"
        )

    release = meta.get("releaseStatus")
    if release in ("eligible", "released"):
        problems = []
        if meta.get("implementationStatus") != "complete":
            problems.append("implementationStatus=complete")
        if meta.get("approvalStatus") != "approved":
            problems.append("approvalStatus=approved")
        if meta.get("verificationStatus") != "passed":
            problems.append("verificationStatus=passed")
        if problems:
            errors.append(
                f"{rel}: R2 releaseStatus={release} exige {' + '.join(problems)}"
            )
        if release == "released" and not meta.get("evidence"):
            errors.append(
                f"{rel}: R2 releaseStatus=released exige evidencia enlazada"
            )


def check_dependencies(rel: str, meta: dict, index: dict[str, tuple[str, dict]]) -> None:
    for dep in meta.get("dependsOn", []) or []:
        dep_id = dep.get("id")
        target = index.get(dep_id)
        if target is None:
            inconsistencies.append(
                f"{rel}: depende de {dep_id}, que no está gobernado o no existe"
            )
            continue
        target_rel, target_meta = target
        for field, expected in (
            ("documentStatus", dep.get("expectedDocumentStatus")),
            ("approvalStatus", dep.get("expectedApprovalStatus")),
            ("verificationStatus", dep.get("expectedVerificationStatus")),
        ):
            if expected is not None and target_meta.get(field) != expected:
                inconsistencies.append(
                    f"{rel}: R3/R4 depende de {dep_id} con {field}={expected}, "
                    f"pero {target_rel} declara {target_meta.get(field)}"
                )


def main() -> int:
    parser = argparse.ArgumentParser(description="Valida el front matter gobernado.")
    parser.add_argument(
        "--strict",
        action="store_true",
        help="promueve las inconsistencias cruzadas (R3/R4) a fallo.",
    )
    args = parser.parse_args()

    if not SCHEMA_PATH.is_file():
        print(f"ERROR: no existe el schema {SCHEMA_PATH}")
        return 1
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))

    if Draft202012Validator is None:
        print("ERROR: falta jsonschema (python3 -m pip install jsonschema)")
        return 1
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)

    documents = collect_documents()
    index: dict[str, tuple[str, dict]] = {}
    parsed: list[tuple[str, dict]] = []

    for path in documents:
        rel = path.relative_to(ROOT).as_posix()
        text = path.read_text(encoding="utf-8")
        fm, _body = split_front_matter(text)
        if fm is None:
            errors.append(f"{rel}: falta el bloque de front matter YAML delimitado por ---")
            continue
        try:
            meta = yaml.safe_load(fm)
        except yaml.YAMLError as exc:
            errors.append(f"{rel}: YAML inválido en el front matter: {exc}")
            continue
        if not isinstance(meta, dict):
            errors.append(f"{rel}: el front matter no es un objeto YAML")
            continue

        for err in sorted(validator.iter_errors(meta), key=lambda e: list(e.path)):
            loc = "/".join(str(p) for p in err.path) or "(raíz)"
            errors.append(f"{rel}: schema [{loc}] {err.message}")

        doc_id = meta.get("id")
        if isinstance(doc_id, str):
            if doc_id in index:
                errors.append(
                    f"{rel}: id duplicado {doc_id} (ya en {index[doc_id][0]})"
                )
            else:
                index[doc_id] = (rel, meta)
        parsed.append((rel, meta))

    for rel, meta in parsed:
        check_transitions(rel, meta)
    for rel, meta in parsed:
        check_dependencies(rel, meta, index)

    print(f"Documentos gobernados analizados: {len(documents)}")
    for message in errors:
        print(f"ERROR: {message}")
    for message in inconsistencies:
        print(f"INCONSISTENCIA: {message}")

    if errors or (args.strict and inconsistencies):
        print(
            f"FALLO: {len(errors)} error(es) y {len(inconsistencies)} inconsistencia(s)."
        )
        return 1
    print(
        f"OK: front matter válido y transiciones coherentes "
        f"({len(inconsistencies)} inconsistencia(s) cruzada(s) registrada(s))."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
