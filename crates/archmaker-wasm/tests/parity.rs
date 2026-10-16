use archmaker_core::{Catalog, CorePort, RealCore};
use archmaker_wasm::{
    canonicalize_json, digest_for_json, guarded_invoke, guarded_invoke_with_core, DEFAULT_DOMAIN,
};
use serde_json::Value;

const MAPS_VECTORS: &str =
    include_str!("../../../contracts/test-vectors/canonicalization/maps.vectors.json");
const DOMAIN_VECTORS: &str =
    include_str!("../../../contracts/test-vectors/canonicalization/domain.vectors.json");
const UNICODE_VECTORS: &str =
    include_str!("../../../contracts/test-vectors/canonicalization/unicode.vectors.json");

const FIXED_DRAFT_ID: &str = "11111111-1111-4111-8111-111111111111";

fn catalog_ref_json() -> Value {
    let core = RealCore::with_embedded().expect("embedded catalog");
    serde_json::to_value(core.active_catalog_ref().expect("active catalog ref"))
        .expect("catalogRef serializable")
}

fn create_input_json() -> String {
    serde_json::json!({
        "draftId": FIXED_DRAFT_ID,
        "catalogRef": catalog_ref_json(),
        "targetRef": {"id": "arch-x86_64", "version": "0.1.0"},
        "requestId": null
    })
    .to_string()
}

fn draft_with_fixture_selections() -> Value {
    let mut draft: Value = serde_json::from_str(
        &guarded_invoke("main", "create_draft", &create_input_json()).expect("create works"),
    )
    .expect("draft JSON");
    draft["selections"] = serde_json::json!([
        {"stepId": "archmaker.sys.base", "value": {"kind": "single", "optionId": "archmaker.kernel.linux"}},
        {"stepId": "archmaker.sys.session", "value": {"kind": "single", "optionId": "archmaker.desktop.gnome"}}
    ]);
    draft
}

fn run_vectors(file_json: &str, file_label: &str) {
    let doc: Value = serde_json::from_str(file_json).expect("vector file is valid JSON");
    let vectors = doc["vectors"]
        .as_array()
        .expect("vector file has vectors array");
    assert!(!vectors.is_empty(), "{file_label}: no vectors");
    for vector in vectors {
        let id = vector["id"].as_str().unwrap_or("?");
        let domain = vector["domain"].as_str().unwrap_or(DEFAULT_DOMAIN);
        let input = &vector["input"];
        let expected_canonical = vector["canonical"]
            .as_str()
            .unwrap_or_else(|| panic!("{file_label}/{id}: no canonical"));
        let expected_digest = vector["digest"]
            .as_str()
            .unwrap_or_else(|| panic!("{file_label}/{id}: no digest"));

        let input_json = input.to_string();
        let native_canonical =
            String::from_utf8(archmaker_canonicalization::canonicalize(input)).expect("utf8");
        let wasm_canonical =
            canonicalize_json(&input_json).unwrap_or_else(|e| panic!("{file_label}/{id}: {e}"));
        assert_eq!(
            native_canonical, expected_canonical,
            "{file_label}/{id}: native canonical mismatch"
        );
        assert_eq!(
            wasm_canonical, expected_canonical,
            "{file_label}/{id}: wasm canonical mismatch"
        );

        let native_digest = archmaker_canonicalization::digest(native_canonical.as_bytes(), domain);
        let wasm_digest = digest_for_json(domain, &wasm_canonical);
        assert_eq!(
            native_digest, expected_digest,
            "{file_label}/{id}: native digest"
        );
        assert_eq!(
            wasm_digest, expected_digest,
            "{file_label}/{id}: wasm digest"
        );
    }
}

#[test]
fn parity_maps_vectors() {
    run_vectors(MAPS_VECTORS, "maps");
}

#[test]
fn parity_domain_vectors() {
    run_vectors(DOMAIN_VECTORS, "domain");
}

#[test]
fn parity_unicode_vectors() {
    run_vectors(UNICODE_VECTORS, "unicode");
}

#[test]
fn parity_create_draft_deterministic_bytes() {
    let input = create_input_json();
    let native: Value = {
        let core = RealCore::with_embedded().expect("embedded catalog");
        let typed = serde_json::from_str(&input).expect("valid input");
        let draft = core.create_draft(typed).expect("create works");
        serde_json::to_value(&draft).expect("serializable")
    };
    let wasm: Value =
        serde_json::from_str(&guarded_invoke("main", "create_draft", &input).expect("wasm create"))
            .expect("draft JSON");
    assert_eq!(native, wasm, "same input -> same draft bytes");
    assert_eq!(wasm["id"], FIXED_DRAFT_ID);
}

