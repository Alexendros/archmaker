//! F1 — fidelidad storagePlan v2: intención de cara → draft → digest → reabrir.

use archmaker_core::{
    draft_content_digest, CorePort, CreateDraftInput, OpenDraftInput, RealCore, SaveDraftInput,
};
use archmaker_domain::{
    declared_storage_plan_from_intent, PlanCertifiabilityStatus, Selection, SelectionValue,
    StorageMode, TargetRef, DRAFT_SCHEMA_V2,
};

fn draft_target() -> TargetRef {
    TargetRef {
        id: "arch-x86_64".to_string(),
        version: "2026.10.0".to_string(),
    }
}

fn base_v2(core: &RealCore) -> archmaker_domain::Draft {
    let catalog_ref = core.active_catalog_ref().expect("catalog");
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref,
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create");
    draft.schema_version = DRAFT_SCHEMA_V2.to_string();
    draft.selections = vec![
        Selection {
            step_id: "kernel".into(),
            value: SelectionValue::Single {
                option_id: "archmaker.kernel.linux".into(),
            },
        },
        Selection {
            step_id: "archmaker.face.apps".into(),
            value: SelectionValue::Multiple {
                option_ids: vec![
                    "archmaker.browser.zen".into(),
                    "archmaker.browser.firefox".into(),
                ],
            },
        },
        Selection {
            step_id: "archmaker.face.encrypt".into(),
            value: SelectionValue::Boolean { value: true },
        },
    ];
    draft
}

#[test]
fn storage_mode_round_trip_and_digest_changes() {
    let core = RealCore::with_embedded().expect("core");
    let mut draft = base_v2(&core);
    draft.storage_plan = Some(declared_storage_plan_from_intent(
        StorageMode::NewDisk,
        true,
        None,
    ));
    let saved = core
        .save_draft(SaveDraftInput {
            draft: draft.clone(),
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect("save new-disk");
    assert_eq!(
        saved.storage_plan.as_ref().map(|p| p.mode),
        Some(StorageMode::NewDisk)
    );
    assert_eq!(
        saved.storage_plan.as_ref().map(|p| p.plan_status),
        Some(PlanCertifiabilityStatus::VerificationPending)
    );
    assert_ne!(
        saved.storage_plan.as_ref().map(|p| p.plan_status),
        Some(PlanCertifiabilityStatus::Certifiable)
    );
    assert_eq!(
        saved.encryption_policy.as_ref().map(|p| p.kind),
        Some(archmaker_domain::EncryptionKind::Luks2)
    );
    assert!(saved.browser_policy.as_ref().is_some_and(|b| b
        .selected_browser_ids
        .contains(&"archmaker.browser.zen".into())));

    let d1 = draft_content_digest(&saved).expect("d1");
    let mut replaced = saved.clone();
    replaced.storage_plan = Some(declared_storage_plan_from_intent(
        StorageMode::ReplaceDisk,
        true,
        Some(true),
    ));
    let saved2 = core
        .save_draft(SaveDraftInput {
            draft: replaced,
            expected_revision: saved.revision,
            changeset: None,
            request_id: None,
        })
        .expect("save replace");
    let d2 = draft_content_digest(&saved2).expect("d2");
    assert_ne!(d1.value, d2.value, "cambiar modo altera digest");

    let opened = core
        .open_draft(OpenDraftInput {
            draft_id: saved2.id.clone(),
            seed: None,
            request_id: None,
        })
        .expect("reopen");
    assert_eq!(
        opened.storage_plan.as_ref().map(|p| p.mode),
        Some(StorageMode::ReplaceDisk)
    );
}

#[test]
fn incomplete_replace_without_loss_ack_is_not_certifiable_and_saveable() {
    let core = RealCore::with_embedded().expect("core");
    let mut draft = base_v2(&core);
    draft.storage_plan = Some(declared_storage_plan_from_intent(
        StorageMode::ReplaceDisk,
        true,
        Some(false),
    ));
    let saved = core
        .save_draft(SaveDraftInput {
            draft,
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect("incomplete replace must save");
    let plan = saved.storage_plan.expect("plan");
    assert_eq!(plan.plan_status, PlanCertifiabilityStatus::NotCertifiable);
    assert_ne!(plan.plan_status, PlanCertifiabilityStatus::Certifiable);
}

#[test]
fn mode_without_intent_ack_never_certifiable() {
    let plan = declared_storage_plan_from_intent(StorageMode::NewDisk, false, None);
    assert_eq!(plan.plan_status, PlanCertifiabilityStatus::NotCertifiable);
}
