#!/usr/bin/env python3
"""Prove-It: discordancia de pin jsonschema no debe tumbar si el comportamiento coincide."""

from __future__ import annotations

import unittest
from pathlib import Path
from unittest import mock

ROOT = Path(__file__).resolve().parents[1]
import sys

sys.path.insert(0, str(ROOT / "tools"))
import validate_closure_evidence as vce  # noqa: E402


class JsonschemaPinBehaviour(unittest.TestCase):
    def test_version_mismatch_warns_but_compares_behaviour(self) -> None:
        schema = {
            "type": "object",
            "additionalProperties": False,
            "required": ["result"],
            "properties": {
                "result": {
                    "type": ["string", "null"],
                    "enum": ["pending", "passed", None],
                }
            },
        }
        instances = [
            ("ok-pending", {"result": "pending"}, True),
            ("bad-extra", {"result": "pending", "x": 1}, False),
        ]
        with mock.patch(
            "importlib.metadata.version", return_value="4.19.2"
        ):
            mismatches, warnings = vce.compare_with_jsonschema(schema, instances)
        self.assertEqual(mismatches, [])
        self.assertTrue(any("4.19.2" in w for w in warnings))

    def test_missing_jsonschema_is_error(self) -> None:
        import sys

        kept = {k: sys.modules[k] for k in list(sys.modules) if k.startswith("jsonschema")}
        for k in list(kept):
            del sys.modules[k]
        try:
            with mock.patch.dict(sys.modules, {"jsonschema": None}):
                mismatches, warnings = vce.compare_with_jsonschema(
                    {"type": "object"}, [("x", {}, True)]
                )
        finally:
            sys.modules.update(kept)
        self.assertTrue(mismatches)
        self.assertIn("falta jsonschema", mismatches[0])
        self.assertEqual(warnings, [])


if __name__ == "__main__":
    unittest.main()
