#!/usr/bin/env python3
"""VAL-F7-CANDIDATE: matriz obligatoria F7 sobre un RC_SHA candidato.

Valida el inventario, opcionalmente ejecuta el subconjunto CI/local y escribe
métricas + un manifiesto con result=pending. Nunca declara RC_SHA_FUNCIONAL
ni aceptación independiente firmada.
"""

from __future__ import annotations

import argparse
import json
import os
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INVENTORY = ROOT / "docs/10-delivery/closure/f7-required-cases.json"
EXAMPLE = (
    ROOT
    / "docs/10-delivery/closure/examples/manifest.f7-candidate.pending.example.json"
)
SCHEMA_REQUIRED_IDS_MIN = 10

REQUIRED_CASE_IDS = {
    "F7-MOTOR-REAL",
    "F7-GEN-EDIT-EXPORT",
    "F7-DBLCLICK",
    "F7-BLOCKING-VALIDATION",
    "F7-TARGET-INCOMPATIBLE",
    "F7-REG-F0",
    "F7-REG-F1",
    "F7-REG-F2",
    "F7-REG-F3",
    "F7-REG-F4",
    "F7-REG-F5",
    "F7-REG-F6",
    "F7-OFFLINE",
    "F7-E2E-X3",
}


def fail(msg: str) -> None:
    print(f"FAIL {msg}", file=sys.stderr)


def git_sha() -> str | None:
    try:
        out = subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        return None
    return out if len(out) == 40 else None


def wasm_pkg_ready() -> bool:
    pkg = ROOT / "crates/archmaker-wasm/pkg"
    return (pkg / "archmaker_wasm.js").is_file() or (
        pkg / "archmaker_wasm_bg.wasm"
    ).is_file()


def validate_inventory(data: dict) -> list[str]:
    errors: list[str] = []
    if data.get("schemaVersion") != "1":
        errors.append("schemaVersion distinto de 1")
    if data.get("phase") != "F7":
        errors.append("phase distinto de F7")
    policy = data.get("policy") or {}
    if policy.get("declareRcShaFuncional") is not False:
        errors.append("policy.declareRcShaFuncional debe ser false")
    if policy.get("independentAcceptance") is not False:
        errors.append("policy.independentAcceptance debe ser false")
    if policy.get("resultUntilAcceptance") != "pending":
        errors.append("policy.resultUntilAcceptance debe ser pending")
    limitations = data.get("limitations")
    if not isinstance(limitations, list) or len(limitations) < 3:
        errors.append("limitations debe listar al menos 3 bloqueos conocidos")
    cases = data.get("cases")
    if not isinstance(cases, list) or len(cases) < SCHEMA_REQUIRED_IDS_MIN:
        errors.append("cases incompleto")
        return errors
    ids = []
    for case in cases:
        if not isinstance(case, dict):
            errors.append("caso no objeto")
            continue
        cid = case.get("id")
        if not isinstance(cid, str) or not cid:
            errors.append("caso sin id")
            continue
        ids.append(cid)
        if "title" not in case or "hooks" not in case or "commands" not in case:
            errors.append(f"{cid}: faltan title/hooks/commands")
        if cid in {"F7-OFFLINE", "F7-E2E-X3"} and case.get("defaultStatus") != "pending":
            errors.append(f"{cid}: defaultStatus debe ser pending sin WASM garantizado")
    missing = REQUIRED_CASE_IDS - set(ids)
    if missing:
        errors.append(f"faltan ids obligatorios: {sorted(missing)}")
    extra = set(ids) - REQUIRED_CASE_IDS
    if extra:
        errors.append(f"ids no reconocidos: {sorted(extra)}")
    return errors


