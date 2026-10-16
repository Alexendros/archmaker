#!/usr/bin/env python3
"""Harness P95 de producto: web-ready / web-prepare (WASM real).

Certifica el presupuesto de producto de honest-ux.md (ready ≤ 3000 ms,
prepare ≤ 1000 ms, n ≥ 30). No usa MockCorePort.

El microbenchmark nativo tools/run_f7_p95_harness.py NO certifica producto.

Uso:
  python3 tools/run_product_p95_harness.py
  python3 tools/run_product_p95_harness.py --samples 30 --out-dir docs/10-delivery/closure
  python3 tools/run_product_p95_harness.py --allow-pending   # diagnóstico; no cierra aceptación
"""

from __future__ import annotations

import argparse
import json
import math
import os
import platform
import signal
import socket
import subprocess
import sys
import time
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
WEB = ROOT / "apps" / "web"
SAMPLE_SCRIPT = WEB / "scripts" / "product-p95-sample.mjs"
WASM_PKG = ROOT / "crates" / "archmaker-wasm" / "pkg"
READY_P95_MS = 3000.0
PREPARE_P95_MS = 1000.0
DEFAULT_SAMPLES = 30
DEFAULT_PORT = 4173
REPORT_NAME = "product-p95-report.json"


def fail(msg: str) -> None:
    print(f"FAIL {msg}", file=sys.stderr)


def utc_now() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def percentile(sorted_values: list[float], p: float) -> float:
    if not sorted_values:
        raise ValueError("sin muestras")
    if len(sorted_values) == 1:
        return sorted_values[0]
    rank = max(1, math.ceil(p / 100.0 * len(sorted_values)))
    return sorted_values[rank - 1]


def git_sha() -> str | None:
    try:
        out = subprocess.check_output(
            ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
        ).strip()
    except (OSError, subprocess.CalledProcessError):
        return None
    return out if len(out) == 40 else None


def cpu_label() -> str:
    try:
        out = subprocess.check_output(["lscpu"], text=True)
    except (OSError, subprocess.CalledProcessError):
        return platform.processor() or platform.machine()
    for line in out.splitlines():
        lower = line.lower()
        if lower.startswith("model name:") or lower.startswith("nombre del modelo:"):
            return line.split(":", 1)[1].strip()
    return platform.processor() or platform.machine()


def wasm_pkg_ready() -> bool:
    required = (
        "archmaker_wasm.js",
        "archmaker_wasm_bg.wasm",
        "archmaker_wasm.d.ts",
    )
    return all((WASM_PKG / name).is_file() for name in required)


def port_free(port: int) -> bool:
    with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as sock:
        sock.setsockopt(socket.SOL_SOCKET, socket.SO_REUSEADDR, 1)
        try:
            sock.bind(("127.0.0.1", port))
        except OSError:
            return False
    return True


def wait_http(url: str, timeout_s: float = 120.0) -> None:
    deadline = time.time() + timeout_s
    while time.time() < deadline:
        try:
            completed = subprocess.run(
                ["curl", "-fsS", "-o", "/dev/null", url],
                check=False,
                capture_output=True,
                text=True,
            )
            if completed.returncode == 0:
                return
        except OSError:
            pass
        time.sleep(0.25)
    raise RuntimeError(f"servidor no respondió en {timeout_s:.0f}s: {url}")


