#!/usr/bin/env python3
"""P6 residual probe: 3× install limpia + offline (bwrap) sobre AppImage Forge.

No cierra portal E2E rfd (sigue blocked sin operador). No publica ni mueve tags.
Escribe docs/10-delivery/closure/examples/desktop-offline-p6-probe.json y
actualiza desktop-evidence.p6-blocked.json (o .passed si todos los casos
automatizables pasan y portal sigue ausente → status permanece blocked).
"""

from __future__ import annotations

import hashlib
import json
import os
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timezone
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
PROBE_PATH = ROOT / "docs/10-delivery/closure/examples/desktop-offline-p6-probe.json"
EVIDENCE_PATH = ROOT / "docs/10-delivery/closure/examples/desktop-evidence.p6-blocked.json"
DEFAULT_APPIMAGE = Path(
    "/home/alexendros/Aplicaciones/Repertorio/archmaker/"
    "target/release/bundle/appimage/Forge_0.1.0_amd64.AppImage"
)


def git_sha() -> str:
    return subprocess.check_output(
        ["git", "rev-parse", "HEAD"], cwd=ROOT, text=True
    ).strip()


def sha256_file(path: Path) -> str:
    h = hashlib.sha256()
    with path.open("rb") as fh:
        for chunk in iter(lambda: fh.read(1024 * 1024), b""):
            h.update(chunk)
    return h.hexdigest()


def launch_alive(appimage: Path, home: Path, *, offline: bool, seconds: float = 5.0) -> dict:
    """Arranca Forge y exige que siga vivo `seconds` (timeout(1) + process group).

    `/tmp` suele ser `noexec`: no usar `--appimage-extract-and-run` ahí.
    Online: FUSE AppImage. Offline: extract a HOME (exec) + bwrap --unshare-net.
    """
    env = os.environ.copy()
    env["HOME"] = str(home)
    env["XDG_DATA_HOME"] = str(home / ".local" / "share")
    env["XDG_CONFIG_HOME"] = str(home / ".config")
    env["XDG_CACHE_HOME"] = str(home / ".cache")
    env["TMPDIR"] = str(home / "tmp")
    (home / "tmp").mkdir(parents=True, exist_ok=True)
    (home / ".cache").mkdir(parents=True, exist_ok=True)
    inner: list[str]
    if offline:
        if not shutil.which("bwrap"):
            return {"ok": False, "error": "bwrap ausente"}
        extract_root = home / "squashfs-root"
        if not (extract_root / "AppRun").is_file():
            try:
                subprocess.run(
                    [str(appimage), "--appimage-extract"],
                    cwd=str(home),
                    env=env,
                    check=True,
                    stdout=subprocess.DEVNULL,
                    stderr=subprocess.PIPE,
                    text=True,
                    timeout=120,
                )
            except (OSError, subprocess.CalledProcessError, subprocess.TimeoutExpired) as exc:
                return {"ok": False, "error": f"extract: {exc}", "offline": True, "home": str(home)}
        apprun = extract_root / "AppRun"
        if not apprun.is_file():
            return {"ok": False, "error": "AppRun ausente tras extract", "offline": True}
        runtime = env.get("XDG_RUNTIME_DIR", f"/run/user/{os.getuid()}")
        # `/` ya incluye `home`; un bind extra a la misma ruta rompe mkdir parents.
        inner = [
            "bwrap",
            "--unshare-net",
            "--die-with-parent",
            "--bind",
            "/",
            "/",
            "--dev",
            "/dev",
            "--proc",
            "/proc",
            "--setenv",
            "HOME",
            str(home),
            "--setenv",
            "TMPDIR",
            str(home / "tmp"),
            "--setenv",
            "XDG_CACHE_HOME",
            str(home / ".cache"),
            "--setenv",
            "DISPLAY",
            env.get("DISPLAY", ""),
            "--setenv",
            "WAYLAND_DISPLAY",
            env.get("WAYLAND_DISPLAY", ""),
            "--setenv",
            "XDG_RUNTIME_DIR",
            runtime,
            str(apprun),
        ]
    else:
        # FUSE (evitar extract en /tmp noexec).
        inner = [str(appimage)]

    # timeout mata el grupo; exit 124 = aún vivo al cortar → éxito de humo.
    cmd = ["timeout", "--kill-after=3s", f"{seconds + 2:.0f}s", *inner]
    try:
        completed = subprocess.run(
            cmd,
            env=env,
            stdout=subprocess.DEVNULL,
            stderr=subprocess.PIPE,
            text=True,
            start_new_session=True,
            timeout=seconds + 15,
        )
    except subprocess.TimeoutExpired as exc:
        return {
            "ok": False,
            "error": f"timeout wrapper: {exc}",
            "offline": offline,
            "home": str(home),
        }
    except OSError as exc:
        return {"ok": False, "error": str(exc), "offline": offline, "home": str(home)}

    # 124 = timeout(1) cortó un proceso aún en ejecución → vivo el intervalo pedido.
    alive_ok = completed.returncode == 124
    return {
        "ok": alive_ok,
        "returncode": completed.returncode,
        "stderr_tail": (completed.stderr or "")[-500:],
        "offline": offline,
        "home": str(home),
    }