#[test]
fn parity_validate_diagnostics() {
    let draft = draft_with_fixture_selections();
    let input = serde_json::json!({
        "draft": draft,
        "catalogRef": catalog_ref_json(),
        "targetRef": {"id": "arch-x86_64", "version": "0.1.0"},
        "requestId": null
    })
    .to_string();
    let native: Value = {
        let core = RealCore::with_embedded().expect("embedded catalog");
        let typed = serde_json::from_str(&input).expect("valid input");
        let result = core.validate_draft(typed).expect("validate works");
        serde_json::to_value(&result).expect("serializable")
    };
    let wasm: Value =
        serde_json::from_str(&guarded_invoke("main", "validate_draft", &input).expect("wasm ok"))
            .expect("result JSON");
    assert_eq!(native, wasm, "same input -> same diagnostics");
    assert!(wasm.get("diagnostics").and_then(|d| d.as_array()).is_some());
    assert!(wasm.get("blocking").and_then(|b| b.as_bool()).is_some());
}

#[test]
fn parity_resolve_and_manifest() {
    let draft = draft_with_fixture_selections();
    let resolve_input = serde_json::json!({
        "draft": draft,
        "catalogRef": catalog_ref_json(),
        "requestId": null
    })
    .to_string();
    let native_resolve: Value = {
        let core = RealCore::with_embedded().expect("embedded catalog");
        let typed = serde_json::from_str(&resolve_input).expect("valid input");
        let result = core.resolve_draft(typed).expect("resolve works");
        serde_json::to_value(&result).expect("serializable")
    };
    let wasm_resolve: Value = serde_json::from_str(
        &guarded_invoke("main", "resolve_draft", &resolve_input).expect("wasm resolve"),
    )
    .expect("result JSON");
    assert_eq!(
        native_resolve, wasm_resolve,
        "same resolution digest and selections"
    );

    let manifest_input = serde_json::json!({
        "draft": draft_with_fixture_selections(),
        "catalogRef": catalog_ref_json(),
        "targetRef": {"id": "archmaker-profile", "version": "0.1.0"},
        "requestId": null
    })
    .to_string();
    let core = RealCore::with_embedded().expect("embedded catalog");
    let native_manifest = core.build_manifest(serde_json::from_str(&manifest_input).unwrap());
    let wasm_manifest = guarded_invoke("main", "build_manifest", &manifest_input);
    match (native_manifest, wasm_manifest) {
        (Ok(native), Ok(wasm_raw)) => {
            let native_json: Value = serde_json::to_value(&native).expect("serializable");
            let wasm_json: Value = serde_json::from_str(&wasm_raw).expect("manifest JSON");
            assert_eq!(native_json, wasm_json, "same input -> same manifest bytes");
            let content_digest = wasm_json["contentDigest"]["value"]
                .as_str()
                .expect("contentDigest");
            let recomputed = {
                let without: Value = {
                    let mut v = wasm_json.clone();
                    v.as_object_mut().expect("object").remove("contentDigest");
                    v
                };
                let canonical = archmaker_canonicalization::canonicalize(&without);
                archmaker_canonicalization::digest(&canonical, DEFAULT_DOMAIN)
            };
            assert_eq!(
                content_digest, recomputed,
                "contentDigest matches canonical bytes (excl. contentDigest field)"
            );
        }
        (Err(native_err), Err(wasm_raw)) => {
            let wasm_err: Value = serde_json::from_str(&wasm_raw).expect("typed error");
            assert_eq!(wasm_err["code"], native_err.code);
        }
        (Ok(_), Err(e)) | (Err(_), Ok(e)) => {
            panic!("native/wasm divergence: {e}");
        }
    }
}

const DESKTOP_CATALOG_JSON: &str =
    include_str!("../../archmaker-core/fixtures/local-catalog-mvp1.json");

fn desktop_core() -> RealCore {
    let catalog: Catalog =
        serde_json::from_str(DESKTOP_CATALOG_JSON).expect("desktop catalog deserializes");
    RealCore::new(catalog)
}

