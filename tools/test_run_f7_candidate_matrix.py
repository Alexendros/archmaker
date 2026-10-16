#!/usr/bin/env python3
"""Tests simulados del runner F7 (sin Playwright ni cargo reales)."""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path
from unittest import mock

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))

import run_f7_candidate_matrix as f7  # noqa: E402


def _cmd_ok(cmd: str, env=None, report_dir=None) -> dict:
    return {
        "command": cmd,
        "commandRun": cmd,
        "exitCode": 0,
        "status": "passed",
        "notes": [],
        "tests": {
            "tests": [{"title": "t1", "status": "passed", "retries": 0}],
            "testCount": 1,
            "passed": 1,
            "failed": 0,
            "skipped": 0,
            "retries": 0,
        },
    }


def _cmd_zero_tests(cmd: str, env=None, report_dir=None) -> dict:
    out = _cmd_ok(cmd, env, report_dir)
    out["tests"] = {
        "tests": [],
        "testCount": 0,
        "passed": 0,
        "failed": 0,
        "skipped": 0,
        "retries": 0,
    }
    out["status"] = "failed"
    out["notes"] = ["cero tests descubiertos"]
    return out


def _cmd_retries(cmd: str, env=None, report_dir=None) -> dict:
    out = _cmd_ok(cmd, env, report_dir)
    out["tests"]["retries"] = 2
    out["tests"]["tests"][0]["retries"] = 2
    out["status"] = "failed"
    out["notes"] = ["retries=2 (debe ser 0)"]
    out["exitCode"] = 0
    return out


