//! Crate de escritorio Tauri: comandos CorePort + guardado nativo acotado.
//!
//! ACL Tauri (AppManifest::commands + capabilities) es la primera capa.
//! `check_invoke` en Rust es la segunda (matriz ventana × operación).

mod hardware_ipc;
mod save_export;

use std::sync::Mutex;

use archmaker_core::{
    check_invoke, unknown_command_error, BuildManifestInput, CoreError, CoreOp, CorePort,
    CreateDraftInput, ExportArtifactInput, ImportDraftInput, LoadCatalogInput, OpenDraftInput,
    RealCore, ResolveDraftInput, SaveDraftInput, ValidateDraftInput, Window,
};
use hardware_ipc::HardwareState;
use save_export::{save_export_with_picker, NativeSavePicker, SaveExportInput};
use tauri::Manager;

struct CoreState(Mutex<RealCore>);

fn core_error_value(err: CoreError) -> serde_json::Value {
    serde_json::to_value(err).expect("CoreError is serializable")
}

fn invalid_input(op: CoreOp, detail: &str) -> serde_json::Value {
    core_error_value(CoreError::schema_invalid(&format!(
        "archmaker-tauri.{}: {detail}",
        op.command_name()
    )))
}

fn gate(window: &tauri::Window, op: CoreOp) -> Result<(), serde_json::Value> {
    let label = window.label().to_string();
    let parsed = Window::parse(&label)
        .ok_or_else(|| core_error_value(unknown_command_error(&label, op.command_name())))?;
    check_invoke(parsed, op).map_err(core_error_value)?;
    Ok(())
}

fn gate_save_export(window: &tauri::Window) -> Result<(), serde_json::Value> {
    let label = window.label();
    match Window::parse(label) {
        Some(Window::Main) => Ok(()),
        Some(other) => Err(core_error_value(CoreError::proto_denied(&format!(
            "archmaker-invoke.{}.save_export_bytes",
            other.as_str()
        )))),
        None => Err(core_error_value(unknown_command_error(
            label,
            "save_export_bytes",
        ))),
    }
}

fn gate_constitute_image(window: &tauri::Window) -> Result<(), serde_json::Value> {
    let label = window.label();
    match Window::parse(label) {
        Some(Window::Main) => Ok(()),
        Some(other) => Err(core_error_value(CoreError::proto_denied(&format!(
            "archmaker-invoke.{}.constitute_image",
            other.as_str()
        )))),
        None => Err(core_error_value(unknown_command_error(
            label,
            "constitute_image",
        ))),
    }
}

fn constitution_payload(code: &str) -> serde_json::Value {
    let message_key = match code {
        archmaker_constitute::CODE_IMAGE_ABSENT => "error.constitution.mkarchisoAbsent",
        archmaker_constitute::CODE_IMAGE_REFUSED => "error.constitution.mkarchisoRefused",
        archmaker_constitute::CODE_SCRIPT => "error.constitution.scriptRejected",
        _ => "error.constitution.unavailable",
    };
    serde_json::json!({
        "code": code,
        "family": "CONST",
        "category": "contract",
        "severity": "error",
        "retryable": false,
        "messageKey": message_key,
        "source": "archmaker-constitute",
    })
}

/// La página no elige ruta ni script. Un objeto vacío pide la imagen;
/// cualquier otro payload se rechaza. Nunca ejecuta `mkarchiso`.
fn constitute_image_decision(input: &serde_json::Value, search_path: &str) -> serde_json::Value {
    let empty = matches!(input, serde_json::Value::Object(map) if map.is_empty());
    if !empty {
        return constitution_payload(archmaker_constitute::CODE_SCRIPT);
    }
    let err = archmaker_constitute::request_live_image(search_path);
    constitution_payload(err.code)
}

fn parse_input<T>(op: CoreOp, input: serde_json::Value) -> Result<T, serde_json::Value>
where
    T: serde::de::DeserializeOwned,
{
    serde_json::from_value(input).map_err(|e| invalid_input(op, &e.to_string()))
}

macro_rules! core_command {
    ($name:ident, $op:expr, $input_ty:ty, $method:ident) => {
        #[tauri::command]
        fn $name(
            window: tauri::Window,
            state: tauri::State<'_, CoreState>,
            input: serde_json::Value,
        ) -> Result<serde_json::Value, serde_json::Value> {
            gate(&window, $op)?;
            let typed: $input_ty = parse_input($op, input)?;
            let core = state
                .0
                .lock()
                .map_err(|_| core_error_value(CoreError::io_failed("archmaker-tauri.state")))?;
            core.$method(typed)
                .map(|out| serde_json::to_value(out).expect("DTO is serializable"))
                .map_err(core_error_value)
        }
    };
}

