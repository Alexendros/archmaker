//! F2: entrega de bytes del renderizador en el DTO Artifact.
//! Corpus P5: ≥20 fixtures por target soportado + 4 rechazos.

use std::path::PathBuf;

use archmaker_core::{
    render_artifact_bytes, BuildManifestInput, CorePort, CreateDraftInput, DestinationHandle,
    ExportArtifactInput, RealCore,
};
use archmaker_domain::{CatalogRef, Draft, Selection, SelectionValue, TargetRef};
use sha2::{Digest, Sha256};

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn catalog_ref() -> CatalogRef {
    RealCore::with_embedded()
        .expect("embedded")
        .active_catalog_ref()
        .expect("active catalog ref")
}

fn draft_target() -> TargetRef {
    TargetRef {
        id: "arch-x86_64".to_string(),
        version: "2026.10.0".to_string(),
    }
}

fn export_target(id: &str) -> TargetRef {
    TargetRef {
        id: id.to_string(),
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

fn valid_draft(core: &RealCore, draft_id: &str) -> Draft {
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: Some(draft_id.to_string()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create");
    draft.selections = vec![
        single("kernel", "archmaker.kernel.linux"),
        single("session", "archmaker.desktop.gnome"),
    ];
    draft
}

fn export_for(
    core: &RealCore,
    target_id: &str,
    draft_id: &str,
) -> (archmaker_domain::Artifact, Vec<u8>) {
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft: valid_draft(core, draft_id),
            catalog_ref: catalog_ref(),
            target_ref: export_target(target_id),
            expected_resolution_digest: None,
            request_id: None,
        })
        .unwrap_or_else(|e| panic!("manifest {target_id}: {e:?}"));
    let rendered = render_artifact_bytes(&manifest, &export_target(target_id)).expect("render");
    let artifact = core
        .export_artifact(ExportArtifactInput {
            manifest,
            target_ref: export_target(target_id),
            destination: DestinationHandle::new(format!("out-{target_id}")).expect("token"),
            request_id: None,
        })
        .unwrap_or_else(|e| panic!("export {target_id}: {e:?}"));
    (artifact, rendered.bytes)
}

#[test]
fn f2_export_carries_renderer_bytes_for_supported_targets() {
    let core = RealCore::with_embedded().expect("embedded");
    let cases = [
        ("archmaker-profile", "application/json"),
        ("archinstall-profile", "application/json"),
        ("report", "text/markdown"),
    ];
    let draft_ids = [
        "0d5e6b1a-f201-4a2b-9c3d-f2f2f2f2f201",
        "0d5e6b1a-f202-4a2b-9c3d-f2f2f2f2f202",
        "0d5e6b1a-f203-4a2b-9c3d-f2f2f2f2f203",
    ];
    for ((target, media), draft_id) in cases.into_iter().zip(draft_ids) {
        let (artifact, rendered) = export_for(&core, target, draft_id);
        let bytes = artifact.bytes.as_ref().expect("bytes presentes");
        assert_eq!(bytes.as_slice(), rendered.as_slice());
        assert_eq!(artifact.binary_digest.value, sha256_hex(bytes));
        assert_eq!(artifact.media_type, media);
        assert_eq!(artifact.artifact_ref.media_type, media);
    }
}

#[test]
fn f2_report_bytes_are_markdown_text_not_json_string() {
    let core = RealCore::with_embedded().expect("embedded");
    let (artifact, rendered) = export_for(&core, "report", "0d5e6b1a-f204-4a2b-9c3d-f2f2f2f2f204");
    let bytes = artifact.bytes.expect("bytes");
    assert_eq!(bytes, rendered);
    let text = std::str::from_utf8(&bytes).expect("utf8");
    assert!(text.starts_with("# ArchMaker report\n"));
    let content = artifact.content.expect("content");
    let as_json = serde_json::to_string(&content).expect("json");
    assert_ne!(as_json.as_bytes(), bytes.as_slice());
    assert!(!as_json.starts_with("# ArchMaker"));
}

#[test]
fn f2_incompatible_target_is_rejected() {
    let core = RealCore::with_embedded().expect("embedded");
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft: valid_draft(&core, "0d5e6b1a-f205-4a2b-9c3d-f2f2f2f2f205"),
            catalog_ref: catalog_ref(),
            target_ref: export_target("archmaker-profile"),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("manifest");
    let err = core
        .export_artifact(ExportArtifactInput {
            manifest,
            target_ref: TargetRef {
                id: "iso".to_string(),
                version: "0.1.0".to_string(),
            },
            destination: DestinationHandle::new("bad.iso".to_string()).expect("token"),
            request_id: None,
        })
        .expect_err("iso no es export");
    assert_eq!(err.code, "AM-TGT-001");
}

#[test]
fn f2_delivery_corpus_digests() {
    let dir = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/closure/delivery");
    let index: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(dir.join("fixture-digests.json")).expect("índice delivery"),
    )
    .expect("json");
    assert!(index.get("sha256").is_none());
    let min_per = index["minFixturesPerTarget"].as_u64().unwrap_or(20) as usize;
    let files = index["files"].as_object().expect("files");
    let mut per_target: std::collections::BTreeMap<String, usize> =
        std::collections::BTreeMap::new();
    let mut rejects = 0usize;
    for (name, meta) in files {
        let bytes = std::fs::read(dir.join(name)).unwrap_or_else(|_| panic!("falta {name}"));
        assert_eq!(
            bytes.len() as u64,
            meta["sizeBytes"].as_u64().expect("size")
        );
        assert_eq!(sha256_hex(&bytes), meta["sha256"].as_str().expect("sha"));
        let class = meta["class"].as_str().unwrap_or("");
        if class == "negative-bytes" {
            rejects += 1;
            continue;
        }
        if let Some(target) = meta["target"].as_str() {
            *per_target.entry(target.to_string()).or_default() += 1;
        }
    }
    for target in ["archmaker-profile", "archinstall-profile", "report"] {
        let n = per_target.get(target).copied().unwrap_or(0);
        assert!(n >= min_per, "corpus {target}: {n} < suelo {min_per}");
    }
    assert!(rejects >= 4, "corpus rejects: {rejects} < 4");
}