fn desktop_catalog_ref_json(core: &RealCore) -> Value {
    serde_json::to_value(core.active_catalog_ref().expect("active catalog ref"))
        .expect("catalogRef serializable")
}

fn draft_with_desktop_selections(core: &RealCore) -> Value {
    let input = serde_json::json!({
        "draftId": FIXED_DRAFT_ID,
        "catalogRef": desktop_catalog_ref_json(core),
        "targetRef": {"id": "arch-x86_64", "version": "0.1.0"},
        "requestId": null
    })
    .to_string();
    let typed = serde_json::from_str(&input).expect("valid create input");
    let draft = core.create_draft(typed).expect("create desktop draft");
    let mut value = serde_json::to_value(&draft).expect("serializable");
    value["selections"] = serde_json::json!([
        {"stepId": "archmaker.base.kernel", "value": {"kind": "single", "optionId": "archmaker.kernel.linux"}},
        {"stepId": "archmaker.base.boot", "value": {"kind": "single", "optionId": "archmaker.boot.systemd-boot"}},
        {"stepId": "archmaker.base.filesystem", "value": {"kind": "single", "optionId": "archmaker.filesystem.btrfs"}},
        {"stepId": "archmaker.environment.compositor", "value": {"kind": "single", "optionId": "archmaker.compositor.gnome"}},
        {"stepId": "archmaker.apps.browser", "value": {"kind": "single", "optionId": "archmaker.browser.firefox"}}
    ]);
    value
}

fn canonical_bytes(value: &Value) -> Vec<u8> {
    archmaker_canonicalization::canonicalize(value)
}

#[test]
fn parity_desktop_catalog_manifest_canonical_bytes() {
    let core = desktop_core();
    let draft = draft_with_desktop_selections(&core);
    let catalog_ref = desktop_catalog_ref_json(&core);

    let resolve_input = serde_json::json!({
        "draft": draft,
        "catalogRef": catalog_ref,
        "requestId": null
    })
    .to_string();
    let native_resolve: Value = {
        let typed = serde_json::from_str(&resolve_input).expect("valid resolve input");
        let result = core.resolve_draft(typed).expect("native resolve");
        serde_json::to_value(&result).expect("serializable")
    };
    let wasm_resolve: Value = serde_json::from_str(
        &guarded_invoke_with_core(&core, "main", "resolve_draft", &resolve_input)
            .expect("wasm resolve"),
    )
    .expect("resolve JSON");
    assert_eq!(
        canonical_bytes(&native_resolve),
        canonical_bytes(&wasm_resolve),
        "desktop resolve: same canonical bytes"
    );

    let manifest_input = serde_json::json!({
        "draft": draft,
        "catalogRef": catalog_ref,
        "targetRef": {"id": "archmaker-profile", "version": "0.1.0"},
        "expectedResolutionDigest": native_resolve["resolutionDigest"],
        "requestId": null
    })
    .to_string();
    let native_manifest = core.build_manifest(serde_json::from_str(&manifest_input).unwrap());
    let wasm_manifest = guarded_invoke_with_core(&core, "main", "build_manifest", &manifest_input);
    match (native_manifest, wasm_manifest) {
        (Ok(native), Ok(wasm_raw)) => {
            let native_json: Value = serde_json::to_value(&native).expect("serializable");
            let wasm_json: Value = serde_json::from_str(&wasm_raw).expect("manifest JSON");
            assert_eq!(
                canonical_bytes(&native_json),
                canonical_bytes(&wasm_json),
                "desktop manifest: same canonical bytes"
            );
            let content_digest = wasm_json["contentDigest"]["value"]
                .as_str()
                .expect("contentDigest");
            let recomputed = {
                let without: Value = {
                    let mut v = wasm_json.clone();
                    v.as_object_mut().expect("object").remove("contentDigest");
                    v
                };
                let canonical = archmaker_canonicalization::canonicalize(&without);
                archmaker_canonicalization::digest(&canonical, DEFAULT_DOMAIN)
            };
            assert_eq!(
                content_digest, recomputed,
                "desktop contentDigest matches canonical bytes"
            );
        }
        (Err(native_err), Err(wasm_raw)) => {
            let wasm_err: Value = serde_json::from_str(&wasm_raw).expect("typed error");
            assert_eq!(wasm_err["code"], native_err.code);
        }
        (Ok(_), Err(e)) | (Err(_), Ok(e)) => {
            panic!("native/wasm desktop divergence: {e}");
        }
    }
}
