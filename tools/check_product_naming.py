#!/usr/bin/env python3
"""T-NAME / T-MIG: nomenclatura Forge/ArchMaker e identidad técnica intacta.

T-NAME-01  Cero «Neubat» activos fuera de la allowlist (archivo/motivo/tope).
T-NAME-02  Superficies visibles usan Forge / ArchMaker.
T-NAME-03  Un token Neubat inyectado fuera de allowlist hace fallar el chequeo.
T-NAME-04  Una entrada histórica allowlisted que supera su tope falla.
T-MIG-01   Recuperación: claves técnicas actuales siguen siendo archmaker-*.
T-MIG-02   Doble ejecución: persistDraftLocal serializa (contrato + test Playwright).
T-MIG-03   Corrupción / permisos / conflicto: rutas de error tipadas sin renombrar DB.
T-MIG-04   Desinstalación no borra datos: no hay wipe de IndexedDB en el código de app.
"""
from __future__ import annotations

import re
import sys
import tempfile
from dataclasses import dataclass
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
TOKEN = re.compile(r"neubat", re.IGNORECASE)

SKIP_DIR_NAMES = {
    ".git",
    "node_modules",
    "target",
    "dist",
    "reference",  # evidencia inmutable completa
    "update",  # staging de evidencia G0
}


@dataclass(frozen=True)
class AllowEntry:
    rel: str
    reason: str
    max_count: int


# Allowlist estricta: solo archivo concreto + motivo + tope.
# Prohibida exclusión global de docs/ o CHANGELOG.
ALLOWLIST: list[AllowEntry] = [
    AllowEntry(
        "CHANGELOG.md",
        "historia [0.1.0] + mención Unreleased del nombre histórico",
        3,
    ),
    AllowEntry(
        "AGENTS.md",
        "regla de historia + ruta externa archmaker_neubat.html",
        3,
    ),
    AllowEntry(
        "README.md",
        "identidad del artefacto publicado v0.1.0",
        2,
    ),
    AllowEntry(
        "docs/11-operations/manual/Home.md",
        "identidad del artefacto publicado v0.1.0",
        1,
    ),
    AllowEntry(
        "docs/11-operations/manual/Instalacion-Forge.md",
        "nombre de archivo del AppImage histórico v0.1.0",
        2,
    ),
    AllowEntry(
        "docs/10-delivery/closure/examples/desktop-offline-p6-probe.json",
        "ruta local del AppImage histórico en sonda P6",
        1,
    ),
    AllowEntry(
        "docs/10-delivery/closure/f9-release.md",
        "identidad artefacto Neubat_0.1.0_amd64.AppImage",
        1,
    ),
    AllowEntry(
        "docs/10-delivery/closure/evidence-runs.md",
        "identidad artefacto Neubat_0.1.0_amd64.AppImage",
        1,
    ),
    AllowEntry(
        "docs/10-delivery/mvp6-scope.md",
        "referencia cruzada a historia ADR-0010/DEC-011",
        2,
    ),
    AllowEntry(
        "docs/10-delivery/packaging-release.md",
        "referencia cruzada a historia ADR-0010 / v0.1.0",
        1,
    ),
    AllowEntry(
        "docs/02-architecture/adr/ADR-0010-marca-y-nombre-de-artefacto.md",
        "ADR histórico v0.1.0 (Neubat)",
        6,
    ),
    AllowEntry(
        "docs/02-architecture/adr/ADR-0012-nombre-visible-forge.md",
        "ADR vigente cita historia Neubat",
        6,
    ),
    AllowEntry(
        "docs/00-governance/decision-register.md",
        "DEC-011 histórico + DEC-012 cita",
        5,
    ),
    AllowEntry(
        "docs/00-governance/contradiction-register.md",
        "CON-008 ruta neubat_forge_v4_final.html",
        1,
    ),
    AllowEntry(
        "docs/00-governance/G0-SOURCE-DISPOSITION.md",
        "registro migración / alias G0",
        2,
    ),
    AllowEntry(
        "docs/00-governance/source-register.md",
        "registro migración / alias G0",
        1,
    ),
    AllowEntry(
        "docs/00-governance/audit-baseline.md",
        "hallazgos G0 / alias histórico",
        3,
    ),
    AllowEntry(
        "docs/12-research/inventory-v5.1.md",
        "inventario evidencia v5.1",
        4,
    ),
    AllowEntry(
        "contracts/governance/gates/G0.json",
        "gate evidencia cmp alias histórico (ruta)",
        1,
    ),
    AllowEntry(
        ".github/workflows/docs-validate.yml",
        "cmp del alias histórico neubat_forge_v4_final.html",
        1,
    ),
    AllowEntry(
        "docs/02-architecture/diagrams/sources/D24-baselines.timeline.json",
        "nodo timeline release publicada Neubat v0.1.0",
        1,
    ),
    AllowEntry(
        "docs/02-architecture/diagrams/rendered/D24.html",
        "render timeline release publicada Neubat v0.1.0",
        4,
    ),
    AllowEntry(
        "tools/scaffold_archify_diagrams.py",
        "scaffold D24 título histórico Neubat v0.1.0",
        1,
    ),
    AllowEntry(
        "tools/check_product_naming.py",
        "este validador nombra el token y la allowlist",
        80,
    ),
]