def ensure_web_build(skip_build: bool) -> dict[str, Any]:
    profile: dict[str, Any] = {
        "mode": "vite-preview",
        "wasmPkgReady": wasm_pkg_ready(),
        "distPresent": (WEB / "dist" / "index.html").is_file(),
        "skipBuild": skip_build,
        "commands": [],
    }
    if skip_build:
        if not profile["wasmPkgReady"]:
            raise RuntimeError("pkg WASM incompleto y --skip-build")
        if not profile["distPresent"]:
            raise RuntimeError("apps/web/dist ausente y --skip-build")
        profile["commands"].append(
            "(reutilizado dist + pkg WASM existentes; --skip-build)"
        )
        return profile

    if not profile["wasmPkgReady"] or not profile["distPresent"]:
        cmd = ["pnpm", "build"]
        profile["commands"].append(" ".join(cmd))
        print("+ " + " ".join(cmd) + " (cwd=apps/web)")
        completed = subprocess.run(cmd, cwd=WEB, check=False)
        if completed.returncode != 0:
            raise RuntimeError("pnpm build falló")
        profile["wasmPkgReady"] = wasm_pkg_ready()
        profile["distPresent"] = (WEB / "dist" / "index.html").is_file()
        if not profile["wasmPkgReady"] or not profile["distPresent"]:
            raise RuntimeError("build incompleto tras pnpm build")
    else:
        profile["commands"].append("(reutilizado dist + pkg WASM existentes)")
    return profile


def start_preview(port: int) -> subprocess.Popen[str]:
    if not port_free(port):
        raise RuntimeError(f"puerto {port} ocupado")
    cmd = [
        "pnpm",
        "exec",
        "vite",
        "preview",
        "--host",
        "127.0.0.1",
        "--port",
        str(port),
        "--strictPort",
    ]
    print("+ " + " ".join(cmd))
    proc = subprocess.Popen(
        cmd,
        cwd=WEB,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
        text=True,
        start_new_session=True,
    )
    return proc


def stop_preview(proc: subprocess.Popen[str] | None) -> None:
    if proc is None:
        return
    try:
        os.killpg(proc.pid, signal.SIGTERM)
    except (ProcessLookupError, PermissionError, OSError):
        try:
            proc.terminate()
        except OSError:
            pass
    try:
        proc.wait(timeout=10)
    except subprocess.TimeoutExpired:
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except (ProcessLookupError, PermissionError, OSError):
            proc.kill()


def playwright_env() -> dict[str, str]:
    env = os.environ.copy()
    pb = env.get("PLAYWRIGHT_BROWSERS_PATH")
    if pb and pb != "0":
        root = Path(pb)
        if not root.is_dir() or not any(root.glob("chromium*")):
            # Sandbox/cache vacío: usar browsers del paquete playwright-core.
            env["PLAYWRIGHT_BROWSERS_PATH"] = "0"
    return env


def run_one_sample(base_url: str, timeout_ms: int) -> dict[str, Any]:
    env = playwright_env()
    env["BASE_URL_WASM"] = base_url
    env["PRODUCT_P95_TIMEOUT_MS"] = str(timeout_ms)
    completed = subprocess.run(
        ["node", str(SAMPLE_SCRIPT)],
        cwd=WEB,
        env=env,
        check=False,
        capture_output=True,
        text=True,
    )
    line = ""
    for raw in (completed.stdout or "").splitlines():
        raw = raw.strip()
        if raw.startswith("{") and "readyMs" in raw:
            line = raw
    if not line:
        return {
            "readyMs": None,
            "prepareMs": None,
            "valid": False,
            "error": f"sin JSON de muestra (exit={completed.returncode}): "
            f"{(completed.stderr or completed.stdout or '')[-400:]}",
        }
    try:
        data = json.loads(line)
    except json.JSONDecodeError as exc:
        return {
            "readyMs": None,
            "prepareMs": None,
            "valid": False,
            "error": f"JSON inválido: {exc}",
        }
    if not isinstance(data, dict):
        return {
            "readyMs": None,
            "prepareMs": None,
            "valid": False,
            "error": "payload no es objeto",
        }
    ready = data.get("readyMs")
    prepare = data.get("prepareMs")
    valid = bool(data.get("valid")) and isinstance(ready, (int, float)) and isinstance(
        prepare, (int, float)
    )
    if not valid:
        return {
            "readyMs": ready if isinstance(ready, (int, float)) else None,
            "prepareMs": prepare if isinstance(prepare, (int, float)) else None,
            "valid": False,
            "error": data.get("error") or "muestra inválida",
        }
    return {
        "readyMs": float(ready),
        "prepareMs": float(prepare),
        "valid": True,
        "error": None,
    }


