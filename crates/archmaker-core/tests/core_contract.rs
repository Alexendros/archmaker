use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

use archmaker_core::{
    catalog_content_digest, is_uuid, render_artifact_bytes, BuildManifestInput, CatalogSource,
    ChangeSet, ChangeSetOp, ChangeSetOperation, ChangeSetOrigin, CorePort, CreateDraftInput,
    DestinationHandle, ExportArtifactInput, ImportDraftInput, LoadCatalogInput, RealCore,
    ResolveDraftInput, SaveDraftInput, ValidateDraftInput,
};
use archmaker_domain::{
    CatalogRef, ContentDigest, ContentKind, DigestAlgorithm, Draft, Selection, SelectionValue,
    TargetRef,
};

fn digest_value(value: &str) -> ContentDigest {
    ContentDigest {
        algorithm: DigestAlgorithm::Sha256,
        value: value.to_string(),
        kind: Some(ContentKind::Content),
    }
}

fn catalog_ref() -> CatalogRef {
    CatalogRef {
        namespace: "archmaker.core".to_string(),
        id: "minimal".to_string(),
        version: "0.1.0".to_string(),
        digest: digest_value(&"0".repeat(64)),
    }
}

fn draft_target() -> TargetRef {
    TargetRef {
        id: "arch-x86_64".to_string(),
        version: "2026.10.0".to_string(),
    }
}

