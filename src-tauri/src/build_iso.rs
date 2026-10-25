use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Mutex;
use std::thread;

use archmaker_core::{
    BuildManifestInput, CoreError, CorePort, DestinationHandle, Draft, ExportArtifactInput,
    TargetRef,
};
use serde::Deserialize;
use tauri::{Emitter, Manager};

use crate::{core_error_value, gate_build_iso, CoreState};

const LIVE_PROFILE_TARGET: &str = "archmaker-live-profile";
const BUILD_TARGET_VERSION: &str = "0.1.0";

#[derive(Default)]
pub struct BuildState {
    cancel: AtomicBool,
    cidfile: Mutex<Option<PathBuf>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct BuildIsoInput {
    pub draft: Draft,
}

fn internal(message: &str) -> serde_json::Value {
    serde_json::json!({
        "family": "tauri",
        "code": "build-iso",
        "message": message,
    })
}

fn resolve_script(app: &tauri::AppHandle) -> Option<PathBuf> {
    if let Ok(dir) = app.path().resource_dir() {
        let candidate = dir.join("scripts/build-iso.sh");
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    let dev = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../scripts/build-iso.sh");
    dev.is_file().then_some(dev)
}

fn docker_available() -> bool {
    Command::new("docker")
        .arg("info")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn emit_lines<R: std::io::Read + Send + 'static>(app: tauri::AppHandle, stream: R) {
    thread::spawn(move || {
        let reader = BufReader::new(stream);
        for line in reader.lines().map_while(Result::ok) {
            let _ = app.emit("build-iso-log", line);
        }
    });
}

fn run_build(
    app: tauri::AppHandle,
    input: BuildIsoInput,
) -> Result<serde_json::Value, serde_json::Value> {
    if !docker_available() {
        return Err(internal(
            "Docker no está disponible. Instálalo desde https://docs.docker.com/engine/install/ y vuelve a intentarlo.",
        ));
    }
    let script = resolve_script(&app)
        .ok_or_else(|| internal("No se encontró scripts/build-iso.sh en este entorno."))?;

    let target = TargetRef {
        id: LIVE_PROFILE_TARGET.to_string(),
        version: BUILD_TARGET_VERSION.to_string(),
    };
    let bundle = {
        let state = app.state::<CoreState>();
        let core = state
            .0
            .lock()
            .map_err(|_| internal("El motor no está disponible."))?;
        let catalog_ref = core.active_catalog_ref().map_err(core_error_value)?;
        let manifest = core
            .build_manifest(BuildManifestInput {
                draft: input.draft,
                catalog_ref,
                target_ref: target.clone(),
                expected_resolution_digest: None,
                request_id: None,
            })
            .map_err(core_error_value)?;
        let artifact = core
            .export_artifact(ExportArtifactInput {
                manifest,
                target_ref: target,
                destination: DestinationHandle::new("bundle.json".to_string()).map_err(|e| {
                    core_error_value(CoreError::schema_invalid(&format!(
                        "archmaker-tauri.build_iso: {e}"
                    )))
                })?,
                request_id: None,
            })
            .map_err(core_error_value)?;
        artifact
            .content
            .ok_or_else(|| internal("El perfil vivo no produjo contenido."))?
    };

    let cache = app
        .path()
        .app_cache_dir()
        .map_err(|_| internal("No hay directorio de caché de la aplicación."))?;
    let build_dir = cache.join("build-iso");
    std::fs::create_dir_all(&build_dir)
        .map_err(|_| internal("No se pudo preparar el directorio de construcción."))?;
    let bundle_path = build_dir.join("bundle.json");
    std::fs::write(
        &bundle_path,
        serde_json::to_vec(&bundle).map_err(|_| internal("El bundle no es serializable."))?,
    )
    .map_err(|_| internal("No se pudo escribir el bundle del perfil."))?;

    let out_dir = app
        .path()
        .app_data_dir()
        .map_err(|_| internal("No hay directorio de datos de la aplicación."))?
        .join("iso");
    std::fs::create_dir_all(&out_dir)
        .map_err(|_| internal("No se pudo preparar el directorio de salida."))?;
    let cidfile = build_dir.join("container.cid");
    let _ = std::fs::remove_file(&cidfile);

    {
        let state = app.state::<BuildState>();
        state.cancel.store(false, Ordering::SeqCst);
        *state
            .cidfile
            .lock()
            .map_err(|_| internal("Estado de construcción bloqueado."))? = Some(cidfile.clone());
    }

    let mut child = Command::new("bash")
        .arg(&script)
        .arg(&bundle_path)
        .arg("--out")
        .arg(&out_dir)
        .arg("--cidfile")
        .arg(&cidfile)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|_| internal("No se pudo iniciar scripts/build-iso.sh."))?;

    let stdout = child.stdout.take().expect("stdout piped");
    let stderr = child.stderr.take().expect("stderr piped");
    emit_lines(app.clone(), stderr);

    let mut iso_path = None;
    let mut sha256 = None;
    let reader = BufReader::new(stdout);
    for line in reader.lines().map_while(Result::ok) {
        if let Some(value) = line.strip_prefix("RESULT_ISO=") {
            iso_path = Some(value.to_string());
        } else if let Some(value) = line.strip_prefix("RESULT_SHA256=") {
            sha256 = Some(value.to_string());
        }
        let _ = app.emit("build-iso-log", line);
    }

    let status = child
        .wait()
        .map_err(|_| internal("La construcción terminó de forma anómala."))?;
    let cancelled = app.state::<BuildState>().cancel.load(Ordering::SeqCst);
    if cancelled {
        return Err(internal("Construcción cancelada."));
    }
    if !status.success() {
        return Err(internal(
            "mkarchiso falló. Revisa el registro de la construcción.",
        ));
    }
    let iso_path = iso_path.ok_or_else(|| internal("La construcción no informó de la ISO."))?;
    let sha256 = sha256.ok_or_else(|| internal("La construcción no informó del SHA-256."))?;
    let _ = app.emit(
        "build-iso-done",
        serde_json::json!({ "isoPath": iso_path, "sha256": sha256 }),
    );
    Ok(serde_json::json!({ "isoPath": iso_path, "sha256": sha256 }))
}

#[tauri::command]
pub async fn build_iso(
    app: tauri::AppHandle,
    window: tauri::Window,
    input: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_build_iso(&window)?;
    let typed: BuildIsoInput = serde_json::from_value(input)
        .map_err(|error| internal(&format!("archmaker-tauri.build_iso: {error}")))?;
    tauri::async_runtime::spawn_blocking(move || run_build(app, typed))
        .await
        .map_err(|_| internal("La construcción se interrumpió."))?
}

#[tauri::command]
pub fn cancel_build_iso(
    app: tauri::AppHandle,
    window: tauri::Window,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_build_iso(&window)?;
    let state = app.state::<BuildState>();
    state.cancel.store(true, Ordering::SeqCst);
    let cidfile = state
        .cidfile
        .lock()
        .map_err(|_| internal("Estado de construcción bloqueado."))?
        .clone();
    if let Some(path) = cidfile {
        if let Ok(id) = std::fs::read_to_string(path) {
            let id = id.trim();
            if !id.is_empty() {
                let _ = Command::new("docker").arg("stop").arg(id).status();
            }
        }
    }
    Ok(serde_json::json!({ "status": "cancelled" }))
}
