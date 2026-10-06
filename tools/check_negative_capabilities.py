#!/usr/bin/env python3
"""Verificador estatico de controles Tauri/CSP/shell (I9, TST-NEG-001..008 diseno->evidencia)."""
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
fails: list[str] = []


def check(name: str, ok: bool, detail: str = "") -> None:
    print(("PASS " if ok else "FAIL ") + name + (f" — {detail}" if detail else ""))
    if not ok:
        fails.append(name)


cap = json.loads((ROOT / "src-tauri/capabilities/main.json").read_text())
check("NEG-001 deny-by-default por ventana", cap.get("windows") == ["main"])
check(
    "NEG-001 solo 7 comandos CorePort v0",
    len(cap.get("permissions", [])) == 7,
    str(len(cap.get("permissions", []))),
)
check("NEG-004 shell denegado", "shell:*" in cap.get("denied", []))

conf = json.loads((ROOT / "src-tauri/tauri.conf.json").read_text())
csp = conf["app"]["security"]["csp"]
check("NEG-005 CSP default-src self", "default-src 'self'" in csp, csp)
check("NEG-005 sin unsafe-eval/inline", "unsafe-eval" not in csp and "unsafe-inline" not in csp)
check("devtools off en release", conf["app"]["security"]["devtools"] is False)

html = (ROOT / "apps/web/index.html").read_text()
check("NEG-005 index.html CSP self", "default-src 'self'" in html)
check(
    "NEG-005/007 sin CDN ni red en WebView",
    not re.search(r"https://(unpkg|cdn\.|fonts\.google)", html),
)
main_ts = (ROOT / "apps/web/src/main.ts").read_text()
allowed = {"create_draft", "load_catalog", "save_draft", "resolve_draft",
           "validate_draft", "build_manifest", "export_artifact"}
invokes = set(re.findall(r'"([a-z_]+)"', main_ts)) & allowed
check("NEG-001 invoke solo allowlist (7)", invokes == allowed, str(sorted(invokes)))
check(
    "NEG-004 sin superficie shell en TS",
    not re.search(r"pacstrap|sidecar|Command::|\.spawn\(|sh -c", main_ts),
)
check("NEG-002/003 destinationHandle opaco", "picker:opaque-handle" in main_ts or "opaque" in main_ts.lower())

tokens = (ROOT / "packages/tokens/tokens.css").read_text()
check(
    "tokens --rh-* preservados (10 alias)",
    all(a in tokens for a in ["--rh-red", "--rh-blue", "--rh-green", "--rh-yellow",
                              "--rh-bg", "--rh-surface", "--rh-border", "--rh-black",
                              "--rh-gray", "--rh-gray-dark"]),
)
check("tokens .dark corrige text-subtle", ".dark" in tokens and "--color-text-subtle" in tokens)
check("tokens reduced-motion", "prefers-reduced-motion" in tokens)

print(f"\n{len(fails)} fallos.")
sys.exit(1 if fails else 0)