fn export_target() -> TargetRef {
    TargetRef {
        id: "archmaker-profile".to_string(),
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

fn valid_draft(core: &RealCore) -> Draft {
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: Some("0d5e6b1a-1111-4a2b-9c3d-111111111111".to_string()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create valid draft");
    draft.selections = vec![
        single("kernel", "archmaker.kernel.linux"),
        single("session", "archmaker.desktop.gnome"),
    ];
    draft
}

fn temp_dir(name: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let dir = std::env::temp_dir().join(format!(
        "archmaker-core-{name}-{}-{nanos}",
        std::process::id()
    ));
    std::fs::create_dir_all(&dir).expect("temp dir");
    dir
}

fn core_with_store(name: &str) -> (RealCore, PathBuf) {
    let dir = temp_dir(name);
    let core = RealCore::with_embedded()
        .expect("embedded catalog parses")
        .with_store_dir(dir.clone());
    (core, dir)
}

#[test]
fn create_draft_valid_defaults() {
    let core = RealCore::with_embedded().expect("embedded");
    let draft = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("valid create");
    assert_eq!(draft.revision, 0);
    assert!(draft.selections.is_empty());
    assert!(draft.content_digest.is_none());
    assert!(is_uuid(&draft.id));
}

#[test]
fn create_draft_honors_given_id() {
    let core = RealCore::with_embedded().expect("embedded");
    let id = "0d5e6b1a-1111-4a2b-9c3d-111111111111".to_string();
    let first = core
        .create_draft(CreateDraftInput {
            draft_id: Some(id.clone()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("valid create");
    let second = core
        .create_draft(CreateDraftInput {
            draft_id: Some(id.clone()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("valid create");
    assert_eq!(first.id, id);
    assert_eq!(second.id, id);
}

#[test]
fn create_draft_rejects_bad_target() {
    let core = RealCore::with_embedded().expect("embedded");
    let error = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref: catalog_ref(),
            target_ref: TargetRef {
                id: "iso".to_string(),
                version: "0.1.0".to_string(),
            },
            preset_ref: None,
            request_id: None,
        })
        .expect_err("unsupported target");
    assert_eq!(error.code, "AM-TGT-001");
}

#[test]
fn create_draft_rejects_unknown_catalog_and_bad_id() {
    let core = RealCore::with_embedded().expect("embedded");
    let mut unknown = catalog_ref();
    unknown.version = "9.9.9".to_string();
    let error = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref: unknown,
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect_err("unknown catalog");
    assert_eq!(error.code, "AM-CAT-001");
    let error = core
        .create_draft(CreateDraftInput {
            draft_id: Some("not-a-uuid".to_string()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect_err("bad draft id");
    assert_eq!(error.code, "AM-SCHEMA-001");
}

#[test]
fn load_catalog_embedded_and_digest() {
    let core = RealCore::with_embedded().expect("embedded");
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
        .expect("embedded loads");
    assert_eq!(catalog.id, "minimal");
    let expected = catalog_content_digest(&catalog).expect("digest");
    let verified = core
        .load_catalog(LoadCatalogInput {
            source: CatalogSource::Embedded,
            catalog_ref: Some(catalog_ref()),
            expected_digest: Some(expected),
            file_path: None,
            expected_provenance: None,
            limits: None,
            request_id: None,
        })
        .expect("digest verifies");
    assert_eq!(verified.version, "0.1.0");
}

#[test]
fn load_catalog_digest_mismatch() {
    let core = RealCore::with_embedded().expect("embedded");
    let error = core
        .load_catalog(LoadCatalogInput {
            source: CatalogSource::Embedded,
            catalog_ref: None,
            expected_digest: Some(digest_value(&"f".repeat(64))),
            file_path: None,
            expected_provenance: None,
            limits: None,
            request_id: None,
        })
        .expect_err("mismatch");
    assert_eq!(error.code, "AM-CAT-002");
}

#[test]
fn load_catalog_file_round_trip_and_missing() {
    use archmaker_core::CatalogProvenance;
    let (core, dir) = core_with_store("load-file");
    let path = dir.join("catalog.json");
    std::fs::write(&path, archmaker_core::EMBEDDED_CATALOG_JSON).expect("write fixture");
    let provenance = CatalogProvenance {
        source_id: "archmaker.repo.fixtures".into(),
        path: "crates/archmaker-core/fixtures/embedded-catalog.json".into(),
        captured_at: "2026-10-06T00:00:00Z".into(),
    };
    let catalog = core
        .load_catalog(LoadCatalogInput {
            source: CatalogSource::File,
            catalog_ref: Some(catalog_ref()),
            expected_digest: None,
            file_path: Some(path),
            expected_provenance: Some(provenance.clone()),
            limits: None,
            request_id: None,
        })
        .expect("file loads");
    assert_eq!(catalog.namespace, "archmaker.core");
    assert!(catalog.provenance.is_some());
    let error = core
        .load_catalog(LoadCatalogInput {
            source: CatalogSource::File,
            catalog_ref: None,
            expected_digest: None,
            file_path: Some(dir.join("catalog.json")),
            expected_provenance: None,
            limits: None,
            request_id: None,
        })
        .expect_err("file without provenance");
    assert_eq!(error.code, "AM-CAT-004");
    let error = core
        .load_catalog(LoadCatalogInput {
            source: CatalogSource::File,
            catalog_ref: None,
            expected_digest: None,
            file_path: Some(dir.join("absent.json")),
            expected_provenance: Some(provenance),
            limits: None,
            request_id: None,
        })
        .expect_err("missing file");
    assert_eq!(error.code, "AM-IO-001");
    let error = core
        .load_catalog(LoadCatalogInput {
            source: CatalogSource::File,
            catalog_ref: None,
            expected_digest: None,
            file_path: None,
            expected_provenance: None,
            limits: None,
            request_id: None,
        })
        .expect_err("file without path");
    assert_eq!(error.code, "AM-CAT-004");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_draft_increments_and_persists() {
    let (core, dir) = core_with_store("save");
    let draft = valid_draft(&core);
    let saved = core
        .save_draft(SaveDraftInput {
            draft,
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect("first save");
    assert_eq!(saved.revision, 1);
    assert!(saved.content_digest.is_some());
    assert!(dir.join(format!("{}.json", saved.id)).exists());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn save_draft_conflict_on_stale_revision() {
    let (core, dir) = core_with_store("conflict");
    let draft = valid_draft(&core);
    let saved = core
        .save_draft(SaveDraftInput {
            draft: draft.clone(),
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect("first save");
    assert_eq!(saved.revision, 1);
    let conflict = core
        .save_draft(SaveDraftInput {
            draft,
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect_err("stale retry conflicts");
    assert_eq!(conflict.code, "AM-DOC-002");
    let conflict = core
        .save_draft(SaveDraftInput {
            draft: saved,
            expected_revision: 7,
            changeset: None,
            request_id: None,
        })
        .expect_err("unknown future revision conflicts");
    assert_eq!(conflict.code, "AM-DOC-002");
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn resolve_draft_valid_and_deterministic() {
    let core = RealCore::with_embedded().expect("embedded");
    let draft = valid_draft(&core);
    let first = core
        .resolve_draft(ResolveDraftInput {
            draft: draft.clone(),
            catalog_ref: catalog_ref(),
            request_id: None,
        })
        .expect("resolves");
    let second = core
        .resolve_draft(ResolveDraftInput {
            draft,
            catalog_ref: catalog_ref(),
            request_id: None,
        })
        .expect("resolves again");
    assert_eq!(first.resolution_digest, second.resolution_digest);
    assert_eq!(first.effective_selections.len(), 2);
    assert!(first.conflicts.is_empty());
    let keys: Vec<(&str, &str, &str)> = first
        .diagnostics
        .iter()
        .map(|diagnostic| {
            (
                diagnostic.path.as_str(),
                diagnostic.code.as_str(),
                diagnostic.source.as_str(),
            )
        })
        .collect();
    let mut sorted = keys.clone();
    sorted.sort();
    assert_eq!(keys, sorted);
}

#[test]
fn resolve_draft_rejects_unknown_option() {
    let core = RealCore::with_embedded().expect("embedded");
    let mut draft = valid_draft(&core);
    draft.selections.push(single("pkg", "archmaker.pkg.absent"));
    let error = core
        .resolve_draft(ResolveDraftInput {
            draft,
            catalog_ref: catalog_ref(),
            request_id: None,
        })
        .expect_err("unknown option");
    assert_eq!(error.code, "AM-RES-001");
}

#[test]
fn validate_draft_clean_and_blocking() {
    let core = RealCore::with_embedded().expect("embedded");
    let clean = core
        .validate_draft(ValidateDraftInput {
            draft: valid_draft(&core),
            catalog_ref: catalog_ref(),
            target_ref: None,
            request_id: None,
        })
        .expect("validates");
    assert!(!clean.blocking);
    let mut blocking_draft = valid_draft(&core);
    blocking_draft.selections = vec![Selection {
        step_id: "compositor".to_string(),
        value: SelectionValue::Multiple {
            option_ids: vec![
                "archmaker.kernel.linux".to_string(),
                "archmaker.desktop.gnome".to_string(),
            ],
        },
    }];
    let blocking = core
        .validate_draft(ValidateDraftInput {
            draft: blocking_draft,
            catalog_ref: catalog_ref(),
            target_ref: None,
            request_id: None,
        })
        .expect("validates blocking");
    assert!(blocking.blocking);
    assert!(blocking
        .diagnostics
        .iter()
        .any(|diagnostic| diagnostic.code == "AM-RULE-001" && diagnostic.blocking));
}

#[test]
fn build_manifest_sealed_and_blocked() {
    let core = RealCore::with_embedded().expect("embedded");
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft: valid_draft(&core),
            catalog_ref: catalog_ref(),
            target_ref: export_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("builds");
    assert_eq!(manifest.domain_label, archmaker_domain::ManifestDomainLabel);
    assert!(manifest.conflicts.is_empty());
    let recomputed = archmaker_core::manifest_content_digest(&manifest).expect("recomputes digest");
    assert_eq!(recomputed, manifest.content_digest);
    let mut blocked = valid_draft(&core);
    blocked.selections = vec![Selection {
        step_id: "compositor".to_string(),
        value: SelectionValue::Multiple {
            option_ids: vec![
                "archmaker.kernel.linux".to_string(),
                "archmaker.desktop.gnome".to_string(),
            ],
        },
    }];
    let error = core
        .build_manifest(BuildManifestInput {
            draft: blocked,
            catalog_ref: catalog_ref(),
            target_ref: export_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect_err("blocked build");
    assert_eq!(error.code, "AM-RES-002");
    let error = core
        .build_manifest(BuildManifestInput {
            draft: valid_draft(&core),
            catalog_ref: catalog_ref(),
            target_ref: TargetRef {
                id: "iso".to_string(),
                version: "0.1.0".to_string(),
            },
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect_err("bad target");
    assert_eq!(error.code, "AM-TGT-001");
}

#[test]
fn build_manifest_honors_expected_resolution_digest() {
    let core = RealCore::with_embedded().expect("embedded");
    let draft = valid_draft(&core);
    let resolved = core
        .resolve_draft(ResolveDraftInput {
            draft: draft.clone(),
            catalog_ref: catalog_ref(),
            request_id: None,
        })
        .expect("resolves");
    core.build_manifest(BuildManifestInput {
        draft: draft.clone(),
        catalog_ref: catalog_ref(),
        target_ref: export_target(),
        expected_resolution_digest: Some(resolved.resolution_digest),
        request_id: None,
    })
    .expect("matching digest builds");
    let error = core
        .build_manifest(BuildManifestInput {
            draft,
            catalog_ref: catalog_ref(),
            target_ref: export_target(),
            expected_resolution_digest: Some(digest_value(&"a".repeat(64))),
            request_id: None,
        })
        .expect_err("other digest refuses");
    assert_eq!(error.code, "AM-RES-002");
}

#[test]
fn export_artifact_byte_idempotent() {
    let (core, dir) = core_with_store("export");
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft: valid_draft(&core),
            catalog_ref: catalog_ref(),
            target_ref: export_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("builds");
    let first = core
        .export_artifact(ExportArtifactInput {
            manifest: manifest.clone(),
            target_ref: export_target(),
            destination: DestinationHandle::new("artifact-a.json".to_string()).expect("handle"),
            request_id: None,
        })
        .expect("exports");
    let second = core
        .export_artifact(ExportArtifactInput {
            manifest: manifest.clone(),
            target_ref: export_target(),
            destination: DestinationHandle::new("artifact-b.json".to_string()).expect("handle"),
            request_id: None,
        })
        .expect("exports again");
    assert_eq!(first.binary_digest, second.binary_digest);
    assert_eq!(first.manifest_ref.content_digest, manifest.content_digest);
    let bytes_a = std::fs::read(dir.join("artifact-a.json")).expect("reads a");
    let bytes_b = std::fs::read(dir.join("artifact-b.json")).expect("reads b");
    assert_eq!(bytes_a, bytes_b);
    let (rendered, _) = render_artifact_bytes(&manifest, &export_target()).expect("renders");
    assert_eq!(rendered, bytes_a);
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn export_artifact_rejects_bad_target_and_handle() {
    let (core, dir) = core_with_store("export-bad");
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft: valid_draft(&core),
            catalog_ref: catalog_ref(),
            target_ref: export_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("builds");
    let error = core
        .export_artifact(ExportArtifactInput {
            manifest: manifest.clone(),
            target_ref: TargetRef {
                id: "iso".to_string(),
                version: "0.1.0".to_string(),
            },
            destination: DestinationHandle::new("out.json".to_string()).expect("handle"),
            request_id: None,
        })
        .expect_err("bad target");
    assert_eq!(error.code, "AM-TGT-001");
    assert!(DestinationHandle::new("/absolute/path".to_string()).is_err());
    assert!(DestinationHandle::new("../escape".to_string()).is_err());
    assert!(DestinationHandle::new(String::new()).is_err());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn end_to_end_create_save_resolve_validate_build_export() {
    let (core, dir) = core_with_store("e2e");
    let created = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("creates");
    let mut draft = created;
    draft.selections = vec![
        single("kernel", "archmaker.kernel.linux"),
        single("session", "archmaker.desktop.gnome"),
    ];
    let saved = core
        .save_draft(SaveDraftInput {
            draft,
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect("saves");
    assert_eq!(saved.revision, 1);
    let validation = core
        .validate_draft(ValidateDraftInput {
            draft: saved.clone(),
            catalog_ref: catalog_ref(),
            target_ref: None,
            request_id: None,
        })
        .expect("validates");
    assert!(!validation.blocking);
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft: saved,
            catalog_ref: catalog_ref(),
            target_ref: export_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("builds");
    let artifact = core
        .export_artifact(ExportArtifactInput {
            manifest,
            target_ref: export_target(),
            destination: DestinationHandle::new("e2e.json".to_string()).expect("handle"),
            request_id: None,
        })
        .expect("exports");
    assert!(dir.join("e2e.json").exists());
    assert_eq!(artifact.media_type, "application/json");
    let _ = std::fs::remove_dir_all(&dir);
}

fn desktop_catalog() -> archmaker_domain::Catalog {
    serde_json::from_str(include_str!("../fixtures/local-catalog-mvp1.json"))
        .expect("local catalog deserializes")
}

fn desktop_catalog_ref() -> CatalogRef {
    CatalogRef {
        namespace: "archmaker.core".to_string(),
        id: "desktop".to_string(),
        version: "1.0.0".to_string(),
        digest: digest_value(&"1".repeat(64)),
    }
}

#[test]
fn load_local_catalog_with_provenance() {
    use archmaker_core::CatalogProvenance;
    let (core, dir) = core_with_store("local-cat");
    let path = dir.join("local-catalog-mvp1.json");
    std::fs::write(&path, include_str!("../fixtures/local-catalog-mvp1.json")).expect("write");
    let provenance = CatalogProvenance {
        source_id: "archmaker.repo.fixtures".into(),
        path: "crates/archmaker-core/fixtures/local-catalog-mvp1.json".into(),
        captured_at: "2026-10-12T00:00:00Z".into(),
    };
    let catalog = core
        .load_catalog(LoadCatalogInput {
            source: CatalogSource::File,
            catalog_ref: Some(desktop_catalog_ref()),
            expected_digest: None,
            file_path: Some(path),
            expected_provenance: Some(provenance),
            limits: None,
            request_id: None,
        })
        .expect("local catalog loads");
    assert_eq!(catalog.id, "desktop");
    assert!(catalog.steps.len() >= 8);
    assert!(catalog.provenance.is_some());
    let _ = std::fs::remove_dir_all(&dir);
}

#[test]
fn import_draft_migrates_v51_sample() {
    let core = RealCore::new(desktop_catalog());
    let bytes = include_bytes!(
        "../../../contracts/json-schema/examples/migration/v5.1-instance.sample.json"
    );
    let result = core
        .import_draft(ImportDraftInput {
            source_bytes: bytes.to_vec(),
            catalog_ref: desktop_catalog_ref(),
            target_ref: draft_target(),
            draft_id: Some("99999999-9999-4999-8999-999999999999".into()),
            confirm_blocking_losses: true,
            request_id: None,
        })
        .expect("import succeeds");
    assert_eq!(result.draft.document_version, "vNext");
    assert_eq!(result.changeset.origin, Some(ChangeSetOrigin::Migration));
    assert!(result
        .draft
        .selections
        .iter()
        .any(|s| matches!(s.value, SelectionValue::Multiple { .. })));
    assert_eq!(result.original_digest.len(), 64);
    // Original bytes remain identical (NFR-MIG-001).
    assert_eq!(
        result.original_digest,
        archmaker_canonicalization::digest(bytes, "archmaker:import:original:v1")
    );
}

#[test]
fn save_draft_applies_changeset_replace() {
    let core = RealCore::with_embedded().expect("embedded");
    let mut draft = valid_draft(&core);
    draft.selections = vec![single("archmaker.sys.kernel", "archmaker.kernel.linux")];
    let changeset = ChangeSet {
        schema_version: "1".into(),
        base_revision: Some(0),
        target_draft_id: Some(draft.id.clone()),
        origin: Some(ChangeSetOrigin::Manual),
        operations: vec![ChangeSetOperation {
            op_id: "op-1".into(),
            op: ChangeSetOp::Replace,
            path: "/selections/0/value".into(),
            selector: None,
            from: None,
            before: Some(SelectionValue::Single {
                option_id: "archmaker.kernel.linux".into(),
            }),
            after: Some(SelectionValue::Single {
                option_id: "archmaker.kernel.linux".into(),
            }),
            note: None,
        }],
        summary: None,
    };
    let saved = core
        .save_draft(SaveDraftInput {
            draft,
            expected_revision: 0,
            changeset: Some(changeset),
            request_id: None,
        })
        .expect("save with changeset");
    assert_eq!(saved.revision, 1);
}
