#!/usr/bin/env python3
"""Validador de trazabilidad ejecutable (fase R3, AUD-005).

Uso:
    python3 tools/validate_traceability.py [--strict]

Comprueba, sobre el repositorio completo:
  (a) unicidad de los IDs gobernados (front matter `docs/**/*.md` y `contracts/**/*.md`);
  (b) estados válidos: front matter contra contracts/governance/document-metadata.schema.json
      y equivalencia narrativa de los ADR (proposed/accepted/rejected/superseded) con sus campos;
  (c) referencias resolubles de la matriz de trazabilidad (docs/00-governance/traceability-matrix.md):
      los IDs gobernados, ADR y gates deben resolver; los IDs de artefactos (FR/NFR/DM/TST/VAL/
      RULE/THR/SRC/CON/AM/RSK/DEC) se resuelven contra el índice global de tokens y su ausencia se
      reporta como ADVERTENCIA de materialización (deuda existente);
  (d) grafo de trazabilidad: filas bien formadas y referencias que resuelven;
  (e) coherencia de gates: manifiestos válidos, gates.md y el bloque derivado de go-no-go.md sin
      contradicción, dependencias de gate satisfechas cuando el gate está `complete`, y resultado
      Go/No-Go de go-no-go.md derivado de los gates.

Las ADVERTENCIAS no fuerzan el fallo; los ERRORES sí (código distinto de cero). Con --strict las
advertencias también devuelven código distinto de cero.

Salida distinta de cero ante cualquier error.
"""

from __future__ import annotations

import argparse
import json
import re
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

sys.path.insert(0, str(Path(__file__).resolve().parent))
import gates  # noqa: E402

ROOT = Path(__file__).resolve().parents[1]
DOC_SCHEMA = ROOT / "contracts" / "governance" / "document-metadata.schema.json"
GOVERNED_GLOBS = ("docs/**/*.md", "contracts/**/*.md")
MATRIX = ROOT / "docs" / "00-governance" / "traceability-matrix.md"
GNG = ROOT / "docs" / "00-governance" / "go-no-go.md"
ADR_DIR = ROOT / "docs" / "02-architecture" / "adr"

ID_RE = re.compile(
    r"\b(OBJ|JNY|UC|FR|NFR|DM|ADR|VAL|THR|TST|RULE|SRC|CON|AM|RSK|DEC|AUD|ISSUE|INV)"
    r"-[A-Z0-9]+(?:-[A-Z0-9]+)*\b"
)
GATE_RE = re.compile(r"\bG(?:[0-9]|10)\b")
PHASES = {"MVP", "v1", "Enterprise", "planning"}

ADR_NARRATIVE = {
    "proposed": ("in-review", "pending"),
    "accepted": ("accepted", "approved"),
    "rejected": (None, "rejected"),
    "superseded": ("superseded", None),
}

errors: list[str] = []
warnings: list[str] = []


