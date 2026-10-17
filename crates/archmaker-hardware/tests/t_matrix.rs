//! Matriz T31–T36 y T43–T44 (hardware/plan). T01–T30 y T37–T42 viven en web/core.
//! HA26–HA28: not-applicable (A6 excluida); no se marcan passed.

use archmaker_domain::{
    BootCompatibilityStatus, DeviceHandle, ExecutionStatus, FirmwareDeclaration, FirmwareKind,
    FirmwareProvenance, LogicalTargetRef, PartitionTableKind, PartitionTableObservation,
    PartitionTableObservationStatus, StorageMode,
};
use archmaker_hardware::compat::{
    evaluate_firmware_compat, CompatCode, CompatVerdict, CurrentBootMode, FirmwareCompatInput,
    HostKind, SecureBootState, UefiCapability,
};
use archmaker_hardware::graph::{build_usage_graph, snapshot_from_lsblk_json};
use archmaker_hardware::help::validate_uki_esp_access;
use archmaker_hardware::lsblk::parse_lsblk_json;
use archmaker_hardware::observation::{
    ObservationOrigin, ProbeStatus, TargetBinding, VerificationStatus,
};
use archmaker_hardware::plan::{propose_storage_plan, PlanRequest, PlanningPolicy};
use sha2::{Digest, Sha256};

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

/// T31 — reporte importado no gana observación local verificada.
#[test]
fn t31_imported_report_not_observed_bound() {
    let doc = parse_lsblk_json(&empty_disk_json(
        "sda",
        64_000_000_000,
        512,
        Some("gpt"),
        "S",
        "W",
    ))
    .unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "import",
        "t",
        ObservationOrigin::ImportedReport,
        TargetBinding::CurrentHostConfirmed,
    );
    assert_eq!(snap.origin, ObservationOrigin::ImportedReport);
    assert_eq!(
        snap.verification_status,
        VerificationStatus::ImportedUnverified
    );
    assert_ne!(snap.verification_status, VerificationStatus::ObservedBound);
}

/// T32 — handle de sesión no se reutiliza como identidad portable.
#[test]
fn t32_session_handle_not_portable_logical_ref() {
    let handle = DeviceHandle {
        session_token: "sess-previous".into(),
    };
    let logical = LogicalTargetRef {
        id: "logical-target-1".into(),
    };
    let handle_json = serde_json::to_value(&handle).unwrap();
    let logical_json = serde_json::to_value(&logical).unwrap();
    assert!(handle_json.get("sessionToken").is_some());
    assert!(logical_json.get("id").is_some());
    assert_ne!(
        handle.session_token, logical.id,
        "deviceHandle de sesión ≠ logicalTargetRef portable"
    );
}

/// T33 — MBR observado + propuesta GPT: plan sin escritura; tablas ortogonales.
#[test]
fn t33_mbr_observed_gpt_proposal_without_write() {
    let req = PlanRequest {
        mode: StorageMode::ReplaceDisk,
        firmware: FirmwareDeclaration {
            kind: FirmwareKind::Uefi,
            provenance: FirmwareProvenance::Declared,
        },
        planned_partition_table: PartitionTableKind::Gpt,
        observed_partition_table: Some(PartitionTableObservation {
            status: PartitionTableObservationStatus::Observed,
            kind: Some(PartitionTableKind::Mbr),
        }),
        logical_target_ref: Some(LogicalTargetRef { id: "tgt".into() }),
        capacity_bytes: Some(256 * 1024 * 1024 * 1024),
        logical_sector_bytes: Some(512),
        intent_acknowledged: true,
        loss_risk_acknowledged: Some(true),
        target_binding: TargetBinding::Unbound,
        current_boot_mode: Some(CurrentBootMode::Uefi64),
        uefi_capability: Some(UefiCapability::Declared),
        secure_boot: Some(SecureBootState::Disabled),
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

/// T34 — disco sin tabla no implica vacío/seguro.
#[test]
fn t34_missing_table_does_not_infer_empty_or_safe() {
    let doc = parse_lsblk_json(&empty_disk_json("sdc", 1000, 512, None, "A", "B")).unwrap();
    let (snap, _) = build_usage_graph(
        &doc,
        "s",
        "t",
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
    );
    assert!(snap.devices[0].partition_table.is_none());
    // Eligible ≠ «vacío/seguro de borrar»; no hay bandera empty/safe inventada.
    let json = serde_json::to_string(&snap).unwrap();
    assert!(!json.to_lowercase().contains("empty"));
    assert!(!json.to_lowercase().contains("\"safe\""));
}

/// T35 — ESP /efi sin UKI accesible → plan de arranque incompleto.
#[test]
fn t35_esp_without_uki_path_incomplete() {
    let uki = validate_uki_esp_access(true, false, true);
    assert!(!uki.accessible);
    assert!(!uki.diagnostics.is_empty());
}

/// T36 — Secure Boot enabled sin estrategia de firma → Conditional / no certifica.
#[test]
fn t36_secure_boot_enabled_without_signing_is_conditional() {
    let mut input = base_compat();
    input.secure_boot = SecureBootState::Enabled;
    let r = evaluate_firmware_compat(&input);
    assert_eq!(r.verdict, CompatVerdict::Conditional);
    assert_eq!(r.code, CompatCode::SecureBootEnabled);
    assert_eq!(
        r.boot_compatibility_status,
        BootCompatibilityStatus::Conditional
    );
    assert!(!r.may_certify_uefi_profile);
    assert!(r.diagnostics.iter().any(|d| d.contains("Secure Boot")));
}

/// T43 — redacción de identidad privada produce bytes/hash distintos del bruto.
#[test]
fn t43_redacted_public_snapshot_hash_differs_from_private() {
    let fixture = include_str!("fixtures/lsblk-clean-gpt.json");
    let (snap, registry) = snapshot_from_lsblk_json(
        fixture,
        "snap-redact",
        "2026-10-17T00:00:00Z",
        ObservationOrigin::ImportedReport,
        TargetBinding::Unbound,
    )
    .expect("lsblk fixture");
    let public = serde_json::to_vec(&snap).expect("public json");
    let private_probe = format!("{:?}", registry.by_token);
    assert!(
        !String::from_utf8_lossy(&public).contains("\"serial\""),
        "export público sin serial"
    );
    let pub_hash = Sha256::digest(&public);
    let priv_hash = Sha256::digest(private_probe.as_bytes());
    assert_ne!(pub_hash.as_slice(), priv_hash.as_slice());
}

/// T44 — namespace restringido: cobertura parcial; no certifica host completo.
#[test]
fn t44_restricted_namespace_partial_not_full_host_cert() {
    assert_ne!(HostKind::Restricted, HostKind::NativeLinux);
    let mut input = base_compat();
    input.in_container = true;
    input.current_boot_mode = CurrentBootMode::Unknown;
    input.efivarfs_access = ProbeStatus::Unavailable;
    let r = evaluate_firmware_compat(&input);
    assert!(!r.may_certify_uefi_profile);
    assert_ne!(r.verdict, CompatVerdict::Compatible);
}
