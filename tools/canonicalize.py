#!/usr/bin/env python3
"""Verificador de referencia independiente de la canonicalización (fase R5, AUD-009/AUD-010).

Herramienta de CI, NO código de producto. Implementa el perfil
`archmaker-canonicalization-profile@v1` (autoridad semántica: ADR-0007 y
docs/03-data/canonicalization-profile-v1.md) y verifica los vectores golden de
`contracts/test-vectors/canonicalization/`.

Uso:
    python3 tools/canonicalize.py              # verifica todos los vectores (exit != 0 si falla)
    python3 tools/canonicalize.py --emit-js    # imprime la implementación JS de paridad
    python3 tools/canonicalize.py --path FILE  # canonicaliza un JSON y muestra bytes/digest

Propiedades exigidas por AUD-010:
  (a) regenera los bytes canónicos y comprueba byte-identidad y digest;
  (b) falla con código distinto de cero ante cualquier discrepancia;
  (c) es determinista (sin estado, sin orden dependiente de hash).
  (d) paridad cruzada: la canonicalización se calcula además con una segunda
      implementación (Node.js, ECMAScript) y ambos resultados deben coincidir.

Decisiones del perfil cubiertas: serialización JCS, UTF-8/BOM, Unicode,
orden de propiedades (UTF-16), arrays/sets, números, unidades (opacas), campos
excluidos, separación de dominio, SHA-256 hexadecimal, digest semántico y
versionado del perfil. Ver el perfil v1 para el detalle normativo.

Limitación declarada: la conformidad byte a byte con RFC 8785 queda «no
verificada — fuente primaria pendiente» (la fuente primaria del RFC no está
capturada en `docs/00-governance/source-register.md`). Los vectores son la
autoridad ejecutable del perfil v1 de archmaker, no una certificación del RFC.
"""

from __future__ import annotations

import argparse
import base64
import decimal
import hashlib
import json
import math
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

PROFILE_ID = "archmaker-canonicalization-profile@v1"
PROFILE_VERSION = "1"
DEFAULT_DOMAIN = "archmaker:manifest:v1"
DOMAIN_SEPARATOR = b"\x00"

ROOT = Path(__file__).resolve().parents[1]
VECTORS_DIR = ROOT / "contracts" / "test-vectors" / "canonicalization"

# Decisión 8 — campos excluidos del payload canónico (efímeros, UI, paths locales,
# diagnósticos). Se eliminan recursivamente ANTES de serializar.
DENYLIST = frozenset(
    {
        "createdAt",
        "updatedAt",
        "loadedAt",
        "lastAccessedAt",
        "uiState",
        "localPath",
        "sourcePath",
        "absolutePath",
        "workingDirectory",
        "diagnostics",
        "diagnosticLog",
    }
)


class CanonicalizationError(ValueError):
    """Entrada no canonicalizable (rechazo determinista)."""


# --------------------------------------------------------------------------- #
# Decisión 3/4 — orden de propiedades por unidades de código UTF-16 (JCS)
# --------------------------------------------------------------------------- #
def utf16_units(text: str) -> list[int]:
    units: list[int] = []
    for ch in text:
        cp = ord(ch)
        if cp > 0xFFFF:
            cp -= 0x10000
            units.append(0xD800 + (cp >> 10))
            units.append(0xDC00 + (cp & 0x3FF))
        else:
            units.append(cp)
    return units


# --------------------------------------------------------------------------- #
# Decisión 1/2/3 — serialización de cadenas UTF-8 con escapes JSON de JCS
# --------------------------------------------------------------------------- #
def escape_string(text: str) -> str:
    out = ['"']
    for ch in text:
        cp = ord(ch)
        if ch == '"':
            out.append('\\"')
        elif ch == "\\":
            out.append("\\\\")
        elif ch == "\b":
            out.append("\\b")
        elif ch == "\t":
            out.append("\\t")
        elif ch == "\n":
            out.append("\\n")
        elif ch == "\f":
            out.append("\\f")
        elif ch == "\r":
            out.append("\\r")
        elif cp < 0x20:
            out.append("\\u%04x" % cp)
        elif 0xD800 <= cp <= 0xDFFF:
            raise CanonicalizationError("surrogate suelto no canonicalizable")
        else:
            out.append(ch)
    out.append('"')
    return "".join(out)