def rel(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def governed_files() -> list[Path]:
    files: list[Path] = []
    for pattern in GOVERNED_GLOBS:
        files.extend(sorted(ROOT.glob(pattern)))
    return files


def load_governed() -> tuple[dict[str, tuple[str, dict]], list[tuple[str, dict]]]:
    index: dict[str, tuple[str, dict]] = {}
    parsed: list[tuple[str, dict]] = []
    if Draft202012Validator is None:
        errors.append("falta jsonschema (python3 -m pip install jsonschema)")
        return index, parsed
    schema = json.loads(DOC_SCHEMA.read_text(encoding="utf-8"))
    validator = Draft202012Validator(schema)
    for path in governed_files():
        r = rel(path)
        _fm, _body = gates.split_front_matter(path.read_text(encoding="utf-8"))
        if not _fm:
            errors.append(f"{r}: falta el front matter YAML")
            continue
        try:
            meta = yaml.safe_load(_fm)
        except yaml.YAMLError as exc:
            errors.append(f"{r}: YAML inválido: {exc}")
            continue
        if not isinstance(meta, dict):
            errors.append(f"{r}: el front matter no es un objeto YAML")
            continue
        for err in sorted(validator.iter_errors(meta), key=lambda e: list(e.path)):
            loc = "/".join(str(p) for p in err.path) or "(raíz)"
            errors.append(f"{r}: estado inválido [{loc}] {err.message}")
        doc_id = meta.get("id")
        if isinstance(doc_id, str):
            if doc_id in index:
                errors.append(f"{r}: ID gobernado duplicado {doc_id} (ya en {index[doc_id][0]})")
            else:
                index[doc_id] = (r, meta)
        parsed.append((r, meta))
    return index, parsed


def check_adr_narrative() -> None:
    for path in sorted(ADR_DIR.glob("ADR-*.md")):
        r = rel(path)
        text = path.read_text(encoding="utf-8")
        fm, _ = gates.split_front_matter(text)
        if not fm:
            continue
        try:
            meta = yaml.safe_load(fm)
        except yaml.YAMLError:
            continue
        match = re.search(r"^-\s*Estado:\s*(\w+)", text, re.MULTILINE)
        if not match:
            warnings.append(f"{r}: ADR sin línea narrativa «- Estado: …»")
            continue
        narrative = match.group(1).strip().lower()
        expected = ADR_NARRATIVE.get(narrative)
        if expected is None:
            errors.append(f"{r}: estado narrativo de ADR desconocido {narrative!r}")
            continue
        exp_doc, exp_app = expected
        if exp_doc is not None and meta.get("documentStatus") != exp_doc:
            errors.append(
                f"{r}: narrativa {narrative!r} exige documentStatus={exp_doc}, declara {meta.get('documentStatus')}"
            )
        if exp_app is not None and meta.get("approvalStatus") != exp_app:
            errors.append(
                f"{r}: narrativa {narrative!r} exige approvalStatus={exp_app}, declara {meta.get('approvalStatus')}"
            )


def global_token_index() -> set[str]:
    tokens: set[str] = set()
    roots = [ROOT / "docs", ROOT / "contracts", ROOT / "reference"]
    for base in roots:
        if not base.is_dir():
            continue
        for path in base.rglob("*"):
            if not path.is_file() or path.suffix.lower() not in {".md", ".json"}:
                continue
            if path == MATRIX:
                continue
            try:
                text = path.read_text(encoding="utf-8")
            except (OSError, UnicodeDecodeError):
                continue
            tokens.update(m.group(0) for m in ID_RE.finditer(text))
    return tokens


def check_matrix(index: dict[str, tuple[str, dict]], manifest_ids: set[str]) -> int:
    if not MATRIX.is_file():
        errors.append(f"no existe {rel(MATRIX)}")
        return 0
    known = global_token_index()
    refs: set[str] = set()
    edges = 0
    for lineno, line in enumerate(MATRIX.read_text(encoding="utf-8").splitlines(), 1):
        if not line.strip().startswith("|"):
            continue
        cells = [c.strip() for c in line.strip().strip("|").split("|")]
        if all(re.fullmatch(r":?-+:?", c) for c in cells if c != ""):
            continue
        row_ids: list[str] = []
        for cell in cells:
            row_ids.extend(m.group(0) for m in ID_RE.finditer(cell))
        if not row_ids:
            continue
        edges += 1
        for token in row_ids:
            refs.add(token)
    for token in sorted(refs):
        if token in index or token in manifest_ids:
            continue
        if token in known:
            continue
        if token.startswith(("TST-", "VAL-")):
            warnings.append(f"matriz de trazabilidad: {token} sin definición materializada (solo aparece en la matriz)")
        else:
            errors.append(f"matriz de trazabilidad: referencia {token} no resoluble")
    return edges


def check_gates(index: dict[str, tuple[str, dict]]) -> set[str]:
    manifests, load_errors = gates.load_manifests()
    for message in load_errors:
        errors.append(f"gates: {message}")
    for message in gates.validate_manifests(manifests):
        errors.append(f"gates: {message}")
    for message in gates.check_docs(manifests):
        errors.append(f"gates: {message}")

    manifest_ids = {str(m.get("id")) for m in manifests}
    for manifest in manifests:
        gate = manifest.get("id")
        for dep in manifest.get("dependsOn", []) or []:
            dep_id = dep.get("id")
            target = index.get(dep_id)
            if target is None:
                errors.append(f"gates: {gate} depende de {dep_id}, que no está gobernado")
                continue
            target_rel, target_meta = target
            if manifest.get("status") != "complete":
                continue
            for field, expected in (
                ("documentStatus", dep.get("expectedDocumentStatus")),
                ("approvalStatus", dep.get("expectedApprovalStatus")),
                ("verificationStatus", dep.get("expectedVerificationStatus")),
            ):
                if expected is not None and target_meta.get(field) != expected:
                    errors.append(
                        f"gates: {gate} está complete y depende de {dep_id} con {field}={expected}, "
                        f"pero {target_rel} declara {target_meta.get(field)}"
                    )

    result, _reason = gates.global_result(manifests)
    if GNG.is_file():
        match = re.search(r"Resultado:\s*\*\*([A-Za-z-]+)", GNG.read_text(encoding="utf-8"))
        if not match:
            errors.append("go-no-go.md: no declara «Resultado: **…**»")
        elif match.group(1) != result:
            errors.append(
                f"go-no-go.md declara {match.group(1)} pero los gates derivan {result}"
            )
    return manifest_ids


def main() -> int:
    parser = argparse.ArgumentParser(description="Valida trazabilidad, estados y coherencia de gates.")
    parser.add_argument("--strict", action="store_true", help="promueve las advertencias a fallo.")
    args = parser.parse_args()

    if not DOC_SCHEMA.is_file():
        print(f"ERROR: no existe {rel(DOC_SCHEMA)}")
        return 1

    index, _parsed = load_governed()
    check_adr_narrative()

    manifests, load_errors = gates.load_manifests()
    for message in load_errors:
        errors.append(f"gates: {message}")
    manifest_ids = {str(m.get("id")) for m in manifests}
    edges = check_matrix(index, manifest_ids)
    check_gates(index)

    print(f"Documentos gobernados analizados: {len(index)}")
    print(f"Referencias de la matriz de trazabilidad evaluadas: {edges} filas")
    for message in errors:
        print(f"ERROR: {message}")
    for message in warnings:
        print(f"ADVERTENCIA: {message}")

    if errors or (args.strict and warnings):
        print(f"FALLO: {len(errors)} error(es) y {len(warnings)} advertencia(s).")
        return 1
    print(f"OK: IDs únicos, estados válidos, referencias y gates coherentes ({len(warnings)} advertencia(s)).")
    return 0


if __name__ == "__main__":
    sys.exit(main())