core_command!(
    create_draft,
    CoreOp::CreateDraft,
    CreateDraftInput,
    create_draft
);
core_command!(
    load_catalog,
    CoreOp::LoadCatalog,
    LoadCatalogInput,
    load_catalog
);
core_command!(save_draft, CoreOp::SaveDraft, SaveDraftInput, save_draft);
core_command!(open_draft, CoreOp::OpenDraft, OpenDraftInput, open_draft);
core_command!(
    resolve_draft,
    CoreOp::ResolveDraft,
    ResolveDraftInput,
    resolve_draft
);
core_command!(
    validate_draft,
    CoreOp::ValidateDraft,
    ValidateDraftInput,
    validate_draft
);
core_command!(
    build_manifest,
    CoreOp::BuildManifest,
    BuildManifestInput,
    build_manifest
);
core_command!(
    export_artifact,
    CoreOp::ExportArtifact,
    ExportArtifactInput,
    export_artifact
);
core_command!(
    import_draft,
    CoreOp::ImportDraft,
    ImportDraftInput,
    import_draft
);

/// Guardado nativo acotado: diálogo elige destino; escritura atómica; sin FS global.
#[tauri::command]
fn save_export_bytes(
    window: tauri::Window,
    input: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_save_export(&window)?;
    let typed: SaveExportInput = serde_json::from_value(input).map_err(|e| {
        core_error_value(CoreError::schema_invalid(&format!(
            "archmaker-tauri.save_export_bytes: {e}"
        )))
    })?;
    save_export_with_picker(&NativeSavePicker, typed)
        .map(|out| serde_json::to_value(out).expect("SaveExportResult serializable"))
        .map_err(core_error_value)
}

/// Pide la imagen viva. No acepta ruta, script ni directorio.
/// Siempre falla de forma tipada: este proceso no lanza `mkarchiso`.
#[tauri::command]
fn constitute_image(
    window: tauri::Window,
    input: serde_json::Value,
) -> Result<serde_json::Value, serde_json::Value> {
    gate_constitute_image(&window)?;
    let search = std::env::var("PATH").unwrap_or_default();
    Err(constitute_image_decision(&input, &search))
}

/// Arranque de producción (runtime Wry).
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_data_dir().expect("app data dir resolves");
            std::fs::create_dir_all(&data_dir).expect("app data dir is writable");
            let core = RealCore::with_desktop()
                .expect("desktop catalog loads")
                .with_store_dir(data_dir);
            app.manage(CoreState(Mutex::new(core)));
            app.manage(HardwareState(std::sync::Mutex::new(
                hardware_ipc::HardwareSession::default(),
            )));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_draft,
            load_catalog,
            save_draft,
            open_draft,
            resolve_draft,
            validate_draft,
            build_manifest,
            export_artifact,
            import_draft,
            save_export_bytes,
            constitute_image,
            hardware_ipc::inspect_host_hardware,
            hardware_ipc::inspect_storage_devices,
            hardware_ipc::refresh_storage_snapshot,
            hardware_ipc::propose_storage_plan,
            hardware_ipc::validate_storage_plan,
            hardware_ipc::import_storage_snapshot
        ])
        .run(tauri::generate_context!())
        .expect("error while running Forge");
}

#[cfg(test)]
mod tests {
    use archmaker_core::{
        check_invoke, CatalogSource, CoreOp, CorePort, LoadCatalogInput, RealCore, Window,
    };
    use tauri::webview::InvokeRequest;
    use tauri::WebviewWindowBuilder;

    use super::save_export::{
        save_export_with_picker, SaveExportInput, SaveExportResult, SavePicker,
    };
    use std::path::PathBuf;
    use std::sync::Mutex;

    fn invoke_url() -> tauri::Url {
        if cfg!(any(windows, target_os = "android")) {
            "http://tauri.localhost".parse().unwrap()
        } else {
            "tauri://localhost".parse().unwrap()
        }
    }

    fn is_acl_denial(err: &impl std::fmt::Debug) -> bool {
        let msg = format!("{err:?}").to_lowercase();
        let aclish = msg.contains("denied")
            || msg.contains("not allowed")
            || msg.contains("forbidden")
            || msg.contains("permission")
            || msg.contains("capability")
            || msg.contains("acl");
        aclish && !msg.contains("unknown command")
    }

    fn hex_sha256(bytes: &[u8]) -> String {
        use sha2::{Digest, Sha256};
        Sha256::digest(bytes)
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect()
    }

