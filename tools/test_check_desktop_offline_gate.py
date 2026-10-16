#!/usr/bin/env python3
"""Tests del gate P6 desktop/offline."""

from __future__ import annotations

import json
import subprocess
import tempfile
import unittest
from pathlib import Path
from unittest import mock

ROOT = Path(__file__).resolve().parents[1]
import sys

sys.path.insert(0, str(ROOT / "tools"))
import check_desktop_offline_gate as gate  # noqa: E402


class DesktopOfflineGate(unittest.TestCase):
    def test_current_tree_is_blocked_ok(self) -> None:
        self.assertEqual(gate.main(), 0)

    def test_rejects_passed_with_mock_note(self) -> None:
        data = json.loads(gate.EVIDENCE.read_text(encoding="utf-8"))
        data["status"] = "passed"
        data["notes"] = ["cerrado con MockCorePort"]
        data["artifact"]["sizeBytes"] = 10
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "ev.json"
            path.write_text(json.dumps(data), encoding="utf-8")
            with mock.patch.object(gate, "EVIDENCE", path):
                with mock.patch.object(gate, "git_head", return_value=data["sourceSha"]):
                    self.assertEqual(gate.main(), 1)


if __name__ == "__main__":
    unittest.main()
