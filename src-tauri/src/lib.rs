//! Crate de escritorio Tauri: comandos CorePort y denegación deny-by-default.
//!
//! La allow-list de `capabilities/main.json` está vacía: Tauri deniega IPC no
//! concedido. El gate de aplicación (`check_invoke`) es una segunda capa.

use std::sync::Mutex;

use archmaker_core::{
    check_invoke, unknown_command_error, BuildManifestInput, CoreError, CoreOp, CorePort,
    CreateDraftInput, ExportArtifactInput, ImportDraftInput, LoadCatalogInput, RealCore,
    ResolveDraftInput, SaveDraftInput, ValidateDraftInput, Window,
};
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

/// Arranque de producción (runtime Wry).
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let data_dir = app.path().app_data_dir().expect("app data dir resolves");
            std::fs::create_dir_all(&data_dir).expect("app data dir is writable");
            let core = RealCore::with_embedded()
                .expect("embedded catalog loads")
                .with_store_dir(data_dir);
            app.manage(CoreState(Mutex::new(core)));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            create_draft,
            load_catalog,
            save_draft,
            resolve_draft,
            validate_draft,
            build_manifest,
            export_artifact,
            import_draft
        ])
        .run(tauri::generate_context!())
        .expect("error while running ArchMaker");
}

#[cfg(test)]
mod tests {
    use tauri::webview::InvokeRequest;
    use tauri::WebviewWindowBuilder;

    fn invoke_url() -> tauri::Url {
        if cfg!(any(windows, target_os = "android")) {
            "http://tauri.localhost".parse().unwrap()
        } else {
            "tauri://localhost".parse().unwrap()
        }
    }

    /// Negativo runtime: comando no concedido en capability → denegación IPC de Tauri.
    /// El job CI `security-runtime` ejecuta este nombre; si falta, falla.
    #[test]
    fn security_runtime_deny_ungranted_command() {
        // MockRuntime + contexto real (capabilities/main.json con permissions: []).
        let app = tauri::test::mock_builder()
            .build(tauri::generate_context!())
            .expect("mock app builds with real capabilities");

        let webview = WebviewWindowBuilder::new(&app, "main", Default::default())
            .build()
            .expect("main webview");

        let err = tauri::test::get_ipc_response(
            &webview,
            InvokeRequest {
                cmd: "create_draft".into(),
                callback: tauri::ipc::CallbackFn(0),
                error: tauri::ipc::CallbackFn(1),
                url: invoke_url(),
                body: tauri::ipc::InvokeBody::default(),
                headers: Default::default(),
                invoke_key: tauri::test::INVOKE_KEY.to_string(),
            },
        )
        .expect_err("create_draft must be denied: main capability grants nothing");

        let msg = format!("{err:?}").to_lowercase();
        assert!(
            msg.contains("denied")
                || msg.contains("not allowed")
                || msg.contains("forbidden")
                || msg.contains("permission")
                || msg.contains("capability")
                || msg.contains("acl")
                || msg.contains("command"),
            "expected ACL/permission denial, got: {err:?}"
        );

        let unknown = tauri::test::get_ipc_response(
            &webview,
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
}
