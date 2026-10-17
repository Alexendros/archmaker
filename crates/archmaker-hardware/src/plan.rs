//! Planificador puro: observedPartitionTable ≠ plannedPartitionTable.
//! Emite VerificationStatuses ortogonales (no un solo «certifiable»).

use archmaker_domain::{
    BootArtifactStrategy, BootCompatibilityStatus, CapacityVerificationStatus, EncryptionKind,
    EncryptionPolicy, ExecutionStatus, FilesystemKind, FirmwareDeclaration, FirmwareKind,
    FirmwareProvenance, HardwareVerificationStatus, LogicalTargetRef, LuksUnlockMethod,
    PartitionRole, PartitionTableKind, PartitionTableObservation, PartitionTableObservationStatus,
    PlanCertifiabilityStatus, PlanValidationStatus, PlannedAction, PlannedPartition, SafeByteCount,
    StorageLayer, StorageMode, StoragePlan, VerificationStatuses,
};
use serde::{Deserialize, Serialize};

use crate::compat::{
    evaluate_firmware_compat, CompatCode, CompatVerdict, CurrentBootMode, FirmwareCompatInput,
    SecureBootState, UefiCapability,
};
use crate::graph::{ReplaceEligibility, StorageSnapshot};
use crate::help::{boot_filesystem_help, validate_uki_esp_access, BootFsHelp, UkiEspAccess};
use crate::observation::{ProbeStatus, TargetBinding, VerificationStatus};
use crate::policy::StoragePlanningLimits;