# --------------------------------------------------------------------------- #
# Decisión 6 — representación numérica (ECMAScript Number::toString / JCS)
# --------------------------------------------------------------------------- #
def serialize_number(value: int | float) -> str:
    if isinstance(value, bool):  # defensa; los bool se tratan antes
        raise CanonicalizationError("bool no es número")
    if isinstance(value, int):
        # Decisión 6: todo número JSON es IEEE-754 binary64. Un literal entero
        # fuera del rango de binary64 se rechaza; dentro del rango se convierte
        # al double más cercano (igual que cualquier parser JSON de ECMAScript).
        try:
            value = float(value)
        except OverflowError as exc:
            raise CanonicalizationError("número fuera del rango IEEE-754") from exc
    if not math.isfinite(value):
        raise CanonicalizationError("número no finito (NaN/Infinity)")
    if value == 0:
        return "0"
    negative = value < 0
    magnitude = -value if negative else value
    dec = decimal.Decimal(repr(magnitude))
    _sign, digits, exponent = dec.as_tuple()
    if not isinstance(exponent, int):
        raise CanonicalizationError("exponente decimal no entero")
    digits = list(digits)
    while len(digits) > 1 and digits[-1] == 0:
        digits.pop()
        exponent += 1
    k = len(digits)
    n = k + exponent
    ds = "".join(str(d) for d in digits)
    if k <= n <= 21:
        body = ds + "0" * (n - k)
    elif 0 < n <= 21:
        body = ds[:n] + "." + ds[n:]
    elif -6 < n <= 0:
        body = "0." + "0" * (-n) + ds
    else:
        e = n - 1
        mantissa = ds[0] if k == 1 else ds[0] + "." + ds[1:]
        body = mantissa + "e" + ("+" if e >= 0 else "-") + str(abs(e))
    return ("-" if negative else "") + body


# --------------------------------------------------------------------------- #
# Decisión 1 — serialización normativa JCS
# --------------------------------------------------------------------------- #
def serialize(value: object) -> str:
    if value is None:
        return "null"
    if value is True:
        return "true"
    if value is False:
        return "false"
    if isinstance(value, str):
        return escape_string(value)
    if isinstance(value, (int, float)):
        return serialize_number(value)
    if isinstance(value, list):
        return "[" + ",".join(serialize(item) for item in value) + "]"
    if isinstance(value, dict):
        items = sorted(value.items(), key=lambda kv: utf16_units(kv[0]))
        return "{" + ",".join(
            escape_string(key) + ":" + serialize(val) for key, val in items
        ) + "}"
    raise CanonicalizationError(f"tipo no soportado: {type(value).__name__}")


# --------------------------------------------------------------------------- #
# Decisión 8 — exclusión de campos efímeros
# --------------------------------------------------------------------------- #
def exclude_ephemeral(value: object) -> object:
    if isinstance(value, list):
        return [exclude_ephemeral(item) for item in value]
    if isinstance(value, dict):
        return {
            key: exclude_ephemeral(val)
            for key, val in value.items()
            if key not in DENYLIST
        }
    return value


# --------------------------------------------------------------------------- #
# Decisión 5 — sets: orden por clave declarada antes de canonicalizar
# --------------------------------------------------------------------------- #
def _resolve_pointer(root: object, pointer: str) -> object:
    current = root
    for part in pointer.split("/"):
        if not part:
            continue
        if not isinstance(current, dict):
            raise CanonicalizationError(f"pointer inválido: {pointer}")
        current = current.get(part)
    return current


def sort_declared_arrays(value: object, specs: list[dict] | None) -> object:
    for spec in specs or []:
        array = _resolve_pointer(value, spec.get("pointer", ""))
        if not isinstance(array, list):
            raise CanonicalizationError(f"pointer no apunta a array: {spec}")
        key = spec.get("by")
        if not isinstance(key, str) or not key:
            raise CanonicalizationError(f"clave de orden inválida: {spec}")
        if any(not isinstance(item, dict) or key not in item for item in array):
            raise CanonicalizationError(f"elemento sin clave de orden: {spec}")
        array.sort(key=lambda item: utf16_units(str(item[key])))
    return value


