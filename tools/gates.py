#!/usr/bin/env python3
"""Genera y valida la tabla de gates desde los manifiestos (fase R3, AUD-006).

Uso:
    python3 tools/gates.py            # valida manifiestos + coherencia con gates.md y go-no-go.md
    python3 tools/gates.py --write    # regenera docs/10-delivery/gates.md y el bloque derivado de go-no-go.md
    python3 tools/gates.py --strict   # (por defecto ya es estricto) se acepta por simetría con los demás validadores

Autoridad única de estado: contracts/governance/gates/G0.json … G10.json.
- `docs/10-delivery/gates.md` se genera por completo desde los manifiestos.
- `docs/00-governance/go-no-go.md` contiene un bloque derivado entre marcadores.
Cualquier contradicción entre los manifiestos y la documentación hace fallar el comando
(código distinto de cero), de modo que la CI falla si la documentación contradice el manifiesto.

Salida distinta de cero ante cualquier fallo.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path

try:
    from jsonschema import Draft202012Validator
except ImportError:  # pragma: no cover
    Draft202012Validator = None

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "contracts" / "governance" / "gate.schema.json"
GATES_DIR = ROOT / "contracts" / "governance" / "gates"
GATES_MD = ROOT / "docs" / "10-delivery" / "gates.md"
GNG_MD = ROOT / "docs" / "00-governance" / "go-no-go.md"

GATE_ORDER = [f"G{i}" for i in range(11)]
PHASE_KEYS = [
    "document-complete",
    "design-ready",
    "implementation-ready",
    "verified",
    "release-ready",
]
PHASE_LABEL = {
    "pending": "pending",
    "in-progress": "in-progress",
    "complete": "complete",
    "failed": "failed",
    "not-applicable": "n/a",
}

BEGIN_MARK = "<!-- BEGIN GATES-DERIVED -->"
END_MARK = "<!-- END GATES-DERIVED -->"

FM_OPEN = "---"


def split_front_matter(text: str) -> tuple[str, str]:
    lines = text.splitlines()
    if not lines or lines[0].strip() != FM_OPEN:
        return "", text
    for idx in range(1, len(lines)):
        if lines[idx].strip() == FM_OPEN:
            fm = "\n".join(lines[1:idx])
            body = "\n".join(lines[idx + 1 :])
            return fm, body
    return "", text


def load_manifests() -> tuple[list[dict], list[str]]:
    errors: list[str] = []
    manifests: dict[str, dict] = {}
    if not GATES_DIR.is_dir():
        return [], [f"no existe el directorio de manifiestos {GATES_DIR}"]
    for path in sorted(GATES_DIR.glob("G*.json")):
        rel = path.relative_to(ROOT).as_posix()
        try:
            data = json.loads(path.read_text(encoding="utf-8"))
        except json.JSONDecodeError as exc:
            errors.append(f"{rel}: JSON inválido: {exc}")
            continue
        if not isinstance(data, dict):
            errors.append(f"{rel}: el manifiesto no es un objeto JSON")
            continue
        gate_id = data.get("id")
        if gate_id != path.stem:
            errors.append(f"{rel}: id {gate_id!r} no coincide con el nombre de fichero {path.stem!r}")
        if isinstance(gate_id, str):
            if gate_id in manifests:
                errors.append(f"{rel}: id de gate duplicado {gate_id}")
            manifests[gate_id] = data
    missing = [g for g in GATE_ORDER if g not in manifests]
    if missing:
        errors.append(f"faltan manifiestos de gate: {', '.join(missing)}")
    extra = [g for g in manifests if g not in GATE_ORDER]
    if extra:
        errors.append(f"manifiestos de gate no esperados: {', '.join(sorted(extra))}")
    ordered = [manifests[g] for g in GATE_ORDER if g in manifests]
    return ordered, errors


def derive_status(phases: dict) -> str:
    states = set(phases.values())
    if "failed" in states:
        return "failed"
    if states <= {"complete", "not-applicable"}:
        return "complete"
    if states & {"complete", "in-progress"}:
        return "in-progress"
    return "pending"


def validate_manifests(manifests: list[dict]) -> list[str]:
    errors: list[str] = []
    if Draft202012Validator is None:
        return ["falta jsonschema (python3 -m pip install jsonschema)"]
    if not SCHEMA_PATH.is_file():
        return [f"no existe el schema {SCHEMA_PATH}"]
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    Draft202012Validator.check_schema(schema)
    validator = Draft202012Validator(schema)

    for manifest in manifests:
        gate = manifest.get("id", "?")
        for err in sorted(validator.iter_errors(manifest), key=lambda e: list(e.path)):
            loc = "/".join(str(p) for p in err.path) or "(raíz)"
            errors.append(f"{gate}: schema [{loc}] {err.message}")

        phases = manifest.get("phases", {})
        if isinstance(phases, dict) and set(phases) == set(PHASE_KEYS):
            derived = derive_status(phases)
            declared = manifest.get("status")
            if declared != derived:
                errors.append(
                    f"{gate}: status declarado {declared!r} contradice las fases (derivado {derived!r})"
                )

            criteria = manifest.get("criteria", [])
            seen: set[str] = set()
            for crit in criteria:
                cid = crit.get("id")
                if cid in seen:
                    errors.append(f"{gate}: id de criterio duplicado {cid}")
                seen.add(cid)
                if not str(cid).startswith(f"{gate}-C"):
                    errors.append(f"{gate}: criterio {cid} no usa el prefijo {gate}-C")

            by_kind = {k: [] for k in ("document-approved", "metric-verified", "decision", "artifact", "review")}
            for crit in criteria:
                by_kind.setdefault(crit.get("kind"), []).append(crit)

            def all_complete(kind: str) -> bool:
                items = by_kind.get(kind, [])
                return all(c.get("status") == "complete" for c in items)

            if phases.get("document-complete") == "complete" and not all_complete("document-approved"):
                errors.append(
                    f"{gate}: fase document-complete=complete pero hay criterios document-approved no complete"
                )
            if phases.get("implementation-ready") == "complete" and not all_complete("artifact"):
                errors.append(
                    f"{gate}: fase implementation-ready=complete pero hay criterios artifact no complete"
                )
            if phases.get("verified") == "complete":
                for kind in ("metric-verified", "decision", "review"):
                    if not all_complete(kind):
                        errors.append(
                            f"{gate}: fase verified=complete pero hay criterios {kind} no complete"
                        )
            if declared == "complete":
                incomplete = [c.get("id") for c in criteria if c.get("status") != "complete"]
                if incomplete:
                    errors.append(
                        f"{gate}: status complete con criterios no complete: {', '.join(map(str, incomplete))}"
                    )
            for crit in criteria:
                if crit.get("status") == "complete" and not crit.get("evidence"):
                    errors.append(
                        f"{gate}: criterio {crit.get('id')} complete sin evidencia enlazada"
                    )
    return errors


def _phase_cell(manifest: dict) -> str:
    phases = manifest.get("phases", {}) or {}
    cells: list[str] = []
    for key in PHASE_KEYS:
        state = phases.get(key, "pending")
        cells.append(PHASE_LABEL.get(state, str(state)))
    return "/".join(cells)


def _pending_criteria(manifest: dict) -> str:
    pend = [
        c.get("id")
        for c in manifest.get("criteria", [])
        if c.get("status") != "complete"
    ]
    return ", ".join(map(str, pend)) if pend else "—"


def render_gates_body(manifests: list[dict]) -> str:
    lines: list[str] = []
    lines.append("# Gates G0–G10")
    lines.append("")
    lines.append(
        "Tabla **generada** desde `contracts/governance/gates/G*.json` por `tools/gates.py`."
    )
    lines.append(
        "No editar a mano: `python3 tools/gates.py` falla si esta tabla contradice los manifiestos."
    )
    lines.append(
        "La semántica de los estados se lee en `docs/00-governance/status-model.md` (DOC-GOV-STATUS-001)."
    )
    lines.append("")
    lines.append("| Gate | Título | Fases (doc/diseño/impl/verif/release) | Estado | Criterios pendientes |")
    lines.append("|---|---|---|---|---|")
    for m in manifests:
        lines.append(
            f"| {m.get('id')} | {m.get('title')} | {_phase_cell(m)} | {m.get('status')} | {_pending_criteria(m)} |"
        )
    lines.append("")
    lines.append("## Criterios por gate")
    lines.append("")
    for m in manifests:
        lines.append(f"### {m.get('id')} — {m.get('title')} (`{m.get('status')}`)")
        lines.append("")
        for c in m.get("criteria", []):
            n_ev = len(c.get("evidence", []) or [])
            suffix = f" · {n_ev} evidencia(s)" if n_ev else ""
            lines.append(
                f"- `{c.get('id')}` · {c.get('kind')} · {c.get('status')} — {c.get('description')}{suffix}"
            )
        lines.append("")
    result, reason = global_result(manifests)
    lines.append("## Derivación")
    lines.append("")
    lines.append(f"- Resultado global derivado: **{result}** — {reason}.")
    lines.append(
        "- `docs/00-governance/go-no-go.md` reproduce esta cobertura desde los mismos manifiestos."
    )
    lines.append("")
    return "\n".join(lines)


def render_gng_block(manifests: list[dict]) -> str:
    result, reason = global_result(manifests)
    lines: list[str] = []
    lines.append(BEGIN_MARK)
    lines.append("## Cobertura de gates (derivada)")
    lines.append("")
    lines.append(
        "Bloque generado desde `contracts/governance/gates/G*.json` por `tools/gates.py`; no editar a mano."
    )
    lines.append("")
    lines.append("| Gate | Estado | Fases (doc/diseño/impl/verif/release) | Falta para `complete` |")
    lines.append("|---|---|---|---|")
    for m in manifests:
        lines.append(
            f"| {m.get('id')} {m.get('title')} | {m.get('status')} | {_phase_cell(m)} | {_pending_criteria(m)} |"
        )
    lines.append("")
    lines.append(f"Resultado derivado: **{result}** — {reason}.")
    lines.append(END_MARK)
    return "\n".join(lines)


def global_result(manifests: list[dict]) -> tuple[str, str]:
    incomplete = [m.get("id") for m in manifests if m.get("status") != "complete"]
    if incomplete:
        return "No-Go", f"no todos los gates están `complete` (faltan {', '.join(map(str, incomplete))})"
    return "Go", "todos los gates G0–G10 están `complete`"


def check_docs(manifests: list[dict]) -> list[str]:
    errors: list[str] = []
    expected_body = render_gates_body(manifests).strip()
    if not GATES_MD.is_file():
        errors.append(f"no existe {GATES_MD.relative_to(ROOT).as_posix()}")
    else:
        _fm, body = split_front_matter(GATES_MD.read_text(encoding="utf-8"))
        if body.strip() != expected_body:
            errors.append(
                "docs/10-delivery/gates.md contradice los manifiestos "
                "(ejecuta `python3 tools/gates.py --write`)"
            )

    expected_block = render_gng_block(manifests)
    if not GNG_MD.is_file():
        errors.append(f"no existe {GNG_MD.relative_to(ROOT).as_posix()}")
    else:
        text = GNG_MD.read_text(encoding="utf-8")
        if BEGIN_MARK not in text or END_MARK not in text:
            errors.append(
                "docs/00-governance/go-no-go.md no contiene el bloque derivado "
                "(ejecuta `python3 tools/gates.py --write`)"
            )
        else:
            start = text.index(BEGIN_MARK)
            end = text.index(END_MARK) + len(END_MARK)
            if text[start:end] != expected_block:
                errors.append(
                    "docs/00-governance/go-no-go.md contradice los manifiestos "
                    "(ejecuta `python3 tools/gates.py --write`)"
                )
    return errors


def write_docs(manifests: list[dict]) -> None:
    fm, _body = split_front_matter(GATES_MD.read_text(encoding="utf-8"))
    GATES_MD.write_text(f"---\n{fm}\n---\n\n{render_gates_body(manifests)}\n", encoding="utf-8")

    text = GNG_MD.read_text(encoding="utf-8")
    block = render_gng_block(manifests)
    if BEGIN_MARK in text and END_MARK in text:
        start = text.index(BEGIN_MARK)
        end = text.index(END_MARK) + len(END_MARK)
        text = text[:start] + block + text[end:]
    else:
        text = text.rstrip() + "\n\n" + block + "\n"
    GNG_MD.write_text(text, encoding="utf-8")


def main() -> int:
    parser = argparse.ArgumentParser(description="Genera y valida la tabla de gates desde los manifiestos.")
    parser.add_argument("--write", action="store_true", help="regenera gates.md y el bloque de go-no-go.md")
    parser.add_argument("--strict", action="store_true", help="aceptado por simetría; el modo ya es estricto")
    args = parser.parse_args()

    manifests, errors = load_manifests()
    errors += validate_manifests(manifests)

    if errors:
        print(f"Manifiestos de gate analizados: {len(manifests)}")
        for message in errors:
            print(f"ERROR: {message}")
        print(f"FALLO: {len(errors)} error(es) en los manifiestos.")
        return 1

    if args.write:
        write_docs(manifests)
        print(f"Manifiestos de gate analizados: {len(manifests)}")
        print("ESCRITO: docs/10-delivery/gates.md y bloque derivado de docs/00-governance/go-no-go.md")
        result, reason = global_result(manifests)
        print(f"Resultado derivado: {result} — {reason}.")
        return 0

    doc_errors = check_docs(manifests)
    result, reason = global_result(manifests)
    print(f"Manifiestos de gate analizados: {len(manifests)}")
    for message in doc_errors:
        print(f"ERROR: {message}")
    if doc_errors:
        print(f"FALLO: {len(doc_errors)} contradicción(es) entre manifiestos y documentación.")
        return 1
    print(f"OK: manifiestos válidos y coherentes con gates.md y go-no-go.md.")
    print(f"Resultado derivado: {result} — {reason}.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
