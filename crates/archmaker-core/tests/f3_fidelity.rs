//! F3 — fidelidad semántica: hostname/cifrado, refs de catálogo y exclusiones.

use archmaker_core::{
    BuildManifestInput, CorePort, CreateDraftInput, RealCore, ResolveDraftInput, ValidateDraftInput,
};
use archmaker_domain::{
    CatalogRef, ContentDigest, ContentKind, DigestAlgorithm, Draft, Selection, SelectionValue,
    TargetRef,
};

fn catalog_ref(core: &RealCore) -> CatalogRef {
    core.active_catalog_ref().expect("active catalog ref")
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
            draft_id: Some("0d5e6b1a-3333-4a2b-9c3d-333333333333".to_string()),
            catalog_ref: catalog_ref(core),
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

#[test]
fn hostname_and_encrypt_intention_reach_manifest() {
    let core = RealCore::with_embedded().expect("embedded");
    let mut draft = valid_draft(&core);
    draft.selections.push(Selection {
        step_id: "archmaker.face.hostname".to_string(),
        value: SelectionValue::Text {
            value: "arch-neubat".to_string(),
        },
    });
    draft.selections.push(Selection {
        step_id: "archmaker.face.encrypt".to_string(),
        value: SelectionValue::Boolean { value: true },
    });

    let resolved = core
        .resolve_draft(ResolveDraftInput {
            draft: draft.clone(),
            catalog_ref: catalog_ref(&core),
            request_id: None,
        })
        .expect("resolve");
    let host = resolved
        .effective_selections
        .iter()
        .find(|s| s.step_id == "archmaker.face.hostname")
        .expect("hostname en effective");
    assert!(matches!(
        &host.value,
        SelectionValue::Text { value } if value == "arch-neubat"
    ));
    let enc = resolved
        .effective_selections
        .iter()
        .find(|s| s.step_id == "archmaker.face.encrypt")
        .expect("encrypt en effective");
    assert!(matches!(
        &enc.value,
        SelectionValue::Boolean { value: true }
    ));

    let manifest = core
        .build_manifest(BuildManifestInput {
            draft,
            catalog_ref: catalog_ref(&core),
            target_ref: export_target(),
            expected_resolution_digest: Some(resolved.resolution_digest),
            request_id: None,
        })
        .expect("manifest");
    let json = serde_json::to_string_pretty(&manifest).expect("json");
    assert!(json.contains("arch-neubat"));
    assert!(json.contains("\"kind\": \"boolean\""));
    assert!(json.contains("archmaker.face.encrypt"));
    assert!(!json.to_lowercase().contains("disk encrypted"));
}

#[test]
fn unsupported_number_and_secret_ref_emit_exclusion_diagnostic() {
    let core = RealCore::with_embedded().expect("embedded");
    let mut draft = valid_draft(&core);
    draft.selections.push(Selection {
        step_id: "archmaker.face.unused-number".to_string(),
        value: SelectionValue::Number {
            value: 42.0,
            unit: None,
        },
    });
    draft.selections.push(Selection {
        step_id: "archmaker.face.unused-secret".to_string(),
        value: SelectionValue::SecretRef {
            reference: "vault://x".to_string(),
        },
    });

    let validated = core
        .validate_draft(ValidateDraftInput {
            draft: draft.clone(),
            catalog_ref: catalog_ref(&core),
            target_ref: None,
            request_id: None,
        })
        .expect("validate");
    let codes: Vec<_> = validated
        .diagnostics
        .iter()
        .filter(|d| d.code == "AM-FACE-001")
        .map(|d| d.path.as_str())
        .collect();
    assert!(
        codes.iter().any(|p| p.contains("unused-number")),
        "number excluido con diagnóstico"
    );
    assert!(
        codes.iter().any(|p| p.contains("unused-secret")),
        "secretRef excluido con diagnóstico"
    );
    assert!(!validated.blocking, "exclusiones face no bloquean");

    let resolved = core
        .resolve_draft(ResolveDraftInput {
            draft,
            catalog_ref: catalog_ref(&core),
            request_id: None,
        })
        .expect("resolve");
    assert!(!resolved
        .effective_selections
        .iter()
        .any(|s| s.step_id.contains("unused")));
}

#[test]
fn catalog_ref_ok_digest_version_and_id() {
    let core = RealCore::with_embedded().expect("embedded");
    let draft = valid_draft(&core);
    let ok = catalog_ref(&core);

    core.resolve_draft(ResolveDraftInput {
        draft: draft.clone(),
        catalog_ref: ok.clone(),
        request_id: None,
    })
    .expect("ref correcta");

    let mut bad_digest = ok.clone();
    bad_digest.digest = ContentDigest {
        algorithm: DigestAlgorithm::Sha256,
        value: "f".repeat(64),
        kind: Some(ContentKind::Content),
    };
    let err = core
        .resolve_draft(ResolveDraftInput {
            draft: draft.clone(),
            catalog_ref: bad_digest,
            request_id: None,
        })
        .expect_err("digest alterado");
    assert_eq!(err.code, "AM-CAT-002");

    let mut bad_version = ok.clone();
    bad_version.version = "9.9.9".to_string();
    let err = core
        .resolve_draft(ResolveDraftInput {
            draft: draft.clone(),
            catalog_ref: bad_version,
            request_id: None,
        })
        .expect_err("versión incompatible");
    assert_eq!(err.code, "AM-CAT-001");

    let mut bad_id = ok;
    bad_id.id = "no-such-catalog".to_string();
    let err = core
        .resolve_draft(ResolveDraftInput {
            draft,
            catalog_ref: bad_id,
            request_id: None,
        })
        .expect_err("id inexistente");
    assert_eq!(err.code, "AM-CAT-001");
}