def validate_example() -> list[str]:
    errors: list[str] = []
    if not EXAMPLE.is_file():
        return [f"falta {EXAMPLE.relative_to(ROOT)}"]
    data = json.loads(EXAMPLE.read_text(encoding="utf-8"))
    if data.get("result") != "pending":
        errors.append("ejemplo F7 no tiene result=pending")
    if data.get("reviewer") is not None or data.get("reviewedAt") is not None:
        errors.append("ejemplo F7 no debe firmar reviewer/reviewedAt")
    residuals = data.get("residuals") or []
    joined = " ".join(residuals).lower()
    for needle in (
        "rc_sha_funcional",
        "independiente",
        "bloqueantes",
        "p95",
        "portal",
    ):
        if needle not in joined:
            errors.append(f"residuals del ejemplo no mencionan {needle!r}")
    ids = data.get("requiredTestIds") or []
    if set(ids) != REQUIRED_CASE_IDS:
        errors.append("requiredTestIds del ejemplo no coincide con la matriz")
    return errors


def run_command(cmd: str, env: dict[str, str] | None = None) -> tuple[str, int]:
    print(f"+ {cmd}")
    completed = subprocess.run(
        cmd,
        shell=True,
        cwd=ROOT,
        env=env or os.environ.copy(),
        check=False,
    )
    return cmd, completed.returncode


def execute_matrix(*, with_e2e: bool) -> dict:
    inventory = json.loads(INVENTORY.read_text(encoding="utf-8"))
    results: list[dict] = []
    wasm_ok = wasm_pkg_ready()
    e2e_enabled = with_e2e and wasm_ok

    for case in inventory["cases"]:
        cid = case["id"]
        status = "pending"
        exit_codes: list[int] = []
        note = case.get("residual")

        if case.get("e2eWasmRequired") and not e2e_enabled:
            status = "pending"
            note = note or "Playwright WASM no disponible o --with-e2e omitido"
            results.append(
                {
                    "id": cid,
                    "status": status,
                    "exitCodes": exit_codes,
                    "note": note,
                }
            )
            continue

        commands = case.get("commands") or []
        if not commands:
            status = "pending"
            note = note or "sin comando ejecutable en este runner"
            results.append(
                {
                    "id": cid,
                    "status": status,
                    "exitCodes": exit_codes,
                    "note": note,
                }
            )
            continue

        # En --execute sin --with-e2e, omitir comandos wasm-flow (siguen pending).
        runnable = []
        for cmd in commands:
            if "wasm-flow" in cmd and not e2e_enabled:
                continue
            runnable.append(cmd)

        if not runnable:
            status = "pending"
            note = "comandos WASM omitidos sin --with-e2e / pkg"
            results.append(
                {
                    "id": cid,
                    "status": status,
                    "exitCodes": exit_codes,
                    "note": note,
                }
            )
            continue

        failed = False
        for cmd in runnable:
            _cmd, code = run_command(cmd)
            exit_codes.append(code)
            if code != 0:
                failed = True
                break
        status = "failed" if failed else "passed"
        results.append(
            {
                "id": cid,
                "status": status,
                "exitCodes": exit_codes,
                "note": note,
            }
        )

    executed = sum(1 for r in results if r["status"] in {"passed", "failed"})
    passed = sum(1 for r in results if r["status"] == "passed")
    failed = sum(1 for r in results if r["status"] == "failed")
    skipped = sum(1 for r in results if r["status"] == "pending")
    return {
        "cases": results,
        "executed": executed,
        "passed": passed,
        "failed": failed,
        "skipped": skipped,
        "retries": 0,
        "wasmPkgReady": wasm_ok,
        "withE2e": e2e_enabled,
    }


