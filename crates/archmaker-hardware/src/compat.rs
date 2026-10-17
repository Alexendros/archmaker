//! Reglas de compatibilidad firmware/arranque (sin IO).
//! unknown/unavailable ≠ incompatible.

use serde::{Deserialize, Serialize};

use crate::observation::{ProbeStatus, TargetBinding};
use archmaker_domain::{
    BootCompatibilityStatus, FirmwareKind, FirmwareProvenance, PartitionTableKind,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompatVerdict {
    Compatible,
    Conditional,
    Incompatible,
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CompatCode {
    Uefi64Ok,
    Uefi32NotUefi64,
    BiosCsmNoPhysicalConclusion,
    ContainerEfiHidden,
    SecureBootEnabled,
    SecureBootDisabled,
    SecureBootUnknown,
    EfivarfsDenied,
    HostNotTarget,
    BootloaderMismatch,
    GptOk,
    MbrNotCertifiable,
    UnsupportedByCurrentPolicy,
    InsufficientEvidence,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CurrentBootMode {
    Uefi64,
    Uefi32,
    LegacyOrCsm,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum UefiCapability {
    /// Capacidad física no inferida; solo se vio la sesión.
    SessionOnly,
    Declared,
    #[default]
    Unknown,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum SecureBootState {
    Enabled,
    Disabled,
    #[default]
    Unknown,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HostKind {
    NativeLinux,
    VirtualMachine,
    Container,
    Restricted,
    #[default]
    Unsupported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FirmwareCompatInput {
    pub current_boot_mode: CurrentBootMode,
    pub uefi_capability: UefiCapability,
    pub secure_boot: SecureBootState,
    pub efivarfs_access: ProbeStatus,
    pub in_container: bool,
    pub planned_firmware: FirmwareKind,
    pub planned_firmware_provenance: FirmwareProvenance,
    pub planned_partition_table: PartitionTableKind,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub observed_partition_table: Option<PartitionTableKind>,
    pub target_binding: TargetBinding,
    pub bootloader_requires_uefi: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FirmwareCompatReport {
    pub verdict: CompatVerdict,
    pub code: CompatCode,
    pub boot_compatibility_status: BootCompatibilityStatus,
    pub diagnostics: Vec<String>,
    pub may_certify_uefi_profile: bool,
}

pub fn evaluate_firmware_compat(input: &FirmwareCompatInput) -> FirmwareCompatReport {
    let mut diagnostics = Vec::new();
    let mut code = CompatCode::InsufficientEvidence;
    let mut may_certify = false;

    if input.in_container
        && matches!(
            input.current_boot_mode,
            CurrentBootMode::Unknown | CurrentBootMode::LegacyOrCsm
        )
    {
        code = CompatCode::ContainerEfiHidden;
        diagnostics.push(
            "EFI no observable en contenedor; unknown/unavailable ≠ incompatibilidad UEFI física."
                .into(),
        );
    }

    if matches!(
        input.target_binding,
        TargetBinding::OtherTarget | TargetBinding::Unbound
    ) {
        diagnostics
            .push("Host≠destino o no confirmado: la observación no certifica el destino.".into());
    }

    if matches!(input.efivarfs_access, ProbeStatus::Unavailable) {
        code = CompatCode::EfivarfsDenied;
        diagnostics.push(
            "efivarfs sin permiso; Secure Boot puede quedar unknown. Cero escrituras.".into(),
        );
    }

    match input.secure_boot {
        SecureBootState::Enabled => {
            diagnostics.push(
                "Secure Boot enabled observado; no se modifica ni se asume firma del cargador."
                    .into(),
            );
        }
        SecureBootState::Disabled => {}
        SecureBootState::Unknown => {
            diagnostics.push("Secure Boot unknown.".into());
        }
    }

    let mut verdict = match input.current_boot_mode {
        CurrentBootMode::Uefi64 => {
            code = CompatCode::Uefi64Ok;
            diagnostics.push("Arranque EFI 64 bits observado en esta sesión.".into());
            if matches!(input.target_binding, TargetBinding::CurrentHostConfirmed)
                && input.planned_firmware == FirmwareKind::Uefi
                && input.planned_partition_table == PartitionTableKind::Gpt
                && matches!(
                    input.planned_firmware_provenance,
                    FirmwareProvenance::Observed
                )
            {
                may_certify = true;
                CompatVerdict::Compatible
            } else if matches!(
                input.planned_firmware_provenance,
                FirmwareProvenance::Declared
            ) {
                diagnostics.push("UEFI declarado: verification-pending.".into());
                CompatVerdict::Conditional
            } else {
                CompatVerdict::Conditional
            }
        }
        CurrentBootMode::Uefi32 => {
            code = CompatCode::Uefi32NotUefi64;
            diagnostics
                .push("EFI 32 bits: no se confunde con UEFI64 ni con arquitectura de CPU.".into());
            may_certify = false;
            CompatVerdict::Incompatible
        }
        CurrentBootMode::LegacyOrCsm => {
            code = CompatCode::BiosCsmNoPhysicalConclusion;
            diagnostics.push("Sesión BIOS/CSM: no se concluye incapacidad UEFI física.".into());
            if input.planned_firmware == FirmwareKind::Uefi {
                diagnostics.push(
                    "Plan UEFI declarado con verification-pending; sugerir arranque UEFI.".into(),
                );
            }
            may_certify = false;
            CompatVerdict::Conditional
        }
        CurrentBootMode::Unknown => {
            if !input.in_container {
                code = CompatCode::BiosCsmNoPhysicalConclusion;
                diagnostics
                    .push("Modo de arranque unknown; no se convierte en incompatible.".into());
            }
            may_certify = false;
            CompatVerdict::Unknown
        }
    };

    if input.planned_partition_table == PartitionTableKind::Mbr {
        code = CompatCode::MbrNotCertifiable;
        diagnostics.push("MBR planificado no certifica.".into());
        may_certify = false;
        if verdict == CompatVerdict::Compatible {
            verdict = CompatVerdict::Conditional;
        }
    } else if input.planned_partition_table == PartitionTableKind::Gpt && may_certify {
        code = CompatCode::GptOk;
    }

    // observed≠planned: solo informativo
    if let Some(obs) = input.observed_partition_table {
        if obs != input.planned_partition_table {
            diagnostics.push(format!(
                "observedPartitionTable({obs:?}) ≠ plannedPartitionTable({:?}); ortogonales.",
                input.planned_partition_table
            ));
        }
    }

    if input.bootloader_requires_uefi && input.planned_firmware != FirmwareKind::Uefi {
        diagnostics.push(
            "systemd-boot/UKI exige UEFI; no válido en BIOS. UnsupportedByCurrentPolicy.".into(),
        );
        code = CompatCode::UnsupportedByCurrentPolicy;
        verdict = CompatVerdict::Incompatible;
        may_certify = false;
    }

    // Host≠destino prevalece: la observación no certifica el destino.
    if matches!(input.target_binding, TargetBinding::OtherTarget) {
        code = CompatCode::HostNotTarget;
        may_certify = false;
        if matches!(verdict, CompatVerdict::Compatible) {
            verdict = CompatVerdict::Conditional;
        }
    }

    // unknown no se fuerza a incompatible salvo UEFI32 / bootloader.
    if matches!(input.current_boot_mode, CurrentBootMode::Unknown)
        && verdict == CompatVerdict::Incompatible
        && !matches!(
            code,
            CompatCode::Uefi32NotUefi64
                | CompatCode::BootloaderMismatch
                | CompatCode::UnsupportedByCurrentPolicy
        )
    {
        verdict = CompatVerdict::Unknown;
    }

    let boot_compatibility_status = match verdict {
        CompatVerdict::Compatible => BootCompatibilityStatus::Compatible,
        CompatVerdict::Conditional => BootCompatibilityStatus::Conditional,
        CompatVerdict::Incompatible => BootCompatibilityStatus::Incompatible,
        CompatVerdict::Unknown => BootCompatibilityStatus::Unknown,
    };

    // Secure Boot enabled sin estrategia de firma/confianza en este incremento:
    // no Incompatible por sí solo, pero sí Conditional / no certifica (T36).
    let (verdict, code, may_certify, boot_compatibility_status) =
        if matches!(input.secure_boot, SecureBootState::Enabled) {
            let verdict = match verdict {
                CompatVerdict::Compatible => CompatVerdict::Conditional,
                other => other,
            };
            let boot_compatibility_status = match verdict {
                CompatVerdict::Compatible => BootCompatibilityStatus::Compatible,
                CompatVerdict::Conditional => BootCompatibilityStatus::Conditional,
                CompatVerdict::Incompatible => BootCompatibilityStatus::Incompatible,
                CompatVerdict::Unknown => BootCompatibilityStatus::Unknown,
            };
            (
                verdict,
                CompatCode::SecureBootEnabled,
                false,
                boot_compatibility_status,
            )
        } else {
            (verdict, code, may_certify, boot_compatibility_status)
        };

    FirmwareCompatReport {
        verdict,
        code,
        boot_compatibility_status,
        diagnostics,
        may_certify_uefi_profile: may_certify,
    }
}
