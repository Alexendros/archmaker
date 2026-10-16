#!/usr/bin/env python3
"""VAL-CLOSURE-EVIDENCE: manifiesto de evidencia de cierre (F0/P2).

Comprueba el esquema mínimo, rechaza manifiestos incompletos y no trata un
ejemplo como evidencia. El manifiesto no puede incluir su propio digest.
La aprobación exige executed == expected, passed == expected, failed == 0,
skipped == 0 y retries == 0, con expected = len(requiredTestIds) > 0, y
technicalResult == "passed" (no null ni ausente en modo aprobación nueva).
Exit 0 con cero tests no cuenta.

Solo biblioteca estándar para el validador propio. Si jsonschema==4.23.0 está
instalado, se compara concordancia sobre corpus positivo/negativo.
"""

from __future__ import annotations

import hashlib
import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCHEMA_PATH = ROOT / "docs/10-delivery/closure/evidence-manifest.schema.json"
EXAMPLES = ROOT / "docs/10-delivery/closure/examples"
CORPUS = ROOT / "crates/archmaker-core/tests/fixtures/closure"
DIGESTS = CORPUS / "fixture-digests.json"

SUPPORTED = {
    "$schema",
    "$id",
    "title",
    "description",
    "type",
    "enum",
    "const",
    "pattern",
    "minLength",
    "minimum",
    "additionalProperties",
    "required",
    "properties",
    "items",
}

SHA_KEYS = {
    "sha256",
    "digest",
    "evidenceSha256",
    "manifestDigest",
    "selfDigest",
}


def fail(message: str) -> None:
    print(f"FAIL {message}")


def matches_type(value: object, type_spec: object) -> bool:
    types = type_spec if isinstance(type_spec, list) else [type_spec]
    for kind in types:
        if kind == "null" and value is None:
            return True
        if kind == "string" and isinstance(value, str):
            return True
        if kind == "integer" and isinstance(value, int) and not isinstance(value, bool):
            return True
        if kind == "array" and isinstance(value, list):
            return True
        if kind == "object" and isinstance(value, dict):
            return True
        if kind == "boolean" and isinstance(value, bool):
            return True
    return False


def assert_supported(schema: object, path: str = "$") -> None:
    if not isinstance(schema, dict):
        raise SystemExit(f"FAIL esquema no objeto en {path}")
    unknown = set(schema) - SUPPORTED
    if unknown:
        raise SystemExit(f"FAIL keyword no implementada en {path}: {sorted(unknown)}")
    properties = schema.get("properties")
    if isinstance(properties, dict):
        for name, child in properties.items():
            assert_supported(child, f"{path}.{name}")
    items = schema.get("items")
    if items is not None:
        assert_supported(items, f"{path}.items")


def validate_against(schema: dict, value: object, path: str, errors: list[str]) -> None:
    if "type" in schema and not matches_type(value, schema["type"]):
        errors.append(f"{path}: tipo distinto de {schema['type']}")
        return
    # enum/const también para null (concordancia con jsonschema draft 2020-12).
    if "const" in schema and value != schema["const"]:
        errors.append(f"{path}: const {schema['const']!r}")
    if "enum" in schema and value not in schema["enum"]:
        errors.append(f"{path}: fuera de enum")
    if value is None:
        return
    if "pattern" in schema and isinstance(value, str):
        if re.fullmatch(schema["pattern"], value) is None:
            errors.append(f"{path}: no cumple el patrón")
    if "minLength" in schema and isinstance(value, str) and len(value) < schema["minLength"]:
        errors.append(f"{path}: más corto que minLength")
    if (
        "minimum" in schema
        and isinstance(value, int)
        and not isinstance(value, bool)
        and value < schema["minimum"]
    ):
        errors.append(f"{path}: por debajo de minimum")
    if isinstance(value, dict):
        properties = schema.get("properties", {})
        if schema.get("additionalProperties") is False:
            for key in sorted(set(value) - set(properties)):
                errors.append(f"{path}.{key}: propiedad no admitida")
        for key in schema.get("required", []):
            if key not in value:
                errors.append(f"{path}.{key}: falta")
        if isinstance(properties, dict):
            for key, child in properties.items():
                if key in value:
                    validate_against(child, value[key], f"{path}.{key}", errors)
    if isinstance(value, list) and "items" in schema:
        for index, item in enumerate(value):
            validate_against(schema["items"], item, f"{path}[{index}]", errors)


def is_example_path(path: Path) -> bool:
    parts = {part.lower() for part in path.parts}
    name = path.name.lower()
    return "examples" in parts or "example" in name


