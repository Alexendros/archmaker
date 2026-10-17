//! Fixtures deterministas HA01–HA25. HA26–HA28 no se marcan.

use archmaker_domain::{
    BootCompatibilityStatus, CapacityVerificationStatus, EncryptionKind, EncryptionPolicy,
    ExecutionStatus, FilesystemKind, FirmwareDeclaration, FirmwareKind, FirmwareProvenance,
    HardwareVerificationStatus, LogicalTargetRef, LuksUnlockMethod, PartitionTableKind,
    PartitionTableObservation, PartitionTableObservationStatus, PlanValidationStatus, StorageMode,
};
use archmaker_hardware::compat::{
    evaluate_firmware_compat, CompatCode, CompatVerdict, CurrentBootMode, FirmwareCompatInput,
    SecureBootState, UefiCapability,
};
use archmaker_hardware::graph::{
    build_usage_graph, snapshot_from_lsblk_json, ProtectionReason, ReplaceEligibility,
};
use archmaker_hardware::help::{boot_filesystem_help, validate_uki_esp_access};
use archmaker_hardware::lsblk::parse_lsblk_json;
use archmaker_hardware::observation::{
    ObservationOrigin, ProbeStatus, TargetBinding, VerificationStatus,
};
use archmaker_hardware::plan::{propose_storage_plan, PlanRequest, PlanningPolicy};

fn base_compat() -> FirmwareCompatInput {
    FirmwareCompatInput {
        current_boot_mode: CurrentBootMode::Uefi64,
        uefi_capability: UefiCapability::SessionOnly,
        secure_boot: SecureBootState::Disabled,
        efivarfs_access: ProbeStatus::Detected,
        in_container: false,
        planned_firmware: FirmwareKind::Uefi,
        planned_firmware_provenance: FirmwareProvenance::Observed,
        planned_partition_table: PartitionTableKind::Gpt,
        observed_partition_table: Some(PartitionTableKind::Gpt),
        target_binding: TargetBinding::CurrentHostConfirmed,
        bootloader_requires_uefi: true,
    }
}

fn empty_disk_json(
    name: &str,
    size: u64,
    log_sec: u64,
    pttype: Option<&str>,
    serial: &str,
    wwn: &str,
) -> String {
    let pt = match pttype {
        Some(v) => format!("\"{v}\""),
        None => "null".into(),
    };
    format!(
        r#"{{"blockdevices":[{{"name":"{name}","path":"/dev/{name}","type":"disk","size":{size},"ro":false,"rm":false,"tran":"sata","model":"TEST","serial":"{serial}","wwn":"{wwn}","log-sec":{log_sec},"phy-sec":{log_sec},"pttype":{pt},"fstype":null,"fsver":null,"mountpoints":[],"children":[]}}]}}"#
    )
}

#[test]
fn ha01_uefi64_mode_and_bitness() {
    let r = evaluate_firmware_compat(&base_compat());
    assert_eq!(r.verdict, CompatVerdict::Compatible);
    assert!(matches!(r.code, CompatCode::Uefi64Ok | CompatCode::GptOk));
    assert_eq!(
        r.boot_compatibility_status,
        BootCompatibilityStatus::Compatible
    );
}

#[test]
fn ha02_uefi32_not_confused_with_uefi64() {
    let mut input = base_compat();
    input.current_boot_mode = CurrentBootMode::Uefi32;
    let r = evaluate_firmware_compat(&input);
    assert_eq!(r.code, CompatCode::Uefi32NotUefi64);
    assert_ne!(r.verdict, CompatVerdict::Compatible);
}

#[test]
fn ha03_bios_csm_does_not_conclude_no_uefi() {
    let mut input = base_compat();
    input.current_boot_mode = CurrentBootMode::LegacyOrCsm;
    input.planned_firmware_provenance = FirmwareProvenance::Declared;
    let r = evaluate_firmware_compat(&input);
    assert_eq!(r.code, CompatCode::BiosCsmNoPhysicalConclusion);
    assert!(r.diagnostics.iter().any(|d| d.contains("no se concluye")));
}

#[test]
fn ha04_container_hidden_efi_unknown() {
    let mut input = base_compat();
    input.in_container = true;
    input.current_boot_mode = CurrentBootMode::Unknown;
    let r = evaluate_firmware_compat(&input);
    assert_eq!(r.verdict, CompatVerdict::Unknown);
    assert_eq!(r.code, CompatCode::ContainerEfiHidden);
}