# --------------------------------------------------------------------------- #
# Decisión 2/6 — parseo estricto (UTF-8 sin BOM, sin duplicados, sin constantes)
# --------------------------------------------------------------------------- #
def _reject_duplicate_pairs(pairs: list[tuple[str, object]]) -> dict:
    result: dict = {}
    for key, val in pairs:
        if key in result:
            raise CanonicalizationError(f"clave duplicada: {key}")
        result[key] = val
    return result


def _reject_constant(name: str) -> object:
    raise CanonicalizationError(f"constante numérica no finita: {name}")


def parse_bytes(raw: bytes) -> object:
    if raw[:3] == b"\xef\xbb\xbf":
        raise CanonicalizationError("BOM inicial no permitido")
    try:
        text = raw.decode("utf-8")
    except UnicodeDecodeError as exc:
        raise CanonicalizationError(f"UTF-8 inválido: {exc.reason}") from exc
    try:
        return json.loads(
            text,
            object_pairs_hook=_reject_duplicate_pairs,
            parse_constant=_reject_constant,
        )
    except json.JSONDecodeError as exc:
        raise CanonicalizationError(f"JSON inválido: {exc.msg}") from exc


# --------------------------------------------------------------------------- #
# Decisión 9/10/11/12 — separación de dominio, SHA-256 hexadecimal, digest
# --------------------------------------------------------------------------- #
def digest_of(domain: str, canonical_text: str) -> str:
    payload = domain.encode("utf-8") + DOMAIN_SEPARATOR + canonical_text.encode("utf-8")
    return hashlib.sha256(payload).hexdigest()


def process_spec(spec: dict) -> tuple[str, str]:
    if "inputBytesB64" in spec:
        value = parse_bytes(base64.b64decode(spec["inputBytesB64"]))
    else:
        value = spec.get("input")
    value = exclude_ephemeral(value)
    value = sort_declared_arrays(value, spec.get("sortArrays"))
    canonical_text = serialize(value)
    domain = spec.get("domain", DEFAULT_DOMAIN)
    return canonical_text, digest_of(domain, canonical_text)


