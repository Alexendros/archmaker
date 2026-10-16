use std::cell::RefCell;

use archmaker_canonicalization as canon;
use archmaker_core::{
    check_invoke_named, BuildManifestInput, CoreError, CoreOp, CorePort, CreateDraftInput,
    ExportArtifactInput, ImportDraftInput, LoadCatalogInput, OpenDraftInput, RealCore,
    ResolveDraftInput, SaveDraftInput, ValidateDraftInput,
};
use wasm_bindgen::prelude::*;

pub const DEFAULT_DOMAIN: &str = "archmaker:manifest:v1";

thread_local! {
    static SESSION_CORE: RefCell<Option<RealCore>> = const { RefCell::new(None) };
}

fn invalid_json_error(source: &str) -> CoreError {
    CoreError::schema_invalid(source)
}

fn error_json(err: &CoreError) -> String {
    serde_json::to_string(err).expect("CoreError is serializable")
}

fn session_core() -> Result<(), String> {
    SESSION_CORE.with(|cell| {
        let mut slot = cell.borrow_mut();
        if slot.is_none() {
            *slot = Some(RealCore::with_desktop().map_err(|e| error_json(&e))?);
        }
        Ok(())
    })
}

fn with_session<F>(f: F) -> Result<String, String>
where
    F: FnOnce(&RealCore) -> Result<String, String>,
{
    session_core()?;
    SESSION_CORE.with(|cell| {
        let slot = cell.borrow();
        let core = slot.as_ref().expect("session core initialized");
        f(core)
    })
}

pub fn canonicalize_json(input_json: &str) -> Result<String, String> {
    let value: serde_json::Value = serde_json::from_str(input_json)
        .map_err(|_| error_json(&invalid_json_error("archmaker-wasm.canonicalize")))?;
    let bytes = canon::canonicalize(&value);
    String::from_utf8(bytes)
        .map_err(|_| error_json(&invalid_json_error("archmaker-wasm.canonicalize")))
}

pub fn digest_for_json(domain: &str, canonical: &str) -> String {
    canon::digest(canonical.as_bytes(), domain)
}

fn dispatch(core: &RealCore, op: CoreOp, input: serde_json::Value) -> Result<String, String> {
    let bad = |e: serde_json::Error| {
        error_json(&CoreError::schema_invalid(&format!(
            "archmaker-wasm.dispatch: {e}"
        )))
    };
    let out = match op {
        CoreOp::CreateDraft => {
            let typed: CreateDraftInput = serde_json::from_value(input).map_err(bad)?;
            serde_json::to_value(core.create_draft(typed).map_err(|e| error_json(&e))?)
                .expect("DTO is serializable")
        }
        CoreOp::LoadCatalog => {
            let typed: LoadCatalogInput = serde_json::from_value(input).map_err(bad)?;
            serde_json::to_value(core.load_catalog(typed).map_err(|e| error_json(&e))?)
                .expect("DTO is serializable")
        }
        CoreOp::SaveDraft => {
            let typed: SaveDraftInput = serde_json::from_value(input).map_err(bad)?;
            serde_json::to_value(core.save_draft(typed).map_err(|e| error_json(&e))?)
                .expect("DTO is serializable")
        }
        CoreOp::OpenDraft => {
            let typed: OpenDraftInput = serde_json::from_value(input).map_err(bad)?;
            serde_json::to_value(core.open_draft(typed).map_err(|e| error_json(&e))?)
                .expect("DTO is serializable")
        }
        CoreOp::ResolveDraft => {
            let typed: ResolveDraftInput = serde_json::from_value(input).map_err(bad)?;
            serde_json::to_value(core.resolve_draft(typed).map_err(|e| error_json(&e))?)
                .expect("DTO is serializable")
        }
        CoreOp::ValidateDraft => {
            let typed: ValidateDraftInput = serde_json::from_value(input).map_err(bad)?;
            serde_json::to_value(core.validate_draft(typed).map_err(|e| error_json(&e))?)
                .expect("DTO is serializable")
        }
        CoreOp::BuildManifest => {
            let typed: BuildManifestInput = serde_json::from_value(input).map_err(bad)?;
            serde_json::to_value(core.build_manifest(typed).map_err(|e| error_json(&e))?)
                .expect("DTO is serializable")
        }
        CoreOp::ExportArtifact => {
            let typed: ExportArtifactInput = serde_json::from_value(input).map_err(bad)?;
            serde_json::to_value(core.export_artifact(typed).map_err(|e| error_json(&e))?)
                .expect("DTO is serializable")
        }
        CoreOp::ImportDraft => {
            let typed: ImportDraftInput = serde_json::from_value(input).map_err(bad)?;
            serde_json::to_value(core.import_draft(typed).map_err(|e| error_json(&e))?)
                .expect("DTO is serializable")
        }
    };
    Ok(out.to_string())
}

