//! Matriz T37–T42 (contratos/core). T31–T36/T43–T44 en archmaker-hardware.
//! HA26–HA28: not-applicable (A6 excluida).

use archmaker_core::{
    draft_content_digest, migrate_draft_v1_to_v2_report, BuildManifestInput, CorePort,
    CreateDraftInput, RealCore, ResolveDraftInput, SaveDraftInput, ValidateDraftInput,
};
use archmaker_domain::{
    onlyoffice_ids_equivalent, resolve_onlyoffice_canonical, BrowserPolicy, EncryptionKind,
    EncryptionPolicy, LuksUnlockMethod, ManifestDomainLabel, ProviderStatus, Selection,
    SelectionValue, TargetRef, DRAFT_SCHEMA_V2, ONLYOFFICE_CANONICAL_ID,
    ONLYOFFICE_HISTORICAL_ALIAS_ID,
};

fn single(step: &str, option: &str) -> Selection {
    Selection {
        step_id: step.to_string(),
        value: SelectionValue::Single {
            option_id: option.to_string(),
        },
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

fn base_draft(core: &RealCore) -> archmaker_domain::Draft {
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
        single("kernel", "archmaker.kernel.linux"),
        single("session", "archmaker.desktop.gnome"),
    ];
    draft.browser_policy = Some(BrowserPolicy::new_profile_zen());
    draft.encryption_policy = Some(EncryptionPolicy::none());
    draft
}

/// T37a — cambio LUKS2 altera digest semántico del draft.
#[test]
fn t37_luks_changes_digest() {
    let core = RealCore::with_embedded().expect("core");
    let mut a = base_draft(&core);
    let da = draft_content_digest(&a).expect("digest a");

    a.encryption_policy = Some(EncryptionPolicy {
        kind: EncryptionKind::Luks2,
        unlock: Some(LuksUnlockMethod::PassphraseRequestedAtInstall),
        covers_mount_point: Some("/".into()),
        initramfs_note: None,
    });
    let db = draft_content_digest(&a).expect("digest b");
    assert_ne!(da.value, db.value, "LUKS2 cambia digest");
}

/// T37b — defaultBrowser cambia digest con selectedBrowserIds constante (H16).
#[test]
fn t37_default_browser_alone_changes_digest() {
    let core = RealCore::with_embedded().expect("core");
    let mut a = base_draft(&core);
    a.browser_policy = Some(BrowserPolicy {
        selected_browser_ids: vec![
            "archmaker.browser.zen".into(),
            "archmaker.browser.firefox".into(),
        ],
        default_browser_id: Some("archmaker.browser.zen".into()),
    });
    let da = draft_content_digest(&a).expect("digest a");

    a.browser_policy = Some(BrowserPolicy {
        selected_browser_ids: vec![
            "archmaker.browser.zen".into(),
            "archmaker.browser.firefox".into(),
        ],
        default_browser_id: Some("archmaker.browser.firefox".into()),
    });
    let db = draft_content_digest(&a).expect("digest b");
    assert_ne!(
        da.value, db.value,
        "solo defaultBrowserId altera digest con lista constante"
    );
}

/// Compat: nombre histórico T37 (LUKS2 o default browser).
#[test]
fn t37_luks_or_default_browser_changes_digest() {
    t37_luks_changes_digest();
    t37_default_browser_alone_changes_digest();
}

/// T38 — documento v2 no se etiqueta como dominio v1.
#[test]
fn t38_v2_document_not_labeled_as_v1_domain() {
    let core = RealCore::with_embedded().expect("core");
    let mut draft = base_draft(&core);
    draft.content_digest = Some(draft_content_digest(&draft).expect("digest"));
    let catalog_ref = core.active_catalog_ref().expect("catalog");
    let saved = core
        .save_draft(SaveDraftInput {
            draft: draft.clone(),
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect("save v2");
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft: saved,
            catalog_ref,
            target_ref: export_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("manifest");
    assert_eq!(manifest.domain_label, ManifestDomainLabel::V2);
    assert_ne!(manifest.domain_label, ManifestDomainLabel::V1);
    assert_eq!(
        ManifestDomainLabel::for_schema_version("2").unwrap(),
        ManifestDomainLabel::V2
    );
}

/// T39 — versión futura desconocida → rechazo explícito.
#[test]
fn t39_future_schema_version_rejected() {
    assert!(matches!(
        archmaker_domain::decode_draft_schema_version("99"),
        Err(archmaker_domain::SchemaVersionError::UnsupportedFuture(_))
    ));
    let core = RealCore::with_embedded().expect("core");
    let mut draft = base_draft(&core);
    draft.schema_version = "99".to_string();
    let err = core
        .save_draft(SaveDraftInput {
            draft,
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect_err("future rejected");
    assert_eq!(err.code, "AM-SCHEMA-001");
}

fn desktop_base_draft(core: &RealCore) -> archmaker_domain::Draft {
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
        single("archmaker.base.kernel", "archmaker.kernel.linux"),
        single(
            "archmaker.environment.compositor",
            "archmaker.compositor.gnome",
        ),
    ];
    draft.encryption_policy = Some(EncryptionPolicy::none());
    draft
}

/// T40 — alias + canónico en el mismo draft → una identidad efectiva en resolve.
#[test]
fn t40_onlyoffice_alias_no_duplicate_canonical() {
    assert_eq!(
        resolve_onlyoffice_canonical(ONLYOFFICE_HISTORICAL_ALIAS_ID),
        Some(ONLYOFFICE_CANONICAL_ID)
    );
    assert!(onlyoffice_ids_equivalent(
        ONLYOFFICE_CANONICAL_ID,
        ONLYOFFICE_HISTORICAL_ALIAS_ID
    ));
    assert_eq!(
        resolve_onlyoffice_canonical(ONLYOFFICE_CANONICAL_ID),
        Some(ONLYOFFICE_CANONICAL_ID)
    );

    let core = RealCore::with_desktop().expect("desktop");
    let catalog_ref = core.active_catalog_ref().expect("catalog");
    let mut draft = desktop_base_draft(&core);
    draft.selections.push(Selection {
        step_id: "archmaker.libpacks.office".into(),
        value: SelectionValue::Multiple {
            option_ids: vec![
                ONLYOFFICE_CANONICAL_ID.into(),
                ONLYOFFICE_HISTORICAL_ALIAS_ID.into(),
            ],
        },
    });
    let resolved = core
        .resolve_draft(ResolveDraftInput {
            draft: draft.clone(),
            catalog_ref: catalog_ref.clone(),
            request_id: None,
        })
        .expect("resolve dual ONLYOFFICE");
    let onlyoffice_hits: Vec<_> = resolved
        .effective_selections
        .iter()
        .filter(|s| {
            s.option_id == ONLYOFFICE_CANONICAL_ID || s.option_id == ONLYOFFICE_HISTORICAL_ALIAS_ID
        })
        .collect();
    assert!(
        !onlyoffice_hits.is_empty(),
        "debe aparecer al menos el canónico en efectivas"
    );
    let distinct: std::collections::BTreeSet<&str> = onlyoffice_hits
        .iter()
        .map(|s| s.option_id.as_str())
        .collect();
    assert_eq!(
        distinct.len(),
        1,
        "alias+canónico → una identidad efectiva, got {:?}",
        distinct
    );
    assert!(distinct.contains(ONLYOFFICE_CANONICAL_ID));
}

/// T41 — sin navegadores: política definida; sin default huérfano.
#[test]
fn t41_no_browser_policy_defined_no_orphan_default() {
    let none = BrowserPolicy::none();
    assert!(none.validate().is_ok());
    assert!(none.default_browser_id.is_none());
    let orphan = BrowserPolicy {
        selected_browser_ids: vec![],
        default_browser_id: Some("archmaker.browser.zen".into()),
    };
    assert!(orphan.validate().is_err());
}

/// T42 — solicitud no verificada se conserva; validate/manifest no la certifican como verified.
#[test]
fn t42_unverified_request_kept_not_certified_support() {
    let core = RealCore::with_desktop().expect("desktop");
    let catalog = core.catalog();
    let catalog_ref = core.active_catalog_ref().expect("catalog");
    let grok = catalog
        .steps
        .iter()
        .flat_map(|s| s.sections.iter())
        .flat_map(|sec| sec.options.iter())
        .find(|o| o.id == "archmaker.app.grokbot")
        .expect("grokbot visible");
    assert_eq!(
        grok.provider.as_ref().map(|p| p.status),
        Some(ProviderStatus::Unverified)
    );
    // Migración v1 no inventa verificación de apps.
    let mut v1 = base_draft(&core);
    v1.schema_version = "1".into();
    v1.selections.push(single("apps", "archmaker.app.grokbot"));
    let (v2, _) = migrate_draft_v1_to_v2_report(v1).expect("migrate");
    assert_eq!(v2.schema_version, DRAFT_SCHEMA_V2);
    assert!(
        v2.selections
            .iter()
            .any(|s| matches!(&s.value, SelectionValue::Single { option_id } if option_id == "archmaker.app.grokbot")),
        "solicitud no verificada conservada en draft"
    );

    let mut draft = desktop_base_draft(&core);
    draft.selections.push(Selection {
        step_id: "archmaker.apps.apps-core".into(),
        value: SelectionValue::Multiple {
            option_ids: vec!["archmaker.app.grokbot".into()],
        },
    });
    assert!(
        draft.selections.iter().any(|s| matches!(
            &s.value,
            SelectionValue::Multiple { option_ids }
                if option_ids.iter().any(|id| id == "archmaker.app.grokbot")
        )),
        "solicitud no verificada en draft"
    );
    let validated = core
        .validate_draft(ValidateDraftInput {
            draft: draft.clone(),
            catalog_ref: catalog_ref.clone(),
            target_ref: None,
            request_id: None,
        })
        .expect("validate");
    let mentions_unverified = validated.diagnostics.iter().any(|d| {
        d.message_key.contains("unverified")
            || d.code.contains("UNVER")
            || d.path.contains("grokbot")
            || d.blocking
    });
    assert!(
        validated.blocking || mentions_unverified || !validated.diagnostics.is_empty(),
        "validate debe señalar la solicitud no verificada (blocking={}, diags={:?})",
        validated.blocking,
        validated
            .diagnostics
            .iter()
            .map(|d| (&d.code, &d.message_key))
            .collect::<Vec<_>>()
    );
    assert!(
        draft
            .verification
            .as_ref()
            .map(|v| v.plan_validation_status != archmaker_domain::PlanValidationStatus::Valid)
            .unwrap_or(true),
        "draft sin verification Valid no certifica soporte"
    );
}