# --------------------------------------------------------------------------- #
# Paridad cruzada — segunda implementación independiente (ECMAScript/Node.js)
# --------------------------------------------------------------------------- #
JS_CANONICALIZER = r"""// archmaker canonicalization reference cross-check (perfil@v1) — Node.js ESM.
// Implementación independiente de la de Python: usa JSON.stringify (Number::toString
// y escapes de ECMAScript) y el orden por unidades UTF-16 nativo de los strings JS.
import { createHash } from "node:crypto";
import { readFileSync } from "node:fs";

const DENYLIST = new Set(["createdAt","updatedAt","loadedAt","lastAccessedAt","uiState","localPath","sourcePath","absolutePath","workingDirectory","diagnostics","diagnosticLog"]);
const DEFAULT_DOMAIN = "archmaker:manifest:v1";
class CanonError extends Error {}

function cmp(a, b) { return a < b ? -1 : (a > b ? 1 : 0); }

function esc(s) {
  let out = '"';
  for (const ch of s) {
    const cp = ch.codePointAt(0);
    if (ch === '"') out += '\\"';
    else if (ch === '\\') out += '\\\\';
    else if (ch === '\b') out += '\\b';
    else if (ch === '\t') out += '\\t';
    else if (ch === '\n') out += '\\n';
    else if (ch === '\f') out += '\\f';
    else if (ch === '\r') out += '\\r';
    else if (cp < 0x20) out += '\\u' + cp.toString(16).padStart(4, '0');
    else if (cp >= 0xd800 && cp <= 0xdfff) throw new CanonError('surrogate suelto');
    else out += ch;
  }
  return out + '"';
}

function num(v) {
  if (!Number.isFinite(v)) throw new CanonError('numero no finito');
  if (Object.is(v, -0)) return '0';
  return JSON.stringify(v);
}

function ser(v) {
  if (v === null) return 'null';
  if (v === true) return 'true';
  if (v === false) return 'false';
  if (typeof v === 'string') return esc(v);
  if (typeof v === 'number') return num(v);
  if (Array.isArray(v)) return '[' + v.map(ser).join(',') + ']';
  if (typeof v === 'object') {
    const keys = Object.keys(v).sort(cmp);
    return '{' + keys.map((k) => esc(k) + ':' + ser(v[k])).join(',') + '}';
  }
  throw new CanonError('tipo no soportado');
}

function exclude(v) {
  if (Array.isArray(v)) return v.map(exclude);
  if (v && typeof v === 'object') {
    const out = {};
    for (const k of Object.keys(v)) if (!DENYLIST.has(k)) out[k] = exclude(v[k]);
    return out;
  }
  return v;
}

function resolvePointer(root, pointer) {
  let cur = root;
  for (const part of pointer.split('/')) {
    if (part === '') continue;
    if (cur === null || typeof cur !== 'object') throw new CanonError('pointer invalido');
    cur = cur[part];
  }
  return cur;
}

function sortArrays(root, specs) {
  for (const spec of specs || []) {
    const arr = resolvePointer(root, spec.pointer || '');
    if (!Array.isArray(arr)) throw new CanonError('pointer no es array');
    if (arr.some((it) => !it || typeof it !== 'object' || !(spec.by in it))) throw new CanonError('elemento sin clave');
    arr.sort((a, b) => cmp(String(a[spec.by]), String(b[spec.by])));
  }
  return root;
}

function parseBytes(raw) {
  if (raw.length >= 3 && raw[0] === 0xef && raw[1] === 0xbb && raw[2] === 0xbf) throw new CanonError('BOM');
  const text = raw.toString('utf8');
  if (Buffer.from(text, 'utf8').compare(raw) !== 0) throw new CanonError('utf8 invalido');
  return JSON.parse(text);
}

export function processSpec(spec) {
  let value = ('inputBytesB64' in spec) ? parseBytes(Buffer.from(spec.inputBytesB64, 'base64')) : spec.input;
  value = exclude(value);
  value = sortArrays(value, spec.sortArrays);
  const canonical = ser(value);
  const domain = spec.domain || DEFAULT_DOMAIN;
  const digest = createHash('sha256').update(Buffer.from(domain, 'utf8')).update(Buffer.from([0])).update(Buffer.from(canonical, 'utf8')).digest('hex');
  return { canonical, digest };
}

const raw = readFileSync(0, 'utf8');
try {
  process.stdout.write(JSON.stringify(processSpec(JSON.parse(raw))));
} catch (err) {
  process.stderr.write(String(err && err.message ? err.message : err));
  process.exit(2);
}
"""


def _js_runtime() -> str | None:
    return shutil.which("node") or shutil.which("bun")


def cross_check_spec(spec: dict, js_path: Path) -> tuple[str, str]:
    runtime = _js_runtime()
    if runtime is None:
        raise CanonicalizationError("sin runtime JS (node/bun) para paridad cruzada")
    proc = subprocess.run(
        [runtime, str(js_path)],
        input=json.dumps(spec, ensure_ascii=True).encode("utf-8"),
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        check=False,
    )
    if proc.returncode != 0:
        raise CanonicalizationError(proc.stderr.decode("utf-8", "replace").strip())
    payload = json.loads(proc.stdout.decode("utf-8"))
    return payload["canonical"], payload["digest"]


# --------------------------------------------------------------------------- #
# Verificación de vectores golden
# --------------------------------------------------------------------------- #
def _load_vectors() -> list[dict]:
    manifest_path = VECTORS_DIR / "manifest.json"
    manifest = json.loads(manifest_path.read_text(encoding="utf-8"))
    vectors: list[dict] = []
    for entry in manifest["files"]:
        data = json.loads((VECTORS_DIR / entry["file"]).read_text(encoding="utf-8"))
        for vector in data["vectors"]:
            vector.setdefault("category", entry["category"])
            vectors.append(vector)
    return vectors


