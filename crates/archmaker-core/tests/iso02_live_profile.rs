//! ISO-02: perfil vivo desde el manifiesto sellado. Sin mkarchiso.

use archmaker_core::{
    render_live_profile, BuildManifestInput, CorePort, CreateDraftInput, DestinationHandle,
    ExportArtifactInput, RealCore,
};
use archmaker_domain::{Selection, SelectionValue, TargetRef, ONLYOFFICE_CANONICAL_ID};

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

fn base_draft(core: &RealCore) -> archmaker_domain::Draft {
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref: core.active_catalog_ref().expect("catalog"),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create");
    draft.selections = vec![
        single("archmaker.base.kernel", "archmaker.kernel.linux"),
        single(
            "archmaker.environment.compositor",
            "archmaker.compositor.gnome",
        ),
    ];
    draft
}

fn seal(core: &RealCore, draft: archmaker_domain::Draft) -> archmaker_domain::Manifest {
    core.build_manifest(BuildManifestInput {
        draft,
        catalog_ref: core.active_catalog_ref().expect("catalog"),
        target_ref: live_target(),
        expected_resolution_digest: None,
        request_id: None,
    })
    .expect("sella")
}

fn export_bytes(core: &RealCore, manifest: archmaker_domain::Manifest) -> Vec<u8> {
    core.export_artifact(ExportArtifactInput {
        manifest,
        target_ref: live_target(),
        destination: DestinationHandle::new("live-profile".to_string()).expect("token"),
        request_id: None,
    })
    .expect("export")
    .bytes
    .expect("bytes")
}

#[test]
fn iso02_same_manifest_same_bytes_without_pacstrap_or_cmd() {
    let core = RealCore::with_desktop().expect("desktop");
    let manifest = seal(&core, base_draft(&core));
    let first = export_bytes(&core, manifest.clone());
    let second = export_bytes(&core, manifest.clone());
    assert_eq!(first, second);

    let rendered = render_live_profile(&manifest, core.catalog(), &live_target()).expect("render");
    assert_eq!(rendered.bytes, first);

    let text = String::from_utf8(first.clone()).expect("utf8");
    assert!(!text.contains("pacstrap"));
    assert!(!text.contains("cmd"));
    let value: serde_json::Value = serde_json::from_slice(&first).expect("json");
    assert_eq!(value["adapter"], "archmaker-live-profile");
    assert_eq!(value["baseDescription"], "derivada de Arch Linux");
    assert_eq!(value["arch"], "x86_64");
    let filename = value["publicFilename"].as_str().expect("filename");
    assert!(filename.starts_with("Forge-"));
    assert!(filename.ends_with("-x86_64.iso"));
    assert!(!filename.contains("Arch"));
    assert!(!filename.starts_with("ARCH"));
    let files = value["files"].as_object().expect("files");
    assert_eq!(files.len(), 4);
    let packages = value["files"]["packages.x86_64"]
        .as_str()
        .expect("packages");
    assert_eq!(packages, "gnome-shell\nlinux\n");
    let profiledef = value["files"]["profiledef.sh"]
        .as_str()
        .expect("profiledef");
    assert!(profiledef.contains("arch=\"x86_64\""));
    assert!(profiledef.contains("iso_name=\"Forge\""));
    assert!(profiledef.contains("derivada de Arch Linux"));
    assert!(profiledef.contains("bootmodes=('bios.syslinux' 'uefi.systemd-boot')"));
    assert!(profiledef.contains("[\"/root/user_configuration.json\"]=\"0:0:600\""));
    let pacman = value["files"]["pacman.conf"].as_str().expect("pacman");
    assert!(pacman.contains("[core]"));
    assert!(pacman.contains("[extra]"));
    let config: serde_json::Value = serde_json::from_str(
        value["files"]["airootfs/root/user_configuration.json"]
            .as_str()
            .expect("config"),
    )
    .expect("config json");
    assert_eq!(config["version"], "2.8.6");
    assert_eq!(config["script"], "guided");
    assert_eq!(config["hostname"], "forge");
    assert_eq!(
        value["deferredFetch"].as_array().expect("deferred").len(),
        0
    );
}

#[test]
fn iso02_non_redistributable_is_named_deferred_not_installed() {
    let core = RealCore::with_desktop().expect("desktop");
    let mut draft = base_draft(&core);
    draft.selections.push(Selection {
        step_id: "archmaker.libpacks.office".to_string(),
        value: SelectionValue::Multiple {
            option_ids: vec![ONLYOFFICE_CANONICAL_ID.to_string()],
        },
    });
    let manifest = seal(&core, draft);
    let bytes = export_bytes(&core, manifest);
    let value: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
    let packages = value["files"]["packages.x86_64"]
        .as_str()
        .expect("packages");
    assert!(!packages.contains("onlyoffice-bin"));
    let deferred = value["deferredFetch"].as_array().expect("deferred");
    assert_eq!(deferred.len(), 1);
    assert_eq!(deferred[0], "onlyoffice-bin");
}

#[test]
fn iso02_blocking_manifest_yields_no_profile() {
    let core = RealCore::with_desktop().expect("desktop");
    let mut draft = base_draft(&core);
    draft.selections.clear();
    let error = core
        .build_manifest(BuildManifestInput {
            draft,
            catalog_ref: core.active_catalog_ref().expect("catalog"),
            target_ref: live_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect_err("bloqueado");
    assert_eq!(error.code, "AM-RES-002");
}

#[test]
fn iso02_unknown_target_still_rejected() {
    let core = RealCore::with_desktop().expect("desktop");
    let manifest = seal(&core, base_draft(&core));
    let error = core
        .export_artifact(ExportArtifactInput {
            manifest,
            target_ref: TargetRef {
                id: "iso".to_string(),
                version: "0.1.0".to_string(),
            },
            destination: DestinationHandle::new("nope".to_string()).expect("token"),
            request_id: None,
        })
        .expect_err("iso");
    assert_eq!(error.code, "AM-TGT-001");
}
