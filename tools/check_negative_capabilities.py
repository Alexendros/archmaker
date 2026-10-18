#!/usr/bin/env python3
"""Verificador estatico de controles Tauri/CSP/shell (I9, TST-NEG-001..008).

Espejo Python del test Rust `crates/archmaker-core/tests/negative_capabilities.rs`.
"""
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


CORE_CMDS = [
    "create_draft",
    "load_catalog",
    "save_draft",
    "open_draft",
    "resolve_draft",
    "validate_draft",
    "build_manifest",
    "export_artifact",
    "import_draft",
]

HW_CMDS = [
    "inspect_host_hardware",
    "inspect_storage_devices",
    "refresh_storage_snapshot",
    "propose_storage_plan",
    "validate_storage_plan",
    "import_storage_snapshot",
]
APP_CMDS = CORE_CMDS + ["save_export_bytes", "constitute_image"] + HW_CMDS

build_rs = (ROOT / "src-tauri/build.rs").read_text()
check(
    "NEG-001 AppManifest::commands",
    "AppManifest::new().commands" in build_rs,
)
for cmd in APP_CMDS:
    check(f"NEG-001 AppManifest lista {cmd}", f'"{cmd}"' in build_rs)

cap = json.loads((ROOT / "src-tauri/capabilities/main.json").read_text())
check("NEG-001 deny-by-default por ventana", cap.get("windows") == ["main"])
perms = cap.get("permissions") or []
check(
    "NEG-001 main concede CorePort + save_export + hardware probe",
    len(perms) == 17
    and all(isinstance(p, str) and p.startswith("allow-") for p in perms)
    and "allow-save-export-bytes" in perms
    and "allow-constitute-image" in perms
    and "allow-inspect-host-hardware" in perms,
    str(perms),
)
check(
    "NEG-001 sin diálogo global libre ni FS/shell plugin",
    "dialog:allow-open" not in perms
    and "dialog:default" not in perms
    and not any(isinstance(p, str) and (p.startswith("fs:") or "shell" in p) for p in perms),
    str(perms),
)

restricted = json.loads((ROOT / "src-tauri/capabilities/restricted.json").read_text())
check(
    "NEG-001 restricted permissions vacía",
    restricted.get("permissions") == [],
    str(restricted.get("permissions")),
)

caps_dir = ROOT / "src-tauri/capabilities"
cap_files = sorted(p.name for p in caps_dir.glob("*.json"))
check(
    "NEG-001 capabilities main+restricted",
    cap_files == ["main.json", "restricted.json"],
    str(cap_files),
)
for reserved in ["dialog", "picker"]:
    check(
        f"NEG-001 {reserved}.json ausente (guardado acotado, no ventana dialog)",
        not (caps_dir / f"{reserved}.json").exists(),
    )

raw_conf = (ROOT / "src-tauri/tauri.conf.json").read_text()
conf = json.loads(raw_conf)
csp = conf["app"]["security"]["csp"]
check("NEG-005 CSP default-src self", "default-src 'self'" in csp, csp)
check("NEG-005 sin unsafe-eval/inline", "unsafe-eval" not in csp and "unsafe-inline" not in csp)
check(
    "NEG-004 shell denegado (ausencia estructural)",
    '"shell"' not in raw_conf or "shell:*" in raw_conf,
)
check(
    "NEG-devtools ausente en app.security (CON-021)",
    "devtools" not in conf["app"]["security"],
    "schema v2 sin devtools en SecurityConfig",
)
check(
    "NEG-F5 frontendDist apps/web/dist",
    conf.get("build", {}).get("frontendDist") == "../apps/web/dist",
    str(conf.get("build", {}).get("frontendDist")),
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

tauri_port = (ROOT / "apps/web/src/tauri-core-port.ts").read_text()
export_delivery = (ROOT / "apps/web/src/export-delivery.ts").read_text()
web_src = "".join(
    p.read_text() for p in sorted((ROOT / "apps/web/src").glob("*.ts*"))
)
allowed = set(CORE_CMDS)
invokes = set(re.findall(r'"([a-z_]+)"', tauri_port)) & allowed
check("NEG-001 invoke CorePort allowlist (9)", invokes == allowed, str(sorted(invokes)))
check(
    "NEG-F5 deliverArtifact invoca save_export_bytes",
    "save_export_bytes" in export_delivery and "deliverArtifact" in export_delivery,
)
check(
    "NEG-004 sin superficie shell en TS",
    not re.search(r"pacstrap|sidecar|Command::|\.spawn\(|sh -c", web_src),
)
check(
    "NEG-002/003 destinationHandle opaco",
    "destination: { token" in web_src and "opaque" in web_src.lower(),
)

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
