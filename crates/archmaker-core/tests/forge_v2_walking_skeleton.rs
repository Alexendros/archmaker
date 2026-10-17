//! Walking skeleton fundacional: draft v2 manual → CorePort → buildManifest/export →
//! reimport vía migración v1→v2. Sin sonda Linux fingida ni escritura en discos.

use archmaker_core::{
    draft_content_digest, manifest_content_digest, migrate_draft_v1_to_v2_report,
    BuildManifestInput, CorePort, CreateDraftInput, DestinationHandle, ExportArtifactInput,
    RealCore, SaveDraftInput,
};
use archmaker_domain::{
    manual_reference_storage_plan, BrowserPolicy, Draft, EncryptionPolicy, ExecutionStatus,
    ManifestDomainLabel, Selection, SelectionValue, TargetRef, VerificationStatuses,
    DRAFT_SCHEMA_V2, MANIFEST_DOMAIN_LABEL_V2, ONLYOFFICE_CANONICAL_ID,
};

fn single(step: &str, option: &str) -> Selection {
    Selection {
        step_id: step.to_string(),
        value: SelectionValue::Single {
            option_id: option.to_string(),
        },
    }
}

fn export_target() -> TargetRef {
    TargetRef {
        id: "archmaker-profile".to_string(),
        version: "0.1.0".to_string(),
    }
}

fn draft_target() -> TargetRef {
    TargetRef {
        id: "arch-x86_64".to_string(),
        version: "2026.10.0".to_string(),
    }
}

fn manual_v2_draft(core: &RealCore) -> Draft {
    let catalog_ref = core.active_catalog_ref().expect("catalog ref");
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref,
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create v1 seed");
    // Elevación manual a v2 sin detección fingida.
    draft.schema_version = DRAFT_SCHEMA_V2.to_string();
    draft.selections = vec![
        single("kernel", "archmaker.kernel.linux"),
        single("session", "archmaker.desktop.gnome"),
    ];
    draft.storage_plan = Some(manual_reference_storage_plan());
    draft.encryption_policy = Some(EncryptionPolicy::none());
    draft.browser_policy = Some(BrowserPolicy {
        selected_browser_ids: vec!["archmaker.browser.firefox".into()],
        default_browser_id: Some("archmaker.browser.firefox".into()),
    });
    draft.verification = Some(VerificationStatuses::foundational_defaults());
    draft.inherited_encrypt_intent = None;
    draft
}

#[test]
fn onlyoffice_canonical_id_is_catalog_known() {
    assert_eq!(ONLYOFFICE_CANONICAL_ID, "archmaker.pkg.onlyoffice-bin");
    let desktop = RealCore::with_desktop().expect("desktop");
    let json = serde_json::to_string(desktop.catalog()).expect("catalog json");
    assert!(
        json.contains(ONLYOFFICE_CANONICAL_ID),
        "catálogo desktop debe contener ONLYOFFICE canónico"
    );
}

#[test]
fn walking_skeleton_v2_save_build_export_and_migrate() {
    let core = RealCore::with_embedded().expect("embedded");
    let mut draft = manual_v2_draft(&core);

    let digest = draft_content_digest(&draft).expect("draft digest v2");
    draft.content_digest = Some(digest.clone());
    assert_eq!(digest.algorithm, archmaker_domain::DigestAlgorithm::Sha256);

    let saved = core
        .save_draft(SaveDraftInput {
            draft: draft.clone(),
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect("save draft v2");
    assert_eq!(saved.schema_version, DRAFT_SCHEMA_V2);
    assert!(saved.storage_plan.is_some());
    assert_eq!(
        saved.verification.as_ref().map(|v| v.execution_status),
        Some(ExecutionStatus::NotExecuted)
    );

    let catalog_ref = core.active_catalog_ref().expect("catalog");
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft: saved.clone(),
            catalog_ref: catalog_ref.clone(),
            target_ref: export_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("build manifest v2");
    assert_eq!(manifest.schema_version, DRAFT_SCHEMA_V2);
    assert_eq!(manifest.domain_label, ManifestDomainLabel::V2);
    assert_eq!(manifest.domain_label.as_str(), MANIFEST_DOMAIN_LABEL_V2);
    assert!(manifest.storage_plan.is_some());
    assert!(manifest.browser_policy.is_some());
    let recomputed = manifest_content_digest(&manifest).expect("recompute");
    assert_eq!(recomputed.value, manifest.content_digest.value);

    let artifact = core
        .export_artifact(ExportArtifactInput {
            manifest: manifest.clone(),
            target_ref: export_target(),
            destination: DestinationHandle::new("forge-v2-skeleton.json".into()).expect("dest"),
            request_id: None,
        })
        .expect("export");
    assert_eq!(artifact.schema_version, DRAFT_SCHEMA_V2);
    assert!(artifact.bytes.as_ref().is_some_and(|b| !b.is_empty()));

    // Migración v1→v2 sobre un draft v1 del mismo núcleo (reimport semántico).
    let mut v1 = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref,
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("v1");
    v1.selections = saved.selections.clone();
    let (v2, report) = migrate_draft_v1_to_v2_report(v1).expect("migrate");
    assert!(!report.idempotent_noop);
    assert_eq!(v2.schema_version, DRAFT_SCHEMA_V2);
    assert_eq!(
        v2.verification.as_ref().map(|v| v.execution_status),
        Some(ExecutionStatus::NotExecuted)
    );
    let (_again, report2) = migrate_draft_v1_to_v2_report(v2).expect("idempotent");
    assert!(report2.idempotent_noop);
}

#[test]
fn rejects_future_schema_version_on_save() {
    let core = RealCore::with_embedded().expect("embedded");
    let mut draft = manual_v2_draft(&core);
    draft.schema_version = "3".to_string();
    let err = core
        .save_draft(SaveDraftInput {
            draft,
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect_err("schema 3 rejected");
    assert_eq!(err.code, "AM-SCHEMA-001");
}