def metric_block(
    samples: list[float | None],
    *,
    threshold: float,
) -> dict[str, Any]:
    """Una muestra inválida/ausente cuenta en el denominador (P95 con +inf)."""
    n = len(samples)
    invalid = sum(1 for s in samples if s is None)
    for_p95 = [float("inf") if s is None else float(s) for s in samples]
    sorted_vals = sorted(for_p95)
    p95 = percentile(sorted_vals, 95) if sorted_vals else float("inf")
    finite = [s for s in samples if s is not None]
    return {
        "samples": samples,
        "n": n,
        "validCount": n - invalid,
        "invalidCount": invalid,
        "p95": p95 if math.isfinite(p95) else None,
        "p95IsInfinite": not math.isfinite(p95),
        "min": min(finite) if finite else None,
        "max": max(finite) if finite else None,
        "thresholdMs": threshold,
        "meetsThreshold": math.isfinite(p95) and p95 <= threshold and invalid == 0,
    }


def assess_desktop() -> dict[str, Any]:
    """Desktop P95: intenta si hay GUI + AppImage del SHA; si no, blocked."""
    display = os.environ.get("DISPLAY") or ""
    wayland = os.environ.get("WAYLAND_DISPLAY") or ""
    appimage_dir = ROOT / "target" / "release" / "bundle" / "appimage"
    forge_images = sorted(appimage_dir.glob("Forge_*.AppImage")) if appimage_dir.is_dir() else []
    missing = [
        "desktop-ready-p95-samples",
        "desktop-prepare-p95-samples",
        "Forge_*.AppImage matching sourceSha under target/release/bundle/appimage",
    ]
    if not forge_images:
        return {
            "result": "blocked",
            "ready": None,
            "prepare": None,
            "display": display or None,
            "waylandDisplay": wayland or None,
            "appImage": None,
            "missingEvidence": missing,
            "reason": (
                "Sin AppImage Forge del árbol bajo prueba; no hay harness desktop "
                "automatizado de ready/prepare. DISPLAY/Wayland no bastan sin binario usable."
            ),
        }
    # Hay AppImage pero no hay automatización desktop P95 en este alcance.
    return {
        "result": "blocked",
        "ready": None,
        "prepare": None,
        "display": display or None,
        "waylandDisplay": wayland or None,
        "appImage": str(forge_images[0]),
        "missingEvidence": [
            "desktop-ready-p95-samples",
            "desktop-prepare-p95-samples",
            "desktop product-p95 automation harness",
        ],
        "reason": (
            "AppImage presente pero no hay medición automatizada desktop-ready/"
            "desktop-prepare (n≥30) en este harness."
        ),
    }


