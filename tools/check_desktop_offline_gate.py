#!/usr/bin/env python3
"""P6: comprueba que el gate desktop/offline es blocked explícito o passed con evidencia.

- Rechaza MockCorePort como cierre.
- Rechaza AppImage histórico v0.1.0 (nombre histórico v0.1.0) como cierre del tip.
- Exige evidencia versionada status=blocked|passed alineada al SOURCE_SHA o HEAD.
"""

from __future__ import annotations

import json
import subprocess
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EVIDENCE = ROOT / "docs/10-delivery/closure/examples/desktop-evidence.p6-blocked.json"
DOC = ROOT / "docs/10-delivery/closure/p6-desktop-offline.md"
SCHEMA = ROOT / "docs/10-delivery/closure/desktop-evidence.schema.json"


def fail(msg: str) -> None:
    print(f"FAIL {msg}", file=sys.stderr)


def git_head() -> str:
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
    ).strip()


def main() -> int:
    errors = 0
    if not DOC.is_file():
        fail(f"falta {DOC.relative_to(ROOT)}")
        errors += 1
    else:
        text = DOC.read_text(encoding="utf-8")
        if "**`blocked`**" not in text and "`blocked`" not in text:
            fail("p6-desktop-offline.md debe declarar blocked o documentar passed")
            errors += 1
        if "MockCorePort" not in text:
            fail("debe prohibir MockCorePort como cierre")
            errors += 1
        if "v0.1.0" not in text:
            fail("debe excluir el AppImage histórico v0.1.0")
            errors += 1

    if not EVIDENCE.is_file():
        fail(f"falta evidencia {EVIDENCE.relative_to(ROOT)}")
        return 1

    data = json.loads(EVIDENCE.read_text(encoding="utf-8"))
    status = data.get("status")
    if status not in {"blocked", "passed", "failed"}:
        fail(f"status inválido: {status!r}")
        errors += 1
    if data.get("caseId") != "F7-OFFLINE-DESKTOP":
        fail("caseId debe ser F7-OFFLINE-DESKTOP")
        errors += 1
    if data.get("schemaVersion") != "1":
        fail("schemaVersion debe ser 1")
        errors += 1

    source = data.get("sourceSha")
    head = git_head()
    if not isinstance(source, str) or len(source) != 40:
        fail("sourceSha inválido")
        errors += 1
    elif source != head:
        if status == "passed":
            fail(
                f"passed exige sourceSha==HEAD ({head[:12]}); evidencia={str(source)[:12]}"
            )
            errors += 1
        else:
            # blocked/failed: el probe puede quedar detrás tras merges; no autoaprueba.
            print(
                f"WARN evidence sourceSha {str(source)[:12]} ≠ HEAD {head[:12]} "
                f"(ok si status={status})"
            )

    notes = data.get("notes") or []
    joined = " ".join(notes).lower()
    if status == "blocked":
        for needle in ("ausente", "appimage", "mock"):
            if needle not in joined and needle not in DOC.read_text(encoding="utf-8").lower():
                fail(f"blocked sin mención de {needle!r} en notes/doc")
                errors += 1
        # No puede afirmar passed
        if "cierra este gate como passed" in joined:
            fail("blocked no puede afirmar cierre passed")
            errors += 1

    if status == "passed":
        # passed exige artifact real no vacío y notes sin mock
        art = data.get("artifact") or {}
        if int(art.get("sizeBytes") or 0) <= 0:
            fail("passed con artifact vacío")
            errors += 1
        if "mock" in joined:
            fail("passed no puede mencionar mock como evidencia")
            errors += 1

    if not SCHEMA.is_file():
        fail(f"falta schema {SCHEMA.relative_to(ROOT)}")
        errors += 1

    if errors:
        print(f"check_desktop_offline_gate: {errors} fallos")
        return 1
    print(
        f"OK check_desktop_offline_gate: status={status} "
        f"sourceSha={str(source)[:12]} (read-only; no autoaprueba)"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