#[test]
fn ha05_secure_boot_states_do_not_block_alone() {
    for state in [
        SecureBootState::Enabled,
        SecureBootState::Disabled,
        SecureBootState::Unknown,
    ] {
        let mut input = base_compat();
        input.secure_boot = state;
        let r = evaluate_firmware_compat(&input);
        assert_eq!(r.verdict, CompatVerdict::Compatible, "{state:?}");
    }
}

#[test]
fn ha06_efivarfs_denied_is_unknown_not_incompatible() {
    let mut input = base_compat();
    input.uefi_capability = UefiCapability::Unavailable;
    input.efivarfs_access = ProbeStatus::Unavailable;
    input.current_boot_mode = CurrentBootMode::Unknown;
    let r = evaluate_firmware_compat(&input);
    assert_eq!(r.verdict, CompatVerdict::Unknown);
    assert_ne!(r.verdict, CompatVerdict::Incompatible);
}

#[test]
fn ha07_host_not_destination() {
    let mut input = base_compat();
    input.target_binding = TargetBinding::OtherTarget;
    input.planned_firmware_provenance = FirmwareProvenance::Observed;
    let r = evaluate_firmware_compat(&input);
    assert_eq!(r.code, CompatCode::HostNotTarget);
}

#[test]
fn ha08_logical_sector_512_and_4k() {
    for sec in [512u64, 4096u64] {
        let doc = parse_lsblk_json(&empty_disk_json(
            "sdx",
            500_000_000_000,
            sec,
            Some("gpt"),
            "S",
            "W",
        ))
        .unwrap();
        let (snap, _) = build_usage_graph(
            &doc,
            "s",
            "t",
            ObservationOrigin::LocalProbe,
            TargetBinding::Unbound,
        );
        assert_eq!(snap.devices[0].logical_sector_bytes, Some(sec));
        let req = PlanRequest {
            mode: StorageMode::NewDisk,
            firmware: FirmwareDeclaration {
                kind: FirmwareKind::Uefi,
                provenance: FirmwareProvenance::Declared,
            },
            planned_partition_table: PartitionTableKind::Gpt,
            observed_partition_table: None,
            logical_target_ref: Some(snap.devices[0].logical_target_ref.clone()),
            capacity_bytes: Some(500_000_000_000),
            logical_sector_bytes: Some(sec),
            intent_acknowledged: true,
            loss_risk_acknowledged: None,
            target_binding: TargetBinding::Unbound,
            current_boot_mode: Some(CurrentBootMode::Uefi64),
            uefi_capability: Some(UefiCapability::SessionOnly),
            secure_boot: Some(SecureBootState::Disabled),
            in_container: false,
            snapshot: Some(snap),
            policy: PlanningPolicy::reference(),
        };
        let plan = propose_storage_plan(&req);
        assert_eq!(
            plan.verification.execution_status,
            ExecutionStatus::NotExecuted
        );
    }
}

#[test]
fn ha09_missing_table_does_not_mean_empty() {
    let doc = parse_lsblk_json(&empty_disk_json("sdc", 1000, 512, None, "A", "B")).unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "s",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    assert!(snap.devices[0].partition_table.is_none());
    assert_eq!(
        snap.devices[0].replace_eligibility,
        ReplaceEligibility::Eligible
    );
}

#[test]
fn ha10_partial_table_not_hardware_verified() {
    let req = PlanRequest {
        mode: StorageMode::NewDisk,
        firmware: FirmwareDeclaration {
            kind: FirmwareKind::Uefi,
            provenance: FirmwareProvenance::Unknown,
        },
        planned_partition_table: PartitionTableKind::Gpt,
        observed_partition_table: Some(PartitionTableObservation {
            status: PartitionTableObservationStatus::Unknown,
            kind: None,
        }),
        logical_target_ref: None,
        capacity_bytes: None,
        logical_sector_bytes: None,
        intent_acknowledged: true,
        loss_risk_acknowledged: None,
        target_binding: TargetBinding::Unbound,
        current_boot_mode: Some(CurrentBootMode::Unknown),
        uefi_capability: Some(UefiCapability::Unknown),
        secure_boot: Some(SecureBootState::Unknown),
        in_container: false,
        snapshot: None,
        policy: PlanningPolicy::reference(),
    };
    let r = propose_storage_plan(&req);
    assert_ne!(
        r.verification.hardware_verification_status,
        HardwareVerificationStatus::Verified
    );
}

