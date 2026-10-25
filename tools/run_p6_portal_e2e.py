#!/usr/bin/env python3
"""E2E del diálogo nativo de guardado (portal XDG / Nautilus).

Conduce la cara real de Forge: Generar → Exportar → diálogo
«Guardar perfil Forge». No usa MockCorePort ni el picker simulado.

Requiere DISPLAY y AT-SPI. `ARCHMAKER_SAVE_DIR` fija la carpeta inicial
del diálogo para no escribir en el home del operador.

Uso:
  python3 tools/run_p6_portal_e2e.py --binary target/debug/archmaker --out-dir "$EVIDENCE"
"""

from __future__ import annotations

import argparse
import hashlib
import json
import os
import signal
import subprocess
import time
from pathlib import Path

DIALOG_TITLE = "Guardar perfil Forge"
APP_NAME = "archmaker"


def fail(msg: str) -> None:
    raise SystemExit(f"FAIL {msg}")


def launch(binary: Path, home: Path, save_dir: Path, log_path: Path) -> subprocess.Popen[bytes]:
    home.mkdir(parents=True, exist_ok=True)
    env = os.environ.copy()
    env["HOME"] = str(home)
    env["XDG_CONFIG_HOME"] = str(home / ".config")
    env["XDG_DATA_HOME"] = str(home / ".local" / "share")
    env["XDG_CACHE_HOME"] = str(home / ".cache")
    env["ARCHMAKER_SAVE_DIR"] = str(save_dir)
    log_path.parent.mkdir(parents=True, exist_ok=True)
    log_file = log_path.open("wb")
    proc = subprocess.Popen(
        [str(binary)],
        env=env,
        stdout=subprocess.DEVNULL,
        stderr=log_file,
        start_new_session=True,
    )
    proc._log_file = log_file  # type: ignore[attr-defined]
    return proc


def stop(proc: subprocess.Popen[bytes], log_path: Path) -> bytes:
    if proc.poll() is None:
        try:
            os.killpg(proc.pid, signal.SIGTERM)
        except ProcessLookupError:
            pass
    try:
        proc.wait(timeout=5)
    except subprocess.TimeoutExpired:
        try:
            os.killpg(proc.pid, signal.SIGKILL)
        except ProcessLookupError:
            pass
        proc.wait(timeout=5)
    log_file = getattr(proc, "_log_file", None)
    if log_file is not None:
        log_file.close()
    if not log_path.is_file():
        return b""
    return log_path.read_bytes()


class Desktop:
    def __init__(self) -> None:
        import gi

        gi.require_version("Atspi", "2.0")
        from gi.repository import Atspi

        self.Atspi = Atspi
        Atspi.init()

    def apps(self) -> list[object]:
        desk = self.Atspi.get_desktop(0)
        found = []
        try:
            count = desk.get_child_count()
        except Exception:
            return found
        for index in range(count):
            try:
                app = desk.get_child_at_index(index)
            except Exception:
                continue
            if app is not None:
                found.append(app)
        return found

    def walk(self, node: object, depth: int = 0) -> list[object]:
        if node is None or depth > 24:
            return []
        found = [node]
        try:
            count = node.get_child_count()
        except Exception:
            return found
        for index in range(count):
            try:
                child = node.get_child_at_index(index)
            except Exception:
                continue
            if child is not None:
                found.extend(self.walk(child, depth + 1))
        return found

    def app_nodes(self, name: str) -> list[object]:
        nodes: list[object] = []
        for app in self.apps():
            try:
                if app.get_name() != name:
                    continue
            except Exception:
                continue
            nodes.extend(self.walk(app))
        return nodes

    def find(self, app: str, label: str, role: str) -> object | None:
        for node in self.app_nodes(app):
            try:
                if (node.get_name() or "") == label and role in node.get_role_name():
                    return node
            except Exception:
                continue
        return None

    def press(self, node: object) -> bool:
        action = node.get_action()
        return bool(action.do_action(0))

    def wait(self, app: str, label: str, role: str, timeout: float = 20.0) -> object | None:
        deadline = time.time() + timeout
        while time.time() < deadline:
            node = self.find(app, label, role)
            if node is not None:
                return node
            time.sleep(0.15)
        return None


