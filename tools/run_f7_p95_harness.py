#!/usr/bin/env python3
"""Harness F7-P95: 30 muestras de inicialización y preparación.

Umbrales (honest-ux.md): init P95 ≤ 3000 ms; prepare P95 ≤ 1000 ms; n = 30.
No fabrica cifras: si no puede medir, falla con residual explícito.

Uso:
  python3 tools/run_f7_p95_harness.py
  python3 tools/run_f7_p95_harness.py --out-dir /tmp/f7-p95 --samples 30
"""

from __future__ import annotations

import argparse
import json
import math
import os
import platform
import re
import subprocess
import sys
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
INIT_P95_MS = 3000.0
PREPARE_P95_MS = 1000.0
DEFAULT_SAMPLES = 30
SAMPLE_LINE = re.compile(
    r'\{\s*"initMs"\s*:\s*([0-9.]+)\s*,\s*"prepareMs"\s*:\s*([0-9.]+)'
)


def fail(msg: str) -> None:
    print(f"FAIL {msg}", file=sys.stderr)


def utc_now() -> str:
    return datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")


def percentile(sorted_values: list[float], p: float) -> float:
    if not sorted_values:
        raise ValueError("sin muestras")
    if len(sorted_values) == 1:
        return sorted_values[0]
    # Nearest-rank (ceil) para P95 con n=30 → índice 29 (0-based: 28) ≈ %95.
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


def build_sample_binary() -> Path:
    env = os.environ.copy()
    env["ARCHMAKER_F7_P95_SAMPLE"] = "1"
    # Preferir target del repo; evitar CARGO_TARGET_DIR de sandboxes sin permiso de ejecución.
    env["CARGO_TARGET_DIR"] = str(ROOT / "target")
    env.setdefault("TMPDIR", "/tmp")
    cmd = [
        "cargo",
        "test",
        "-p",
        "archmaker-core",
        "--test",
        "f7_p95_sample",
        "--locked",
        "--no-run",
        "--message-format=json",
    ]
    print("+ " + " ".join(cmd))
    completed = subprocess.run(
        cmd,
        cwd=ROOT,
        env=env,
        check=False,
        capture_output=True,
        text=True,
    )
    if completed.returncode != 0:
        sys.stderr.write(completed.stderr or "")
        sys.stderr.write(completed.stdout or "")
        raise RuntimeError("cargo test --no-run falló")
    executable: Path | None = None
    for line in (completed.stdout or "").splitlines():
        line = line.strip()
        if not line.startswith("{"):
            continue
        try:
            msg = json.loads(line)
        except json.JSONDecodeError:
            continue
        if msg.get("reason") != "compiler-artifact":
            continue
        target = msg.get("target") or {}
        if target.get("name") != "f7_p95_sample":
            continue
        filenames = msg.get("filenames") or []
        for name in filenames:
            path = Path(name)
            if path.is_file() and os.access(path, os.X_OK):
                executable = path
    if executable is None:
        raise RuntimeError("no se localizó el binario f7_p95_sample")
    return executable