#[test]
fn ha11_esp_policy_without_format() {
    let help = boot_filesystem_help(FilesystemKind::Ext4, FirmwareKind::Uefi);
    assert!(!help.format_button_present);
    assert!(!help.a6_execution_available);
    let uki = validate_uki_esp_access(true, false, true);
    assert!(!uki.accessible);
}

#[test]
fn ha12_luks_lvm_parents_protected() {
    let raw = r#"{"blockdevices":[{"name":"sda","path":"/dev/sda","type":"disk","size":500000000000,"ro":false,"rm":false,"tran":"sata","model":"SYS","serial":"ROOT1","wwn":"0xr","log-sec":512,"phy-sec":512,"pttype":"gpt","fstype":null,"fsver":null,"mountpoints":[],"children":[{"name":"sda2","path":"/dev/sda2","type":"part","size":400000000000,"ro":false,"rm":false,"tran":null,"model":null,"serial":null,"wwn":null,"log-sec":512,"phy-sec":512,"pttype":null,"fstype":"crypto_LUKS","fsver":null,"mountpoints":[],"children":[{"name":"vg-root","path":"/dev/mapper/vg-root","type":"lvm","size":400000000000,"ro":false,"rm":false,"tran":null,"model":null,"serial":null,"wwn":null,"log-sec":512,"phy-sec":512,"pttype":null,"fstype":"ext4","fsver":null,"mountpoints":["/"],"children":[]}]}]}]}"#;
    let doc = parse_lsblk_json(raw).unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "s",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    assert_eq!(
        snap.devices[0].replace_eligibility,
        ReplaceEligibility::Protected
    );
    assert!(snap.devices[0]
        .protections
        .contains(&ProtectionReason::RootFilesystem));
}

#[test]
fn ha13_live_loop_protected() {
    let raw = r#"{"blockdevices":[{"name":"loop0","path":"/dev/loop0","type":"loop","size":1000,"ro":true,"rm":false,"tran":null,"model":null,"serial":null,"wwn":null,"log-sec":512,"phy-sec":512,"pttype":null,"fstype":"squashfs","fsver":null,"mountpoints":["/run/archiso/bootmnt"],"children":[]},{"name":"sdd","path":"/dev/sdd","type":"disk","size":8000000000,"ro":false,"rm":true,"tran":"usb","model":"LIVE","serial":"USB1","wwn":"0xu","log-sec":512,"phy-sec":512,"pttype":"dos","fstype":null,"fsver":null,"mountpoints":[],"children":[{"name":"sdd1","path":"/dev/sdd1","type":"part","size":8000000000,"ro":false,"rm":true,"tran":null,"model":null,"serial":null,"wwn":null,"log-sec":512,"phy-sec":512,"pttype":null,"fstype":"iso9660","fsver":null,"mountpoints":["/run/archiso/bootmnt"],"children":[]}]}]}"#;
    let doc = parse_lsblk_json(raw).unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "s",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    let usb = snap
        .devices
        .iter()
        .find(|d| d.model.as_deref() == Some("LIVE"))
        .unwrap();
    assert_eq!(usb.replace_eligibility, ReplaceEligibility::Protected);
    assert!(usb.protections.contains(&ProtectionReason::LiveMedia));
}