def main() -> int:
    appimage = Path(os.environ.get("FORGE_APPIMAGE", DEFAULT_APPIMAGE))
    if not appimage.is_file():
        print(f"FAIL falta AppImage: {appimage}", file=sys.stderr)
        return 1

    source = git_sha()
    digest = sha256_file(appimage)
    cases: dict[str, dict] = {}

    # Homes bajo $HOME (exec); /tmp del sistema es noexec.
    base = Path.home() / ".cache" / "archmaker-p6-probe"
    base.mkdir(parents=True, exist_ok=True)

    # 3× clean install (independent HOME)
    install_results = []
    for i in range(3):
        tmp = Path(tempfile.mkdtemp(prefix=f"forge-p6-i{i}-", dir=str(base)))
        home = tmp / "home"
        home.mkdir()
        result = launch_alive(appimage, home, offline=False, seconds=5.0)
        install_results.append(result)
        shutil.rmtree(tmp, ignore_errors=True)
    cases["clean-install-x3"] = {
        "status": "passed" if all(r.get("ok") for r in install_results) else "failed",
        "results": install_results,
    }

    # Offline via extract+bwrap --unshare-net
    tmp = Path(tempfile.mkdtemp(prefix="forge-p6-off-", dir=str(base)))
    home = tmp / "home"
    home.mkdir()
    off = launch_alive(appimage, home, offline=True, seconds=5.0)
    shutil.rmtree(tmp, ignore_errors=True)
    cases["boot-offline"] = {
        "status": "passed" if off.get("ok") else "failed",
        "result": off,
    }

    # Portal E2E: no automatizable sin sustituir rfd — blocked explícito
    cases["gui-dialog-real"] = {
        "status": "blocked",
        "reason": "E2E rfd/portal nativo requiere operador; no se sustituye por picker simulado",
    }
    cases["gui-cancel"] = {"status": "blocked", "reason": "depende de gui-dialog-real"}
    cases["gui-overwrite"] = {"status": "blocked", "reason": "depende de gui-dialog-real"}
    cases["gui-destination-noperms"] = {
        "status": "blocked",
        "reason": "depende de gui-dialog-real",
    }

    probe = {
        "schemaVersion": "1",
        "sourceSha": source,
        "capturedAt": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "appimage": {
            "path": str(appimage),
            "sha256": digest,
            "sizeBytes": appimage.stat().st_size,
            "published": False,
        },
        "cases": cases,
        "gateStatus": "blocked",
        "notes": [
            "3× install y offline (bwrap) automatizados sobre AppImage Forge local",
            "Portal E2E rfd permanece blocked (sin sustitución por Mock/picker)",
            "No se publica ni se mueve tags",
        ],
    }
    if (
        cases["clean-install-x3"]["status"] == "passed"
        and cases["boot-offline"]["status"] == "passed"
    ):
        probe["notes"].append(
            "Residuales automatizables passed; gate P6 sigue blocked por portal E2E"
        )

    PROBE_PATH.write_text(
        json.dumps(probe, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )
    probe_digest = hashlib.sha256(PROBE_PATH.read_bytes()).hexdigest()

    evidence = {
        "schemaVersion": "1",
        "caseId": "F7-OFFLINE-DESKTOP",
        "sourceSha": source,
        "status": "blocked",
        "capturedAt": probe["capturedAt"],
        "artifact": {
            "name": "desktop-offline-p6-probe.json",
            "sizeBytes": PROBE_PATH.stat().st_size,
            "sha256": probe_digest,
            "mediaType": "application/json",
        },
        "identity": {
            "host": os.uname().nodename,
            "operator": "archmaker-agent-p6-residuals",
        },
        "notes": [
            f"DISPLAY={os.environ.get('DISPLAY')} WAYLAND_DISPLAY={os.environ.get('WAYLAND_DISPLAY')}",
            f"AppImage Forge local sha256={digest} (no published)",
            f"clean-install-x3={cases['clean-install-x3']['status']}",
            f"boot-offline(bwrap)={cases['boot-offline']['status']}",
            "AUSENTE: E2E portal rfd cancelar/guardar/sobrescribir/destino sin permisos",
            "MockCorePort no se usa como cierre",
            "AppImage histórico v0.1.0 NO cierra este gate",
            "Estado=blocked: portal E2E ausente; no pending ambiguo",
        ],
    }
    EVIDENCE_PATH.write_text(
        json.dumps(evidence, indent=2, ensure_ascii=False) + "\n", encoding="utf-8"
    )

    print(json.dumps({"probe": str(PROBE_PATH), "cases": {
        k: v.get("status") for k, v in cases.items()
    }}, ensure_ascii=False, indent=2))
    if cases["clean-install-x3"]["status"] != "passed":
        print("FAIL clean-install-x3", file=sys.stderr)
        return 1
    if cases["boot-offline"]["status"] != "passed":
        print("FAIL boot-offline", file=sys.stderr)
        return 1
    print("OK residuales automatizables P6; portal E2E sigue blocked")
    return 0


if __name__ == "__main__":
    sys.exit(main())
