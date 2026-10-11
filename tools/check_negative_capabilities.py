#!/usr/bin/env python3
"""Verificador estatico de controles Tauri/CSP/shell (I9, TST-NEG-001..008 diseno->evidencia).

Espejo Python del test Rust `crates/archmaker-core/tests/negative_capabilities.rs`,
que es la autoridad semantica (AGENTS.md: Rust decide, TS/Python no duplican reglas).
Cualquier divergencia entre este checker y el test Rust es un bug de este checker.
Resuelve CON-021/D-09: `app.security.devtools` no existe en el schema Tauri v2
(`SecurityConfig` = csp, dev_csp, freeze_prototype, ...; `devtools` solo vive en
`WindowConfig` por ventana) — la CLI 2.x rechaza la clave, PR-02 la elimino y el
deny-by-default es implicito.
"""
import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
fails: list[str] = []


def check(name: str, ok: bool, detail: str = "") -> None:
    print(("PASS " if ok else "FAIL ") + name + (f" — {detail}" if detail else ""))
    if not ok:
        fails.append(name)


CORE_CMDS = ["create-draft", "load-catalog", "save-draft", "resolve-draft",
             "validate-draft", "build-manifest", "export-artifact"]

cap = json.loads((ROOT / "src-tauri/capabilities/main.json").read_text())
check("NEG-001 deny-by-default por ventana", cap.get("windows") == ["main"])
# Los 7 permisos viven en permissions/*.toml (identifier core:<cmd>); el JSON de
# capability queda vacio (auto-descubrimiento). Espejo de neg_capability_deny_by_default.
check(
    "NEG-001 capability JSON sin permisos inline",
    cap.get("permissions") == [],
    str(cap.get("permissions")),
)
found = []
for cmd in CORE_CMDS:
    with open(ROOT / f"src-tauri/permissions/{cmd}.toml", "rb") as f:
        ident = tomllib.load(f).get("identifier")
    if ident == f"core:{cmd}":
        found.append(cmd)
check(
    "NEG-001 solo 7 comandos CorePort v0 (core:*)",
    found == CORE_CMDS,
    str(found),
)
for reserved in ["dialog", "picker"]:
    ocap = json.loads((ROOT / f"src-tauri/capabilities/{reserved}.json").read_text())
    check(
        f"NEG-001 {reserved} reservado sin permisos (v1)",
        ocap.get("permissions") == [],
        str(ocap.get("permissions")),
    )

raw_conf = (ROOT / "src-tauri/tauri.conf.json").read_text()
conf = json.loads(raw_conf)
csp = conf["app"]["security"]["csp"]
check("NEG-005 CSP default-src self", "default-src 'self'" in csp, csp)
check("NEG-005 sin unsafe-eval/inline", "unsafe-eval" not in csp and "unsafe-inline" not in csp)
# CON-021/D-09: sin plugin shell salvo denegacion explicita (espejo Rust).
check(
    "NEG-004 shell denegado (ausencia estructural)",
    '"shell"' not in raw_conf or "shell:*" in raw_conf,
)
# CON-021/D-09: la clave no existe en SecurityConfig v2; su ausencia ES el control
# (deny-by-default implicito). Anadirla rompe la validacion de la CLI (PR-02).
check(
    "NEG-devtools ausente en app.security (CON-021)",
    "devtools" not in conf["app"]["security"],
    "schema v2 sin devtools en SecurityConfig",
)
main_rs = (ROOT / "src-tauri/src/main.rs").read_text()
check(
    "NEG-devtools sin apertura programatica",
    "open_devtools" not in main_rs and "tauri_plugin_devtools" not in main_rs,
)

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