#[test]
fn ha14_mounted_swap_raid_not_ignored() {
    let raw = r#"{"blockdevices":[{"name":"sda","path":"/dev/sda","type":"disk","size":1000,"ro":false,"rm":false,"tran":"sata","model":"R","serial":"RAID1","wwn":"0xraid","log-sec":512,"phy-sec":512,"pttype":"gpt","fstype":null,"fsver":null,"mountpoints":[],"children":[{"name":"sda1","path":"/dev/sda1","type":"part","size":1000,"ro":false,"rm":false,"tran":null,"model":null,"serial":null,"wwn":null,"log-sec":512,"phy-sec":512,"pttype":null,"fstype":"linux_raid_member","fsver":null,"mountpoints":[],"children":[]}]},{"name":"sdb","path":"/dev/sdb","type":"disk","size":1000,"ro":false,"rm":false,"tran":"sata","model":"S","serial":"SWAP1","wwn":"0xsw","log-sec":512,"phy-sec":512,"pttype":"gpt","fstype":null,"fsver":null,"mountpoints":[],"children":[{"name":"sdb1","path":"/dev/sdb1","type":"part","size":1000,"ro":false,"rm":false,"tran":null,"model":null,"serial":null,"wwn":null,"log-sec":512,"phy-sec":512,"pttype":null,"fstype":"swap","fsver":null,"mountpoints":["[SWAP]"],"children":[]}]}]}"#;
    let doc = parse_lsblk_json(raw).unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "s",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    assert!(snap
        .devices
        .iter()
        .all(|d| d.replace_eligibility == ReplaceEligibility::Protected));
}

#[test]
fn ha15_duplicate_identity_blocks() {
    let raw = r#"{"blockdevices":[{"name":"sda","path":"/dev/sda","type":"disk","size":1000,"ro":false,"rm":false,"tran":"sata","model":"A","serial":"DUP","wwn":"0xdup","log-sec":512,"phy-sec":512,"pttype":"gpt","fstype":null,"fsver":null,"mountpoints":[],"children":[]},{"name":"sdb","path":"/dev/sdb","type":"disk","size":2000,"ro":false,"rm":false,"tran":"sata","model":"B","serial":"DUP","wwn":"0xdup","log-sec":512,"phy-sec":512,"pttype":"gpt","fstype":null,"fsver":null,"mountpoints":[],"children":[]}]}"#;
    let doc = parse_lsblk_json(raw).unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "s",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    assert!(snap.devices.iter().all(|d| {
        d.protections.contains(&ProtectionReason::DuplicateIdentity)
            || d.replace_eligibility == ReplaceEligibility::Ambiguous
    }));
    assert_eq!(
        snap.verification_status,
        VerificationStatus::AmbiguousIdentity
    );
}