def build_report(
    *,
    samples_n: int,
    ready_samples: list[float | None],
    prepare_samples: list[float | None],
    sample_errors: list[str | None],
    build_profile: dict[str, Any],
    desktop: dict[str, Any],
    started: str,
    base_url: str,
    allow_pending: bool,
) -> dict[str, Any]:
    ready = metric_block(ready_samples, threshold=READY_P95_MS)
    prepare = metric_block(prepare_samples, threshold=PREPARE_P95_MS)
    n_ok = samples_n >= DEFAULT_SAMPLES
    web_ok = bool(ready["meetsThreshold"] and prepare["meetsThreshold"] and n_ok)
    desktop_ok = desktop.get("result") == "passed"

    web_result = "passed" if web_ok else "failed"
    if allow_pending and not web_ok:
        web_result = "pending"

    residuals: list[str] = []
    if not n_ok:
        residuals.append(f"n={samples_n} < {DEFAULT_SAMPLES}")
    if ready["invalidCount"]:
        residuals.append(f"web-ready: {ready['invalidCount']} muestras inválidas/ausentes")
    if prepare["invalidCount"]:
        residuals.append(
            f"web-prepare: {prepare['invalidCount']} muestras inválidas/ausentes"
        )
    if ready["p95"] is not None and not ready["meetsThreshold"]:
        residuals.append(
            f"web-ready P95 {ready['p95']:.3f} ms > {READY_P95_MS}"
        )
    elif ready["p95IsInfinite"]:
        residuals.append("web-ready P95 indefinido (muestras inválidas en denominador)")
    if prepare["p95"] is not None and not prepare["meetsThreshold"]:
        residuals.append(
            f"web-prepare P95 {prepare['p95']:.3f} ms > {PREPARE_P95_MS}"
        )
    elif prepare["p95IsInfinite"]:
        residuals.append("web-prepare P95 indefinido (muestras inválidas en denominador)")
    if desktop.get("result") == "blocked":
        residuals.append(
            "desktop blocked: " + ", ".join(desktop.get("missingEvidence") or [])
        )

    # Aceptación global de rendimiento exige web + desktop.
    if allow_pending:
        global_result = "pending"
        acceptance_closed = False
    elif desktop.get("result") == "blocked":
        global_result = "blocked"
        acceptance_closed = False
    elif web_ok and desktop_ok:
        global_result = "passed"
        acceptance_closed = True
    else:
        global_result = "failed"
        acceptance_closed = False

    technical = "passed" if web_ok else ("pending" if allow_pending else "failed")
    if desktop.get("result") == "blocked" and web_ok and not allow_pending:
        technical = "partial"

    return {
        "schemaVersion": "1",
        "phase": "F7-PRODUCT-P95",
        "kind": "product",
        "certifiesProduct": True,
        "acceptanceClosed": acceptance_closed,
        "allowPending": allow_pending,
        "sourceSha": git_sha(),
        "startedAt": started,
        "completedAt": utc_now(),
        "n": samples_n,
        "warmupDiscarded": 1,
        "thresholdsMs": {"readyP95": READY_P95_MS, "prepareP95": PREPARE_P95_MS},
        "buildProfile": build_profile,
        "baseUrl": base_url,
        "platform": {
            "os": f"{platform.system()} {platform.release()}",
            "cpu": cpu_label(),
            "node": subprocess.check_output(["node", "-v"], text=True).strip(),
        },
        "web": {
            "readyMs": ready,
            "prepareMs": prepare,
            "sampleErrors": sample_errors,
            "result": web_result,
        },
        "desktop": desktop,
        "microbenchmark": {
            "tool": "tools/run_f7_p95_harness.py",
            "kind": "microbenchmark",
            "certifiesProduct": False,
            "note": "Init/prepare nativo sobre fixture embebido; no cierra web-ready/web-prepare.",
        },
        "result": global_result,
        "technicalResult": technical,
        "residuals": residuals,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--samples", type=int, default=DEFAULT_SAMPLES)
    parser.add_argument("--out-dir", type=Path, default=None)
    parser.add_argument("--port", type=int, default=DEFAULT_PORT)
    parser.add_argument(
        "--skip-build",
        action="store_true",
        help="Reutiliza dist/pkg existentes (sigue registrando el perfil)",
    )
    parser.add_argument(
        "--timeout-ms",
        type=int,
        default=60_000,
        help="Timeout por fase de muestra (ready/prepare)",
    )
    parser.add_argument(
        "--allow-pending",
        action="store_true",
        help="Diagnóstico: escribe informe pending y sale 0; NUNCA cierra aceptación",
    )
    args = parser.parse_args(argv)

    started = utc_now()
    if args.samples < 1:
        fail("samples debe ser ≥ 1")
        return 1
    if not SAMPLE_SCRIPT.is_file():
        fail(f"falta {SAMPLE_SCRIPT.relative_to(ROOT)}")
        return 1

    proc: subprocess.Popen[str] | None = None
    try:
        build_profile = ensure_web_build(args.skip_build)
        base_url = f"http://127.0.0.1:{args.port}/"
        build_profile["previewPort"] = args.port
        build_profile["previewCommand"] = (
            f"pnpm exec vite preview --host 127.0.0.1 --port {args.port} --strictPort"
        )
        proc = start_preview(args.port)
        wait_http(base_url)

        # Calentamiento (no cuenta en n).
        warm = run_one_sample(base_url, args.timeout_ms)
        if not warm.get("valid"):
            fail(f"calentamiento: {warm.get('error')}")
            if not args.allow_pending:
                return 1

        ready_samples: list[float | None] = []
        prepare_samples: list[float | None] = []
        sample_errors: list[str | None] = []
        for index in range(args.samples):
            sample = run_one_sample(base_url, args.timeout_ms)
            if sample.get("valid"):
                ready_samples.append(float(sample["readyMs"]))
                prepare_samples.append(float(sample["prepareMs"]))
                sample_errors.append(None)
                print(
                    json.dumps(
                        {
                            "sample": index + 1,
                            "readyMs": sample["readyMs"],
                            "prepareMs": sample["prepareMs"],
                            "valid": True,
                        },
                        ensure_ascii=False,
                    )
                )
            else:
                # Inválida/ausente: cuenta en el denominador.
                ready_samples.append(None)
                prepare_samples.append(None)
                err = str(sample.get("error") or "inválida")
                sample_errors.append(err)
                print(
                    json.dumps(
                        {
                            "sample": index + 1,
                            "readyMs": None,
                            "prepareMs": None,
                            "valid": False,
                            "error": err,
                        },
                        ensure_ascii=False,
                    )
                )

        desktop = assess_desktop()
        report = build_report(
            samples_n=args.samples,
            ready_samples=ready_samples,
            prepare_samples=prepare_samples,
            sample_errors=sample_errors,
            build_profile=build_profile,
            desktop=desktop,
            started=started,
            base_url=base_url,
            allow_pending=args.allow_pending,
        )
    except RuntimeError as exc:
        fail(str(exc))
        if args.allow_pending:
            desktop = assess_desktop()
            report = {
                "schemaVersion": "1",
                "phase": "F7-PRODUCT-P95",
                "kind": "product",
                "certifiesProduct": True,
                "acceptanceClosed": False,
                "allowPending": True,
                "sourceSha": git_sha(),
                "startedAt": started,
                "completedAt": utc_now(),
                "n": args.samples,
                "result": "pending",
                "technicalResult": "pending",
                "residuals": [str(exc)],
                "desktop": desktop,
                "microbenchmark": {
                    "tool": "tools/run_f7_p95_harness.py",
                    "kind": "microbenchmark",
                    "certifiesProduct": False,
                },
            }
        else:
            return 1
    finally:
        stop_preview(proc)

    out_dir = args.out_dir or (ROOT / "docs" / "10-delivery" / "closure")
    out_dir.mkdir(parents=True, exist_ok=True)
    out_path = out_dir / REPORT_NAME
    out_path.write_text(
        json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    summary = {
        "wrote": str(out_path),
        "result": report.get("result"),
        "technicalResult": report.get("technicalResult"),
        "acceptanceClosed": report.get("acceptanceClosed"),
        "webReadyP95": (report.get("web") or {}).get("readyMs", {}).get("p95")
        if isinstance(report.get("web"), dict)
        else None,
        "webPrepareP95": (report.get("web") or {}).get("prepareMs", {}).get("p95")
        if isinstance(report.get("web"), dict)
        else None,
        "desktop": (report.get("desktop") or {}).get("result"),
    }
    print(json.dumps(summary, ensure_ascii=False))

    if args.allow_pending:
        # Diagnóstico: exit 0, aceptación nunca cerrada.
        return 0
    if report.get("acceptanceClosed"):
        return 0
    # Web medible puede pasar con desktop blocked (exit 0 parcial) si web OK.
    web = report.get("web") or {}
    if web.get("result") == "passed" and report.get("result") == "blocked":
        return 0
    if report.get("technicalResult") == "passed":
        return 0
    fail("P95 de producto no cumple umbrales o n insuficiente")
    return 1


if __name__ == "__main__":
    sys.exit(main())