def find_dialog(desktop: Desktop) -> object | None:
    for app in desktop.apps():
        for node in desktop.walk(app):
            try:
                name = node.get_name() or ""
                role = node.get_role_name()
            except Exception:
                continue
            if name == DIALOG_TITLE and "frame" in role:
                return node
    return None


def wait_dialog(desktop: Desktop, timeout: float = 30.0) -> object | None:
    deadline = time.time() + timeout
    while time.time() < deadline:
        frame = find_dialog(desktop)
        if frame is not None:
            return frame
        time.sleep(0.15)
    return None


def open_dialog(desktop: Desktop, proc: subprocess.Popen[bytes]) -> tuple[object | None, str]:
    generar = desktop.wait(APP_NAME, "GENERAR", "button", timeout=25)
    if generar is None or proc.poll() is not None:
        return None, "la cara no llegó a Generar"
    desktop.press(generar)
    exportar = desktop.wait(APP_NAME, "EXPORTAR", "button", timeout=25)
    if exportar is None or proc.poll() is not None:
        return None, "no apareció Exportar"
    desktop.press(exportar)
    frame = wait_dialog(desktop)
    if frame is None or proc.poll() is not None:
        return None, "el diálogo nativo no apareció o el proceso murió"
    return frame, ""


def press_in(desktop: Desktop, frame: object, label: str) -> bool:
    for node in desktop.walk(frame):
        if (node.get_name() or "") == label and "button" in node.get_role_name():
            return desktop.press(node)
    return False


def button_names(desktop: Desktop, frame: object) -> list[str]:
    names: list[str] = []
    for node in desktop.walk(frame):
        try:
            if "button" in node.get_role_name():
                names.append(node.get_name() or "")
        except Exception:
            continue
    return names


def is_showing(node: object) -> bool:
    try:
        from gi.repository import Atspi

        return bool(node.get_state_set().contains(Atspi.StateType.SHOWING))
    except Exception:
        return False


def mouse_click(node: object) -> bool:
    try:
        from gi.repository import Atspi

        comp = node.get_component()
        if comp is None:
            return False
        ext = comp.get_extents(Atspi.CoordType.SCREEN)
        if ext.width <= 0 or ext.height <= 0:
            return False
        x = ext.x + ext.width // 2
        y = ext.y + ext.height // 2
        return bool(Atspi.generate_mouse_event(x, y, "b1c"))
    except Exception:
        return False


def try_press_node(node: object) -> bool:
    acted = False
    try:
        action = node.get_action()
        if action is not None and action.get_n_actions() >= 1:
            acted = bool(action.do_action(0))
    except Exception:
        acted = False
    if acted:
        return True
    try:
        from gi.repository import Atspi

        comp = node.get_component()
        if comp is None:
            return False
        ext = comp.get_extents(Atspi.CoordType.SCREEN)
        x = ext.x + max(ext.width, 1) // 2
        y = ext.y + max(ext.height, 1) // 2
        return bool(Atspi.generate_mouse_event(x, y, "b1c"))
    except Exception:
        return False


def confirm_replace(desktop: Desktop) -> list[str] | None:
    """Pulsa el control de reemplazo del diálogo del portal, si aparece."""
    deadline = time.time() + 5
    while time.time() < deadline:
        for app_name in ("org.gnome.Nautilus", "xdg-desktop-portal-gnome", "xdg-desktop-portal-gtk"):
            for node in desktop.app_nodes(app_name):
                try:
                    name = node.get_name() or ""
                    role = node.get_role_name()
                    parent = node.get_parent()
                except Exception:
                    continue
                if "button" not in role:
                    continue
                if not is_showing(node):
                    continue
                if not any(word in name.lower() for word in ("reemplaz", "sobrescrib", "replace")):
                    continue
                if mouse_click(node):
                    return [app_name, role, name]
        time.sleep(0.2)
    return None


def node_blob(node: object) -> str:
    parts: list[str] = []
    try:
        name = (node.get_name() or "").strip()
        if name:
            parts.append(name)
    except Exception:
        pass
    try:
        count = node.get_character_count()
        if count:
            text = (node.get_text(0, count) or "").strip()
            if text and text not in parts:
                parts.append(text)
    except Exception:
        pass
    return " ".join(parts)


