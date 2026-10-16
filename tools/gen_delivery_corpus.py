#!/usr/bin/env python3
"""Genera el corpus de entrega P5: ≥20 fixtures por target + 4 rechazos.

Uso:
  python3 tools/gen_delivery_corpus.py
"""

from __future__ import annotations

import hashlib
import json
import sys
from collections import Counter
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DELIVERY = ROOT / "crates/archmaker-core/tests/fixtures/closure/delivery"
MIN_PER_TARGET = 20
TARGETS = ("archmaker-profile", "archinstall-profile", "report")


def main() -> int:
    DELIVERY.mkdir(parents=True, exist_ok=True)
    for stale in DELIVERY.iterdir():
        if stale.name == "fixture-digests.json":
            continue
        if stale.is_file():
            stale.unlink()

    files: dict[str, dict] = {}

    for i in range(MIN_PER_TARGET):
        variants = [
            (
                "archmaker-profile",
                "json",
                json.dumps(
                    {
                        "adapter": "archmaker-profile",
                        "n": i,
                        "note": "delivery-corpus",
                    },
                    separators=(",", ":"),
                ).encode()
                + b"\n",
            ),
            (
                "archinstall-profile",
                "json",
                json.dumps(
                    {
                        "adapter": "archinstall-profile",
                        "n": i,
                        "note": "delivery-corpus",
                    },
                    separators=(",", ":"),
                ).encode()
                + b"\n",
            ),
            (
                "report",
                "md",
                f"# ArchMaker report\n\nfixture {i:02d}\n".encode(),
            ),
        ]
        for target, ext, data in variants:
            name = f"{target}-{i:02d}.{ext}"
            (DELIVERY / name).write_bytes(data)
            files[name] = {
                "sha256": hashlib.sha256(data).hexdigest(),
                "sizeBytes": len(data),
                "class": "delivery",
                "target": target,
            }

    rejects = [
        (
            "reject-empty.meta.json",
            {
                "binaryDigest": {
                    "algorithm": "sha256",
                    "value": "a" * 64,
                },
                "mediaType": "application/json",
                "reason": "bytes-missing",
            },
        ),
        (
            "reject-digest-mismatch.meta.json",
            {
                "binaryDigest": {
                    "algorithm": "sha256",
                    "value": "b" * 64,
                },
                "mediaType": "application/json",
                "reason": "digest-mismatch",
                "claimedBytes": "not-matching",
            },
        ),
        (
            "reject-bad-target.meta.json",
            {
                "targetRef": {"id": "iso", "version": "0.1.0"},
                "mediaType": "application/octet-stream",
                "reason": "unsupported-target",
            },
        ),
        (
            "reject-truncated-json.meta.json",
            {
                "binaryDigest": {
                    "algorithm": "sha256",
                    "value": "c" * 64,
                },
                "mediaType": "application/json",
                "content": {"incomplete": True},
                "reason": "truncated-profile",
            },
        ),
    ]
    for name, obj in rejects:
        data = (json.dumps(obj, indent=2, ensure_ascii=False) + "\n").encode()
        (DELIVERY / name).write_bytes(data)
        files[name] = {
            "sha256": hashlib.sha256(data).hexdigest(),
            "sizeBytes": len(data),
            "class": "negative-bytes",
        }

    index = {
        "algorithm": "sha256",
        "schemaVersion": 1,
        "minFixturesPerTarget": MIN_PER_TARGET,
        "supportedTargets": list(TARGETS),
        "files": files,
        "note": "P5 delivery corpus: ≥20 fixtures/target + 4 rejects",
    }
    (DELIVERY / "fixture-digests.json").write_text(
        json.dumps(index, indent=2, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )

    counts = Counter(v["target"] for v in files.values() if v.get("target"))
    neg = sum(1 for v in files.values() if v["class"] == "negative-bytes")
    for target in TARGETS:
        if counts[target] < MIN_PER_TARGET:
            print(f"FAIL: {target} tiene {counts[target]} < {MIN_PER_TARGET}", file=sys.stderr)
            return 1
    if neg < 4:
        print(f"FAIL: rejects={neg} < 4", file=sys.stderr)
        return 1
    print(f"OK delivery corpus: {dict(counts)} rejects={neg} total={len(files)}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