def approvable(data: dict, path: Path) -> tuple[bool, str]:
    if is_example_path(path):
        return False, "un ejemplo no es evidencia"
    if data.get("result") != "passed":
        return False, "result distinto de passed"
    # Modo aprobación nueva: exige technicalResult=passed (no null ni omitido).
    if data.get("technicalResult") != "passed":
        return False, "technicalResult distinto de passed"
    ids = data.get("requiredTestIds")
    if not isinstance(ids, list) or len(ids) == 0:
        return False, "expected es 0; exit 0 sin tests no cuenta"
    expected = len(ids)
    if data.get("executed") != expected or data.get("passed") != expected:
        return False, "executed o passed distintos de expected"
    if data.get("failed") != 0 or data.get("skipped") != 0 or data.get("retries") != 0:
        return False, "failed, skipped o retries distintos de 0"
    if not isinstance(data.get("reviewer"), str) or not data["reviewer"]:
        return False, "reviewer ausente"
    if not isinstance(data.get("reviewedAt"), str) or not data["reviewedAt"]:
        return False, "reviewedAt ausente"
    if not isinstance(data.get("sourceSha"), str) or not data["sourceSha"]:
        return False, "sourceSha ausente"
    approval = data.get("approval")
    if isinstance(approval, dict):
        if approval.get("status") != "passed":
            return False, "approval.status distinto de passed"
        digest = approval.get("approvalRecordSha256")
        if not isinstance(digest, str) or len(digest) != 64:
            return False, "approval.approvalRecordSha256 ausente"
    return True, "ok"


def sha256_hex(data: bytes) -> str:
    return hashlib.sha256(data).hexdigest()


def compare_with_jsonschema(schema: dict, instances: list[tuple[str, object, bool]]) -> list[str]:
    """Compara validador propio con jsonschema==4.23.0. Devuelve errores de discordancia."""
    try:
        from importlib.metadata import version as pkg_version

        from jsonschema import Draft202012Validator
    except ImportError:
        return ["falta jsonschema (pip install jsonschema==4.23.0)"]

    try:
        js_version = pkg_version("jsonschema")
    except Exception:  # noqa: BLE001
        js_version = "?"
    if js_version != "4.23.0":
        return [f"jsonschema versión {js_version!r}; se exige 4.23.0"]

    mismatches: list[str] = []
    validator = Draft202012Validator(schema)
    for label, instance, expect_valid in instances:
        own_errors: list[str] = []
        validate_against(schema, instance, "$", own_errors)
        std_errors = list(validator.iter_errors(instance))
        own_ok = len(own_errors) == 0
        std_ok = len(std_errors) == 0
        if own_ok != std_ok:
            mismatches.append(
                f"{label}: propio={'ok' if own_ok else 'fail'} "
                f"jsonschema={'ok' if std_ok else 'fail'}"
            )
        if own_ok != expect_valid:
            mismatches.append(
                f"{label}: propio={'ok' if own_ok else 'fail'} "
                f"esperado={'ok' if expect_valid else 'fail'}"
            )
        if std_ok != expect_valid:
            mismatches.append(
                f"{label}: jsonschema={'ok' if std_ok else 'fail'} "
                f"esperado={'ok' if expect_valid else 'fail'}"
            )
    return mismatches


def check_corpus() -> int:
    errors = 0
    if not DIGESTS.is_file():
        fail(f"falta {DIGESTS.relative_to(ROOT)}")
        return 1
    index = json.loads(DIGESTS.read_text(encoding="utf-8"))
    if any(key in index for key in SHA_KEYS):
        fail("fixture-digests.json incluye un digest de sí mismo")
        errors += 1
    files = index.get("files")
    if not isinstance(files, dict) or set(files) != {
        "positive-profile.json",
        "negative-incomplete.json",
        "binary-payload.bin",
    }:
        fail("el corpus no tiene las tres clases positive/negative/binary")
        return errors + 1
    classes = set()
    for name, meta in files.items():
        blob = (CORPUS / name).read_bytes()
        digest = sha256_hex(blob)
        if meta.get("sha256") != digest or meta.get("sizeBytes") != len(blob):
            fail(f"digest o tamaño distinto en {name}")
            errors += 1
        classes.add(meta.get("class"))
        if name.endswith(".json"):
            parsed = json.loads(blob.decode("utf-8"))
            if name.startswith("positive") and parsed.get("class") != "positive":
                fail("el fixture positivo no declara class=positive")
                errors += 1
            if name.startswith("negative") and "sourceSha" in parsed:
                fail("el fixture negativo parece un manifiesto completo")
                errors += 1
        else:
            try:
                json.loads(blob.decode("utf-8"))
            except (UnicodeDecodeError, json.JSONDecodeError):
                pass
            else:
                fail("el fixture binario se parseó como JSON")
                errors += 1
    if classes != {"positive", "negative", "binary"}:
        fail(f"clases del corpus: {sorted(classes)}")
        errors += 1
    return errors


