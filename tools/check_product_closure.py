#!/usr/bin/env python3
"""VAL-PRODUCT-CLOSURE: verificador de cierre de solo lectura.

No autoaprueba, no publica y no modifica evidencias. Consume el inventario
canónico (f7-required-cases / mandatory-cases) y un manifiesto técnico opcional.
"""

from __future__ import annotations

import argparse
import json
import sys
from pathlib import Path
from typing import Any

ROOT = Path(__file__).resolve().parents[1]
INVENTORY_MD = ROOT / "docs/10-delivery/closure/inventory.md"
INVENTORY = ROOT / "docs/10-delivery/closure/f7-required-cases.json"
MANDATORY = ROOT / "docs/10-delivery/closure/mandatory-cases.md"
SCHEMA_PATH = ROOT / "docs/10-delivery/closure/evidence-manifest.schema.json"

sys.path.insert(0, str(ROOT / "tools"))


def fail(msg: str) -> None:
    print(f"FAIL {msg}", file=sys.stderr)


def evaluate_manifest(
    manifest: dict[str, Any],
    *,
    inventory_ids: list[str],
    approval_record: dict[str, Any] | None = None,
) -> dict[str, Any]:
    """Evalúa un manifiesto sin mutarlo. Nunca establece approval."""
    import validate_closure_evidence as vce

    diagnostics: list[dict[str, str]] = []
    schema = json.loads(SCHEMA_PATH.read_text(encoding="utf-8"))
    schema_errors: list[str] = []
    vce.validate_against(schema, manifest, "$", schema_errors)
    for err in schema_errors:
        diagnostics.append(
            {
                "code": "SCHEMA",
                "subject": "manifest",
                "evidence": err,
                "impact": "estructura inválida",
                "repair": "corregir manifiesto",
            }
        )

    required = manifest.get("requiredTestIds")
    if isinstance(required, list):
        if len(required) != len(set(required)):
            diagnostics.append(
                {
                    "code": "DUPLICATE_IDS",
                    "subject": "requiredTestIds",
                    "evidence": "ids duplicados",
                    "impact": "denominador ambiguo",
                    "repair": "unicidad de IDs",
                }
            )
        missing = set(inventory_ids) - set(required) if inventory_ids else set()
        if inventory_ids and missing:
            diagnostics.append(
                {
                    "code": "MISSING_REQUIRED",
                    "subject": "requiredTestIds",
                    "evidence": ",".join(sorted(missing)),
                    "impact": "cobertura incompleta",
                    "repair": "restaurar IDs del inventario",
                }
            )

    dirty = manifest.get("dirtyTrackedFiles")
    if isinstance(dirty, list) and dirty:
        diagnostics.append(
            {
                "code": "DIRTY_CHECKOUT",
                "subject": "dirtyTrackedFiles",
                "evidence": ",".join(dirty[:5]),
                "impact": "fuente no limpia",
                "repair": "limpiar checkout antes de aceptar",
            }
        )

    residuals = manifest.get("residuals") or []
    if any(isinstance(r, str) and "specimen" in r.lower() for r in residuals):
        diagnostics.append(
            {
                "code": "EXAMPLE_SPECIMEN",
                "subject": "residuals",
                "evidence": "specimen",
                "impact": "contenido de ejemplo",
                "repair": "usar corrida real, no ejemplo",
            }
        )

    source = manifest.get("sourceSha")
    for art in manifest.get("artifacts") or []:
        if not isinstance(art, dict):
            continue
        build = art.get("buildSourceSha")
        if (
            isinstance(source, str)
            and isinstance(build, str)
            and build
            and source
            and build != source
        ):
            diagnostics.append(
                {
                    "code": "ARTIFACT_SOURCE_MISMATCH",
                    "subject": art.get("name") or "artifact",
                    "evidence": f"buildSourceSha={build} sourceSha={source}",
                    "impact": "artefacto de otro candidato",
                    "repair": "reenlazar artifact al sourceSha del manifiesto",
                }
            )

    if approval_record is not None:
        rec_src = approval_record.get("productSourceSha")
        if (
            isinstance(source, str)
            and isinstance(rec_src, str)
            and rec_src
            and source
            and rec_src != source
        ):
            diagnostics.append(
                {
                    "code": "APPROVAL_CANDIDATE_MISMATCH",
                    "subject": "approval",
                    "evidence": f"record={rec_src} manifest={source}",
                    "impact": "aprobación de otro candidato",
                    "repair": "usar registro del mismo productSourceSha",
                }
            )

    # Aprobación semántica (solo lectura): no escribe result.
    ok, reason = vce.approvable(
        manifest, Path("closure-evidence") / "run" / "manifest.json"
    )
    if not ok:
        diagnostics.append(
            {
                "code": "NOT_APPROVABLE",
                "subject": "approvable",
                "evidence": reason,
                "impact": "no apto para aceptación",
                "repair": "completar evidencias y technicalResult=passed",
            }
        )

    technical = manifest.get("technicalResult")
    if technical == "partial":
        diagnostics.append(
            {
                "code": "PARTIAL_NOT_PASSED",
                "subject": "technicalResult",
                "evidence": "partial",
                "impact": "CI/ejecución incompleta",
                "repair": "completar matriz; no etiquetar passed",
            }
        )

    blocking = [
        d
        for d in diagnostics
        if d["code"]
        not in {
            # Informativos que no impiden modo diagnóstico
        }
    ]
    if blocking:
        tech_out = "failed" if any(
            d["code"] in {"SCHEMA", "NOT_APPROVABLE", "ARTIFACT_SOURCE_MISMATCH",
                          "APPROVAL_CANDIDATE_MISMATCH", "DIRTY_CHECKOUT",
                          "EXAMPLE_SPECIMEN", "PARTIAL_NOT_PASSED"}
            for d in blocking
        ) else "partial"
        result = "pending"
        approval_status = "pending"
    else:
        tech_out = "passed" if technical == "passed" else (technical or "pending")
        result = "pending"  # este verificador NUNCA autoaprueba
        approval_status = "pending"

    expected = len(inventory_ids) if inventory_ids else (
        len(required) if isinstance(required, list) else 0
    )
    return {
        "result": result,
        "technicalResult": tech_out if blocking else (technical or "pending"),
        "approvalStatus": approval_status,
        "expected": expected,
        "executed": manifest.get("executed"),
        "passed": manifest.get("passed"),
        "failed": manifest.get("failed"),
        "skipped": manifest.get("skipped"),
        "retries": manifest.get("retries"),
        "diagnostics": diagnostics,
        "readOnly": True,
        "autoApproved": False,
    }


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--manifest",
        type=Path,
        default=None,
        help="Manifiesto técnico a evaluar (solo lectura)",
    )
    parser.add_argument(
        "--approval-record",
        type=Path,
        default=None,
        help="Registro externo de aprobación (opcional, solo lectura)",
    )
    parser.add_argument(
        "--strict",
        action="store_true",
        help="Exit no cero si hay diagnósticos bloqueantes (nunca autoaprueba)",
    )
    parser.add_argument(
        "--json",
        action="store_true",
        help="Imprime el informe JSON en stdout",
    )
    args = parser.parse_args(argv)

    if not INVENTORY_MD.is_file():
        fail(f"falta inventario canónico {INVENTORY_MD}")
        return 1
    if not INVENTORY.is_file():
        fail(f"falta inventario {INVENTORY}")
        return 1
    if not MANDATORY.is_file():
        fail(f"falta {MANDATORY}")
        return 1

    inventory = json.loads(INVENTORY.read_text(encoding="utf-8"))
    ids = [c["id"] for c in inventory.get("cases") or [] if isinstance(c, dict)]

    if args.manifest is None:
        report = {
            "result": "pending",
            "technicalResult": "partial",
            "approvalStatus": "pending",
            "expected": len(ids),
            "executed": None,
            "passed": None,
            "failed": None,
            "skipped": None,
            "retries": None,
            "diagnostics": [
                {
                    "code": "NO_MANIFEST",
                    "subject": "manifest",
                    "evidence": "no suministrado",
                    "impact": "solo inventario verificado",
                    "repair": "pasar --manifest para evaluación completa",
                }
            ],
            "readOnly": True,
            "autoApproved": False,
            "inventoryCaseCount": len(ids),
        }
    else:
        if not args.manifest.is_file():
            fail(f"falta manifiesto {args.manifest}")
            return 1
        manifest = json.loads(args.manifest.read_text(encoding="utf-8"))
        approval = None
        if args.approval_record is not None:
            if not args.approval_record.is_file():
                fail(f"falta approval-record {args.approval_record}")
                return 1
            approval = json.loads(args.approval_record.read_text(encoding="utf-8"))
        report = evaluate_manifest(
            manifest, inventory_ids=ids, approval_record=approval
        )

    # Garantía: nunca autoaprueba.
    if report.get("autoApproved") or report.get("approvalStatus") == "passed":
        fail("el verificador intentó autoaprobar; abortando")
        return 1
    if report.get("result") == "passed":
        # Solo lectura: forzar pending en la salida del checker.
        report["result"] = "pending"
        report["approvalStatus"] = "pending"
        report.setdefault("diagnostics", []).append(
            {
                "code": "NO_AUTO_APPROVE",
                "subject": "result",
                "evidence": "forced-pending",
                "impact": "aceptación humana requerida",
                "repair": "registro de aprobación externo",
            }
        )

    if args.json:
        print(json.dumps(report, indent=2, ensure_ascii=False))
    else:
        print(
            f"OK check_product_closure (read-only): "
            f"cases={report.get('expected')} "
            f"technicalResult={report.get('technicalResult')} "
            f"diagnostics={len(report.get('diagnostics') or [])} "
            f"autoApproved=false"
        )

    if args.strict and report.get("diagnostics"):
        for d in report["diagnostics"]:
            fail(f"{d['code']}: {d.get('evidence')}")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main())