class WasmPkgReadyTests(unittest.TestCase):
    def test_requires_all_three_files(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            pkg = Path(tmp) / "pkg"
            pkg.mkdir()
            (pkg / "archmaker_wasm.js").write_text("x", encoding="utf-8")
            with mock.patch.object(f7, "ROOT", Path(tmp).parent):
                # Point wasm path via monkeypatch of function body paths
                pass
        # Direct unit of the helper with patched Path
        with mock.patch.object(f7, "ROOT", Path(tmpfile := tempfile.mkdtemp())):
            pkg = Path(tmpfile) / "crates/archmaker-wasm/pkg"
            pkg.mkdir(parents=True)
            self.assertFalse(f7.wasm_pkg_ready())
            (pkg / "archmaker_wasm.js").write_text("a", encoding="utf-8")
            self.assertFalse(f7.wasm_pkg_ready())
            (pkg / "archmaker_wasm_bg.wasm").write_bytes(b"\0")
            self.assertFalse(f7.wasm_pkg_ready())
            (pkg / "archmaker_wasm.d.ts").write_text("export {}", encoding="utf-8")
            self.assertTrue(f7.wasm_pkg_ready())


class InventoryTests(unittest.TestCase):
    def test_rejects_duplicate_ids(self) -> None:
        data = json.loads(f7.INVENTORY.read_text(encoding="utf-8"))
        data["cases"] = data["cases"] + [data["cases"][0]]
        errors = f7.validate_inventory(data)
        self.assertTrue(any("duplicados" in e for e in errors))

    def test_inventory_and_example_ok(self) -> None:
        data = json.loads(f7.INVENTORY.read_text(encoding="utf-8"))
        self.assertEqual(f7.validate_inventory(data), [])
        self.assertEqual(f7.validate_example(), [])


class ExecuteMatrixTests(unittest.TestCase):
    def test_partial_when_wasm_commands_skipped(self) -> None:
        # Inventario real: casos WASM quedan partial/pending sin e2e.
        metrics = f7.execute_matrix(with_e2e=False, strict=True, run_command=_cmd_ok)
        wasm_cases = [
            c
            for c in metrics["cases"]
            if c["id"]
            in {
                "F7-MOTOR-REAL-WASM",
                "F7-GEN-EDIT-EXPORT-WASM",
                "F7-OFFLINE-WEB",
                "F7-E2E-X3",
            }
        ]
        self.assertTrue(wasm_cases)
        for case in wasm_cases:
            self.assertIn(case["status"], {"partial", "pending"})
        self.assertEqual(metrics["technicalResult"], "partial")
        self.assertNotEqual(metrics["startedAt"], None)
        self.assertNotEqual(metrics["completedAt"], None)

    def test_timestamps_differ_or_documented(self) -> None:
        metrics = f7.execute_matrix(with_e2e=False, strict=False, run_command=_cmd_ok)
        self.assertIn("startedAt", metrics)
        self.assertIn("completedAt", metrics)
        # Misma resolución de segundo es posible; el contrato exige campos reales.
        self.assertRegex(metrics["startedAt"], r"^\d{4}-\d{2}-\d{2}T")
        self.assertRegex(metrics["completedAt"], r"^\d{4}-\d{2}-\d{2}T")

    def test_zero_tests_marks_failed(self) -> None:
        metrics = f7.execute_matrix(
            with_e2e=False, strict=True, run_command=_cmd_zero_tests
        )
        self.assertGreater(metrics["failed"], 0)
        self.assertEqual(metrics["technicalResult"], "failed")

    def test_retries_in_report_fail_command(self) -> None:
        metrics = f7.execute_matrix(with_e2e=False, strict=True, run_command=_cmd_retries)
        self.assertGreater(metrics["failed"], 0)
        self.assertGreater(metrics["retries"], 0)

    def test_desktop_offline_stays_pending(self) -> None:
        metrics = f7.execute_matrix(with_e2e=False, strict=False, run_command=_cmd_ok)
        desktop = next(c for c in metrics["cases"] if c["id"] == "F7-OFFLINE-DESKTOP")
        self.assertEqual(desktop["status"], "pending")

    def test_strict_gate_rejects_partial(self) -> None:
        metrics = f7.execute_matrix(with_e2e=False, strict=True, run_command=_cmd_ok)
        errors = f7.strict_gate(metrics)
        self.assertTrue(errors)

    def test_manifest_keeps_result_pending_and_validates_schema(self) -> None:
        metrics = f7.execute_matrix(with_e2e=False, strict=False, run_command=_cmd_ok)
        # Forzar timestamps distintos en el manifiesto
        metrics["startedAt"] = "2026-10-16T20:00:00Z"
        metrics["completedAt"] = "2026-10-16T20:05:00Z"
        manifest = f7.build_pending_manifest(
            metrics,
            "c90919c12416bab53312f4f7f117c99c05f36dba",
            harness_sha="aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        self.assertEqual(manifest["result"], "pending")
        self.assertIsNone(manifest["reviewer"])
        self.assertEqual(manifest["technicalResult"], metrics["technicalResult"])
        self.assertEqual(
            manifest["harness"]["sourceSha"],
            "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        )
        self.assertEqual(
            manifest["harness"]["productSourceSha"],
            "c90919c12416bab53312f4f7f117c99c05f36dba",
        )
        self.assertEqual(manifest["approval"]["status"], "pending")
        schema_errors = f7.validate_manifest_against_schema(manifest)
        self.assertEqual(schema_errors, [], msg=schema_errors)

    def test_main_validate_only_ok(self) -> None:
        self.assertEqual(f7.main(["--validate-only"]), 0)

    def test_main_strict_fails_without_full_matrix(self) -> None:
        fake_metrics = {
            "cases": [{"id": "F7-OFFLINE-DESKTOP", "status": "pending", "commands": [], "note": "x"}],
            "executed": 0,
            "passed": 0,
            "failed": 0,
            "partial": 0,
            "skipped": 1,
            "retries": 0,
            "testCount": 0,
            "wasmPkgReady": False,
            "wasmPkgFiles": {},
            "withE2e": False,
            "strict": True,
            "technicalResult": "partial",
            "startedAt": "2026-10-16T20:00:00Z",
            "completedAt": "2026-10-16T20:01:00Z",
            "reportRoot": "/tmp",
        }
        with tempfile.TemporaryDirectory() as tmp:
            with mock.patch.object(f7, "execute_matrix", return_value=fake_metrics):
                code = f7.main(["--execute", "--strict", "--out-dir", tmp])
            self.assertEqual(code, 1)
            manifest = json.loads((Path(tmp) / "manifest.json").read_text(encoding="utf-8"))
            self.assertEqual(manifest["result"], "pending")
            self.assertEqual(manifest["technicalResult"], "partial")


class ParseHelpersTests(unittest.TestCase):
    def test_parse_cargo_summary(self) -> None:
        out = "running 2 tests\ntest result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out\n"
        summary = f7.parse_cargo_summary(out)
        self.assertEqual(summary["testCount"], 2)
        self.assertEqual(summary["passed"], 2)

    def test_parse_cargo_summary_missing(self) -> None:
        summary = f7.parse_cargo_summary("no tests ran")
        self.assertEqual(summary["testCount"], 0)
        self.assertIn("parseError", summary)

    def test_parse_playwright_report(self) -> None:
        report = {
            "suites": [
                {
                    "specs": [
                        {
                            "title": "motor real",
                            "tests": [
                                {
                                    "status": "expected",
                                    "results": [{"status": "passed"}],
                                }
                            ],
                        }
                    ],
                    "suites": [],
                }
            ]
        }
        summary = f7.parse_playwright_report(report)
        self.assertEqual(summary["testCount"], 1)
        self.assertEqual(summary["retries"], 0)


if __name__ == "__main__":
    unittest.main()
