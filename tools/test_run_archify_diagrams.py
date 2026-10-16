#!/usr/bin/env python3
"""Pruebas negativas y de contrato del pipeline Archify (P4)."""

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

import run_archify_diagrams as archify  # noqa: E402


class ArchifyPipelineNegatives(unittest.TestCase):
    def test_archify_ok_exit_nonzero_with_ok_true(self) -> None:
        bad = subprocess.CompletedProcess(
            args=["node", "archify"],
            returncode=1,
            stdout='{"ok": true, "ignored": true}\n',
            stderr="",
        )
        self.assertFalse(archify.archify_ok(bad))

    def test_archify_ok_exit0_without_receipt(self) -> None:
        empty = subprocess.CompletedProcess(
            args=["node", "archify"], returncode=0, stdout="plain text\n", stderr=""
        )
        self.assertFalse(archify.archify_ok(empty))

    def test_archify_ok_happy(self) -> None:
        ok = subprocess.CompletedProcess(
            args=["node", "archify"],
            returncode=0,
            stdout='{"ok": true}\n',
            stderr="",
        )
        self.assertTrue(archify.archify_ok(ok))

    def test_reject_path_traversal(self) -> None:
        with self.assertRaises(SystemExit):
            archify.reject_traversal("../etc/passwd", label="test")
        with self.assertRaises(SystemExit):
            archify.reject_traversal("/etc/passwd", label="test")

    def test_safe_under_blocks_escape(self) -> None:
        with self.assertRaises(SystemExit):
            archify.safe_under(archify.DIAG, "../Cargo.toml", label="test")

    def test_verify_catalog_rejects_duplicate_ids(self) -> None:
        catalog = {
            "requiredIds": list(archify.REQUIRED_IDS),
            "entries": [
                {"id": "D01", "source": "sources/x.json", "codeRefs": ["Cargo.toml"]},
                {"id": "D01", "source": "sources/y.json", "codeRefs": ["Cargo.toml"]},
            ]
            + [
                {
                    "id": f"D{i:02d}",
                    "source": f"sources/D{i:02d}.json",
                    "codeRefs": ["Cargo.toml"],
                }
                for i in range(2, 25)
            ],
        }
        # 25 entries with duplicate D01 — length wrong or duplicate
        with self.assertRaises(SystemExit):
            archify.verify_catalog_ids(catalog)

    def test_verify_catalog_rejects_d25(self) -> None:
        entries = [
            {
                "id": f"D{i:02d}",
                "source": f"sources/D{i:02d}.json",
                "codeRefs": ["Cargo.toml"],
            }
            for i in range(1, 25)
        ]
        entries[0] = {
            "id": "D25",
            "source": "sources/D25.json",
            "codeRefs": ["Cargo.toml"],
        }
        catalog = {"requiredIds": list(archify.REQUIRED_IDS), "entries": entries}
        with self.assertRaises(SystemExit):
            archify.verify_catalog_ids(catalog)

    def test_verify_code_refs_empty(self) -> None:
        with self.assertRaises(SystemExit):
            archify.verify_code_refs({"id": "D01", "codeRefs": []})

    def test_verify_code_refs_missing_file(self) -> None:
        with self.assertRaises(SystemExit):
            archify.verify_code_refs(
                {"id": "D01", "codeRefs": ["does/not/exist-anywhere.rs"]}
            )

    def test_verify_code_refs_ok(self) -> None:
        archify.verify_code_refs({"id": "D01", "codeRefs": ["Cargo.toml"]})

    def test_renderer_mismatch_fails_main(self) -> None:
        # Carga real del catálogo; fuerza renderer distinto.
        code = archify.main(["--renderer-sha", "0" * 40, "--check", "--skip-deliver"])
        self.assertEqual(code, 1)

    def test_check_does_not_mutate_tree(self) -> None:
        before = {
            p.relative_to(ROOT).as_posix(): archify.sha256_file(p)
            for p in archify.DIAG.rglob("*")
            if p.is_file()
        }
        # Sin Archify real: stub doctor + validate ok y skip deliver.
        fake_cli = Path(tempfile.mkdtemp()) / "archify.mjs"
        fake_cli.write_text("// stub\n", encoding="utf-8")
        checkout = fake_cli.parent
        # resolve_archify + verify_renderer_checkout necesitan git pin; mockear.
        with mock.patch.object(archify, "resolve_archify", return_value=fake_cli):
            with mock.patch.object(archify, "verify_renderer_checkout"):
                with mock.patch.object(
                    archify,
                    "run_node",
                    side_effect=lambda cli, args: subprocess.CompletedProcess(
                        args=["node", *args],
                        returncode=0,
                        stdout='{"ok": true}\n',
                        stderr="",
                    ),
                ):
                    code = archify.main(["--check", "--skip-deliver"])
        after = {
            p.relative_to(ROOT).as_posix(): archify.sha256_file(p)
            for p in archify.DIAG.rglob("*")
            if p.is_file()
        }
        self.assertEqual(before, after)
        self.assertEqual(code, 0)

    def test_altered_as_built_hash_fails(self) -> None:
        manifest = json.loads(archify.AS_BUILT_MANIFEST.read_text(encoding="utf-8"))
        catalog = json.loads(archify.CATALOG.read_text(encoding="utf-8"))
        # Clonar manifiesto con hash falso.
        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            bad = dict(manifest)
            paths = dict(bad["paths"])
            first = next(iter(paths))
            paths[first] = "0" * 64
            bad["paths"] = paths
            # Parchear ruta del módulo temporalmente.
            original = archify.AS_BUILT_MANIFEST
            try:
                patched = tmp_path / "as-built-manifest.json"
                patched.write_text(json.dumps(bad), encoding="utf-8")
                archify.AS_BUILT_MANIFEST = patched
                with self.assertRaises(SystemExit):
                    archify.verify_as_built(catalog)
            finally:
                archify.AS_BUILT_MANIFEST = original

    def test_output_dir_must_be_outside_tree(self) -> None:
        with self.assertRaises(SystemExit):
            archify.ensure_outside_tree(ROOT / "docs", label="test")


if __name__ == "__main__":
    unittest.main()