def build_pending_manifest(metrics: dict | None, source_sha: str | None) -> dict:
    inventory = json.loads(INVENTORY.read_text(encoding="utf-8"))
    residuals = list(inventory.get("limitations") or [])
    residuals.extend(
        [
            "No RC_SHA_FUNCIONAL: candidato documentado, no aceptación firmada",
            "Sin aceptación independiente firmada",
            "F7-OFFLINE y F7-E2E-X3 pending sin Playwright WASM garantizado",
        ]
    )
    if metrics:
        for case in metrics.get("cases") or []:
            if case.get("status") == "pending" and case.get("note"):
                residuals.append(f"{case['id']}: {case['note']}")
    now = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    return {
        "schemaVersion": "1",
        "repository": "https://github.com/Alexendros/archmaker",
        "phase": "F7",
        "sourceSha": source_sha,
        "baseSha": None,
        "eventSha": None,
        "prHeadSha": None,
        "workflowSha": None,
        "mergeSha": None,
        "workflowRunId": None,
        "runAttempt": None,
        "eventName": None,
        "startedAt": now,
        "completedAt": now,
        "toolchain": None,
        "platform": None,
        "dirtyTrackedFiles": None,
        "fixtureSetDigest": None,
        "requiredTestIds": [c["id"] for c in inventory["cases"]],
        "executed": None if metrics is None else metrics["executed"],
        "passed": None if metrics is None else metrics["passed"],
        "failed": None if metrics is None else metrics["failed"],
        "skipped": None if metrics is None else metrics["skipped"],
        "retries": 0 if metrics is not None else None,
        "artifacts": None,
        "result": "pending",
        "reviewer": None,
        "reviewedAt": None,
        "residuals": residuals,
    }


def write_outputs(out_dir: Path, metrics: dict | None, manifest: dict) -> None:
    out_dir.mkdir(parents=True, exist_ok=True)
    if metrics is not None:
        (out_dir / "metrics.json").write_text(
            json.dumps(metrics, indent=2, ensure_ascii=False) + "\n",
            encoding="utf-8",
        )
    (out_dir / "manifest.json").write_text(
        json.dumps(manifest, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )
    print(f"Wrote {out_dir / 'manifest.json'} (result=pending)")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--validate-only",
        action="store_true",
        help="Solo valida inventario y ejemplo pending (modo CI)",
    )
    parser.add_argument(
        "--execute",
        action="store_true",
        help="Ejecuta el subconjunto local/CI de la matriz",
    )
    parser.add_argument(
        "--with-e2e",
        action="store_true",
        help="Incluye wasm-flow y E2E×3 si el pkg WASM está presente",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=None,
        help="Directorio para metrics.json y manifest.json (fuera del árbol recomendado)",
    )
    args = parser.parse_args()

    if not INVENTORY.is_file():
        fail(f"falta {INVENTORY}")
        return 1

    inventory = json.loads(INVENTORY.read_text(encoding="utf-8"))
    errors = validate_inventory(inventory)
    errors.extend(validate_example())
    if errors:
        for err in errors:
            fail(err)
        print(f"{len(errors)} fallos en la matriz F7")
        return 1

    print(
        f"OK inventario F7: {len(inventory['cases'])} casos; "
        "result=pending; sin RC_SHA_FUNCIONAL ni aceptación independiente"
    )

    metrics = None
    if args.execute and not args.validate_only:
        metrics = execute_matrix(with_e2e=args.with_e2e)
        print(
            json.dumps(
                {
                    "executed": metrics["executed"],
                    "passed": metrics["passed"],
                    "failed": metrics["failed"],
                    "skipped": metrics["skipped"],
                    "wasmPkgReady": metrics["wasmPkgReady"],
                    "withE2e": metrics["withE2e"],
                },
                ensure_ascii=False,
            )
        )
        if metrics["failed"] > 0:
            fail("hay casos failed en la matriz F7")
            # Aún escribimos manifiesto pending; el exit code refleja la regresión.
            source = git_sha()
            manifest = build_pending_manifest(metrics, source)
            out = args.out_dir or Path("/tmp/archmaker-f7-candidate")
            write_outputs(out, metrics, manifest)
            return 1

    if args.out_dir or args.execute:
        source = git_sha()
        manifest = build_pending_manifest(metrics, source)
        if manifest.get("result") != "pending":
            fail("el manifiesto F7 debe permanecer pending")
            return 1
        out = args.out_dir or Path("/tmp/archmaker-f7-candidate")
        write_outputs(out, metrics, manifest)

    return 0


if __name__ == "__main__":
    sys.exit(main())
