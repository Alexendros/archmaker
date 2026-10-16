#!/usr/bin/env python3
"""Genera las 24 especificaciones Archify as-built bajo docs/02-architecture/diagrams/.

No inventa runtime: solo contenedores/crates presentes en el workspace.
D25/D26 no se generan (condicionados a mediciones / ERD real).
"""

from __future__ import annotations

import json
import subprocess
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DIAG = ROOT / "docs/02-architecture/diagrams"
SOURCES = DIAG / "sources"
RENDERER_SHA = "ca67c9f16be0c8cad409cd45190aa052e0649b84"


def git_sha() -> str:
    return subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()


def write(name: str, payload: dict) -> None:
    SOURCES.mkdir(parents=True, exist_ok=True)
    path = SOURCES / name
    path.write_text(json.dumps(payload, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"wrote {path.relative_to(ROOT)}")


def arch(
    diagram_id: str,
    title: str,
    components: list,
    connections: list,
    cards: list,
    boundaries: list | None = None,
) -> dict:
    return {
        "schema_version": 1,
        "diagram_type": "architecture",
        "meta": {
            "title": title,
            "output": f"rendered/{diagram_id}.html",
            "quality_profile": "showcase",
            "subtitle": "Forge as-built (MVP); runner/ISO/Enterprise fuera de alcance",
        },
        "components": components,
        "boundaries": boundaries or [],
        "connections": connections,
        "cards": cards,
    }


def main() -> int:
    source_sha = git_sha()
    catalog_entries = []

    # D01 context
    write(
        "D01-context.architecture.json",
        arch(
            "D01",
            "D01 Contexto del configurador Forge",
            [
                {"id": "user", "type": "external", "label": "Usuario", "sublabel": "Sencillo / Avanzado", "pos": [40, 280], "size": [130, 64]},
                {"id": "forge", "type": "frontend", "label": "Forge", "sublabel": "configurador", "pos": [280, 280], "size": [150, 64], "tag": "público"},
                {"id": "arch_docs", "type": "external", "label": "Docs Arch", "sublabel": "referencia", "pos": [520, 140], "size": [140, 60]},
                {"id": "gh_release", "type": "cloud", "label": "GitHub Release", "sublabel": "AppImage v0.1.0", "pos": [520, 280], "size": [160, 64]},
                {"id": "future", "type": "external", "label": "Futuro", "sublabel": "Runner / ISO", "pos": [520, 420], "size": [160, 64], "tag": "no shipped"},
            ],
            [
                {"id": "u-n", "from": "user", "to": "forge", "label": "selección → export", "variant": "emphasis"},
                {"id": "n-docs", "from": "forge", "to": "arch_docs", "label": "lee documentación", "variant": "dashed"},
                {"id": "n-rel", "from": "forge", "to": "gh_release", "label": "descarga AppImage", "variant": "dashed"},
                {"id": "n-fut", "from": "forge", "to": "future", "label": "fuera de veredicto", "variant": "dashed"},
            ],
            [
                {"dot": "cyan", "title": "Producto", "items": ["Forge es la interfaz gráfica de ArchMaker", "Archify es herramienta de diagramación, no el producto"]},
                {"dot": "amber", "title": "Límites", "items": ["Runner, ISO, root y discos no están implementados", "Enterprise documentado solo en C4 aspiracional"]},
            ],
        ),
    )
    catalog_entries.append({"id": "D01", "title": "Contexto del configurador y actores", "type": "architecture", "source": "sources/D01-context.architecture.json", "status": "actual"})

    # D02 containers
    write(
        "D02-containers.architecture.json",
        arch(
            "D02",
            "D02 Contenedores: web, WASM, Tauri, almacenamiento",
            [
                {"id": "browser", "type": "frontend", "label": "Navegador", "sublabel": "Vite + React", "pos": [40, 120], "size": [140, 64]},
                {"id": "webview", "type": "frontend", "label": "WebView", "sublabel": "Tauri 2", "pos": [40, 280], "size": [140, 64]},
                {"id": "wasm", "type": "backend", "label": "archmaker-wasm", "sublabel": "wasm-bindgen", "pos": [280, 120], "size": [160, 64]},
                {"id": "tauri", "type": "backend", "label": "src-tauri", "sublabel": "Rust host", "pos": [280, 280], "size": [160, 64]},
                {"id": "core", "type": "backend", "label": "archmaker-core", "sublabel": "autoridad semántica", "pos": [520, 200], "size": [170, 64], "tag": "Rust"},
                {"id": "idb", "type": "database", "label": "IndexedDB", "sublabel": "drafts web", "pos": [280, 420], "size": [140, 60]},
                {"id": "fs", "type": "database", "label": "Filesystem", "sublabel": "store_dir", "pos": [520, 360], "size": [140, 60]},
                {"id": "ls", "type": "database", "label": "localStorage", "sublabel": "lastDraftId", "pos": [40, 420], "size": [140, 60]},
            ],
            [
                {"id": "b-w", "from": "browser", "to": "wasm", "label": "WasmCorePort", "variant": "emphasis"},
                {"id": "v-t", "from": "webview", "to": "tauri", "label": "invoke", "variant": "emphasis"},
                {"id": "w-c", "from": "wasm", "to": "core", "label": "CorePort"},
                {"id": "t-c", "from": "tauri", "to": "core", "label": "RealCore"},
                {"id": "b-idb", "from": "browser", "to": "idb", "label": "draft-persistence", "variant": "dashed"},
                {"id": "b-ls", "from": "browser", "to": "ls", "label": "keys UI", "variant": "dashed"},
                {"id": "t-fs", "from": "tauri", "to": "fs", "label": "save/open", "variant": "dashed"},
            ],
            [
                {"dot": "emerald", "title": "Paridad", "items": ["Misma semántica CorePort en WASM y Tauri", "MockCorePort solo con ?mock=1 u opt-in explícito"]},
                {"dot": "cyan", "title": "Almacenamiento", "items": ["Web: IndexedDB + localStorage", "Desktop: filesystem vía store_dir; sin SQL"]},
            ],
            [{"kind": "region", "label": "Producto shipped", "wraps": ["browser", "webview", "wasm", "tauri", "core", "idb", "fs", "ls"]}],
        ),
    )
    catalog_entries.append({"id": "D02", "title": "Contenedores web/WASM/Tauri/almacenamiento", "type": "architecture", "source": "sources/D02-containers.architecture.json", "status": "actual"})

    # D03 core components
    write(
        "D03-core-components.architecture.json",
        arch(
            "D03",
            "D03 Componentes del core Rust (as-built)",
            [
                {"id": "domain", "type": "backend", "label": "archmaker-domain", "sublabel": "tipos", "pos": [40, 200], "size": [150, 60]},
                {"id": "canon", "type": "backend", "label": "canonicalization", "sublabel": "bytes canónicos", "pos": [220, 80], "size": [160, 60]},
                {"id": "rules", "type": "backend", "label": "archmaker-rules", "sublabel": "reglas", "pos": [220, 200], "size": [150, 60]},
                {"id": "res", "type": "backend", "label": "resolution", "sublabel": "resolver", "pos": [220, 320], "size": [150, 60]},
                {"id": "core", "type": "backend", "label": "archmaker-core", "sublabel": "facade + RealCore", "pos": [440, 200], "size": [170, 64], "tag": "port"},
                {"id": "wasm", "type": "cloud", "label": "archmaker-wasm", "sublabel": "bindings", "pos": [680, 120], "size": [150, 60]},
                {"id": "tauri", "type": "cloud", "label": "src-tauri", "sublabel": "desktop host", "pos": [680, 280], "size": [150, 60]},
            ],
            [
                {"id": "d-core", "from": "domain", "to": "core"},
                {"id": "c-core", "from": "canon", "to": "core"},
                {"id": "r-core", "from": "rules", "to": "core"},
                {"id": "res-core", "from": "res", "to": "core"},
                {"id": "core-w", "from": "core", "to": "wasm", "label": "compile"},
                {"id": "core-t", "from": "core", "to": "tauri", "label": "link"},
            ],
            [
                {"dot": "rose", "title": "No dibujar", "items": ["Crates del module-map aspiracional (api, runner, …) no existen en Cargo.toml", "TypeScript no duplica reglas de dominio"]},
            ],
        ),
    )
    catalog_entries.append({"id": "D03", "title": "Componentes del core Rust", "type": "architecture", "source": "sources/D03-core-components.architecture.json", "status": "actual"})

    # D04 frontend adapters
    write(
        "D04-frontend-coreport.architecture.json",
        arch(
            "D04",
            "D04 Frontend y adaptadores CorePort",
            [
                {"id": "app", "type": "frontend", "label": "App.tsx", "sublabel": "Forge face", "pos": [40, 220], "size": [140, 64]},
                {"id": "face", "type": "frontend", "label": "forge-face", "sublabel": "UI state", "pos": [40, 80], "size": [140, 60]},
                {"id": "create", "type": "backend", "label": "create-core-port", "sublabel": "factory", "pos": [260, 220], "size": [160, 64]},
                {"id": "tauri_p", "type": "backend", "label": "TauriCorePort", "sublabel": "invoke", "pos": [500, 120], "size": [150, 60]},
                {"id": "wasm_p", "type": "backend", "label": "WasmCorePort", "sublabel": "wasm_invoke", "pos": [500, 220], "size": [150, 60]},
                {"id": "mock_p", "type": "backend", "label": "MockCorePort", "sublabel": "opt-in", "pos": [500, 320], "size": [150, 60], "tag": "?mock=1"},
                {"id": "ts", "type": "backend", "label": "coreport.ts", "sublabel": "tipos TS", "pos": [260, 360], "size": [160, 60]},
            ],
            [
                {"id": "a-c", "from": "app", "to": "create", "variant": "emphasis"},
                {"id": "f-a", "from": "face", "to": "app"},
                {"id": "c-t", "from": "create", "to": "tauri_p", "label": "desktop"},
                {"id": "c-w", "from": "create", "to": "wasm_p", "label": "browser"},
                {"id": "c-m", "from": "create", "to": "mock_p", "label": "mock", "variant": "dashed"},
                {"id": "ts-all", "from": "ts", "to": "create", "variant": "dashed"},
            ],
            [{"dot": "emerald", "title": "Arranque", "items": ["Desktop → Tauri", "Browser → WASM", "Mock nunca por fallo silencioso"]}],
        ),
    )
    catalog_entries.append({"id": "D04", "title": "Frontend y adaptadores CorePort", "type": "architecture", "source": "sources/D04-frontend-coreport.architecture.json", "status": "actual"})

    # D05 desktop deploy
    write(
        "D05-desktop-linux.architecture.json",
        arch(
            "D05",
            "D05 Despliegue desktop Linux (AppImage Forge)",
            [
                {"id": "user", "type": "external", "label": "Usuario Linux", "sublabel": "x86_64", "pos": [40, 220], "size": [140, 60]},
                {"id": "appimage", "type": "cloud", "label": "Forge AppImage", "sublabel": "bundle", "pos": [260, 220], "size": [170, 64], "tag": "v0.1.0"},
                {"id": "webview", "type": "frontend", "label": "WebView", "sublabel": "UI", "pos": [500, 120], "size": [140, 60]},
                {"id": "host", "type": "backend", "label": "Proceso Rust", "sublabel": "src-tauri", "pos": [500, 280], "size": [150, 60]},
                {"id": "portal", "type": "security", "label": "rfd / portal", "sublabel": "save dialog", "pos": [720, 280], "size": [150, 60]},
                {"id": "fs", "type": "database", "label": "Filesystem", "sublabel": "export + drafts", "pos": [720, 120], "size": [150, 60]},
            ],
            [
                {"id": "u-a", "from": "user", "to": "appimage", "variant": "emphasis"},
                {"id": "a-w", "from": "appimage", "to": "webview"},
                {"id": "a-h", "from": "appimage", "to": "host"},
                {"id": "h-p", "from": "host", "to": "portal", "label": "save_export"},
                {"id": "h-f", "from": "host", "to": "fs"},
                {"id": "w-h", "from": "webview", "to": "host", "label": "IPC invoke", "variant": "security"},
            ],
            [
                {"dot": "amber", "title": "Residuales", "items": ["Firma cosign local y 3× GUI pendientes en F9", "Humo portal nativo aún no es E2E de aceptación"]},
            ],
        ),
    )
    catalog_entries.append({"id": "D05", "title": "Despliegue desktop Linux", "type": "architecture", "source": "sources/D05-desktop-linux.architecture.json", "status": "actual"})

    # D06 web wasm
    write(
        "D06-web-wasm.architecture.json",
        arch(
            "D06",
            "D06 Despliegue web y carga WASM",
            [
                {"id": "browser", "type": "frontend", "label": "Navegador", "pos": [40, 220], "size": [130, 60]},
                {"id": "vite", "type": "frontend", "label": "Vite app", "sublabel": "apps/web", "pos": [240, 220], "size": [140, 60]},
                {"id": "pkg", "type": "backend", "label": "pkg/", "sublabel": "js+wasm+d.ts", "pos": [440, 120], "size": [150, 60]},
                {"id": "worker", "type": "backend", "label": "WasmCorePort", "sublabel": "init + invoke", "pos": [440, 280], "size": [150, 60]},
                {"id": "core", "type": "backend", "label": "core (wasm32)", "pos": [660, 200], "size": [150, 60]},
                {"id": "idb", "type": "database", "label": "IndexedDB", "pos": [240, 380], "size": [140, 60]},
            ],
            [
                {"id": "b-v", "from": "browser", "to": "vite", "variant": "emphasis"},
                {"id": "v-p", "from": "vite", "to": "pkg", "label": "import"},
                {"id": "v-w", "from": "vite", "to": "worker"},
                {"id": "w-c", "from": "worker", "to": "core"},
                {"id": "v-i", "from": "vite", "to": "idb", "variant": "dashed"},
            ],
            [{"dot": "cyan", "title": "Build", "items": ["tools/build-wasm.sh → wasm-bindgen --target web", "Paridad nativo/WASM en tests"]}],
        ),
    )
    catalog_entries.append({"id": "D06", "title": "Despliegue web y carga WASM", "type": "architecture", "source": "sources/D06-web-wasm.architecture.json", "status": "actual"})

    def seq(diagram_id: str, title: str, participants: list, messages: list, cards: list) -> dict:
        ys = [180 + i * 42 for i in range(len(messages))]
        msgs = []
        for i, m in enumerate(messages):
            item = {"from": m[0], "to": m[1], "y": ys[i], "label": m[2]}
            if len(m) > 3:
                item["variant"] = m[3]
            msgs.append(item)
        height = max(480, 220 + len(messages) * 48)
        return {
            "schema_version": 1,
            "diagram_type": "sequence",
            "meta": {
                "title": title,
                "output": f"rendered/{diagram_id}.html",
                "quality_profile": "showcase",
                "animation": "trace",
                "viewBox": [900, height],
            },
            "participants": participants,
            "messages": msgs,
            "cards": cards,
        }

    write(
        "D07-init.sequence.json",
        seq(
            "D07",
            "D07 Inicialización, catálogo y creación de perfil",
            [
                {"id": "ui", "type": "frontend", "label": "UI", "sublabel": "App"},
                {"id": "factory", "type": "backend", "label": "create-core-port"},
                {"id": "core", "type": "backend", "label": "CorePort"},
            ],
            [
                ("ui", "factory", "boot adapter", "emphasis"),
                ("factory", "core", "loadCatalog / createDraft", "emphasis"),
                ("core", "ui", "Draft + CatalogRef", "return"),
            ],
            [{"dot": "cyan", "title": "Inicio", "items": ["Adapter según entorno", "Catálogo embebido / producto"]}],
        ),
    )
    catalog_entries.append({"id": "D07", "title": "Inicialización, catálogo y perfil", "type": "sequence", "source": "sources/D07-init.sequence.json", "status": "actual"})

    write(
        "D08-resolve-validate-manifest.sequence.json",
        seq(
            "D08",
            "D08 Resolver → validar → construir manifiesto",
            [
                {"id": "ui", "type": "frontend", "label": "UI"},
                {"id": "core", "type": "backend", "label": "RealCore"},
            ],
            [
                ("ui", "core", "resolveDraft", "emphasis"),
                ("core", "ui", "ResolveResult + resolutionDigest", "return"),
                ("ui", "core", "validateDraft", "emphasis"),
                ("core", "ui", "ValidationResult", "return"),
                ("ui", "core", "buildManifest", "emphasis"),
                ("core", "ui", "Manifest + contentDigest", "return"),
            ],
            [{"dot": "emerald", "title": "Pipeline", "items": ["Autoridad en Rust", "Invalidación si el draft cambia"]}],
        ),
    )
    catalog_entries.append({"id": "D08", "title": "Resolver → validar → manifiesto", "type": "sequence", "source": "sources/D08-resolve-validate-manifest.sequence.json", "status": "actual"})

    write(
        "D09-export-web.sequence.json",
        seq(
            "D09",
            "D09 Exportación web y digest",
            [
                {"id": "ui", "type": "frontend", "label": "UI"},
                {"id": "core", "type": "backend", "label": "Core"},
                {"id": "browser", "type": "external", "label": "Browser download"},
            ],
            [
                ("ui", "core", "exportArtifact", "emphasis"),
                ("core", "ui", "bytes + binaryDigest", "return"),
                ("ui", "browser", "download blob", "dashed"),
            ],
            [{"dot": "cyan", "title": "Digests", "items": ["contentDigest ≠ binaryDigest", "Comprobación en tests de fidelidad"]}],
        ),
    )
    catalog_entries.append({"id": "D09", "title": "Exportación web y digest", "type": "sequence", "source": "sources/D09-export-web.sequence.json", "status": "actual"})

    write(
        "D10-desktop-save.sequence.json",
        seq(
            "D10",
            "D10 Guardado desktop y cancelación",
            [
                {"id": "ui", "type": "frontend", "label": "WebView"},
                {"id": "host", "type": "backend", "label": "save_export"},
                {"id": "portal", "type": "security", "label": "rfd"},
                {"id": "fs", "type": "database", "label": "FS"},
            ],
            [
                ("ui", "host", "saveExport", "emphasis"),
                ("host", "portal", "pick path", "security"),
                ("portal", "host", "Cancelled | Path", "return"),
                ("host", "fs", "write if accepted", "dashed"),
                ("host", "ui", "SaveExportResult", "return"),
            ],
            [{"dot": "amber", "title": "Cancelación", "items": ["Cancelar no escribe", "Sobrescritura explícita"]}],
        ),
    )
    catalog_entries.append({"id": "D10", "title": "Guardado desktop y cancelación", "type": "sequence", "source": "sources/D10-desktop-save.sequence.json", "status": "actual"})

    write(
        "D11-import-consent.sequence.json",
        seq(
            "D11",
            "D11 Importación con informe y consentimiento",
            [
                {"id": "ui", "type": "frontend", "label": "UI"},
                {"id": "import", "type": "backend", "label": "consented-import"},
                {"id": "core", "type": "backend", "label": "importDraft"},
            ],
            [
                ("ui", "import", "parse + report", "emphasis"),
                ("import", "ui", "diff / warnings", "return"),
                ("ui", "import", "user consents", "security"),
                ("import", "core", "importDraft", "emphasis"),
                ("core", "ui", "Draft migrated", "return"),
            ],
            [{"dot": "rose", "title": "Consentimiento", "items": ["Sin aceptación no se aplica", "Informe visible antes"]}],
        ),
    )
    catalog_entries.append({"id": "D11", "title": "Importación con consentimiento", "type": "sequence", "source": "sources/D11-import-consent.sequence.json", "status": "actual"})

    write(
        "D12-persistence.sequence.json",
        seq(
            "D12",
            "D12 Guardado, revisión y recuperación",
            [
                {"id": "ui", "type": "frontend", "label": "UI"},
                {"id": "core", "type": "backend", "label": "Core"},
                {"id": "store", "type": "database", "label": "IDB / FS"},
            ],
            [
                ("ui", "core", "saveDraft", "emphasis"),
                ("core", "store", "persist", "dashed"),
                ("ui", "core", "openDraft", "emphasis"),
                ("core", "store", "read", "dashed"),
                ("core", "ui", "Draft", "return"),
            ],
            [{"dot": "emerald", "title": "Persistencia", "items": ["Desktop FS", "Web IndexedDB", "Sin tablas SQL"]}],
        ),
    )
    catalog_entries.append({"id": "D12", "title": "Persistencia y recuperación", "type": "sequence", "source": "sources/D12-persistence.sequence.json", "status": "actual"})

    def life(diagram_id: str, title: str, states: list, transitions: list, main: list, cards: list) -> dict:
        return {
            "schema_version": 3,
            "diagram_type": "lifecycle",
            "meta": {
                "title": title,
                "output": f"rendered/{diagram_id}.html",
                "quality_profile": "showcase",
                "animation": "trace",
            },
            "mainPath": main,
            "states": states,
            "transitions": transitions,
            "cards": cards,
        }

    write(
        "D13-draft-lifecycle.lifecycle.json",
        life(
            "D13",
            "D13 Ciclo de vida del draft",
            [
                {"id": "empty", "type": "start", "label": "Empty", "step": "01"},
                {"id": "editing", "type": "active", "label": "Editing", "step": "02"},
                {"id": "dirty", "type": "waiting", "label": "Dirty", "step": "03"},
                {"id": "saved", "type": "success", "label": "Saved", "step": "04"},
                {"id": "migrating", "type": "active", "label": "Migrating", "tag": "import"},
            ],
            [
                {"id": "e-ed", "from": "empty", "to": "editing", "label": "createDraft"},
                {"id": "ed-d", "from": "editing", "to": "dirty", "label": "change"},
                {"id": "d-s", "from": "dirty", "to": "saved", "label": "saveDraft"},
                {"id": "s-ed", "from": "saved", "to": "editing", "label": "reopen"},
                {"id": "m-ed", "from": "migrating", "to": "editing", "label": "consent ok"},
            ],
            ["empty", "editing", "dirty", "saved"],
            [{"dot": "cyan", "title": "Draft", "items": ["Estados según lifecycles.md", "Migración solo con consentimiento"]}],
        ),
    )
    catalog_entries.append({"id": "D13", "title": "Ciclo de vida del draft", "type": "lifecycle", "source": "sources/D13-draft-lifecycle.lifecycle.json", "status": "actual"})

    write(
        "D14-prepare-export.lifecycle.json",
        life(
            "D14",
            "D14 Preparación y exportación con invalidación",
            [
                {"id": "idle", "type": "start", "label": "Idle", "step": "01"},
                {"id": "resolving", "type": "active", "label": "Resolving", "step": "02"},
                {"id": "prepared", "type": "success", "label": "Prepared", "step": "03"},
                {"id": "exporting", "type": "active", "label": "Exporting", "step": "04"},
                {"id": "stale", "type": "failure", "label": "Stale", "tag": "invalidate"},
            ],
            [
                {"id": "i-r", "from": "idle", "to": "resolving", "label": "prepare"},
                {"id": "r-p", "from": "resolving", "to": "prepared", "label": "ok"},
                {"id": "p-e", "from": "prepared", "to": "exporting", "label": "export"},
                {"id": "e-i", "from": "exporting", "to": "idle", "label": "done"},
                {"id": "p-s", "from": "prepared", "to": "stale", "label": "draft changed", "variant": "security"},
                {"id": "s-r", "from": "stale", "to": "resolving", "label": "re-prepare"},
            ],
            ["idle", "resolving", "prepared", "exporting"],
            [{"dot": "amber", "title": "Invalidación", "items": ["isPreparedCurrent en App.tsx", "No reutilizar manifiesto obsoleto"]}],
        ),
    )
    catalog_entries.append({"id": "D14", "title": "Preparación/exportación e invalidación", "type": "lifecycle", "source": "sources/D14-prepare-export.lifecycle.json", "status": "actual"})

    write(
        "D15-import-lifecycle.lifecycle.json",
        life(
            "D15",
            "D15 Importación: análisis, aceptación y fallo",
            [
                {"id": "idle", "type": "start", "label": "Idle", "step": "01"},
                {"id": "analyzing", "type": "active", "label": "Analyzing", "step": "02"},
                {"id": "awaiting", "type": "waiting", "label": "Await consent", "step": "03"},
                {"id": "applied", "type": "success", "label": "Applied", "step": "04"},
                {"id": "rejected", "type": "failure", "label": "Rejected"},
            ],
            [
                {"id": "i-a", "from": "idle", "to": "analyzing", "label": "file"},
                {"id": "a-w", "from": "analyzing", "to": "awaiting", "label": "report"},
                {"id": "w-ap", "from": "awaiting", "to": "applied", "label": "accept"},
                {"id": "w-r", "from": "awaiting", "to": "rejected", "label": "deny", "variant": "security"},
                {"id": "a-r", "from": "analyzing", "to": "rejected", "label": "invalid", "variant": "security"},
            ],
            ["idle", "analyzing", "awaiting", "applied"],
            [{"dot": "rose", "title": "Fallo", "items": ["Rechazo deja el perfil previo", "Sin consentimiento no hay write"]}],
        ),
    )
    catalog_entries.append({"id": "D15", "title": "Importación lifecycle", "type": "lifecycle", "source": "sources/D15-import-lifecycle.lifecycle.json", "status": "actual"})

    write(
        "D16-domain-coreport.class.json",
        {
            "schema_version": 1,
            "diagram_type": "class",
            "meta": {
                "title": "D16 Dominio, DTO y CorePort",
                "subtitle": "9 operaciones reales en port.rs; TypeScript mirror en coreport.ts",
                "output": "rendered/D16.html",
                "quality_profile": "showcase",
            },
            "layout": {"mode": "grid"},
            "types": [
                {
                    "id": "coreport",
                    "label": "CorePort",
                    "kind": "interface",
                    "row": 0,
                    "col": 1,
                    "methods": [
                        {"name": "createDraft", "visibility": "public", "abstract": True},
                        {"name": "saveDraft", "visibility": "public", "abstract": True},
                        {"name": "openDraft", "visibility": "public", "abstract": True},
                        {"name": "importDraft", "visibility": "public", "abstract": True},
                        {"name": "resolveDraft", "visibility": "public", "abstract": True},
                        {"name": "validateDraft", "visibility": "public", "abstract": True},
                        {"name": "buildManifest", "visibility": "public", "abstract": True},
                        {"name": "exportArtifact", "visibility": "public", "abstract": True},
                        {"name": "loadCatalog", "visibility": "public", "abstract": True},
                    ],
                },
                {
                    "id": "draft",
                    "label": "Draft",
                    "kind": "class",
                    "row": 1,
                    "col": 0,
                    "attributes": [
                        {"name": "id", "type": "Uuid", "visibility": "public"},
                        {"name": "selections", "type": "Selection[]", "visibility": "public"},
                        {"name": "contentDigest", "type": "ContentDigest?", "visibility": "public"},
                    ],
                },
                {
                    "id": "manifest",
                    "label": "Manifest",
                    "kind": "class",
                    "row": 1,
                    "col": 2,
                    "attributes": [
                        {"name": "contentDigest", "type": "ContentDigest", "visibility": "public"},
                        {"name": "resolutionDigest", "type": "ContentDigest", "visibility": "public"},
                    ],
                },
                {
                    "id": "realcore",
                    "label": "RealCore",
                    "kind": "class",
                    "row": 0,
                    "col": 0,
                    "methods": [{"name": "implements CorePort", "visibility": "public"}],
                },
            ],
            "relationships": [
                {"from": "realcore", "to": "coreport", "kind": "realization"},
                {"from": "coreport", "to": "draft", "kind": "association", "label": "I/O"},
                {"from": "coreport", "to": "manifest", "kind": "association", "label": "build"},
            ],
        },
    )
    catalog_entries.append({"id": "D16", "title": "Tipos dominio/DTO/CorePort", "type": "class", "source": "sources/D16-domain-coreport.class.json", "status": "actual"})

    def dataflow(diagram_id: str, title: str, stages: list, nodes: list, flows: list, cards: list) -> dict:
        return {
            "schema_version": 1,
            "diagram_type": "dataflow",
            "meta": {
                "title": title,
                "output": f"rendered/{diagram_id}.html",
                "quality_profile": "showcase",
                "animation": "trace",
                "viewBox": [1100, 720],
            },
            "stages": [{"label": s} for s in stages],
            "nodes": nodes,
            "flows": flows,
            "cards": cards,
        }

    write(
        "D17-profile-persist.dataflow.json",
        dataflow(
            "D17",
            "D17 Flujo de datos del perfil y persistencia",
            ["UI", "Adapter", "Core", "Store"],
            [
                {"id": "face", "type": "frontend", "label": "forge-face", "stage": 0, "row": 1},
                {"id": "draft_map", "type": "frontend", "label": "draft-for-core", "stage": 0, "row": 3},
                {"id": "port", "type": "backend", "label": "CorePort adapter", "stage": 1, "row": 2},
                {"id": "core", "type": "backend", "label": "RealCore", "stage": 2, "row": 2},
                {"id": "idb", "type": "database", "label": "IndexedDB", "stage": 3, "row": 1},
                {"id": "fs", "type": "database", "label": "Filesystem", "stage": 3, "row": 3},
            ],
            [
                {
                    "from": "face",
                    "to": "draft_map",
                    "label": "map",
                    "variant": "emphasis",
                    "route": "straight",
                    "labelDy": 28,
                },
                {"from": "draft_map", "to": "port", "label": "DTO", "variant": "emphasis", "route": "straight"},
                {"from": "port", "to": "core", "label": "ops", "variant": "emphasis", "route": "straight"},
                {"from": "core", "to": "idb", "label": "web", "variant": "dashed", "route": "vertical-channel"},
                {"from": "core", "to": "fs", "label": "desktop", "variant": "dashed", "route": "vertical-channel"},
            ],
            [{"dot": "cyan", "title": "Autoridad", "items": ["Rust decide semántica", "TS solo presenta y adapta"]}],
        ),
    )
    catalog_entries.append({"id": "D17", "title": "Flujo de datos del perfil", "type": "dataflow", "source": "sources/D17-profile-persist.dataflow.json", "status": "actual"})

    write(
        "D18-canonicalization.dataflow.json",
        dataflow(
            "D18",
            "D18 Canonicalización y digests",
            ["Draft", "Canon", "Resolve", "Artifact"],
            [
                {"id": "draft", "type": "frontend", "label": "Draft JSON", "stage": 0, "row": 2},
                {"id": "canon", "type": "backend", "label": "canonicalization", "stage": 1, "row": 2, "tag": "contentDigest"},
                {"id": "res", "type": "backend", "label": "resolution", "stage": 2, "row": 1, "tag": "resolutionDigest"},
                {"id": "man", "type": "backend", "label": "Manifest", "stage": 2, "row": 3},
                {"id": "bin", "type": "cloud", "label": "Rendered bytes", "stage": 3, "row": 2, "tag": "binaryDigest"},
            ],
            [
                {"from": "draft", "to": "canon", "label": "canonicalize", "variant": "emphasis", "route": "straight"},
                {"from": "canon", "to": "res", "label": "resolve", "route": "vertical-channel"},
                {"from": "canon", "to": "man", "label": "seal", "route": "vertical-channel"},
                {"from": "man", "to": "bin", "label": "render", "variant": "emphasis", "route": "straight"},
            ],
            [
                {
                    "dot": "rose",
                    "title": "Separar digests",
                    "items": [
                        "contentDigest: contenido canónico",
                        "resolutionDigest: resultado de resolución",
                        "binaryDigest: bytes entregados",
                    ],
                }
            ],
        ),
    )
    catalog_entries.append({"id": "D18", "title": "Canonicalización y digests", "type": "dataflow", "source": "sources/D18-canonicalization.dataflow.json", "status": "actual"})

    write(
        "D19-trust-ipc.dataflow.json",
        dataflow(
            "D19",
            "D19 Fronteras de confianza, IPC y destinos",
            ["Untrusted UI", "IPC", "Privileged host", "Destinations"],
            [
                {"id": "ui", "type": "frontend", "label": "WebView / Browser", "stage": 0, "row": 2},
                {"id": "ipc", "type": "security", "label": "Tauri invoke / WASM", "stage": 1, "row": 2, "tag": "deny-by-default"},
                {"id": "host", "type": "backend", "label": "Rust host", "stage": 2, "row": 2},
                {"id": "fs", "type": "database", "label": "User FS", "stage": 3, "row": 1},
                {"id": "dl", "type": "external", "label": "Browser download", "stage": 3, "row": 3},
                {"id": "runner", "type": "external", "label": "Runner future", "stage": 3, "row": 4, "tag": "not shipped"},
            ],
            [
                {"from": "ui", "to": "ipc", "label": "typed ops", "variant": "security", "route": "straight"},
                {"from": "ipc", "to": "host", "label": "gate", "variant": "emphasis", "route": "straight"},
                {"from": "host", "to": "fs", "label": "write", "route": "vertical-channel"},
                {"from": "host", "to": "dl", "label": "blob", "route": "vertical-channel"},
                {
                    "from": "host",
                    "to": "runner",
                    "label": "none",
                    "variant": "dashed",
                    "route": "bottom-channel",
                    "labelDy": 24,
                },
            ],
            [{"dot": "rose", "title": "Confianza", "items": ["Capabilities mínimas", "Sin scripts arbitrarios", "Runner no implementado"]}],
        ),
    )
    catalog_entries.append({"id": "D19", "title": "Fronteras de confianza e IPC", "type": "dataflow", "source": "sources/D19-trust-ipc.dataflow.json", "status": "actual"})

    def workflow(diagram_id: str, title: str, lanes: list, nodes: list, edges: list, main: list, cards: list) -> dict:
        return {
            "schema_version": 1,
            "diagram_type": "workflow",
            "meta": {
                "title": title,
                "output": f"rendered/{diagram_id}.html",
                "quality_profile": "showcase",
                "animation": "trace",
            },
            "lanes": lanes,
            "mainPath": main,
            "nodes": nodes,
            "edges": edges,
            "cards": cards,
        }

    write(
        "D20-ci-pipeline.workflow.json",
        workflow(
            "D20",
            "D20 Pipeline de CI y validadores",
            [
                {"id": "dev", "label": "Developer"},
                {"id": "ci", "label": "GitHub Actions"},
                {"id": "gates", "label": "Validators"},
            ],
            [
                {"id": "push", "lane": "dev", "col": 0, "type": "frontend", "label": "Push"},
                {"id": "ci_job", "lane": "ci", "col": 1, "type": "backend", "label": "ci.yml"},
                {"id": "rust", "lane": "gates", "col": 2, "type": "backend", "label": "cargo"},
                {"id": "web", "lane": "ci", "col": 3, "type": "frontend", "label": "pnpm"},
                {"id": "f7", "lane": "gates", "col": 4, "type": "security", "label": "F7"},
                {"id": "docs", "lane": "gates", "col": 5, "type": "external", "label": "docs"},
            ],
            [
                {"from": "push", "to": "ci_job"},
                {"from": "ci_job", "to": "rust", "route": "drop", "fromSide": "bottom", "toSide": "top"},
                {"from": "rust", "to": "web"},
                {"from": "web", "to": "f7"},
                {"from": "f7", "to": "docs"},
            ],
            ["push", "ci_job", "rust", "web", "f7", "docs"],
            [{"dot": "emerald", "title": "CI", "items": ["--strict F7 es evidencia local, no job CI", "Verde CI ≠ RC_SHA_FUNCIONAL"]}],
        ),
    )
    catalog_entries.append({"id": "D20", "title": "Pipeline CI y validadores", "type": "workflow", "source": "sources/D20-ci-pipeline.workflow.json", "status": "actual"})

    write(
        "D21-release-sign.workflow.json",
        workflow(
            "D21",
            "D21 Build, firma, publicación y verificación",
            [
                {"id": "build", "label": "Build"},
                {"id": "sign", "label": "Sigstore"},
                {"id": "publish", "label": "GitHub Release"},
                {"id": "verify", "label": "Verifier"},
            ],
            [
                {"id": "pkg", "lane": "build", "col": 0, "type": "backend", "label": "AppImage"},
                {"id": "sbom", "lane": "build", "col": 1, "type": "backend", "label": "SBOM"},
                {"id": "cosign", "lane": "sign", "col": 2, "type": "security", "label": "cosign"},
                {"id": "rel", "lane": "publish", "col": 3, "type": "cloud", "label": "Release"},
                {"id": "local", "lane": "verify", "col": 4, "type": "external", "label": "verify", "width": 110},
            ],
            [
                {"from": "pkg", "to": "sbom"},
                {"from": "sbom", "to": "cosign", "route": "drop", "fromSide": "bottom", "toSide": "top"},
                {"from": "cosign", "to": "rel", "route": "drop", "fromSide": "bottom", "toSide": "top"},
                {"from": "rel", "to": "local", "route": "drop", "fromSide": "bottom", "toSide": "top"},
            ],
            ["pkg", "sbom", "cosign", "rel", "local"],
            [
                {
                    "dot": "amber",
                    "title": "v0.1.0",
                    "items": [
                        "RC_SHA 9297ed7… inamovible",
                        "Verificación parcial: cosign local y 3× GUI pendientes",
                    ],
                }
            ],
        ),
    )
    catalog_entries.append({"id": "D21", "title": "Build firma publicación", "type": "workflow", "source": "sources/D21-release-sign.workflow.json", "status": "actual"})

    write(
        "D22-f0-f9.workflow.json",
        workflow(
            "D22",
            "D22 Aceptación F0–F9 y aprobaciones",
            [
                {"id": "code", "label": "Código"},
                {"id": "func", "label": "Funcional F7"},
                {"id": "dist", "label": "Distribución F9"},
            ],
            [
                {"id": "f0", "lane": "code", "col": 0, "type": "backend", "label": "F0–F6"},
                {"id": "f7", "lane": "func", "col": 1, "type": "security", "label": "F7"},
                {"id": "f8", "lane": "dist", "col": 2, "type": "backend", "label": "F8"},
                {"id": "f9", "lane": "dist", "col": 3, "type": "cloud", "label": "F9"},
                {"id": "approve", "lane": "func", "col": 4, "type": "security", "label": "Approve", "tag": "pending"},
            ],
            [
                {"from": "f0", "to": "f7", "route": "drop", "fromSide": "bottom", "toSide": "top"},
                {"from": "f7", "to": "approve", "label": "review", "variant": "dashed"},
                {"from": "f0", "to": "f8"},
                {"from": "f8", "to": "f9"},
            ],
            ["f0", "f7", "approve"],
            [
                {
                    "dot": "rose",
                    "title": "Tres capas",
                    "items": [
                        "Baseline de codigo, aprobacion funcional y distribucion son decisiones distintas",
                        "Runner no declara result=passed",
                    ],
                }
            ],
        ),
    )
    catalog_entries.append({"id": "D22", "title": "Aceptación F0–F9", "type": "workflow", "source": "sources/D22-f0-f9.workflow.json", "status": "actual"})

    write(
        "D23-crates-tree.tree.json",
        {
            "schema_version": 1,
            "diagram_type": "tree",
            "meta": {
                "title": "D23 Árbol de crates, apps y documentación",
                "subtitle": "Contención as-built; enlaces no son llamadas runtime",
                "output": "rendered/D23.html",
                "quality_profile": "showcase",
            },
            "nodes": [
                {"id": "repo", "label": "archmaker", "sublabel": "workspace"},
                {"id": "crates", "label": "crates/", "parent": "repo"},
                {"id": "domain", "label": "archmaker-domain", "parent": "crates"},
                {"id": "canon", "label": "archmaker-canonicalization", "parent": "crates"},
                {"id": "rules", "label": "archmaker-rules", "parent": "crates"},
                {"id": "res", "label": "archmaker-resolution", "parent": "crates"},
                {"id": "core", "label": "archmaker-core", "parent": "crates"},
                {"id": "wasm", "label": "archmaker-wasm", "parent": "crates"},
                {"id": "tauri", "label": "src-tauri", "parent": "repo"},
                {"id": "apps", "label": "apps/web", "parent": "repo"},
                {"id": "docs", "label": "docs/", "parent": "repo"},
                {"id": "c4", "label": "02-architecture/c4", "parent": "docs"},
                {"id": "diagrams", "label": "02-architecture/diagrams", "parent": "docs"},
                {"id": "closure", "label": "10-delivery/closure", "parent": "docs"},
            ],
        },
    )
    catalog_entries.append({"id": "D23", "title": "Árbol crates/apps/docs", "type": "tree", "source": "sources/D23-crates-tree.tree.json", "status": "actual"})

    write(
        "D24-baselines.timeline.json",
        {
            "schema_version": 1,
            "diagram_type": "timeline",
            "meta": {
                "title": "D24 Baselines y releases",
                "subtitle": "Hitos documentados; v0.1.0 inamovible",
                "timezone": "Europe/Madrid",
                "quality_profile": "showcase",
                "output": "rendered/D24.html",
                "evidence": "illustrative",
            },
            "lanes": [
                {"id": "baseline", "label": "Baselines"},
                {"id": "closure", "label": "Cierre"},
                {"id": "release", "label": "Release"},
            ],
            "events": [
                {
                    "id": "mvp5",
                    "at": "2026-09-01T00:00:00+02:00",
                    "title": "implementation-baseline-mvp5",
                    "detail": "Baseline vigente de código MVP-5",
                    "lane": "baseline",
                    "kind": "change",
                },
                {
                    "id": "f0f8",
                    "at": "2026-10-15T12:00:00+02:00",
                    "title": "Cierre F0–F8 en main",
                    "detail": "Documentación y matriz; aprobación funcional pending",
                    "lane": "closure",
                    "kind": "change",
                },
                {
                    "id": "v010",
                    "at": "2026-10-16T12:00:00+02:00",
                    "title": "Neubat v0.1.0",
                    "detail": "RC_SHA 9297ed7…; verificación parcial",
                    "lane": "release",
                    "kind": "action",
                },
                {
                    "id": "c909",
                    "at": "2026-10-16T22:47:00+02:00",
                    "title": "main c90919c",
                    "detail": "Corte revisado post PR #122",
                    "lane": "baseline",
                    "kind": "change",
                },
            ],
        },
    )
    catalog_entries.append({"id": "D24", "title": "Historial baselines y releases", "type": "timeline", "source": "sources/D24-baselines.timeline.json", "status": "actual"})

    for entry in catalog_entries:
        entry["scope"] = "forge-mvp-as-built"
        entry["sourceSha"] = source_sha
        entry["rendererSha"] = RENDERER_SHA
        entry["codeRefs"] = []
        entry["validation"] = {"status": "pending", "reviewer": None}

    catalog = {
        "schemaVersion": "1",
        "title": "Catálogo de diagramas Archify (Forge / ArchMaker)",
        "sourceSha": source_sha,
        "rendererSha": RENDERER_SHA,
        "rendererPin": f"$HOME/.local/share/archify/{RENDERER_SHA}",
        "requiredIds": [f"D{i:02d}" for i in range(1, 25)],
        "conditioned": {
            "D25": "Waterfall de rendimiento: solo con 30 muestras reales (ver F7-P95)",
            "D26": "ERD físico: no aplica; persistencia es FS/IndexedDB, no SQL",
        },
        "entries": catalog_entries,
    }
    DIAG.mkdir(parents=True, exist_ok=True)
    (DIAG / "catalog.json").write_text(
        json.dumps(catalog, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    print(f"wrote catalog with {len(catalog_entries)} entries @ {source_sha[:12]}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