def app_lines(desktop: Desktop) -> list[str]:
    wanted = {
        APP_NAME,
        "org.gnome.Nautilus",
        "xdg-desktop-portal-gnome",
        "xdg-desktop-portal-gtk",
    }
    lines: list[str] = []
    for app in desktop.apps():
        try:
            app_name = app.get_name() or ""
        except Exception:
            continue
        if app_name not in wanted:
            continue
        for node in desktop.walk(app):
            try:
                role = node.get_role_name()
            except Exception:
                continue
            blob = node_blob(node)
            if blob:
                lines.append(f"{app_name}:{role}:{blob}")
    return lines


def wait_new_file(directory: Path, before: set[str], timeout: float = 8.0) -> Path | None:
    deadline = time.time() + timeout
    while time.time() < deadline:
        for path in directory.iterdir():
            if path.is_file() and path.name not in before and path.stat().st_size > 0:
                return path
        time.sleep(0.2)
    return None


def run(binary: Path, work: Path) -> dict[str, object]:
    desktop = Desktop()
    cases: dict[str, dict[str, object]] = {}
    save_dir = work / "save"
    nop_dir = work / "noperm"
    save_dir.mkdir(parents=True)
    nop_dir.mkdir()
    os.chmod(nop_dir, 0o755)

    def session(tag: str, dest: Path) -> tuple[subprocess.Popen[bytes], object | None, bytes, str]:
        log_path = work / f"{tag}.log"
        proc = launch(binary, work / f"home-{tag}", dest, log_path)
        frame, reason = open_dialog(desktop, proc)
        err = b""
        if frame is None:
            err = stop(proc, log_path)
        return proc, frame, err, reason

    proc, frame, err, reason = session("cancel", save_dir)
    closed = False
    buttons: list[str] = []
    if frame is not None:
        buttons = button_names(desktop, frame)
        closed = press_in(desktop, frame, "Cerrar")
        time.sleep(1.0)
        err = stop(proc, work / "cancel.log")
    cases["gui-cancel"] = {
        "status": "passed" if frame is not None and closed and not list(save_dir.iterdir()) and b"panicked" not in err else "failed",
        "reason": reason,
        "closed": closed,
        "buttons": buttons,
        "files": [p.name for p in save_dir.iterdir()],
        "panic": b"panicked" in err,
    }

    proc, frame, err, reason = session("save", save_dir)
    saved = False
    files: list[Path] = []
    if frame is not None:
        saved = press_in(desktop, frame, "Guardar")
        time.sleep(1.5)
        err = stop(proc, work / "save.log")
    files = [p for p in save_dir.iterdir() if p.is_file() and p.stat().st_size > 0]
    cases["gui-dialog-real"] = {
        "status": "passed" if saved and files and b"panicked" not in err else "failed",
        "reason": reason,
        "files": [p.name for p in files],
        "panic": b"panicked" in err,
    }

    before_overwrite = {p.name for p in save_dir.iterdir() if p.is_file()}
    proc, frame, err, reason = session("overwrite", save_dir)
    first = False
    pressed = False
    replace = None
    touched = False
    target_name = ""
    if frame is None:
        cases["gui-overwrite"] = {
            "status": "failed",
            "reason": reason,
            "panic": b"panicked" in err,
        }
    else:
        first = press_in(desktop, frame, "Guardar")
        created = wait_new_file(save_dir, before_overwrite)
        target_name = created.name if created is not None else ""
        stamped = created.stat().st_mtime_ns if created is not None else 0
        digest = (
            hashlib.sha256(created.read_bytes()).hexdigest() if created is not None else ""
        )
        rejected = False
        export_reject = desktop.wait(APP_NAME, "EXPORTAR", "button", timeout=20)
        if created is None or export_reject is None or proc.poll() is not None:
            err = stop(proc, work / "overwrite.log")
            reason = "sin archivo o sin Exportar para rechazar"
        else:
            desktop.press(export_reject)
            frame_reject = wait_dialog(desktop)
            if frame_reject is None:
                reason = "el diálogo de rechazo no apareció"
            else:
                press_in(desktop, frame_reject, "Cerrar")
                time.sleep(0.6)
                rejected = (
                    created.is_file()
                    and created.stat().st_mtime_ns == stamped
                    and hashlib.sha256(created.read_bytes()).hexdigest() == digest
                )
            export_again = desktop.wait(APP_NAME, "EXPORTAR", "button", timeout=20)
            if export_again is None or proc.poll() is not None:
                err = stop(proc, work / "overwrite.log")
                reason = reason or "sin Exportar para aceptar la sobrescritura"
            else:
                desktop.press(export_again)
                frame2 = wait_dialog(desktop)
                if frame2 is not None:
                    pressed = press_in(desktop, frame2, "Guardar")
                    replace = confirm_replace(desktop)
                    deadline = time.time() + 10
                    while time.time() < deadline and created.is_file():
                        if created.stat().st_mtime_ns > stamped or (
                            hashlib.sha256(created.read_bytes()).hexdigest() != digest
                        ):
                            touched = True
                            break
                        time.sleep(0.2)
                else:
                    reason = "el diálogo de sobrescritura no apareció"
                err = stop(proc, work / "overwrite.log")
        cases["gui-overwrite"] = {
            "status": "passed"
            if first and rejected and pressed and b"panicked" not in err and touched
            else "failed",
            "reason": reason,
            "rejectedOverwrite": rejected,
            "replaceDialog": replace,
            "touchedExisting": touched,
            "target": target_name,
            "files": [p.name for p in save_dir.iterdir() if p.is_file()],
            "panic": b"panicked" in err,
        }

    proc, frame, err, reason = session("noperm", nop_dir)
    pressed = False
    notices: list[str] = []
    digest_before = ""
    digest_after = ""
    try:
        if frame is not None:
            pressed = press_in(desktop, frame, "Guardar")
            created = wait_new_file(nop_dir, set())
            if created is not None:
                digest_before = hashlib.sha256(created.read_bytes()).hexdigest()
                os.chmod(nop_dir, 0o555)
                export_again = desktop.wait(APP_NAME, "EXPORTAR", "button", timeout=20)
                if export_again is None or proc.poll() is not None:
                    reason = "sin segundo Exportar"
                else:
                    desktop.press(export_again)
                    frame2 = wait_dialog(desktop)
                    if frame2 is None:
                        reason = "el segundo diálogo no apareció"
                    else:
                        press_in(desktop, frame2, "Guardar")
                        confirm_replace(desktop)
                        time.sleep(1.5)
                        keyword = ("fallo ipc", "permiso", "no se pudo", "permission")
                        notices = [
                            line
                            for line in app_lines(desktop)
                            if any(word in line.lower() for word in keyword)
                        ]
                if created.is_file():
                    digest_after = hashlib.sha256(created.read_bytes()).hexdigest()
            else:
                reason = "el primer guardado no creó archivo"
            err = stop(proc, work / "noperm.log")
    finally:
        os.chmod(nop_dir, 0o755)
    unchanged = bool(digest_before) and digest_before == digest_after
    showed_error = any("fallo ipc" in line.lower() or "permis" in line.lower() for line in notices)
    cases["gui-destination-noperms"] = {
        "status": "passed" if pressed and unchanged and showed_error and b"panicked" not in err else "failed",
        "reason": reason,
        "unchanged": unchanged,
        "notices": notices[:12],
        "panic": b"panicked" in err,
    }

    statuses = {item["status"] for item in cases.values()}
    return {
        "status": "passed" if statuses == {"passed"} else "failed",
        "dialogTitle": DIALOG_TITLE,
        "cases": cases,
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--binary", type=Path, required=True)
    parser.add_argument("--out-dir", type=Path, required=True)
    parser.add_argument(
        "--app-name",
        default="",
        help="Nombre AT-SPI de la ventana. Por defecto, el nombre del binario.",
    )
    args = parser.parse_args()
    global APP_NAME
    APP_NAME = args.app_name or args.binary.stem
    if not args.binary.is_file():
        fail(f"falta binario {args.binary}")
    if not os.environ.get("DISPLAY") and not os.environ.get("WAYLAND_DISPLAY"):
        fail("sin DISPLAY ni WAYLAND_DISPLAY")
    args.out_dir.mkdir(parents=True, exist_ok=True)
    work = args.out_dir / "portal-work"
    if work.exists():
        import shutil

        shutil.rmtree(work)
    report = run(args.binary, work)
    path = args.out_dir / "portal-e2e.json"
    path.write_text(json.dumps(report, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    print(json.dumps({"wrote": str(path), "status": report["status"]}, ensure_ascii=False))
    return 0 if report["status"] == "passed" else 1


if __name__ == "__main__":
    raise SystemExit(main())