pub const PLANNING_POLICY_VERSION: &str = "archmaker-storage-policy/1";

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlanningPolicy {
    pub version: String,
    pub limits: PlanningLimitsDto,
    pub root_filesystem: FilesystemKind,
    pub encryption: EncryptionPolicy,
    pub bootloader_requires_uefi: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlanningLimitsDto {
    pub esp_size_bytes: u64,
    pub esp_min_bytes: u64,
    pub gpt_reserve_bytes: u64,
    pub luks2_header_reserve_bytes: u64,
}

impl PlanningPolicy {
    pub fn reference() -> Self {
        let lim = StoragePlanningLimits::reference();
        Self {
            version: PLANNING_POLICY_VERSION.into(),
            limits: PlanningLimitsDto {
                esp_size_bytes: lim.esp_size_bytes,
                esp_min_bytes: lim.esp_min_bytes,
                gpt_reserve_bytes: lim.gpt_reserve_bytes,
                luks2_header_reserve_bytes: lim.luks2_header_reserve_bytes,
            },
            root_filesystem: FilesystemKind::Ext4,
            encryption: EncryptionPolicy::none(),
            bootloader_requires_uefi: true,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlanRequest {
    pub mode: StorageMode,
    pub firmware: FirmwareDeclaration,
    /// Tabla planificada (intención).
    pub planned_partition_table: PartitionTableKind,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub observed_partition_table: Option<PartitionTableObservation>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub logical_target_ref: Option<LogicalTargetRef>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub capacity_bytes: Option<u64>,
    /// Sector lógico real o declarado; None = no inventar 512.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub logical_sector_bytes: Option<u64>,
    pub intent_acknowledged: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub loss_risk_acknowledged: Option<bool>,
    pub target_binding: TargetBinding,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub current_boot_mode: Option<CurrentBootMode>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub uefi_capability: Option<UefiCapability>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub secure_boot: Option<SecureBootState>,
    pub in_container: bool,
    /// Si hay snapshot, se usa para eligibility/sector/tabla observada.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub snapshot: Option<StorageSnapshot>,
    pub policy: PlanningPolicy,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlanResult {
    pub storage_plan: StoragePlan,
    pub verification: VerificationStatuses,
    pub policy_version: String,
    pub compat_verdict: CompatVerdict,
    pub compat_code: CompatCode,
    pub diagnostics: Vec<String>,
    pub boot_help: BootFsHelp,
    pub uki_esp_access: UkiEspAccess,
    /// Digest semántico del plan+política (sin autoincluirse).
    pub plan_digest: String,
}

fn align_up(value: u64, sector: u64) -> u64 {
    if sector == 0 {
        return value;
    }
    value.div_ceil(sector) * sector
}

fn observed_kind(obs: &Option<PartitionTableObservation>) -> Option<PartitionTableKind> {
    obs.as_ref().and_then(|o| o.kind)
}

pub fn propose_storage_plan(req: &PlanRequest) -> PlanResult {
    let mut diagnostics = Vec::new();
    let policy = &req.policy;

    // Resolver capacidad / sector desde snapshot si hay handle lógico.
    let mut capacity = req.capacity_bytes;
    let mut logical_sector = req.logical_sector_bytes;
    let mut observed_table = req.observed_partition_table.clone();
    let mut device_protected = false;
    let mut identity_blocked = false;

    if let Some(snap) = &req.snapshot {
        if let Some(pref) = &req.logical_target_ref {
            if let Some(dev) = snap
                .devices
                .iter()
                .find(|d| d.logical_target_ref.id == pref.id)
            {
                capacity = capacity.or(dev.size_bytes);
                logical_sector = logical_sector.or(dev.logical_sector_bytes);
                if observed_table.is_none() {
                    observed_table = Some(PartitionTableObservation {
                        status: if dev.partition_table.is_some() {
                            PartitionTableObservationStatus::Observed
                        } else {
                            PartitionTableObservationStatus::Unknown
                        },
                        kind: dev.partition_table,
                    });
                }
                match dev.replace_eligibility {
                    ReplaceEligibility::Protected => {
                        device_protected = true;
                        diagnostics.push(
                            "Destino protegido (root/ESP/mount/swap/LVM/RAID/dm/live); sin verificación de reemplazo."
                                .into(),
                        );
                    }
                    ReplaceEligibility::Ambiguous => {
                        identity_blocked = true;
                        diagnostics.push(
                            "Identidad ambigua o insuficiente; bloquea verificación (no escritura)."
                                .into(),
                        );
                    }
                    _ => {}
                }
            } else {
                diagnostics.push("logicalTargetRef no encontrado en snapshot.".into());
                identity_blocked = true;
            }
        }
        if matches!(
            snap.verification_status,
            VerificationStatus::AmbiguousIdentity
        ) {
            identity_blocked = true;
        }
    }

    // Sin sector observado/declarado: no inventar 512 para geometría verificada.
    let sector_for_layout = logical_sector;
    if sector_for_layout.is_none() {
        diagnostics.push(
            "Sector lógico no observado ni declarado; geometría no se verifica con 512 inventado."
                .into(),
        );
    }

    let boot_mode = req.current_boot_mode.unwrap_or(CurrentBootMode::Unknown);
    let uefi_cap = req.uefi_capability.unwrap_or(UefiCapability::Unknown);
    let secure = req.secure_boot.unwrap_or(SecureBootState::Unknown);

    let compat = evaluate_firmware_compat(&FirmwareCompatInput {
        current_boot_mode: boot_mode,
        uefi_capability: uefi_cap,
        secure_boot: secure,
        efivarfs_access: ProbeStatus::Unknown,
        in_container: req.in_container,
        planned_firmware: req.firmware.kind,
        planned_firmware_provenance: req.firmware.provenance,
        planned_partition_table: req.planned_partition_table,
        observed_partition_table: observed_kind(&observed_table),
        target_binding: req.target_binding,
        bootloader_requires_uefi: policy.bootloader_requires_uefi,
    });
    diagnostics.extend(compat.diagnostics.clone());

    // Capas de referencia: ESP FAT32 /efi UKI; sistema [LUKS2?]→FS→/
    let esp_size = policy.limits.esp_size_bytes;
    let mut system_layers = Vec::new();
    if matches!(policy.encryption.kind, EncryptionKind::Luks2) {
        system_layers.push(StorageLayer::Luks2 {
            unlock: LuksUnlockMethod::PassphraseRequestedAtInstall,
        });
    }
    system_layers.push(StorageLayer::Filesystem {
        fs: policy.root_filesystem,
        mount_point: "/".into(),
    });

    let mut plan_validation = PlanValidationStatus::Valid;
    let mut capacity_status = CapacityVerificationStatus::Unverified;

    let system_size = if let Some(cap) = capacity {
        let reserve = policy.limits.gpt_reserve_bytes
            + if matches!(policy.encryption.kind, EncryptionKind::Luks2) {
                policy.limits.luks2_header_reserve_bytes
            } else {
                0
            };
        let usable = cap.saturating_sub(reserve);
        if usable <= esp_size {
            plan_validation = PlanValidationStatus::Invalid;
            capacity_status = CapacityVerificationStatus::Insufficient;
            diagnostics.push("Capacidad insuficiente para ESP + raíz tras reservas.".into());
            0
        } else {
            let raw = usable - esp_size;
            let aligned = sector_for_layout.map(|s| align_up(raw, s)).unwrap_or(raw);
            // Evitar solape: si alinear empuja fuera, rechazar.
            if aligned + esp_size + reserve > cap {
                plan_validation = PlanValidationStatus::Invalid;
                capacity_status = CapacityVerificationStatus::Insufficient;
                diagnostics.push("Solape o fuera de capacidad tras alineación.".into());
                0
            } else {
                if sector_for_layout.is_some()
                    && matches!(req.firmware.provenance, FirmwareProvenance::Observed)
                    && matches!(req.target_binding, TargetBinding::CurrentHostConfirmed)
                    && !identity_blocked
                    && !device_protected
                {
                    capacity_status = CapacityVerificationStatus::Sufficient;
                } else if capacity.is_some() {
                    capacity_status = CapacityVerificationStatus::Unverified;
                    diagnostics.push(
                        "Capacidad conocida pero no plenamente verificada (procedencia/destino)."
                            .into(),
                    );
                }
                aligned
            }
        }
    } else {
        diagnostics.push("Capacidad desconocida → capacity-unverified.".into());
        capacity_status = CapacityVerificationStatus::Unknown;
        0
    };

    if matches!(req.mode, StorageMode::ReplaceDisk) && req.loss_risk_acknowledged != Some(true) {
        plan_validation = PlanValidationStatus::Invalid;
        diagnostics.push("replace-disk exige lossRiskAcknowledged=true.".into());
    }
    if !req.intent_acknowledged {
        plan_validation = PlanValidationStatus::Invalid;
        diagnostics.push("intentAcknowledged requerido.".into());
    }

    // Hardware verification ortogonal.
    let hardware_status = if identity_blocked || device_protected {
        HardwareVerificationStatus::Unavailable
    } else if matches!(req.target_binding, TargetBinding::OtherTarget) {
        diagnostics.push("Host≠destino: observación no certifica el destino.".into());
        HardwareVerificationStatus::NotVerified
    } else if matches!(req.firmware.provenance, FirmwareProvenance::Observed)
        && matches!(req.target_binding, TargetBinding::CurrentHostConfirmed)
        && matches!(boot_mode, CurrentBootMode::Uefi64)
        && matches!(req.planned_partition_table, PartitionTableKind::Gpt)
    {
        HardwareVerificationStatus::Verified
    } else if matches!(req.firmware.provenance, FirmwareProvenance::Declared) {
        HardwareVerificationStatus::Pending
    } else {
        HardwareVerificationStatus::NotVerified
    };

    let boot_status = compat.boot_compatibility_status;
    if matches!(compat.code, CompatCode::UnsupportedByCurrentPolicy)
        && matches!(plan_validation, PlanValidationStatus::Valid)
    {
        // Plan puede ser sintácticamente válido pero no compatible con política.
        diagnostics.push(
            "Plan válido sintácticamente; bootCompatibility incompatible por política.".into(),
        );
    }

    let partitions = vec![
        PlannedPartition {
            role: PartitionRole::Esp,
            size_bytes: SafeByteCount::new(esp_size).unwrap_or(SafeByteCount::new(0).unwrap()),
            layers: vec![StorageLayer::Fat32 {
                mount_point: "/efi".into(),
            }],
            planned_action: match req.mode {
                StorageMode::NewDisk => PlannedAction::Create,
                StorageMode::ReplaceDisk => PlannedAction::Replace,
            },
        },
        PlannedPartition {
            role: PartitionRole::System,
            size_bytes: SafeByteCount::new(system_size).unwrap_or(SafeByteCount::new(0).unwrap()),
            layers: system_layers,
            planned_action: match req.mode {
                StorageMode::NewDisk => PlannedAction::Create,
                StorageMode::ReplaceDisk => PlannedAction::Replace,
            },
        },
    ];

    // Validación de capas LUKS (ESP fuera).
    if matches!(policy.encryption.kind, EncryptionKind::Luks2) {
        if let Some(cov) = &policy.encryption.covers_mount_point {
            if cov != "/" {
                plan_validation = PlanValidationStatus::Invalid;
                diagnostics.push("LUKS2 solo puede cubrir la raíz /.".into());
            }
        }
    }

    let uki = validate_uki_esp_access(
        matches!(req.planned_partition_table, PartitionTableKind::Gpt)
            && matches!(req.firmware.kind, FirmwareKind::Uefi),
        esp_size >= policy.limits.esp_min_bytes,
        true, // artefacto declarado en plan; sin montar ESP
    );
    if !uki.accessible {
        diagnostics.extend(uki.diagnostics.clone());
    }

    let storage_plan = StoragePlan {
        mode: req.mode,
        firmware: req.firmware.clone(),
        partition_table: req.planned_partition_table,
        observed_partition_table: observed_table.or(Some(PartitionTableObservation {
            status: PartitionTableObservationStatus::Unknown,
            kind: None,
        })),
        logical_target_ref: req.logical_target_ref.clone(),
        declared_capacity_bytes: capacity.and_then(|c| SafeByteCount::new(c).ok()),
        partitions,
        boot_artifact_strategy: BootArtifactStrategy::UkiOnEsp,
        intent_acknowledged: req.intent_acknowledged,
        loss_risk_acknowledged: req.loss_risk_acknowledged,
        // Campo legado no autoritativo; los ejes ortogonales mandan.
        plan_status: match (
            &plan_validation,
            &hardware_status,
            &capacity_status,
            &boot_status,
        ) {
            (
                PlanValidationStatus::Valid,
                HardwareVerificationStatus::Verified,
                CapacityVerificationStatus::Sufficient,
                BootCompatibilityStatus::Compatible,
            ) => PlanCertifiabilityStatus::Certifiable,
            (
                _,
                _,
                CapacityVerificationStatus::Unknown | CapacityVerificationStatus::Unverified,
                _,
            ) => PlanCertifiabilityStatus::CapacityUnverified,
            (_, HardwareVerificationStatus::Pending, _, _) => {
                PlanCertifiabilityStatus::VerificationPending
            }
            _ => PlanCertifiabilityStatus::NotCertifiable,
        },
    };

    let verification = VerificationStatuses {
        plan_validation_status: plan_validation,
        hardware_verification_status: hardware_status,
        boot_compatibility_status: boot_status,
        capacity_verification_status: capacity_status,
        execution_status: ExecutionStatus::NotExecuted,
    };

    let plan_digest = {
        use sha2::{Digest, Sha256};
        let mut h = Sha256::new();
        h.update(policy.version.as_bytes());
        h.update(serde_json::to_vec(&storage_plan).unwrap_or_default());
        h.update(serde_json::to_vec(&policy.encryption).unwrap_or_default());
        format!("{:x}", h.finalize())
    };

    PlanResult {
        storage_plan,
        verification,
        policy_version: policy.version.clone(),
        compat_verdict: compat.verdict,
        compat_code: compat.code,
        diagnostics,
        boot_help: boot_filesystem_help(policy.root_filesystem, req.firmware.kind),
        uki_esp_access: uki,
        plan_digest,
    }
}

pub fn validate_storage_plan(req: &PlanRequest, existing: &StoragePlan) -> PlanResult {
    let mut proposed = propose_storage_plan(req);
    // Revalidar coherencia del plan existente vs política.
    if existing.boot_artifact_strategy != BootArtifactStrategy::UkiOnEsp {
        proposed.verification.plan_validation_status = PlanValidationStatus::Invalid;
        proposed
            .diagnostics
            .push("bootArtifactStrategy distinto de uki-on-esp.".into());
    }
    if existing.partition_table != req.planned_partition_table {
        proposed
            .diagnostics
            .push("plannedPartitionTable del request no coincide con plan existente.".into());
        proposed.verification.plan_validation_status = PlanValidationStatus::Invalid;
    }
    // A6 fuera: execution siempre not-executed.
    proposed.verification.execution_status = ExecutionStatus::NotExecuted;
    proposed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn observed_mbr_can_propose_gpt_without_write() {
        let mut req = PlanRequest {
            mode: StorageMode::NewDisk,
            firmware: FirmwareDeclaration {
                kind: FirmwareKind::Uefi,
                provenance: FirmwareProvenance::Declared,
            },
            planned_partition_table: PartitionTableKind::Gpt,
            observed_partition_table: Some(PartitionTableObservation {
                status: PartitionTableObservationStatus::Observed,
                kind: Some(PartitionTableKind::Mbr),
            }),
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
        let result = propose_storage_plan(&req);
        assert_eq!(result.storage_plan.partition_table, PartitionTableKind::Gpt);
        assert_eq!(
            result
                .storage_plan
                .observed_partition_table
                .as_ref()
                .and_then(|o| o.kind),
            Some(PartitionTableKind::Mbr)
        );
        assert_eq!(
            result.verification.execution_status,
            ExecutionStatus::NotExecuted
        );
        assert_ne!(
            result.storage_plan.partition_table,
            result
                .storage_plan
                .observed_partition_table
                .as_ref()
                .and_then(|o| o.kind)
                .unwrap()
        );
        let _ = &mut req;
    }
}