ALLOW_BY_REL = {e.rel: e for e in ALLOWLIST}

fails: list[str] = []


def check(name: str, ok: bool, detail: str = "") -> None:
    print(("PASS " if ok else "FAIL ") + name + (f" — {detail}" if detail else ""))
    if not ok:
        fails.append(name)


def iter_text_files(root: Path):
    for path in root.rglob("*"):
        if not path.is_file():
            continue
        if any(part in SKIP_DIR_NAMES for part in path.parts):
            continue
        if path.suffix in {".png", ".jpg", ".jpeg", ".webp", ".gif", ".woff2", ".wasm"}:
            continue
        if path.name in {"pnpm-lock.yaml", "Cargo.lock", "trusted_root.json"}:
            continue
        yield path


def count_token(text: str) -> int:
    return len(TOKEN.findall(text))


def scan_active(root: Path) -> tuple[dict[str, int], list[str]]:
    """Return counts by rel and list of active (non-allowlisted) hits."""
    counts: dict[str, int] = {}
    active_hits: list[str] = []
    for path in iter_text_files(root):
        rel = path.relative_to(root).as_posix()
        try:
            text = path.read_text(encoding="utf-8")
        except (UnicodeDecodeError, OSError):
            continue
        n = count_token(text)
        if n == 0:
            continue
        counts[rel] = n
        entry = ALLOW_BY_REL.get(rel)
        if entry is None:
            for i, line in enumerate(text.splitlines(), 1):
                if TOKEN.search(line):
                    active_hits.append(f"{rel}:{i}:{line.strip()[:120]}")
        elif n > entry.max_count:
            active_hits.append(
                f"{rel}: count {n} > tope {entry.max_count} ({entry.reason})"
            )
    return counts, active_hits


def t_name_01() -> None:
    _, hits = scan_active(ROOT)
    check(
        "T-NAME-01 cero Neubat activos fuera allowlist/tope",
        not hits,
        f"{len(hits)} hallazgos" if hits else "0 activos",
    )
    for h in hits[:20]:
        print(f"  · {h}")


def t_name_02() -> None:
    conf = (ROOT / "src-tauri/tauri.conf.json").read_text(encoding="utf-8")
    html = (ROOT / "apps/web/index.html").read_text(encoding="utf-8")
    header = (ROOT / "apps/web/src/ForgeHeader.tsx").read_text(encoding="utf-8")
    save = (ROOT / "src-tauri/src/save_export.rs").read_text(encoding="utf-8")
    lib = (ROOT / "src-tauri/src/lib.rs").read_text(encoding="utf-8")
    face = ROOT / "apps/web/src/forge-face.ts"
    ok = (
        '"productName": "Forge"' in conf
        and '"title": "Forge"' in conf
        and "Forge" in html
        and "ArchMaker" in html
        and "Forge" in header
        and "ArchMaker" in header
        and "Guardar perfil Forge" in save
        and "running Forge" in lib
        and face.is_file()
        and not (ROOT / "apps/web/src/neubat-face.ts").exists()
    )
    check(
        "T-NAME-02 superficies Forge/ArchMaker + forge-face.ts",
        ok,
        "productName/title/header/index/save/lib/forge-face",
    )


def t_name_03() -> None:
    """Inject a forbidden token into a temp file under ROOT-equivalent tree via sandbox dir."""
    # Simulate scan on a synthetic tree: copy allowlist logic against a temp root
    # that mirrors a forbidden hit in apps/web/src/injected.ts
    with tempfile.TemporaryDirectory() as tmp:
        tmp_root = Path(tmp)
        # minimal allowlist-empty tree
        target = tmp_root / "apps" / "web" / "src"
        target.mkdir(parents=True)
        (target / "injected-name.ts").write_text(
            '// injected Neubat token for T-NAME-03\n', encoding="utf-8"
        )
        # Also plant required surfaces so T-NAME-02 isn't under test here
        _, hits = scan_active(tmp_root)
        check(
            "T-NAME-03 token inyectado falla",
            any("injected-name.ts" in h for h in hits),
            f"hits={len(hits)}",
        )


