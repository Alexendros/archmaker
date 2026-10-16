#!/usr/bin/env python3
"""VAL-F7-CANDIDATE: matriz obligatoria F7 sobre un RC_SHA candidato.

Valida el inventario, opcionalmente ejecuta la matriz y escribe métricas +
manifiesto. Separa technicalResult (ejecución) de result (aprobación).
Nunca declara RC_SHA_FUNCIONAL ni firma reviewer/reviewedAt.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import platform
import re
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path
from typing import Any, Callable

ROOT = Path(__file__).resolve().parents[1]
INVENTORY = ROOT / "docs/10-delivery/closure/f7-required-cases.json"
EXAMPLE = (
    ROOT
    / "docs/10-delivery/closure/examples/manifest.f7-candidate.pending.example.json"
)
SCHEMA_PATH = ROOT / "docs/10-delivery/closure/evidence-manifest.schema.json"
VALIDATE_SCRIPT = ROOT / "tools/validate_closure_evidence.py"
FIXTURE_DIGESTS = (
    ROOT / "crates/archmaker-core/tests/fixtures/closure/fixture-digests.json"
)
SCHEMA_REQUIRED_IDS_MIN = 10

# Conjunto atómico: un subconjunto de comandos no cierra el caso completo.
REQUIRED_CASE_IDS = {
    "F7-MOTOR-REAL",
    "F7-MOTOR-REAL-WASM",
    "F7-GEN-EDIT-EXPORT",
    "F7-GEN-EDIT-EXPORT-WASM",
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
    "F7-P95",
    "F7-OFFLINE-WEB",
    "F7-OFFLINE-DESKTOP",
    "F7-E2E-X3",
}

WASM_PKG_REQUIRED = (
    "archmaker_wasm.js",
    "archmaker_wasm_bg.wasm",
    "archmaker_wasm.d.ts",
)

RunCommandFn = Callable[[str, dict[str, str] | None, Path | None], dict[str, Any]]


def fail(msg: str) -> None:
    print(f"FAIL {msg}", file=sys.stderr)


def utc_now() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def git_sha(cwd: Path | None = None) -> str | None:
    try:
        out = subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=cwd or ROOT, text=True
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        return None
    return out if len(out) == 40 else None


def file_sha256(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as handle:
        for chunk in iter(lambda: handle.read(65536), b""):
            h.update(chunk)
    return h.hexdigest()


def wasm_pkg_files() -> dict[str, bool]:
    pkg = ROOT / "crates/archmaker-wasm/pkg"
    return {name: (pkg / name).is_file() for name in WASM_PKG_REQUIRED}


def wasm_pkg_ready() -> bool:
    return all(wasm_pkg_files().values())


def capture_toolchain() -> dict[str, str | None]:
    def ver(cmd: list[str]) -> str | None:
        try:
            out = subprocess.check_output(cmd, cwd=ROOT, text=True, stderr=subprocess.STDOUT)
        except (OSError, subprocess.CalledProcessError):
            return None
        return out.strip().splitlines()[0] if out.strip() else None

    return {
        "rustc": ver(["rustc", "--version"]),
        "cargo": ver(["cargo", "--version"]),
        "node": ver(["node", "--version"]),
        "pnpm": ver(["pnpm", "--version"]),
    }


def capture_platform() -> dict[str, Any]:
    memory: int | None = None
    meminfo = Path("/proc/meminfo")
    if meminfo.is_file():
        for line in meminfo.read_text(encoding="utf-8").splitlines():
            if line.startswith("MemTotal:"):
                parts = line.split()
                if len(parts) >= 2 and parts[1].isdigit():
                    memory = int(parts[1]) * 1024
                break
    return {
        "os": f"{platform.system()} {platform.release()}",
        "cpu": platform.processor() or platform.machine(),
        "memoryBytes": memory,
        "browser": None,
        "webview": None,
    }


def dirty_tracked_files() -> list[str] | None:
    try:
        out = subprocess.check_output(
            ["git", "status", "--porcelain", "--untracked-files=no"],
            cwd=ROOT,
            text=True,
        )
    except (OSError, subprocess.CalledProcessError):
        return None
    files: list[str] = []
    for line in out.splitlines():
        if not line.strip():
            continue
        path = line[3:].strip() if len(line) > 3 else line.strip()
        if path:
            files.append(path)
    return files


def fixture_set_digest() -> str | None:
    if not FIXTURE_DIGESTS.is_file():
        return None
    return file_sha256(FIXTURE_DIGESTS)


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
    ids: list[str] = []
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
        cmds = case.get("commands") or []
        if not isinstance(cmds, list):
            errors.append(f"{cid}: commands debe ser lista")
        wasm_cases = {
            "F7-MOTOR-REAL-WASM",
            "F7-GEN-EDIT-EXPORT-WASM",
            "F7-OFFLINE-WEB",
            "F7-E2E-X3",
        }
        if cid in wasm_cases or case.get("e2eWasmRequired"):
            if not case.get("e2eWasmRequired"):
                errors.append(f"{cid}: e2eWasmRequired debe ser true")
        if cid == "F7-OFFLINE-DESKTOP":
            ds = case.get("defaultStatus")
            if ds not in {"pending", "executable"}:
                errors.append(f"{cid}: defaultStatus inválido")
            if case.get("defaultStatus") == "pending" and cmds:
                errors.append(f"{cid}: pending no debe listar commands ejecutables de CI")
        if cid in {"F7-OFFLINE-WEB", "F7-E2E-X3", "F7-P95"}:
            ds = case.get("defaultStatus")
            if ds not in {"pending", "executable"}:
                errors.append(f"{cid}: defaultStatus debe ser pending o executable")
            if ds == "executable" and not cmds:
                errors.append(f"{cid}: executable exige commands no vacíos")
            last = case.get("lastVerified")
            if last is not None:
                if not isinstance(last, dict) or last.get("status") not in {
                    "passed",
                    "failed",
                    "pending",
                }:
                    errors.append(f"{cid}: lastVerified.status inválido")
                if last.get("status") == "passed" and last.get("retries") not in (0, None):
                    errors.append(f"{cid}: lastVerified.retries debe ser 0 si passed")
    if len(ids) != len(set(ids)):
        dupes = sorted({i for i in ids if ids.count(i) > 1})
        errors.append(f"ids de caso duplicados: {dupes}")
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


def ensure_retries_zero(cmd: str) -> str:
    if "playwright" not in cmd:
        return cmd
    if "--retries=" in cmd or "--retries " in cmd:
        return re.sub(r"--retries(=\s*|\s+)\d+", "--retries=0", cmd)
    return f"{cmd} --retries=0"


def is_playwright_cmd(cmd: str) -> bool:
    return "playwright" in cmd


def is_cargo_test_cmd(cmd: str) -> bool:
    return bool(re.search(r"\bcargo\s+test\b", cmd))


def parse_playwright_report(report: dict[str, Any]) -> dict[str, Any]:
    suites = report.get("suites") or []
    tests: list[dict[str, Any]] = []

    def walk(node: dict[str, Any]) -> None:
        for suite in node.get("suites") or []:
            walk(suite)
        for spec in node.get("specs") or []:
            for test in spec.get("tests") or []:
                results = test.get("results") or []
                status = test.get("status") or (results[-1].get("status") if results else "unknown")
                retries = max(0, len(results) - 1) if results else 0
                tests.append(
                    {
                        "title": spec.get("title") or test.get("title") or "",
                        "status": status,
                        "retries": retries,
                        "expectedStatus": test.get("expectedStatus"),
                    }
                )

    root = {"suites": suites}
    walk(root)
    # Playwright top-level may nest differently
    if not tests and "suites" in report:
        for suite in suites:
            walk(suite)

    passed = sum(1 for t in tests if t["status"] in {"expected", "passed"})
    failed = sum(1 for t in tests if t["status"] in {"unexpected", "failed", "timedOut"})
    skipped = sum(1 for t in tests if t["status"] in {"skipped", "ignored"})
    flaky = sum(1 for t in tests if t["status"] == "flaky")
    retries = sum(int(t["retries"]) for t in tests)
    return {
        "tests": tests,
        "testCount": len(tests),
        "passed": passed,
        "failed": failed + flaky,
        "skipped": skipped,
        "retries": retries,
    }


def parse_cargo_summary(output: str) -> dict[str, Any]:
    # test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out
    pattern = re.compile(
        r"test result:\s+\w+\.\s+(\d+)\s+passed;\s+(\d+)\s+failed;\s+(\d+)\s+ignored",
        re.MULTILINE,
    )
    matches = list(pattern.finditer(output))
    if not matches:
        return {
            "tests": [],
            "testCount": 0,
            "passed": 0,
            "failed": 0,
            "skipped": 0,
            "retries": 0,
            "parseError": "sin resumen test result",
        }
    passed = failed = ignored = 0
    for match in matches:
        passed += int(match.group(1))
        failed += int(match.group(2))
        ignored += int(match.group(3))
    total = passed + failed + ignored
    tests = [
        {"title": f"cargo-summary-{i}", "status": "passed", "retries": 0}
        for i in range(passed)
    ]
    tests.extend(
        {"title": f"cargo-failed-{i}", "status": "failed", "retries": 0}
        for i in range(failed)
    )
    return {
        "tests": tests,
        "testCount": total,
        "passed": passed,
        "failed": failed,
        "skipped": ignored,
        "retries": 0,
    }


def default_run_command(
    cmd: str, env: dict[str, str] | None = None, report_dir: Path | None = None
) -> dict[str, Any]:
    """Ejecuta un comando y adjunta informe de tests cuando aplica."""
    env_map = (env or os.environ).copy()
    work_dir = report_dir or Path(tempfile.mkdtemp(prefix="f7-cmd-"))
    work_dir.mkdir(parents=True, exist_ok=True)
    cmd_run = ensure_retries_zero(cmd)
    playwright_json = work_dir / "playwright-report.json"
    stdout_path = work_dir / "stdout.txt"
    stderr_path = work_dir / "stderr.txt"

    if is_playwright_cmd(cmd_run):
        # Reporter JSON a fichero; line para consola.
        cmd_run = (
            f"{cmd_run} --reporter=json"
            if "--reporter" not in cmd_run
            else cmd_run
        )
        env_map["PLAYWRIGHT_JSON_OUTPUT_NAME"] = str(playwright_json)

    print(f"+ {cmd_run}")
    completed = subprocess.run(
        cmd_run,
        shell=True,
        cwd=ROOT,
        env=env_map,
        check=False,
        capture_output=True,
        text=True,
    )
    stdout_path.write_text(completed.stdout or "", encoding="utf-8")
    stderr_path.write_text(completed.stderr or "", encoding="utf-8")
    combined = (completed.stdout or "") + "\n" + (completed.stderr or "")

    test_summary: dict[str, Any] = {
        "tests": [],
        "testCount": 0,
        "passed": 0,
        "failed": 0,
        "skipped": 0,
        "retries": 0,
    }
    if is_playwright_cmd(cmd):
        report_obj = None
        if playwright_json.is_file() and playwright_json.stat().st_size > 0:
            try:
                report_obj = json.loads(playwright_json.read_text(encoding="utf-8"))
            except json.JSONDecodeError:
                report_obj = None
        if report_obj is None:
            # Algunos entornos vuelcan JSON en stdout.
            try:
                report_obj = json.loads(completed.stdout or "")
            except json.JSONDecodeError:
                report_obj = None
        if report_obj is not None:
            test_summary = parse_playwright_report(report_obj)
        else:
            test_summary["parseError"] = "informe Playwright ausente o inválido"
    elif is_cargo_test_cmd(cmd):
        test_summary = parse_cargo_summary(combined)
    else:
        # Comando genérico (python harness, scripts): exit code es la autoridad;
        # se exige al menos un "test" sintético si exit 0 para no inventar cero tests
        # en modo estricto cuando el harness declara n muestras.
        if completed.returncode == 0:
            test_summary = {
                "tests": [{"title": "command", "status": "passed", "retries": 0}],
                "testCount": 1,
                "passed": 1,
                "failed": 0,
                "skipped": 0,
                "retries": 0,
            }
        else:
            test_summary = {
                "tests": [{"title": "command", "status": "failed", "retries": 0}],
                "testCount": 1,
                "passed": 0,
                "failed": 1,
                "skipped": 0,
                "retries": 0,
            }

    status = "passed"
    notes: list[str] = []
    if completed.returncode != 0:
        status = "failed"
        notes.append(f"exit={completed.returncode}")
    if test_summary.get("testCount", 0) == 0:
        status = "failed"
        notes.append("cero tests descubiertos")
    if test_summary.get("failed", 0) > 0:
        status = "failed"
        notes.append("tests failed en informe")
    if test_summary.get("retries", 0) > 0:
        status = "failed"
        notes.append(f"retries={test_summary['retries']} (debe ser 0)")
    if test_summary.get("parseError"):
        status = "failed"
        notes.append(str(test_summary["parseError"]))

    return {
        "command": cmd,
        "commandRun": cmd_run,
        "exitCode": completed.returncode,
        "status": status,
        "notes": notes,
        "tests": test_summary,
        "stdoutPath": str(stdout_path),
        "stderrPath": str(stderr_path),
    }


def command_requires_wasm(cmd: str) -> bool:
    return "wasm-flow" in cmd or "wasm-offline" in cmd


def execute_matrix(
    *,
    with_e2e: bool,
    strict: bool,
    run_command: RunCommandFn | None = None,
    report_root: Path | None = None,
) -> dict[str, Any]:
    inventory = json.loads(INVENTORY.read_text(encoding="utf-8"))
    runner = run_command or default_run_command
    results: list[dict[str, Any]] = []
    wasm_ok = wasm_pkg_ready()
    e2e_enabled = with_e2e and wasm_ok
    started = utc_now()
    report_root = report_root or Path(tempfile.mkdtemp(prefix="f7-matrix-"))
    report_root.mkdir(parents=True, exist_ok=True)

    for case in inventory["cases"]:
        cid = case["id"]
        note = case.get("residual")
        commands: list[str] = list(case.get("commands") or [])
        command_results: list[dict[str, Any]] = []

        # Escritorio offline: sin comando automatizado en este runner.
        if cid == "F7-OFFLINE-DESKTOP" or case.get("defaultStatus") == "pending" and not commands:
            results.append(
                {
                    "id": cid,
                    "status": "pending",
                    "commands": [],
                    "note": note or "sin comando ejecutable en este runner",
                }
            )
            continue

        if case.get("e2eWasmRequired") and not e2e_enabled:
            results.append(
                {
                    "id": cid,
                    "status": "pending" if not strict else "partial",
                    "commands": [],
                    "note": note
                    or "Playwright WASM no disponible o --with-e2e omitido",
                }
            )
            continue

        if not commands:
            results.append(
                {
                    "id": cid,
                    "status": "pending",
                    "commands": [],
                    "note": note or "sin comando ejecutable en este runner",
                }
            )
            continue

        # Todos los comandos del caso son obligatorios; no omitir wasm-flow
        # silenciosamente. Sin e2e, el caso queda partial/pending.
        skipped_cmds: list[str] = []
        runnable: list[str] = []
        for cmd in commands:
            if command_requires_wasm(cmd) and not e2e_enabled:
                skipped_cmds.append(cmd)
            else:
                runnable.append(cmd)

        if skipped_cmds and not runnable:
            results.append(
                {
                    "id": cid,
                    "status": "partial" if strict else "pending",
                    "commands": [
                        {
                            "command": c,
                            "status": "skipped",
                            "exitCode": None,
                            "tests": {"testCount": 0},
                            "notes": ["omitido sin --with-e2e / pkg WASM completo"],
                        }
                        for c in skipped_cmds
                    ],
                    "note": "comandos WASM omitidos sin --with-e2e / pkg completo",
                }
            )
            continue

        if skipped_cmds and runnable:
            # Subconjunto: NUNCA passed.
            for cmd in skipped_cmds:
                command_results.append(
                    {
                        "command": cmd,
                        "status": "skipped",
                        "exitCode": None,
                        "tests": {"testCount": 0, "passed": 0, "failed": 0, "skipped": 0, "retries": 0},
                        "notes": ["omitido sin --with-e2e / pkg WASM completo"],
                    }
                )

        case_dir = report_root / cid
        case_dir.mkdir(parents=True, exist_ok=True)
        for index, cmd in enumerate(runnable):
            cmd_dir = case_dir / f"cmd-{index:02d}"
            cmd_dir.mkdir(parents=True, exist_ok=True)
            command_results.append(runner(cmd, None, cmd_dir))

        statuses = {c["status"] for c in command_results}
        if "failed" in statuses:
            case_status = "failed"
        elif "skipped" in statuses:
            case_status = "partial"
        elif statuses == {"passed"} and len(command_results) == len(commands):
            case_status = "passed"
        elif not command_results:
            case_status = "pending"
        else:
            case_status = "partial"

        results.append(
            {
                "id": cid,
                "status": case_status,
                "commands": command_results,
                "note": note,
            }
        )

    completed = utc_now()
    executed = sum(1 for r in results if r["status"] in {"passed", "failed", "partial"})
    passed = sum(1 for r in results if r["status"] == "passed")
    failed = sum(1 for r in results if r["status"] == "failed")
    partial = sum(1 for r in results if r["status"] == "partial")
    skipped = sum(1 for r in results if r["status"] == "pending")
    retries_total = 0
    test_total = 0
    for case in results:
        for cmd in case.get("commands") or []:
            tests = cmd.get("tests") or {}
            retries_total += int(tests.get("retries") or 0)
            test_total += int(tests.get("testCount") or 0)

    if failed > 0:
        technical = "failed"
    elif partial > 0 or (strict and skipped > 0):
        technical = "partial"
    elif passed == len(results) and skipped == 0 and partial == 0:
        technical = "passed"
    elif executed == 0:
        technical = "pending"
    else:
        technical = "partial"

    return {
        "cases": results,
        "executed": executed,
        "passed": passed,
        "failed": failed,
        "partial": partial,
        "skipped": skipped,
        "retries": retries_total,
        "testCount": test_total,
        "wasmPkgReady": wasm_ok,
        "wasmPkgFiles": wasm_pkg_files(),
        "withE2e": e2e_enabled,
        "strict": strict,
        "technicalResult": technical,
        "startedAt": started,
        "completedAt": completed,
        "reportRoot": str(report_root),
    }


def build_pending_manifest(
    metrics: dict[str, Any] | None,
    source_sha: str | None,
    *,
    harness_sha: str | None = None,
) -> dict[str, Any]:
    inventory = json.loads(INVENTORY.read_text(encoding="utf-8"))
    residuals = list(inventory.get("limitations") or [])
    residuals.extend(
        [
            "No RC_SHA_FUNCIONAL: candidato documentado, no aceptación firmada",
            "Sin aceptación independiente firmada",
            "P95 F6 pending sin 30 muestras en máquina fijada en F0"
            if not metrics
            else "Manifiesto F7: result=pending hasta aprobación independiente",
        ]
    )
    for case in inventory.get("cases") or []:
        if not isinstance(case, dict):
            continue
        if case.get("id") == "F7-OFFLINE-DESKTOP":
            residuals.append(
                "F7-OFFLINE-DESKTOP: escritorio offline pending (sin humo de red/portal)"
            )
        residual = case.get("residual")
        if case.get("id") == "F7-P95" and residual:
            residuals.append(f"F7-P95: {residual}")
        if case.get("id") == "F7-REG-F6" and residual:
            residuals.append(f"F7-REG-F6: {residual}")
    if metrics:
        for case in metrics.get("cases") or []:
            if case.get("status") in {"pending", "partial"} and case.get("note"):
                residuals.append(f"{case['id']}: {case['note']}")

    started = metrics.get("startedAt") if metrics else None
    completed = metrics.get("completedAt") if metrics else None
    if metrics and started and completed and started == completed:
        # Defensa: si el reloj no avanzó, al menos documentar el par real.
        pass

    harness_obj = None
    if harness_sha or source_sha:
        harness_obj = {
            "sourceSha": harness_sha or source_sha,
            "productSourceSha": source_sha,
            "runner": "tools/run_f7_candidate_matrix.py",
        }

    technical = None if metrics is None else metrics.get("technicalResult")
    commands_flat: list[dict[str, Any]] | None = None
    tests_flat: list[dict[str, Any]] | None = None
    if metrics:
        commands_flat = []
        tests_flat = []
        for case in metrics.get("cases") or []:
            for cmd in case.get("commands") or []:
                commands_flat.append(
                    {
                        "caseId": case.get("id"),
                        "command": cmd.get("command"),
                        "status": cmd.get("status"),
                        "exitCode": cmd.get("exitCode"),
                        "notes": cmd.get("notes"),
                    }
                )
                for test in (cmd.get("tests") or {}).get("tests") or []:
                    tests_flat.append(
                        {
                            "caseId": case.get("id"),
                            "command": cmd.get("command"),
                            "title": test.get("title"),
                            "status": test.get("status"),
                            "retries": test.get("retries"),
                        }
                    )

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
        "startedAt": started,
        "completedAt": completed,
        "toolchain": capture_toolchain() if metrics is not None else None,
        "platform": capture_platform() if metrics is not None else None,
        "dirtyTrackedFiles": dirty_tracked_files() if metrics is not None else None,
        "fixtureSetDigest": fixture_set_digest() if metrics is not None else None,
        "requiredTestIds": [c["id"] for c in inventory["cases"]],
        "executed": None if metrics is None else metrics["executed"],
        "passed": None if metrics is None else metrics["passed"],
        "failed": None if metrics is None else metrics["failed"],
        "skipped": None if metrics is None else metrics["skipped"],
        "retries": None if metrics is None else metrics["retries"],
        "artifacts": None,
        "result": "pending",
        "reviewer": None,
        "reviewedAt": None,
        "residuals": residuals,
        "harness": harness_obj,
        "technicalResult": technical,
        "commands": commands_flat,
        "tests": tests_flat,
        "approval": {
            "status": "pending",
            "approvalRecordSha256": None,
            "approvalRecordCommit": None,
            "reviewer": None,
            "reviewedAt": None,
        },
    }


def validate_manifest_against_schema(manifest: dict[str, Any]) -> list[str]:
    if not SCHEMA_PATH.is_file():
        return [f"falta schema {SCHEMA_PATH}"]
    # Reutilizar el validador del repo vía import ligero del módulo.
    sys.path.insert(0, str(ROOT / "tools"))
    try:
        import validate_closure_evidence as vce  # type: ignore
    except ImportError:
        return ["no se pudo importar validate_closure_evidence"]
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    errors: list[str] = []
    vce.validate_against(schema, manifest, "$", errors)
    return errors


def write_outputs(out_dir: Path, metrics: dict[str, Any] | None, manifest: dict) -> None:
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


def strict_gate(metrics: dict[str, Any]) -> list[str]:
    errors: list[str] = []
    if metrics.get("technicalResult") != "passed":
        errors.append(
            f"technicalResult={metrics.get('technicalResult')!r} (se exige passed)"
        )
    if metrics.get("failed", 0) > 0:
        errors.append("hay casos failed")
    if metrics.get("partial", 0) > 0:
        errors.append("hay casos partial (comandos omitidos o incompletos)")
    if metrics.get("skipped", 0) > 0:
        errors.append("hay casos pending/omitidos")
    if metrics.get("retries", 0) > 0:
        errors.append("retries agregados distintos de 0")
    if metrics.get("testCount", 0) == 0:
        errors.append("cero tests en toda la matriz")
    if not metrics.get("wasmPkgReady"):
        errors.append("pkg WASM incompleto")
    for case in metrics.get("cases") or []:
        if case.get("status") != "passed":
            errors.append(f"{case.get('id')}: status={case.get('status')}")
            continue
        for cmd in case.get("commands") or []:
            if cmd.get("status") != "passed":
                errors.append(f"{case.get('id')}: comando {cmd.get('status')}")
            tests = cmd.get("tests") or {}
            if int(tests.get("testCount") or 0) == 0:
                errors.append(f"{case.get('id')}: comando con cero tests")
            if int(tests.get("retries") or 0) > 0:
                errors.append(f"{case.get('id')}: retries en informe")
    return errors


def main(argv: list[str] | None = None) -> int:
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
        help="Incluye casos WASM si el pkg completo está presente",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Cierre técnico: falla si hay omitidos/partial/failed/cero tests",
    )
    parser.add_argument(
        "--out-dir",
        type=Path,
        default=None,
        help="Directorio para metrics.json y manifest.json",
    )
    args = parser.parse_args(argv)

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
    exit_code = 0
    if args.execute and not args.validate_only:
        out = args.out_dir or Path("/tmp/archmaker-f7-candidate")
        report_root = out / "command-reports"
        metrics = execute_matrix(
            with_e2e=args.with_e2e, strict=args.strict, report_root=report_root
        )
        print(
            json.dumps(
                {
                    "executed": metrics["executed"],
                    "passed": metrics["passed"],
                    "failed": metrics["failed"],
                    "partial": metrics["partial"],
                    "skipped": metrics["skipped"],
                    "retries": metrics["retries"],
                    "testCount": metrics["testCount"],
                    "technicalResult": metrics["technicalResult"],
                    "wasmPkgReady": metrics["wasmPkgReady"],
                    "withE2e": metrics["withE2e"],
                    "strict": metrics["strict"],
                },
                ensure_ascii=False,
            )
        )
        if metrics["failed"] > 0:
            fail("hay casos failed en la matriz F7")
            exit_code = 1
        if args.strict:
            gate_errors = strict_gate(metrics)
            if gate_errors:
                for err in gate_errors:
                    fail(f"strict: {err}")
                exit_code = 1

    if args.out_dir or args.execute:
        source = git_sha()
        harness = git_sha()  # mismo árbol; externo puede sobrescribir en evidencia
        manifest = build_pending_manifest(metrics, source, harness_sha=harness)
        if manifest.get("result") != "pending":
            fail("el manifiesto F7 debe permanecer pending")
            return 1
        schema_errors = validate_manifest_against_schema(manifest)
        if schema_errors:
            for err in schema_errors:
                fail(f"schema: {err}")
            exit_code = 1
        out = args.out_dir or Path("/tmp/archmaker-f7-candidate")
        write_outputs(out, metrics, manifest)

    return exit_code


if __name__ == "__main__":
    sys.exit(main())
