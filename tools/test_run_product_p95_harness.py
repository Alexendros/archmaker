#!/usr/bin/env python3
"""Tests unitarios del harness P95 de producto (sin Playwright)."""

from __future__ import annotations

import math
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "tools"))

import run_product_p95_harness as h  # noqa: E402


class PercentileTests(unittest.TestCase):
    def test_nearest_rank_p95_n30(self) -> None:
        values = list(range(1, 31))  # 1..30
        # ceil(0.95*30)=29 → índice 28 → 29
        self.assertEqual(h.percentile(values, 95), 29)


class MetricBlockTests(unittest.TestCase):
    def test_invalid_counts_in_denominator(self) -> None:
        samples: list[float | None] = [10.0] * 29 + [None]
        block = h.metric_block(samples, threshold=1000.0)
        self.assertEqual(block["n"], 30)
        self.assertEqual(block["invalidCount"], 1)
        # Nearest-rank P95 puede seguir finito; la inválida falla meetsThreshold.
        self.assertFalse(block["meetsThreshold"])

    def test_enough_invalids_make_p95_infinite(self) -> None:
        # Índice P95 (0-based) = 28 → hace falta ≥2 inválidas al final ordenado.
        samples: list[float | None] = [10.0] * 28 + [None, None]
        block = h.metric_block(samples, threshold=1000.0)
        self.assertEqual(block["invalidCount"], 2)
        self.assertTrue(block["p95IsInfinite"])
        self.assertFalse(block["meetsThreshold"])

    def test_all_valid_meets_threshold(self) -> None:
        samples: list[float | None] = [100.0] * 30
        block = h.metric_block(samples, threshold=1000.0)
        self.assertEqual(block["p95"], 100.0)
        self.assertTrue(block["meetsThreshold"])


class ReportAcceptanceTests(unittest.TestCase):
    def test_allow_pending_never_closes_acceptance(self) -> None:
        ready = [50.0] * 30
        prepare = [20.0] * 30
        report = h.build_report(
            samples_n=30,
            ready_samples=ready,
            prepare_samples=prepare,
            sample_errors=[None] * 30,
            build_profile={"mode": "test"},
            desktop={"result": "passed", "missingEvidence": []},
            started="2026-01-01T00:00:00Z",
            base_url="http://127.0.0.1:4173/",
            allow_pending=True,
        )
        self.assertEqual(report["result"], "pending")
        self.assertFalse(report["acceptanceClosed"])
        self.assertTrue(report["allowPending"])

    def test_desktop_blocked_blocks_global_even_if_web_ok(self) -> None:
        ready = [50.0] * 30
        prepare = [20.0] * 30
        desktop = h.assess_desktop()
        # Forzar blocked aunque el entorno tenga DISPLAY.
        desktop = {
            "result": "blocked",
            "missingEvidence": ["desktop-ready-p95-samples"],
            "reason": "test",
        }
        report = h.build_report(
            samples_n=30,
            ready_samples=ready,
            prepare_samples=prepare,
            sample_errors=[None] * 30,
            build_profile={"mode": "test"},
            desktop=desktop,
            started="2026-01-01T00:00:00Z",
            base_url="http://127.0.0.1:4173/",
            allow_pending=False,
        )
        self.assertEqual(report["web"]["result"], "passed")
        self.assertEqual(report["result"], "blocked")
        self.assertFalse(report["acceptanceClosed"])
        self.assertEqual(report["technicalResult"], "partial")

    def test_microbenchmark_labeled_not_certifying(self) -> None:
        report = h.build_report(
            samples_n=30,
            ready_samples=[1.0] * 30,
            prepare_samples=[1.0] * 30,
            sample_errors=[None] * 30,
            build_profile={},
            desktop={"result": "blocked", "missingEvidence": ["x"]},
            started="t",
            base_url="u",
            allow_pending=False,
        )
        self.assertEqual(report["microbenchmark"]["kind"], "microbenchmark")
        self.assertFalse(report["microbenchmark"]["certifiesProduct"])


class DesktopAssessTests(unittest.TestCase):
    def test_assess_desktop_names_missing_evidence(self) -> None:
        desktop = h.assess_desktop()
        self.assertEqual(desktop["result"], "blocked")
        self.assertIn("desktop-ready-p95-samples", desktop["missingEvidence"])
        self.assertIn("desktop-prepare-p95-samples", desktop["missingEvidence"])


if __name__ == "__main__":
    unittest.main()