def run_one_sample(binary: Path) -> tuple[float, float]:
    env = os.environ.copy()
    env["ARCHMAKER_F7_P95_SAMPLE"] = "1"
    env["CARGO_TARGET_DIR"] = str(ROOT / "target")
    # libtest: ejecutar el test y capturar stdout
    completed = subprocess.run(
        [str(binary), "--exact", "f7_p95_one_sample_prints_json_when_env_set", "--nocapture"],
        cwd=ROOT,
        env=env,
        check=False,
        capture_output=True,
        text=True,
    )
    combined = (completed.stdout or "") + "\n" + (completed.stderr or "")
    if completed.returncode != 0:
        raise RuntimeError(f"muestra exit={completed.returncode}: {combined[-500:]}")
    match = SAMPLE_LINE.search(combined)
    if not match:
        raise RuntimeError(f"sin línea JSON de muestra en salida: {combined[-500:]}")
    return float(match.group(1)), float(match.group(2))


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--samples", type=int, default=DEFAULT_SAMPLES)
    parser.add_argument("--out-dir", type=Path, default=None)
    parser.add_argument(
        "--allow-pending",
        action="store_true",
        help="Escribe informe pending y sale 0 sin exigir umbrales (solo diagnóstico)",
    )
    args = parser.parse_args(argv)

    started = utc_now()
    if args.samples < 1:
        fail("samples debe ser ≥ 1")
        return 1

    try:
        binary = build_sample_binary()
    except RuntimeError as exc:
        fail(str(exc))
        return 1

    # Descarte de calentamiento (no cuenta en n).
    try:
        run_one_sample(binary)
    except RuntimeError as exc:
        fail(f"calentamiento: {exc}")
        return 1

    init_samples: list[float] = []
    prepare_samples: list[float] = []
    for index in range(args.samples):
        try:
            init_ms, prepare_ms = run_one_sample(binary)
        except RuntimeError as exc:
            fail(f"muestra {index + 1}/{args.samples}: {exc}")
            return 1
        init_samples.append(init_ms)
        prepare_samples.append(prepare_ms)
        print(
            json.dumps(
                {"sample": index + 1, "initMs": init_ms, "prepareMs": prepare_ms},
                ensure_ascii=False,
            )
        )

    init_sorted = sorted(init_samples)
    prepare_sorted = sorted(prepare_samples)
    init_p95 = percentile(init_sorted, 95)
    prepare_p95 = percentile(prepare_sorted, 95)
    completed = utc_now()

    report = {
        "schemaVersion": "1",
        "phase": "F7-P95",
        "sourceSha": git_sha(),
        "startedAt": started,
        "completedAt": completed,
        "n": args.samples,
        "warmupDiscarded": 1,
        "fixture": "embedded-catalog",
        "thresholdsMs": {"initP95": INIT_P95_MS, "prepareP95": PREPARE_P95_MS},
        "initMs": {
            "samples": init_samples,
            "p95": init_p95,
            "min": init_sorted[0],
            "max": init_sorted[-1],
        },
        "prepareMs": {
            "samples": prepare_samples,
            "p95": prepare_p95,
            "min": prepare_sorted[0],
            "max": prepare_sorted[-1],
        },
        "platform": {
            "os": f"{platform.system()} {platform.release()}",
            "cpu": platform.processor() or platform.machine(),
        },
        "cache": {
            "policy": "one-warmup-then-n-samples-same-binary",
            "binary": str(binary),
        },
        "result": "pending",
    }

    init_ok = init_p95 <= INIT_P95_MS
    prepare_ok = prepare_p95 <= PREPARE_P95_MS
    n_ok = args.samples >= DEFAULT_SAMPLES
    if init_ok and prepare_ok and n_ok:
        report["result"] = "passed"
        report["technicalResult"] = "passed"
    else:
        report["result"] = "failed" if not args.allow_pending else "pending"
        report["technicalResult"] = "failed" if not (init_ok and prepare_ok and n_ok) else "passed"
        report["residuals"] = []
        if not n_ok:
            report["residuals"].append(f"n={args.samples} < {DEFAULT_SAMPLES}")
        if not init_ok:
            report["residuals"].append(f"init P95 {init_p95:.3f} ms > {INIT_P95_MS}")
        if not prepare_ok:
            report["residuals"].append(
                f"prepare P95 {prepare_p95:.3f} ms > {PREPARE_P95_MS}"
            )

    out_dir = args.out_dir or Path("/tmp/archmaker-f7-p95")
    out_dir.mkdir(parents=True, exist_ok=True)
    out_path = out_dir / "p95-report.json"
    out_path.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({"wrote": str(out_path), "result": report["result"], "initP95": init_p95, "prepareP95": prepare_p95}, ensure_ascii=False))

    if args.allow_pending:
        return 0
    if report.get("technicalResult") != "passed":
        fail("P95 no cumple umbrales o n insuficiente; no se declaran cifras aprobadas")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