def main() -> int:
    errors = 0
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    assert_supported(schema)
    if schema.get("additionalProperties") is not False:
        fail("el esquema debe rechazar propiedades extra")
        errors += 1
    required = schema.get("required")
    if not isinstance(required, list) or "result" not in required:
        fail("el esquema no exige result")
        errors += 1

    tr_schema = (schema.get("properties") or {}).get("technicalResult") or {}
    if None not in (tr_schema.get("enum") or []):
        fail("technicalResult.enum debe incluir null (pending)")
        errors += 1

    pending_path = EXAMPLES / "manifest.pending.example.json"
    pending = json.loads(pending_path.read_text(encoding="utf-8"))
    pending_errors: list[str] = []
    validate_against(schema, pending, "$", pending_errors)
    if pending_errors:
        fail(f"el ejemplo pending no cumple el esquema: {pending_errors[:3]}")
        errors += 1
    if pending.get("result") != "pending":
        fail("el ejemplo pending no está en result=pending")
        errors += 1
    ok, reason = approvable(pending, pending_path)
    if ok:
        fail("el ejemplo pending se aprobó como evidencia")
        errors += 1
    elif "ejemplo" not in reason and pending.get("result") == "pending":
        pass

    f7_pending_path = EXAMPLES / "manifest.f7-candidate.pending.example.json"
    if f7_pending_path.is_file():
        f7_pending = json.loads(f7_pending_path.read_text(encoding="utf-8"))
        f7_errors: list[str] = []
        validate_against(schema, f7_pending, "$", f7_errors)
        if f7_errors:
            fail(f"ejemplo F7 pending no cumple esquema: {f7_errors[:3]}")
            errors += 1
        if f7_pending.get("technicalResult") is not None:
            # Pending puede emitir null o omitir; si emite valor, debe ser válido.
            pass
        ok, _ = approvable(f7_pending, f7_pending_path)
        if ok:
            fail("ejemplo F7 pending se aprobó")
            errors += 1

    complete_path = EXAMPLES / "manifest.looks-complete.example.json"
    complete = json.loads(complete_path.read_text(encoding="utf-8"))
    complete_errors: list[str] = []
    validate_against(schema, complete, "$", complete_errors)
    if complete_errors:
        fail(f"el ejemplo completo no cumple el esquema: {complete_errors[:3]}")
        errors += 1
    ok, reason = approvable(complete, complete_path)
    if ok or "ejemplo" not in reason:
        fail(f"un ejemplo no debe contar como evidencia ({reason})")
        errors += 1
    synthetic_path = Path("closure-evidence") / "run" / "manifest.json"
    ok, reason = approvable(complete, synthetic_path)
    if not ok:
        fail(f"el mismo contenido fuera de examples debería aprobar: {reason}")
        errors += 1

    # Manifiesto antiguo sin technicalResult: legible (schema) pero no aprobable.
    legacy = dict(complete)
    legacy.pop("technicalResult", None)
    legacy_errors: list[str] = []
    validate_against(schema, legacy, "$", legacy_errors)
    if legacy_errors:
        fail(f"manifiesto antiguo sin technicalResult debe leerse: {legacy_errors[:3]}")
        errors += 1
    ok, reason = approvable(legacy, synthetic_path)
    if ok or "technicalResult" not in reason:
        fail(f"manifiesto antiguo sin technicalResult no debe aprobar ({reason})")
        errors += 1

    incomplete = {"schemaVersion": "1", "result": "passed"}
    incomplete_errors: list[str] = []
    validate_against(schema, incomplete, "$", incomplete_errors)
    if not incomplete_errors:
        fail("un manifiesto incompleto fue aceptado")
        errors += 1

    with_digest = dict(complete)
    with_digest["sha256"] = "ab" * 32
    digest_errors: list[str] = []
    validate_against(schema, with_digest, "$", digest_errors)
    if not any("sha256" in item for item in digest_errors):
        fail("un manifiesto con su propio digest fue aceptado")
        errors += 1

    zero = dict(complete)
    zero["requiredTestIds"] = []
    zero["executed"] = 0
    zero["passed"] = 0
    ok, reason = approvable(zero, synthetic_path)
    if ok or "0" not in reason:
        fail(f"cero tests no debe aprobar ({reason})")
        errors += 1

    dirty = dict(complete)
    dirty["failed"] = 1
    ok, _reason = approvable(dirty, synthetic_path)
    if ok:
        fail("failed=1 fue aprobado")
        errors += 1

    # Concordancia propio ↔ jsonschema sobre pending (null) e incompleto.
    compare_instances: list[tuple[str, object, bool]] = [
        ("pending-example", pending, True),
        ("looks-complete", complete, True),
        ("incomplete", incomplete, False),
        ("self-digest", with_digest, False),
        ("legacy-no-technicalResult", legacy, True),
    ]
    if f7_pending_path.is_file():
        compare_instances.append(("f7-pending-null", f7_pending, True))
    mismatches = compare_with_jsonschema(schema, compare_instances)
    for msg in mismatches:
        fail(msg)
        errors += 1

    errors += check_corpus()
    if errors:
        print(f"{errors} fallos en la evidencia de cierre")
        return 1
    print(
        "OK evidencia de cierre: esquema, null en enum, jsonschema 4.23.0, "
        "ejemplos rechazados, legacy legible no aprobable, corpus con digest"
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