pub fn core_op_json(op_name: &str, input_json: &str) -> Result<String, String> {
    let op = CoreOp::parse_command(op_name)
        .ok_or_else(|| error_json(&invalid_json_error("archmaker-wasm.op")))?;
    let input: serde_json::Value = serde_json::from_str(input_json)
        .map_err(|_| error_json(&invalid_json_error("archmaker-wasm.op")))?;
    with_session(|core| dispatch(core, op, input))
}

/// One-shot invoke against a fresh embedded catalog (native tests / parity harness).
pub fn guarded_invoke(
    window_label: &str,
    op_name: &str,
    input_json: &str,
) -> Result<String, String> {
    let core = RealCore::with_embedded().map_err(|e| error_json(&e))?;
    guarded_invoke_with_core(&core, window_label, op_name, input_json)
}

/// Browser production path: session core + desktop catalog.
pub fn guarded_invoke_session(
    window_label: &str,
    op_name: &str,
    input_json: &str,
) -> Result<String, String> {
    with_session(|core| guarded_invoke_with_core(core, window_label, op_name, input_json))
}

/// Dispatch WASM guarded by window allowlist against an injected `RealCore`.
pub fn guarded_invoke_with_core(
    core: &RealCore,
    window_label: &str,
    op_name: &str,
    input_json: &str,
) -> Result<String, String> {
    let op = check_invoke_named(window_label, op_name).map_err(|e| error_json(&e))?;
    let input: serde_json::Value = serde_json::from_str(input_json)
        .map_err(|_| error_json(&invalid_json_error("archmaker-wasm.invoke")))?;
    dispatch(core, op, input)
}

#[wasm_bindgen]
pub fn wasm_canonicalize(input_json: &str) -> Result<String, JsValue> {
    canonicalize_json(input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_digest(domain: &str, canonical: &str) -> String {
    digest_for_json(domain, canonical)
}

#[wasm_bindgen]
pub fn wasm_invoke(window_label: &str, op_name: &str, input_json: &str) -> Result<String, JsValue> {
    guarded_invoke_session(window_label, op_name, input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_create_draft(input_json: &str) -> Result<String, JsValue> {
    core_op_json("create_draft", input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_load_catalog(input_json: &str) -> Result<String, JsValue> {
    core_op_json("load_catalog", input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_save_draft(input_json: &str) -> Result<String, JsValue> {
    core_op_json("save_draft", input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_open_draft(input_json: &str) -> Result<String, JsValue> {
    core_op_json("open_draft", input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_resolve_draft(input_json: &str) -> Result<String, JsValue> {
    core_op_json("resolve_draft", input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_validate_draft(input_json: &str) -> Result<String, JsValue> {
    core_op_json("validate_draft", input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_build_manifest(input_json: &str) -> Result<String, JsValue> {
    core_op_json("build_manifest", input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_export_artifact(input_json: &str) -> Result<String, JsValue> {
    core_op_json("export_artifact", input_json).map_err(|e| JsValue::from_str(&e))
}

#[wasm_bindgen]
pub fn wasm_import_draft(input_json: &str) -> Result<String, JsValue> {
    core_op_json("import_draft", input_json).map_err(|e| JsValue::from_str(&e))
}