    /// Positiva (gate + core de producción) y negativa ACL (IPC sin permiso).
    #[test]
    fn security_runtime_deny_ungranted_command() {
        assert!(
            check_invoke(Window::Main, CoreOp::LoadCatalog).is_ok(),
            "main must allow load_catalog at the application gate"
        );
        let core = RealCore::with_desktop().expect("desktop catalog");
        let catalog = core
            .load_catalog(LoadCatalogInput {
                source: CatalogSource::Embedded,
                catalog_ref: None,
                expected_digest: None,
                file_path: None,
                expected_provenance: None,
                limits: None,
                request_id: None,
            })
            .expect("authorized load_catalog must succeed");
        assert_eq!(catalog.id, "desktop");

        let app = tauri::test::mock_builder()
            .build(tauri::generate_context!())
            .expect("mock app builds with real capabilities");

        let restricted = WebviewWindowBuilder::new(&app, "restricted", Default::default())
            .build()
            .expect("restricted webview");

        let err = tauri::test::get_ipc_response(
            &restricted,
            InvokeRequest {
                cmd: "load_catalog".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: invoke_url(),
                body: tauri::ipc::InvokeBody::default(),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .expect_err("restricted window must be denied by ACL");

        assert!(
            is_acl_denial(&err),
            "expected ACL/permission denial (not a generic 'command' match), got: {err:?}"
        );

        let main = WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .expect("main webview");
        let unknown = tauri::test::get_ipc_response(
            &main,
            InvokeRequest {
                cmd: "shell_execute".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: invoke_url(),
                body: tauri::ipc::InvokeBody::default(),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .expect_err("shell_execute must be denied");
        assert!(
            !format!("{unknown:?}").is_empty(),
            "unknown command must produce an error"
        );
    }

    /// ACL: ventana restricted no tiene `allow-save-export-bytes`.
    /// MockRuntime no inyecta `tauri::Window` en handlers; la denegación
    /// se observa en la capa ACL antes del comando (igual que CorePort).
    #[test]
    fn save_export_bytes_denied_on_restricted_window() {
        let app = tauri::test::mock_builder()
            .build(tauri::generate_context!())
            .expect("mock app");

        let restricted = WebviewWindowBuilder::new(&app, "restricted", Default::default())
            .build()
            .expect("restricted");

        let err = tauri::test::get_ipc_response(
            &restricted,
            InvokeRequest {
                cmd: "save_export_bytes".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: invoke_url(),
                body: tauri::ipc::InvokeBody::default(),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .expect_err("restricted must not save");
        assert!(
            is_acl_denial(&err),
            "expected ACL denial for save_export_bytes, got: {err:?}"
        );
    }

    #[test]
    fn constitute_image_denied_on_restricted_window() {
        let app = tauri::test::mock_builder()
            .build(tauri::generate_context!())
            .expect("mock app");
        let restricted = WebviewWindowBuilder::new(&app, "restricted", Default::default())
            .build()
            .expect("restricted");
        let err = tauri::test::get_ipc_response(
            &restricted,
            InvokeRequest {
                cmd: "constitute_image".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: invoke_url(),
                body: tauri::ipc::InvokeBody::default(),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .expect_err("restricted must not request the image");
        assert!(
            is_acl_denial(&err),
            "expected ACL denial for constitute_image, got: {err:?}"
        );
    }

    #[test]
    fn constitute_image_decision_does_not_create_files_or_accept_a_script() {
        let dir =
            std::env::temp_dir().join(format!("archmaker-constitute-image-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let search = dir.to_str().expect("utf8");

        let absent = super::constitute_image_decision(&serde_json::json!({}), search);
        assert_eq!(absent["code"], "constitution-mkarchiso-absent");
        assert_eq!(absent["messageKey"], "error.constitution.mkarchisoAbsent");
        assert_eq!(absent["source"], "archmaker-constitute");
        assert!(std::fs::read_dir(&dir).unwrap().next().is_none());

        let tool = dir.join("mkarchiso");
        std::fs::write(&tool, b"not-executed\n").unwrap();
        let refused = super::constitute_image_decision(&serde_json::json!({}), search);
        assert_eq!(refused["code"], "constitution-mkarchiso-refused");
        assert_eq!(refused["messageKey"], "error.constitution.mkarchisoRefused");
        assert_eq!(std::fs::read(&tool).unwrap(), b"not-executed\n");
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);

        let script =
            super::constitute_image_decision(&serde_json::json!({"path": "/tmp/run.sh"}), search);
        assert_eq!(script["code"], "constitution-script-rejected");
        assert_eq!(std::fs::read_dir(&dir).unwrap().count(), 1);

        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn save_export_bytes_unknown_command_and_dead_transport_surface() {
        let app = tauri::test::mock_builder()
            .build(tauri::generate_context!())
            .expect("mock app");
        let main = WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .expect("main");

        let unknown = tauri::test::get_ipc_response(
            &main,
            InvokeRequest {
                cmd: "fs_write_anywhere".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: invoke_url(),
                body: tauri::ipc::InvokeBody::default(),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .expect_err("unknown FS command must fail");
        assert!(!format!("{unknown:?}").is_empty());

        // Transporte caído / comando no enlazado en MockRuntime: main tiene
        // permiso ACL pero sin handler registrado → error de invoke (no mock).
        let no_handler = tauri::test::get_ipc_response(
            &main,
            InvokeRequest {
                cmd: "save_export_bytes".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: invoke_url(),
                body: tauri::ipc::InvokeBody::default(),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .expect_err("sin handler no hay éxito silencioso ni mock");
        assert!(!format!("{no_handler:?}").is_empty());
    }

    #[test]
    fn frontend_dist_is_real_web_build_not_mock() {
        let conf = include_str!("../tauri.conf.json");
        assert!(
            conf.contains("\"frontendDist\": \"../apps/web/dist\""),
            "frontendDist must point at apps/web/dist"
        );
        assert!(!conf.contains("MockRuntime"));
        let dist = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../apps/web/dist/index.html");
        assert!(
            dist.exists(),
            "apps/web/dist/index.html must exist for desktop binary (build web first)"
        );
        let html = std::fs::read_to_string(&dist).expect("read index");
        assert!(
            html.contains("Forge") || html.contains("ArchMaker") || html.contains("root"),
            "dist must be the real web frontend"
        );
    }

    #[test]
    fn save_export_contract_eight_scenarios_via_scripted_picker() {
        struct P(Mutex<Option<Option<PathBuf>>>);
        impl SavePicker for P {
            fn pick_save_path(&self, _: &str) -> Option<PathBuf> {
                self.0.lock().unwrap().take().flatten()
            }
        }
        let bytes = b"{\"ok\":true}";
        let digest = hex_sha256(bytes);
        let dir = std::env::temp_dir().join(format!("archmaker-f5-ipc-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();

        // 1 cancel
        let p = P(Mutex::new(Some(None)));
        assert_eq!(
            save_export_with_picker(
                &p,
                SaveExportInput {
                    bytes: bytes.to_vec(),
                    suggested_name: "a.json".into(),
                    expected_digest: digest.clone(),
                }
            )
            .unwrap(),
            SaveExportResult::Cancelled
        );

        // 2 new file
        let path = dir.join("a.json");
        let p = P(Mutex::new(Some(Some(path.clone()))));
        let saved = save_export_with_picker(
            &p,
            SaveExportInput {
                bytes: bytes.to_vec(),
                suggested_name: "a.json".into(),
                expected_digest: digest.clone(),
            },
        )
        .unwrap();
        assert!(matches!(
            saved,
            SaveExportResult::Saved {
                overwritten: false,
                ..
            }
        ));

        // 3 overwrite
        let p = P(Mutex::new(Some(Some(path.clone()))));
        let saved = save_export_with_picker(
            &p,
            SaveExportInput {
                bytes: bytes.to_vec(),
                suggested_name: "a.json".into(),
                expected_digest: digest.clone(),
            },
        )
        .unwrap();
        assert!(matches!(
            saved,
            SaveExportResult::Saved {
                overwritten: true,
                ..
            }
        ));

        // 4 digest mismatch
        let p = P(Mutex::new(Some(Some(dir.join("b.json")))));
        assert!(save_export_with_picker(
            &p,
            SaveExportInput {
                bytes: bytes.to_vec(),
                suggested_name: "b.json".into(),
                expected_digest: "f".repeat(64),
            }
        )
        .is_err());

        // 5 empty bytes
        assert!(save_export_with_picker(
            &P(Mutex::new(Some(Some(dir.join("c.json"))))),
            SaveExportInput {
                bytes: vec![],
                suggested_name: "c.json".into(),
                expected_digest: digest.clone(),
            }
        )
        .is_err());

        // 6 path-like name
        assert!(save_export_with_picker(
            &P(Mutex::new(Some(Some(dir.join("d.json"))))),
            SaveExportInput {
                bytes: bytes.to_vec(),
                suggested_name: "../d.json".into(),
                expected_digest: digest.clone(),
            }
        )
        .is_err());

        // 7 absolute name
        assert!(save_export_with_picker(
            &P(Mutex::new(Some(Some(dir.join("e.json"))))),
            SaveExportInput {
                bytes: bytes.to_vec(),
                suggested_name: "/tmp/e.json".into(),
                expected_digest: digest.clone(),
            }
        )
        .is_err());

        // 8 bytes written match
        assert_eq!(std::fs::read(&path).unwrap(), bytes);

        let _ = std::fs::remove_dir_all(&dir);
    }
}
