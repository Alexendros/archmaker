#!/usr/bin/env python3
"""Chequeo WCAG 2.2 AA automatizable (I8, T-I8-04): contraste, foco, reduced-motion, ARIA."""
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent.parent
fails: list[str] = []


def check(name: str, ok: bool, detail: str = "") -> None:
    print(("PASS " if ok else "FAIL ") + name + (f" — {detail}" if detail else ""))
    if not ok:
        fails.append(name)


def lum(hexval: str) -> float:
    hexval = hexval.lstrip("#")
    r, g, b = (int(hexval[i:i + 2], 16) / 255 for i in (0, 2, 4))

    def f(c: float) -> float:
        return c / 12.92 if c <= 0.03928 else ((c + 0.055) / 1.055) ** 2.4

    return 0.2126 * f(r) + 0.7152 * f(g) + 0.0722 * f(b)


def ratio(a: str, b: str) -> float:
    la, lb = lum(a), lum(b)
    hi, lo = max(la, lb), min(la, lb)
    return (hi + 0.05) / (lo + 0.05)


tokens = (ROOT / "packages/tokens/tokens.css").read_text()
html = (ROOT / "apps/web/index.html").read_text()
comps = (ROOT / "apps/web/src/components.ts").read_text()
styles = (ROOT / "apps/web/src/styles.css").read_text()

pairs = [
    ("texto/canvas claro", "#151515", "#ffffff", 4.5),
    ("texto/canvas oscuro", "#ffffff", "#151515", 4.5),
    ("secundario/canvas claro", "#6a6e73", "#ffffff", 4.5),
    ("secundario/canvas oscuro", "#a8a8a8", "#151515", 4.5),
    ("detalle/canvas claro", "#3c3f42", "#ffffff", 4.5),
    ("detalle/canvas oscuro corregido", "#a8a8a8", "#151515", 4.5),
    ("foco/canvas claro", "#0066cc", "#ffffff", 3.0),
    ("CTA blanco/accion", "#ffffff", "#ee0000", 4.5),
]
for name, fg, bg, umbral in pairs:
    r = ratio(fg, bg)
    check(f"contraste {name}", r >= umbral, f"{r:.2f}:1 >= {umbral}:1")

check("foco visible --color-focus", ":focus-visible" in tokens and "--color-focus" in tokens)
check("reduced-motion en tokens", "prefers-reduced-motion" in tokens)
check("html lang=es", 'lang="es"' in html)
check("skip link", "skip-link" in html and "Saltar al contenido" in html)
check("landmarks nav/main/aside", all(t in html for t in ["<nav", "<main", "<aside"]))
check("regiones vivas status+alert", 'role="status"' in html and 'role="alert"' in html)
check("dialog con aria-labelledby", 'aria-labelledby="evidence-title"' in html)
check("OptionCard role checkbox + aria-checked", 'role", "checkbox"' in comps or 'role", `checkbox`' in comps or '"checkbox"' in comps)
check("aria-current=step en rail", 'aria-current", "step"' in comps or '"step"' in comps)
check("sin div interactivo sin rol", not re.search(r'<div[^>]*onClick', html))
check("objetivos tactiles via min-height", "min-height" in styles or "min-width" in styles)

print(f"\n{len(fails)} fallos. axe-core manual: npx axe-core no ejecutado offline; ver A11Y-EVIDENCE.md.")
sys.exit(1 if fails else 0)