def verify() -> int:
    runtime = _js_runtime()
    failures = 0
    vector_count = 0
    cross_checked = 0
    cross_skipped: list[str] = []

    js_path: Path | None = None
    tmp_dir: Path | None = None
    if runtime is not None:
        tmp_dir = Path(tempfile.mkdtemp(prefix="archmaker-canon-"))
        js_path = tmp_dir / "canon.mjs"
        js_path.write_text(JS_CANONICALIZER, encoding="utf-8")

    try:
        for vector in _load_vectors():
            vector_count += 1
            vector_id = vector["id"]
            spec = {
                key: vector[key]
                for key in ("input", "inputBytesB64", "sortArrays", "domain")
                if key in vector
            }
            if vector.get("expect") == "reject":
                try:
                    process_spec(spec)
                except CanonicalizationError:
                    pass
                else:
                    print(f"FALLO {vector_id}: se esperaba rechazo y no se rechazó")
                    failures += 1
                    continue
                if runtime is None or js_path is None or vector.get("crossCheckJs") is False:
                    cross_skipped.append(vector_id)
                    continue
                try:
                    cross_check_spec(spec, js_path)
                except CanonicalizationError:
                    cross_checked += 1
                except Exception:  # noqa: BLE001
                    cross_skipped.append(vector_id)
                else:
                    print(f"FALLO {vector_id}: JS no rechazó una entrada inválida")
                    failures += 1
                continue

            try:
                canonical_text, digest = process_spec(spec)
            except CanonicalizationError as exc:
                print(f"FALLO {vector_id}: rechazo inesperado: {exc}")
                failures += 1
                continue

            if canonical_text != vector["canonical"]:
                print(f"FALLO {vector_id}: bytes canónicos no coinciden")
                print(f"  esperado: {vector['canonical']!r}")
                print(f"  obtenido: {canonical_text!r}")
                failures += 1
            if digest != vector["digest"]:
                print(f"FALLO {vector_id}: digest no coincide")
                print(f"  esperado: {vector['digest']}")
                print(f"  obtenido: {digest}")
                failures += 1

            if js_path is not None:
                try:
                    js_canonical, js_digest = cross_check_spec(spec, js_path)
                except CanonicalizationError as exc:
                    print(f"FALLO {vector_id}: paridad JS rechazó: {exc}")
                    failures += 1
                except Exception as exc:  # noqa: BLE001
                    cross_skipped.append(vector_id)
                    print(f"AVISO {vector_id}: paridad cruzada no disponible: {exc}")
                else:
                    cross_checked += 1
                    if js_canonical != canonical_text or js_digest != digest:
                        print(f"FALLO {vector_id}: divergencia Python/JS")
                        failures += 1
    finally:
        if tmp_dir is not None:
            shutil.rmtree(tmp_dir, ignore_errors=True)

    print(f"Vectores verificados: {vector_count}")
    print(f"Paridad cruzada Python/JS: {cross_checked} vector(es)")
    if cross_skipped:
        print(f"Paridad omitida (rechazo no reproducible en JS): {sorted(set(cross_skipped))}")
    if runtime is None:
        print("AVISO: sin runtime JS; paridad cruzada no ejecutada (limitación declarada).")
    if failures:
        print(f"FALLO: {failures} discrepancia(s) en el corpus de canonicalización.")
        return 1
    print("OK: bytes canónicos y digest reproducibles y concordantes con el corpus.")
    return 0


def _print_file(path: Path) -> int:
    raw = path.read_bytes()
    parsed = parse_bytes(raw)
    canonical_text = serialize(exclude_ephemeral(parsed))
    digest = digest_of(DEFAULT_DOMAIN, canonical_text)
    print(canonical_text)
    print(digest)
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description="Verificador de canonicalización (perfil v1).")
    parser.add_argument("--emit-js", action="store_true", help="imprime la implementación JS de paridad")
    parser.add_argument("--path", type=Path, help="canonicaliza un archivo JSON y muestra bytes y digest")
    args = parser.parse_args()
    if args.emit_js:
        sys.stdout.write(JS_CANONICALIZER)
        return 0
    if args.path is not None:
        if not args.path.is_file():
            print(f"ERROR: no existe {args.path}")
            return 2
        return _print_file(args.path)
    if not VECTORS_DIR.is_dir():
        print(f"ERROR: no existe {VECTORS_DIR}")
        return 2
    return verify()


if __name__ == "__main__":
    sys.exit(main())