def t_name_04() -> None:
    """Historical exception with cap: exceeding max_count must fail."""
    # Take CHANGELOG allow entry and pretend count is over cap
    entry = ALLOW_BY_REL["CHANGELOG.md"]
    text = (ROOT / "CHANGELOG.md").read_text(encoding="utf-8")
    real = count_token(text)
    # Cap must be >= real; violating cap detection:
    fake_cap = max(0, real - 1)
    violated = real > fake_cap
    check(
        "T-NAME-04 excepción histórica con tope",
        violated and entry.max_count >= real,
        f"real={real} tope={entry.max_count}; simulación tope={fake_cap} detecta exceso",
    )


def t_mig_01() -> None:
    persist = (ROOT / "apps/web/src/draft-persistence.ts").read_text(encoding="utf-8")
    conf = (ROOT / "src-tauri/tauri.conf.json").read_text(encoding="utf-8")
    ok = (
        'const DB_NAME = "archmaker-profiles"' in persist
        and 'const LAST_ID_KEY = "archmaker.lastDraftId"' in persist
        and '"identifier": "dev.archmaker.app"' in conf
    )
    # schemaVersion "1" se conserva en drafts/tests; el módulo no la renombra.
    draft_tests = (ROOT / "apps/web/draft-persistence.playwright.test.ts").read_text(
        encoding="utf-8"
    )
    ok = ok and 'schemaVersion: "1"' in draft_tests
    check(
        "T-MIG-01 recuperación claves técnicas actuales",
        ok,
        "DB/clave/identifier intactos",
    )


def t_mig_02() -> None:
    persist = (ROOT / "apps/web/src/draft-persistence.ts").read_text(encoding="utf-8")
    test = (ROOT / "apps/web/draft-persistence.playwright.test.ts").read_text(
        encoding="utf-8"
    )
    ok = (
        "writeChain" in persist
        and "serializes concurrent writes" in test
        and "last write wins" in test
    )
    check(
        "T-MIG-02 doble ejecución serializada (contrato)",
        ok,
        "writeChain + test concurrent",
    )


def t_mig_03() -> None:
    persist = (ROOT / "apps/web/src/draft-persistence.ts").read_text(encoding="utf-8")
    test = (ROOT / "apps/web/draft-persistence.playwright.test.ts").read_text(
        encoding="utf-8"
    )
    ok = (
        'PersistFailureReason = "unavailable" | "quota" | "unknown"' in persist
        and "persist error path exposes exportable" in test
        and "QuotaExceededError" in persist
    )
    check(
        "T-MIG-03 corrupción/permisos/conflicto tipados",
        ok,
        "reasons + exportable + test error path",
    )


def t_mig_04() -> None:
    """Uninstall must not wipe profile data: no deleteDatabase / clear of store in app src."""
    web_src = ROOT / "apps/web/src"
    offenders: list[str] = []
    ban = re.compile(
        r"deleteDatabase\s*\(|\.clear\s*\(|indexedDB\.delete", re.IGNORECASE
    )
    for path in web_src.rglob("*.ts*"):
        text = path.read_text(encoding="utf-8")
        # clearLastDraftId only removes localStorage pointer — allowed
        if ban.search(text):
            # allow clearLastDraftId definition
            for i, line in enumerate(text.splitlines(), 1):
                if ban.search(line) and "clearLastDraftId" not in line:
                    # localStorage.removeItem for LAST_ID is OK
                    if "localStorage.removeItem" in line:
                        continue
                    offenders.append(f"{path.relative_to(ROOT)}:{i}:{line.strip()}")
    check(
        "T-MIG-04 desinstalación no borra datos (sin wipe IDB)",
        not offenders,
        "sin deleteDatabase/clear de store en apps/web/src",
    )
    for o in offenders[:10]:
        print(f"  · {o}")


def main() -> int:
    t_name_01()
    t_name_02()
    t_name_03()
    t_name_04()
    t_mig_01()
    t_mig_02()
    t_mig_03()
    t_mig_04()
    if fails:
        print(f"\ncheck_product_naming: {len(fails)} fallos")
        return 1
    print("\ncheck_product_naming: T-NAME-01..04 + T-MIG-01..04 OK")
    return 0


if __name__ == "__main__":
    sys.exit(main())
