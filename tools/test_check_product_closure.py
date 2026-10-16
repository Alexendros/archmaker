#!/usr/bin/env python3
"""Tests del verificador de solo lectura check_product_closure."""

from __future__ import annotations

import json
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))

import check_product_closure as cpc  # noqa: E402


class CheckProductClosureTests(unittest.TestCase):
    def test_main_diagnostic_exit_zero(self) -> None:
        self.assertEqual(cpc.main([]), 0)

    def test_main_never_auto_approves(self) -> None:
        code = cpc.main(["--json"])
        self.assertEqual(code, 0)

    def test_artifact_other_sha_diagnosed(self) -> None:
        manifest = {
            "schemaVersion": "1",
            "repository": "x",
            "phase": "F0",
            "sourceSha": "a" * 40,
            "baseSha": None,
            "eventSha": None,
            "prHeadSha": None,
            "workflowSha": None,
            "mergeSha": None,
            "workflowRunId": None,
            "runAttempt": None,
            "eventName": None,
            "startedAt": None,
            "completedAt": None,
            "toolchain": None,
            "platform": None,
            "dirtyTrackedFiles": [],
            "fixtureSetDigest": "0" * 64,
            "requiredTestIds": ["A"],
            "executed": 1,
            "passed": 1,
            "failed": 0,
            "skipped": 0,
            "retries": 0,
            "artifacts": [
                {
                    "name": "x",
                    "sizeBytes": 1,
                    "sha256": "b" * 64,
                    "mediaType": "application/octet-stream",
                    "buildSourceSha": "c" * 40,
                }
            ],
            "result": "pending",
            "reviewer": None,
            "reviewedAt": None,
            "residuals": [],
            "technicalResult": "partial",
        }
        report = cpc.evaluate_manifest(manifest, inventory_ids=["A"])
        codes = {d["code"] for d in report["diagnostics"]}
        self.assertIn("ARTIFACT_SOURCE_MISMATCH", codes)
        self.assertFalse(report["autoApproved"])
        self.assertEqual(report["approvalStatus"], "pending")

    def test_approval_other_candidate(self) -> None:
        manifest = {
            "schemaVersion": "1",
            "repository": None,
            "phase": None,
            "sourceSha": "a" * 40,
            "baseSha": None,
            "eventSha": None,
            "prHeadSha": None,
            "workflowSha": None,
            "mergeSha": None,
            "workflowRunId": None,
            "runAttempt": None,
            "eventName": None,
            "startedAt": None,
            "completedAt": None,
            "toolchain": None,
            "platform": None,
            "dirtyTrackedFiles": None,
            "fixtureSetDigest": None,
            "requiredTestIds": ["A"],
            "executed": 1,
            "passed": 1,
            "failed": 0,
            "skipped": 0,
            "retries": 0,
            "artifacts": [],
            "result": "pending",
            "reviewer": None,
            "reviewedAt": None,
            "residuals": [],
            "technicalResult": "passed",
        }
        report = cpc.evaluate_manifest(
            manifest,
            inventory_ids=["A"],
            approval_record={"productSourceSha": "b" * 40},
        )
        codes = {d["code"] for d in report["diagnostics"]}
        self.assertIn("APPROVAL_CANDIDATE_MISMATCH", codes)

    def test_strict_without_manifest_fails(self) -> None:
        self.assertEqual(cpc.main(["--strict"]), 1)

    def test_strict_with_incomplete_manifest_fails(self) -> None:
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "m.json"
            path.write_text(
                json.dumps({"schemaVersion": "1", "result": "pending"}),
                encoding="utf-8",
            )
            self.assertEqual(cpc.main(["--manifest", str(path), "--strict"]), 1)


if __name__ == "__main__":
    unittest.main()
