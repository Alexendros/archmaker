//! PR5: bundle de overlay de humo para construir la ISO en Docker.

use archmaker_core::{
    render_live_profile, BuildManifestInput, CorePort, CreateDraftInput, RealCore,
};
use archmaker_domain::{Selection, SelectionValue, TargetRef};
use serde_json::Value;

fn draft_target() -> TargetRef {
    TargetRef {
        id: "arch-x86_64".to_string(),
        version: "2026.10.0".to_string(),
    }
}

fn live_target() -> TargetRef {
    TargetRef {
        id: "archmaker-live-profile".to_string(),
        version: "0.1.0".to_string(),
    }
}

fn single(step: &str, option: &str) -> Selection {
    Selection {
        step_id: step.to_string(),
        value: SelectionValue::Single {
            option_id: option.to_string(),
        },
    }
}

fn smoke_draft(core: &RealCore) -> archmaker_domain::Draft {
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref: core.active_catalog_ref().expect("catalog"),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create");
    draft.selections = vec![single("archmaker.base.kernel", "archmaker.kernel.linux")];
    draft
}

fn strip_prefix(text: &str) -> String {
    let mut out = String::with_capacity(text.len());
    let mut rest = text;
    while let Some(pos) = rest.find("FORGE_") {
        out.push_str(&rest[..pos]);
        out.push_str("FORGE_<prefix>");
        let after = &rest[pos + "FORGE_".len()..];
        let skip = after
            .char_indices()
            .nth(12)
            .map(|(index, _)| index)
            .unwrap_or(after.len());
        rest = &after[skip..];
    }
    out.push_str(rest);
    out
}

fn normalize(files: &Value) -> Value {
    let mut clone = files.clone();
    let profile = clone["profiledef.sh"].clone();
    if let Some(text) = profile.as_str() {
        clone["profiledef.sh"] = Value::String(strip_prefix(text));
    }
    clone
}

#[test]
fn archiso_smoke_overlay_fixture_matches_render() {
    let core = RealCore::with_desktop().expect("desktop");
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft: smoke_draft(&core),
            catalog_ref: core.active_catalog_ref().expect("catalog"),
            target_ref: live_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("seal");
    let rendered = render_live_profile(&manifest, core.catalog(), &live_target()).expect("render");
    let value: Value = serde_json::from_slice(&rendered.bytes).expect("json");

    let scratch = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/ainspect");
    std::fs::create_dir_all(&scratch).expect("scratch");
    std::fs::write(
        scratch.join("smoke-bundle.json"),
        serde_json::to_vec_pretty(&value).expect("pretty"),
    )
    .expect("write");

    assert_eq!(value["files"]["packages.x86_64"], "linux\n");
    assert!(value["files"]["airootfs/root/user_configuration.json"].is_string());

    let fixture_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("fixtures/archiso-overlay-smoke.json");
    let fixture: Value = serde_json::from_str(
        &std::fs::read_to_string(&fixture_path).expect("fixture de overlay de humo"),
    )
    .expect("fixture json");
    assert_eq!(normalize(&value["files"]), normalize(&fixture["files"]));
}
