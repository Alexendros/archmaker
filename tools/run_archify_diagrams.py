#!/usr/bin/env python3
"""Valida y entrega las 24 vistas Archify del catálogo Forge.

Requiere Archify fijado por commit (por defecto ca67c9f…).
Usa AS_BUILT_SOURCE_SHA + manifiesto de rutas/hashes (no exige sourceSha == HEAD).
Falla si falta una vista obligatoria, si rendererSha no coincide, si el checkout
del renderer no es el pin, o si validate/deliver fallan.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIAG = ROOT / "docs/02-architecture/diagrams"
CATALOG = DIAG / "catalog.json"
AS_BUILT_MANIFEST = DIAG / "as-built-manifest.json"
DEFAULT_RENDERER_SHA = "ca67c9f16be0c8cad409cd45190aa052e0649b84"
REQUIRED_IDS = [f"D{i:02d}" for i in range(1, 25)]
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


def git_sha(cwd: Path | None = None) -> str:
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=cwd or ROOT, text=True
    ).strip()


def reject_traversal(rel: str, *, label: str) -> None:
    if not rel or rel.startswith("/") or rel.startswith("\\"):
        fail(f"{label}: ruta absoluta no permitida ({rel!r})")
        raise SystemExit(1)
    parts = Path(rel).parts
    if ".." in parts:
        fail(f"{label}: path traversal rechazado ({rel!r})")
        raise SystemExit(1)


def safe_under(base: Path, rel: str, *, label: str) -> Path:
    reject_traversal(rel, label=label)
    base_r = base.resolve()
    out = (base / rel).resolve()
    try:
        out.relative_to(base_r)
    except ValueError:
        fail(f"{label}: path traversal rechazado ({rel!r})")
        raise SystemExit(1) from None
    return out


def ensure_outside_tree(path: Path, *, label: str) -> Path:
    resolved = path.expanduser().resolve()
    root = ROOT.resolve()
    if resolved == root or root in resolved.parents:
        fail(f"{label}: la salida debe estar fuera del árbol del repo ({resolved})")
        raise SystemExit(1)
    return resolved


def resolve_archify(sha: str) -> Path:
    env_path = os.environ.get("ARCHIFY_CLI")
    if env_path:
        path = Path(env_path)
        if path.is_file():
            return path.resolve()
    candidate = Path.home() / ".local/share/archify" / sha / "archify/bin/archify.mjs"
    if candidate.is_file():
        return candidate.resolve()
    raise FileNotFoundError(
        f"Archify no encontrado en {candidate}. "
        f"Clona tt-a1i/archify en detach {sha} o define ARCHIFY_CLI."
    )


def archify_checkout_root(cli: Path) -> Path:
    """Raíz del checkout git de Archify a partir del CLI (.../archify/bin/archify.mjs)."""
    # .../<sha>/archify/bin/archify.mjs → parents[2] = <sha>
    return cli.resolve().parents[2]


def verify_renderer_checkout(cli: Path, expected_sha: str) -> None:
    root = archify_checkout_root(cli)
    if not (root / ".git").exists() and not (root / ".git").is_file():
        # worktree o .git dir
        git_dir = root / ".git"
        if not git_dir.exists():
            fail(f"checkout Archify sin .git en {root}")
            raise SystemExit(1)
    head = git_sha(root)
    if head != expected_sha:
        fail(
            f"renderer checkout HEAD={head} distinto del pin {expected_sha}; "
            "no se actualiza el pin sin revalidar"
        )
        raise SystemExit(1)


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
    """Éxito solo con exit 0 y recibo JSON con ok:true. ok:true no salva exit≠0."""
    if completed.returncode != 0:
        return False
    for blob in (completed.stdout, completed.stderr):
        if not blob:
            continue
        payload = _first_json_object(blob)
        if payload and payload.get("ok") is True:
            return True
    return False


def load_as_built_manifest() -> dict:
    if not AS_BUILT_MANIFEST.is_file():
        fail(f"falta manifiesto as-built {AS_BUILT_MANIFEST}")
        raise SystemExit(1)
    return json.loads(AS_BUILT_MANIFEST.read_text(encoding="utf-8"))


def verify_as_built(catalog: dict) -> str:
    """Comprueba AS_BUILT_SOURCE_SHA + manifiesto de rutas/hashes. No exige == HEAD."""
    manifest = load_as_built_manifest()
    as_built = manifest.get("asBuiltSourceSha")
    if not isinstance(as_built, str) or len(as_built) != 40:
        fail("as-built-manifest.asBuiltSourceSha inválido")
        raise SystemExit(1)
    if catalog.get("sourceSha") != as_built:
        fail(
            f"catalog.sourceSha={catalog.get('sourceSha')} ≠ "
            f"AS_BUILT_SOURCE_SHA={as_built}"
        )
        raise SystemExit(1)
    paths = manifest.get("paths")
    if not isinstance(paths, dict) or not paths:
        fail("as-built-manifest.paths vacío o inválido")
        raise SystemExit(1)
    for rel, expected in sorted(paths.items()):
        reject_traversal(rel, label="as-built")
        path = safe_under(ROOT, rel, label="as-built")
        if not path.is_file():
            fail(f"as-built: falta ruta {rel}")
            raise SystemExit(1)
        digest = sha256_file(path)
        if digest != expected:
            fail(
                f"as-built: hash alterado en {rel}: "
                f"esperado {expected[:12]}… actual {digest[:12]}…"
            )
            raise SystemExit(1)
    return as_built


def verify_catalog_ids(catalog: dict) -> list[dict]:
    required = list(catalog.get("requiredIds") or [])
    if required != REQUIRED_IDS:
        fail(f"requiredIds debe ser exactamente D01–D24; got {required}")
        raise SystemExit(1)
    entries = catalog.get("entries") or []
    ids = [e.get("id") for e in entries]
    if len(ids) != len(REQUIRED_IDS):
        fail(f"se esperan {len(REQUIRED_IDS)} entradas; hay {len(ids)}")
        raise SystemExit(1)
    if sorted(ids) != REQUIRED_IDS:
        fail(f"IDs de catálogo no son exactamente D01–D24: {ids}")
        raise SystemExit(1)
    if len(set(ids)) != len(ids):
        fail(f"IDs duplicados en catálogo: {ids}")
        raise SystemExit(1)
    for eid in ("D25", "D26"):
        if eid in ids:
            fail(f"{eid} no debe generarse")
            raise SystemExit(1)
    return entries


def verify_code_refs(entry: dict) -> None:
    eid = entry["id"]
    refs = entry.get("codeRefs")
    if not isinstance(refs, list) or not refs:
        fail(f"{eid}: codeRefs vacío o ausente")
        raise SystemExit(1)
    for ref in refs:
        if not isinstance(ref, str):
            fail(f"{eid}: codeRef no string ({ref!r})")
            raise SystemExit(1)
        reject_traversal(ref, label=f"{eid} codeRef")
        path = safe_under(ROOT, ref, label=f"{eid} codeRef")
        if not path.is_file():
            fail(f"{eid}: codeRef roto {ref}")
            raise SystemExit(1)


def composition_viewport_warnings(payload: dict | None) -> int:
    if not payload:
        return 0
    composition = payload.get("composition") or {}
    count = 0
    for issue in composition.get("issues") or []:
        if not isinstance(issue, dict):
            continue
        code = str(issue.get("code") or "")
        if code == "composition/viewport-height":
            count += 1
    return count


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--renderer-sha", default=DEFAULT_RENDERER_SHA)
    parser.add_argument("--quality", default="showcase")
    parser.add_argument("--skip-deliver", action="store_true")
    parser.add_argument(
        "--check",
        action="store_true",
        help="Solo lectura: no escribe en el árbol; usa salida temporal fuera del repo",
    )
    parser.add_argument(
        "--output-dir",
        type=Path,
        default=None,
        help="Directorio de salida fuera del árbol (obligatorio si no es --check con temp)",
    )
    args = parser.parse_args(argv)

    if not CATALOG.is_file():
        fail(f"falta {CATALOG}")
        return 1

    catalog = json.loads(CATALOG.read_text(encoding="utf-8"))
    try:
        entries = verify_catalog_ids(catalog)
        as_built = verify_as_built(catalog)
    except SystemExit as exc:
        return int(exc.code) if isinstance(exc.code, int) else 1

    if catalog.get("rendererSha") != args.renderer_sha:
        fail("catalog.rendererSha no coincide con --renderer-sha")
        return 1

    try:
        cli = resolve_archify(args.renderer_sha)
    except FileNotFoundError as exc:
        fail(str(exc))
        return 1

    try:
        verify_renderer_checkout(cli, args.renderer_sha)
    except SystemExit as exc:
        return int(exc.code) if isinstance(exc.code, int) else 1

    doctor = run_node(cli, ["doctor"])
    if doctor.returncode != 0:
        fail("archify doctor falló")
        sys.stderr.write(doctor.stderr or doctor.stdout or "")
        return 1

    tmp_root: tempfile.TemporaryDirectory[str] | None = None
    if args.check:
        if args.output_dir is not None:
            out_base = ensure_outside_tree(args.output_dir, label="--output-dir")
            out_base.mkdir(parents=True, exist_ok=True)
        else:
            tmp_root = tempfile.TemporaryDirectory(prefix="archify-check-", dir="/tmp")
            out_base = Path(tmp_root.name)
        rendered_dir = out_base / "rendered"
        evidence_dir = out_base / "evidence"
        validation_dir = out_base / "validation"
        index_path = out_base / "index.md"
        write_enabled = True  # escribe solo fuera del árbol
    elif args.output_dir is not None:
        out_base = ensure_outside_tree(args.output_dir, label="--output-dir")
        rendered_dir = out_base / "rendered"
        evidence_dir = out_base / "evidence"
        validation_dir = out_base / "validation"
        index_path = out_base / "index.md"
        write_enabled = True
    else:
        rendered_dir = DIAG / "rendered"
        evidence_dir = DIAG / "evidence"
        validation_dir = DIAG / "validation"
        index_path = DIAG / "index.md"
        write_enabled = True

    if write_enabled:
        for d in (rendered_dir, evidence_dir, validation_dir):
            d.mkdir(parents=True, exist_ok=True)

    # Snapshot para --check: el árbol del repo no debe mutar.
    tree_before: dict[str, str] | None = None
    if args.check:
        tree_before = {}
        for path in DIAG.rglob("*"):
            if path.is_file():
                rel = path.relative_to(ROOT).as_posix()
                tree_before[rel] = sha256_file(path)

    index_rows: list[str] = [
        "---",
        "id: DOC-ARCH-DIAG-INDEX-001",
        "phase: MVP",
        "priority: P1",
        "documentStatus: draft",
        "approvalStatus: pending",
        "implementationStatus: partial",
        "verificationStatus: not-verified",
        "releaseStatus: ineligible",
        "owners:",
        "  - architecture",
        "reviewers:",
        "  - independent-reviewer",
        "---",
        "",
        "# Diagramas Archify (Forge / ArchMaker)",
        "",
        f"- AS_BUILT_SOURCE_SHA: `{as_built}`",
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
        try:
            reject_traversal(rel, label=f"{eid} source")
            src = safe_under(DIAG, rel, label=f"{eid} source")
        except SystemExit:
            errors += 1
            continue
        if not src.is_file():
            fail(f"{eid}: falta fuente {rel}")
            errors += 1
            continue
        if entry.get("sourceSha") != as_built:
            fail(f"{eid}: sourceSha de entrada ≠ AS_BUILT_SOURCE_SHA")
            errors += 1
            continue
        if entry.get("rendererSha") != args.renderer_sha:
            fail(f"{eid}: rendererSha de entrada desalineado")
            errors += 1
            continue
        try:
            verify_code_refs(entry)
        except SystemExit:
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
        validate_blob = (validate.stdout or validate.stderr or "") + "\n"
        if write_enabled:
            (validation_dir / f"{eid}-validate.json").write_text(
                validate_blob, encoding="utf-8"
            )
        if not archify_ok(validate):
            fail(f"{eid}: validate exit={validate.returncode}")
            sys.stderr.write((validate.stderr or validate.stdout or "")[-2000:])
            errors += 1
            continue
        payload = _first_json_object(validate.stdout or "") or _first_json_object(
            validate.stderr or ""
        )
        vp_warn = composition_viewport_warnings(payload)
        if vp_warn and eid in {f"D{i:02d}" for i in range(17, 23)}:
            fail(f"{eid}: composition/viewport-height sin corregir ({vp_warn})")
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
            if write_enabled:
                receipt_path = evidence_dir / f"{eid}-receipt.json"
                receipt_path.write_text(
                    (deliver.stdout or deliver.stderr or "") + "\n", encoding="utf-8"
                )
            if not archify_ok(deliver) or not html_path.is_file():
                fail(f"{eid}: deliver exit={deliver.returncode}")
                sys.stderr.write((deliver.stderr or deliver.stdout or "")[-2000:])
                errors += 1
                continue
            if write_enabled:
                src_digest = sha256_file(src)
                html_digest = sha256_file(html_path)
                meta = {
                    "id": eid,
                    "source": rel,
                    "asBuiltSourceSha": as_built,
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
        if tmp_root is not None:
            tmp_root.cleanup()
        print(f"{errors} fallos en diagramas Archify")
        return 1

    if write_enabled:
        index_path.write_text("\n".join(index_rows) + "\n", encoding="utf-8")

    if args.check and tree_before is not None:
        tree_after: dict[str, str] = {}
        for path in DIAG.rglob("*"):
            if path.is_file():
                rel = path.relative_to(ROOT).as_posix()
                tree_after[rel] = sha256_file(path)
        if tree_after != tree_before:
            fail("--check mutó el árbol de diagrams/; debe ser solo lectura")
            if tmp_root is not None:
                tmp_root.cleanup()
            return 1

    if tmp_root is not None:
        tmp_root.cleanup()

    print(f"OK {len(entries)}/{len(REQUIRED_IDS)} diagramas Archify (AS_BUILT={as_built[:12]})")
    return 0


if __name__ == "__main__":
    sys.exit(main())