#[test]
fn ha16_refresh_marks_stale_on_change() {
    let doc1 = parse_lsblk_json(&empty_disk_json("sda", 1000, 512, Some("gpt"), "X", "Y")).unwrap();
    let (snap1, _) = build_usage_graph(
        &doc1,
        "s1",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    let doc2 = parse_lsblk_json(&empty_disk_json("sda", 2000, 512, Some("gpt"), "X", "Y")).unwrap();
    let (snap2, _) = build_usage_graph(
        &doc2,
        "s2",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    assert_ne!(snap1.semantic_fingerprint, snap2.semantic_fingerprint);
}

#[test]
fn ha17_overlap_or_insufficient_capacity_rejected() {
    let req = PlanRequest {
        mode: StorageMode::NewDisk,
        firmware: FirmwareDeclaration {
            kind: FirmwareKind::Uefi,
            provenance: FirmwareProvenance::Declared,
        },
        planned_partition_table: PartitionTableKind::Gpt,
        observed_partition_table: None,
        logical_target_ref: None,
        capacity_bytes: Some(100 * 1024 * 1024),
        logical_sector_bytes: Some(512),
        intent_acknowledged: true,
        loss_risk_acknowledged: None,
        target_binding: TargetBinding::Unbound,
        current_boot_mode: Some(CurrentBootMode::Uefi64),
        uefi_capability: Some(UefiCapability::SessionOnly),
        secure_boot: Some(SecureBootState::Disabled),
        in_container: false,
        snapshot: None,
        policy: PlanningPolicy::reference(),
    };
    let r = propose_storage_plan(&req);
    assert_eq!(
        r.verification.plan_validation_status,
        PlanValidationStatus::Invalid
    );
    assert_eq!(
        r.verification.capacity_verification_status,
        CapacityVerificationStatus::Insufficient
    );
}

#[test]
fn ha18_bootloader_incompatible_with_bios() {
    let mut input = base_compat();
    input.planned_firmware = FirmwareKind::Bios;
    let r = evaluate_firmware_compat(&input);
    assert_eq!(r.code, CompatCode::UnsupportedByCurrentPolicy);
    let help = boot_filesystem_help(FilesystemKind::Ext4, FirmwareKind::Bios);
    assert!(help.bootloader_limits.iter().any(|l| l.contains("BIOS")));
}

#[test]
fn ha19_invalid_lsblk_json_does_not_yield_fake_snapshot() {
    assert!(parse_lsblk_json("{not-json").is_err());
}

#[test]
fn ha20_opaque_handles_no_dev_paths() {
    let doc = parse_lsblk_json(&empty_disk_json("sda", 1000, 512, Some("gpt"), "S", "W")).unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "s",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    assert!(!snap.devices[0].handle_token.contains("/dev/"));
    assert!(!snap.devices[0].logical_target_ref.id.contains("/dev/"));
}

#[test]
fn ha21_plan_has_no_secrets() {
    let mut policy = PlanningPolicy::reference();
    policy.encryption = EncryptionPolicy {
        kind: EncryptionKind::Luks2,
        unlock: Some(LuksUnlockMethod::PassphraseRequestedAtInstall),
        covers_mount_point: Some("/".into()),
        initramfs_note: Some("pedir passphrase en install".into()),
    };
    let req = PlanRequest {
        mode: StorageMode::NewDisk,
        firmware: FirmwareDeclaration {
            kind: FirmwareKind::Uefi,
            provenance: FirmwareProvenance::Declared,
        },
        planned_partition_table: PartitionTableKind::Gpt,
        observed_partition_table: None,
        logical_target_ref: Some(LogicalTargetRef {
            id: "lt-test".into(),
        }),
        capacity_bytes: Some(128 * 1024 * 1024 * 1024),
        logical_sector_bytes: Some(512),
        intent_acknowledged: true,
        loss_risk_acknowledged: None,
        target_binding: TargetBinding::Unbound,
        current_boot_mode: Some(CurrentBootMode::Uefi64),
        uefi_capability: Some(UefiCapability::SessionOnly),
        secure_boot: Some(SecureBootState::Disabled),
        in_container: false,
        snapshot: None,
        policy,
    };
    let r = propose_storage_plan(&req);
    let json = serde_json::to_string(&r.storage_plan).unwrap();
    assert!(!json.to_lowercase().contains("password"));
    assert!(!r.plan_digest.is_empty());
    assert_eq!(
        r.verification.execution_status,
        ExecutionStatus::NotExecuted
    );
}

#[test]
fn ha21b_public_snapshot_omits_serial_wwn() {
    let fixture = include_str!("fixtures/lsblk-clean-gpt.json");
    let (snap, registry) =
        snapshot_from_lsblk_json(
            fixture,
            "snap-privacy",
            "2026-10-17T00:00:00Z",
            ObservationOrigin::ImportedReport,
            TargetBinding::Unbound,
        )
        .expect("lsblk fixture");
    assert!(!registry.by_token.is_empty(), "registry private exists in-session");
    let json = serde_json::to_string(&snap).expect("snapshot json");
    assert!(!json.contains("\"serial\""), "public snapshot must not embed serial");
    assert!(!json.contains("\"wwn\""), "public snapshot must not embed wwn");
    assert!(!json.contains("/dev/"), "public snapshot must not embed /dev paths");
    // Registry is not Serialize — private identities stay out of default export.
    let _ = registry;
}

#[test]
fn ha22_manual_profile_without_hardware_keeps_gaps() {
    let req = PlanRequest {
        mode: StorageMode::NewDisk,
        firmware: FirmwareDeclaration {
            kind: FirmwareKind::Uefi,
            provenance: FirmwareProvenance::Declared,
        },
        planned_partition_table: PartitionTableKind::Gpt,
        observed_partition_table: Some(PartitionTableObservation {
            status: PartitionTableObservationStatus::Unknown,
            kind: None,
        }),
        logical_target_ref: None,
        capacity_bytes: None,
        logical_sector_bytes: None,
        intent_acknowledged: true,
        loss_risk_acknowledged: None,
        target_binding: TargetBinding::Unbound,
        current_boot_mode: None,
        uefi_capability: None,
        secure_boot: None,
        in_container: false,
        snapshot: None,
        policy: PlanningPolicy::reference(),
    };
    let r = propose_storage_plan(&req);
    assert_eq!(
        r.verification.hardware_verification_status,
        HardwareVerificationStatus::Pending
    );
    assert_eq!(
        r.verification.capacity_verification_status,
        CapacityVerificationStatus::Unknown
    );
}

#[test]
fn ha23_policy_change_changes_digest() {
    let mut req = PlanRequest {
        mode: StorageMode::NewDisk,
        firmware: FirmwareDeclaration {
            kind: FirmwareKind::Uefi,
            provenance: FirmwareProvenance::Declared,
        },
        planned_partition_table: PartitionTableKind::Gpt,
        observed_partition_table: None,
        logical_target_ref: None,
        capacity_bytes: Some(128 * 1024 * 1024 * 1024),
        logical_sector_bytes: Some(512),
        intent_acknowledged: true,
        loss_risk_acknowledged: None,
        target_binding: TargetBinding::Unbound,
        current_boot_mode: Some(CurrentBootMode::Uefi64),
        uefi_capability: Some(UefiCapability::SessionOnly),
        secure_boot: Some(SecureBootState::Disabled),
        in_container: false,
        snapshot: None,
        policy: PlanningPolicy::reference(),
    };
    let a = propose_storage_plan(&req);
    req.policy.encryption = EncryptionPolicy {
        kind: EncryptionKind::Luks2,
        unlock: Some(LuksUnlockMethod::PassphraseRequestedAtInstall),
        covers_mount_point: Some("/".into()),
        initramfs_note: None,
    };
    let b = propose_storage_plan(&req);
    assert_ne!(a.plan_digest, b.plan_digest);
}

#[test]
fn ha24_zero_writes_execution_not_executed_observed_ne_planned() {
    let help = boot_filesystem_help(FilesystemKind::Btrfs, FirmwareKind::Uefi);
    assert!(!help.format_button_present);
    assert!(!help.a6_execution_available);
    let req = PlanRequest {
        mode: StorageMode::NewDisk,
        firmware: FirmwareDeclaration {
            kind: FirmwareKind::Uefi,
            provenance: FirmwareProvenance::Observed,
        },
        planned_partition_table: PartitionTableKind::Gpt,
        observed_partition_table: Some(PartitionTableObservation {
            status: PartitionTableObservationStatus::Observed,
            kind: Some(PartitionTableKind::Mbr),
        }),
        logical_target_ref: None,
        capacity_bytes: Some(256 * 1024 * 1024 * 1024),
        logical_sector_bytes: Some(512),
        intent_acknowledged: true,
        loss_risk_acknowledged: None,
        target_binding: TargetBinding::CurrentHostConfirmed,
        current_boot_mode: Some(CurrentBootMode::Uefi64),
        uefi_capability: Some(UefiCapability::SessionOnly),
        secure_boot: Some(SecureBootState::Enabled),
        in_container: false,
        snapshot: None,
        policy: PlanningPolicy::reference(),
    };
    let r = propose_storage_plan(&req);
    assert_eq!(
        r.verification.execution_status,
        ExecutionStatus::NotExecuted
    );
    assert_eq!(r.storage_plan.partition_table, PartitionTableKind::Gpt);
    assert_eq!(
        r.storage_plan
            .observed_partition_table
            .as_ref()
            .and_then(|o| o.kind),
        Some(PartitionTableKind::Mbr)
    );
}

#[test]
fn ha25_browser_path_is_manual_origin() {
    let doc = parse_lsblk_json(&empty_disk_json("vd", 1000, 512, Some("gpt"), "M", "N")).unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "manual",
        "t",
        ObservationOrigin::ManualDeclaration,
        TargetBinding::Unbound,
    );
    assert_eq!(snap.origin, ObservationOrigin::ManualDeclaration);
    assert_eq!(snap.verification_status, VerificationStatus::DeclaredOnly);
}

/// HA15 refuerzo: identidad ambigua (sin serial/wwn/path) bloquea.
#[test]
fn ha15b_insufficient_identity_ambiguous() {
    let raw = r#"{"blockdevices":[{"name":"sda","type":"disk","size":1000,"ro":false,"rm":false,"tran":"sata","model":"X","log-sec":512,"phy-sec":512,"pttype":"gpt","fstype":null,"fsver":null,"mountpoints":[],"children":[]}]}"#;
    let doc = parse_lsblk_json(raw).unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "s",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    assert_eq!(
        snap.devices[0].replace_eligibility,
        ReplaceEligibility::Ambiguous
    );
}
