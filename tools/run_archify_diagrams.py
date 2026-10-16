#!/usr/bin/env python3
"""Valida y entrega las 24 vistas Archify del catálogo Forge.

Requiere Archify fijado por commit (por defecto ca67c9f…).
Falla si falta una vista obligatoria, si sourceSha/rendererSha no coinciden
o si validate/deliver fallan.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIAG = ROOT / "docs/02-architecture/diagrams"
CATALOG = DIAG / "catalog.json"
DEFAULT_RENDERER_SHA = "ca67c9f16be0c8cad409cd45190aa052e0649b84"
TYPE_SUFFIX = {
    "architecture": ".architecture.json",
    "sequence": ".sequence.json",
    "lifecycle": ".lifecycle.json",
    "class": ".class.json",
    "dataflow": ".dataflow.json",
    "workflow": ".workflow.json",
    "tree": ".tree.json",
    "timeline": ".timeline.json",
}


def fail(msg: str) -> None:
    print(f"FAIL {msg}", file=sys.stderr)


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def git_sha() -> str:
    return subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()


def resolve_archify(sha: str) -> Path:
    env_path = os.environ.get("ARCHIFY_CLI")
    if env_path:
        path = Path(env_path)
        if path.is_file():
            return path
    candidate = Path.home() / ".local/share/archify" / sha / "archify/bin/archify.mjs"
    if candidate.is_file():
        return candidate
    raise FileNotFoundError(
        f"Archify no encontrado en {candidate}. "
        f"Clona tt-a1i/archify en detach {sha} o define ARCHIFY_CLI."
    )


def archify_type(path: Path) -> str:
    name = path.name
    for kind, suffix in TYPE_SUFFIX.items():
        if name.endswith(suffix):
            return kind
    data = json.loads(path.read_text(encoding="utf-8"))
    return str(data.get("diagram_type") or "architecture")


def run_node(cli: Path, args: list[str]) -> subprocess.CompletedProcess[str]:
    env = os.environ.copy()
    env["ARCHIFY_UPDATE_CHECK_DISABLED"] = "1"
    # Archify crea temporales; forzar /tmp evita sandboxes sin permiso de escritura.
    env["TMPDIR"] = "/tmp"
    env["TMP"] = "/tmp"
    env["TEMP"] = "/tmp"
    cmd = ["node", str(cli), *args]
    print("+ " + " ".join(cmd))
    return subprocess.run(cmd, cwd=ROOT, env=env, check=False, capture_output=True, text=True)


def _first_json_object(blob: str) -> dict | None:
    start = blob.find("{")
    if start < 0:
        return None
    decoder = json.JSONDecoder()
    try:
        payload, _end = decoder.raw_decode(blob[start:])
    except json.JSONDecodeError:
        return None
    return payload if isinstance(payload, dict) else None


def archify_ok(completed: subprocess.CompletedProcess[str]) -> bool:
    if completed.returncode == 0:
        return True
    # Algunos entornos fallan al limpiar temporales tras emitir JSON ok:true.
    for blob in (completed.stdout, completed.stderr):
        if not blob:
            continue
        payload = _first_json_object(blob)
        if payload and payload.get("ok") is True:
            return True
    return False


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--renderer-sha", default=DEFAULT_RENDERER_SHA)
    parser.add_argument("--quality", default="showcase")
    parser.add_argument("--skip-deliver", action="store_true")
    args = parser.parse_args(argv)

    if not CATALOG.is_file():
        fail(f"falta {CATALOG}")
        return 1

    catalog = json.loads(CATALOG.read_text(encoding="utf-8"))
    required = set(catalog.get("requiredIds") or [])
    entries = catalog.get("entries") or []
    found_ids = {e.get("id") for e in entries}
    missing = sorted(required - found_ids)
    if missing:
        fail(f"faltan entradas de catálogo: {missing}")
        return 1

    head = git_sha()
    if catalog.get("sourceSha") != head:
        fail(
            f"catalog.sourceSha={catalog.get('sourceSha')} distinto de HEAD={head}; "
            "regenera con tools/scaffold_archify_diagrams.py"
        )
        return 1
    if catalog.get("rendererSha") != args.renderer_sha:
        fail("catalog.rendererSha no coincide con --renderer-sha")
        return 1

    try:
        cli = resolve_archify(args.renderer_sha)
    except FileNotFoundError as exc:
        fail(str(exc))
        return 1

    doctor = run_node(cli, ["doctor"])
    if doctor.returncode != 0:
        fail("archify doctor falló")
        sys.stderr.write(doctor.stderr or doctor.stdout or "")
        return 1

    rendered_dir = DIAG / "rendered"
    evidence_dir = DIAG / "evidence"
    validation_dir = DIAG / "validation"
    for d in (rendered_dir, evidence_dir, validation_dir):
        d.mkdir(parents=True, exist_ok=True)

    index_rows: list[str] = [
        "# Diagramas Archify (Forge / ArchMaker)",
        "",
        f"- sourceSha: `{head}`",
        f"- rendererSha: `{args.renderer_sha}`",
        f"- generado: `{datetime.now(timezone.utc).strftime('%Y-%m-%dT%H:%M:%SZ')}`",
        "",
        "| ID | Título | HTML |",
        "| --- | --- | --- |",
    ]

    errors = 0
    for entry in entries:
        eid = entry["id"]
        rel = entry["source"]
        src = DIAG / rel
        if not src.is_file():
            fail(f"{eid}: falta fuente {rel}")
            errors += 1
            continue
        if entry.get("sourceSha") != head:
            fail(f"{eid}: sourceSha de entrada desalineado")
            errors += 1
            continue
        if entry.get("rendererSha") != args.renderer_sha:
            fail(f"{eid}: rendererSha de entrada desalineado")
            errors += 1
            continue

        dtype = archify_type(src)
        validate = run_node(
            cli,
            [
                "validate",
                dtype,
                str(src),
                "--repo-root",
                str(ROOT),
                "--quality",
                args.quality,
                "--json",
            ],
        )
        (validation_dir / f"{eid}-validate.json").write_text(
            (validate.stdout or validate.stderr or "") + "\n", encoding="utf-8"
        )
        if not archify_ok(validate):
            fail(f"{eid}: validate exit={validate.returncode}")
            sys.stderr.write((validate.stderr or validate.stdout or "")[-2000:])
            errors += 1
            continue

        html_name = f"{eid}.html"
        html_path = rendered_dir / html_name
        if not args.skip_deliver:
            deliver = run_node(
                cli,
                [
                    "deliver",
                    dtype,
                    str(src),
                    str(html_path),
                    "--repo-root",
                    str(ROOT),
                    "--quality",
                    args.quality,
                    "--json",
                ],
            )
            receipt_path = evidence_dir / f"{eid}-receipt.json"
            receipt_path.write_text(
                (deliver.stdout or deliver.stderr or "") + "\n", encoding="utf-8"
            )
            if not archify_ok(deliver) or not html_path.is_file():
                fail(f"{eid}: deliver exit={deliver.returncode}")
                sys.stderr.write((deliver.stderr or deliver.stdout or "")[-2000:])
                errors += 1
                continue
            src_digest = sha256_file(src)
            html_digest = sha256_file(html_path)
            meta = {
                "id": eid,
                "source": rel,
                "sourceSha": head,
                "rendererSha": args.renderer_sha,
                "sourceSha256": src_digest,
                "htmlSha256": html_digest,
                "html": f"rendered/{html_name}",
            }
            (evidence_dir / f"{eid}-digests.json").write_text(
                json.dumps(meta, indent=2) + "\n", encoding="utf-8"
            )
            index_rows.append(
                f"| {eid} | {entry.get('title', '')} | [HTML](rendered/{html_name}) |"
            )
        else:
            index_rows.append(f"| {eid} | {entry.get('title', '')} | _(skip deliver)_ |")

    if errors:
        print(f"{errors} fallos en diagramas Archify")
        return 1

    (DIAG / "index.md").write_text("\n".join(index_rows) + "\n", encoding="utf-8")
    print(f"OK {len(entries)}/{len(required)} diagramas Archify")
    return 0


if __name__ == "__main__":
    sys.exit(main())
