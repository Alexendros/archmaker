use archmaker_core::{
    BuildManifestInput, CorePort, CreateDraftInput, DestinationHandle, ExportArtifactInput,
    RealCore,
};
use archmaker_domain::{Selection, SelectionValue, TargetRef};

fn single(step: &str, option: &str) -> Selection {
    Selection {
        step_id: step.to_string(),
        value: SelectionValue::Single {
            option_id: option.to_string(),
        },
    }
}

#[test]
fn archinstall_config_matches_pinned_contract_fixture() {
    let core = RealCore::with_embedded().expect("embedded core");
    let catalog_ref = core.active_catalog_ref().expect("catalog ref");
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref: catalog_ref.clone(),
            target_ref: TargetRef {
                id: "arch-x86_64".to_string(),
                version: "2026.10.0".to_string(),
            },
            preset_ref: None,
            request_id: None,
        })
        .expect("creates draft");
    draft.selections = vec![
        single("kernel", "archmaker.kernel.linux"),
        single("session", "archmaker.desktop.gnome"),
    ];
    let target = TargetRef {
        id: "archinstall-profile".to_string(),
        version: "0.1.0".to_string(),
    };
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft,
            catalog_ref,
            target_ref: target.clone(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("builds manifest");
    let artifact = core
        .export_artifact(ExportArtifactInput {
            manifest,
            target_ref: target,
            destination: DestinationHandle::new("user_configuration.json".to_string())
                .expect("handle"),
            request_id: None,
        })
        .expect("exports archinstall configuration");

    let expected: serde_json::Value = serde_json::from_str(include_str!(
        "../fixtures/archinstall-user-configuration-4.5.json"
    ))
    .expect("golden fixture");
    assert_eq!(artifact.content.expect("content"), expected);
    assert_eq!(artifact.media_type, "application/json");
    assert!(artifact.experimental.is_none());
}

#[test]
fn archinstall_config_maps_desktop_catalog_and_defers_aur() {
    let core = RealCore::with_desktop().expect("desktop core");
    let catalog_ref = core.active_catalog_ref().expect("catalog ref");
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref: catalog_ref.clone(),
            target_ref: TargetRef {
                id: "arch-x86_64".to_string(),
                version: "2026.10.0".to_string(),
            },
            preset_ref: None,
            request_id: None,
        })
        .expect("creates draft");
    draft.selections = vec![
        single("archmaker.base.kernel", "archmaker.kernel.linux"),
        single("archmaker.base.boot", "archmaker.boot.systemd-boot"),
        single(
            "archmaker.environment.compositor",
            "archmaker.compositor.kde",
        ),
        single("archmaker.environment.dm", "archmaker.dm.sddm"),
        single(
            "archmaker.libpacks.office",
            "archmaker.pkg.libreoffice-fresh",
        ),
        single("archmaker.apps.browser", "archmaker.browser.brave"),
    ];
    let target = TargetRef {
        id: "archinstall-profile".to_string(),
        version: "0.1.0".to_string(),
    };
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft,
            catalog_ref,
            target_ref: target.clone(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("builds manifest");
    let artifact = core
        .export_artifact(ExportArtifactInput {
            manifest,
            target_ref: target,
            destination: DestinationHandle::new("desktop.json".to_string()).expect("handle"),
            request_id: None,
        })
        .expect("exports archinstall configuration");
    let config = artifact.content.expect("content");

    assert_eq!(config["kernels"], serde_json::json!(["linux"]));
    assert_eq!(config["bootloader_config"]["bootloader"], "Systemd-boot");
    assert_eq!(
        config["profile_config"]["profile"]["details"],
        serde_json::json!(["KDE Plasma"])
    );
    assert_eq!(config["profile_config"]["greeter"], "sddm");
    assert_eq!(config["packages"], serde_json::json!(["libreoffice-fresh"]));
    assert!(config.get("disk_config").is_none());

    let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../target/ainspect");
    std::fs::create_dir_all(&dir).expect("scratch dir");
    std::fs::write(
        dir.join("desktop.json"),
        serde_json::to_vec_pretty(&config).expect("json"),
    )
    .expect("writes config");
}
