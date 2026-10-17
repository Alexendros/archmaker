//! Lectura sysfs EFI / Secure Boot (solo lectura; sin escrituras en efivarfs).

use std::fs;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};

use serde::{Deserialize, Serialize};

use crate::compat::{CurrentBootMode, SecureBootState, UefiCapability};
use crate::observation::{ObservationOrigin, ProbeObservation, TargetBinding, VerificationStatus};

fn now_rfc3339ish() -> String {
    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0);
    format!("{secs}")
}

fn read_trimmed(path: &Path) -> Result<String, std::io::Error> {
    let s = fs::read_to_string(path)?;
    Ok(s.trim().to_string())
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FirmwareProbeReport {
    pub current_boot_mode: ProbeObservation<CurrentBootMode>,
    pub uefi_capability: ProbeObservation<UefiCapability>,
    pub fw_platform_size: ProbeObservation<u32>,
    pub secure_boot: ProbeObservation<SecureBootState>,
    pub efivarfs_access: ProbeObservation<bool>,
    pub in_container: ProbeObservation<bool>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HostHardwareReport {
    pub observed_at: String,
    pub origin: ObservationOrigin,
    pub target_binding: TargetBinding,
    pub verification_status: VerificationStatus,
    pub os: ProbeObservation<String>,
    pub arch: ProbeObservation<String>,
    pub firmware: FirmwareProbeReport,
    /// Escrituras EFI realizadas por esta sonda (debe ser 0).
    pub efi_writes: u32,
}

fn detect_container(at: &str) -> ProbeObservation<bool> {
    let docker = Path::new("/.dockerenv").exists();
    let cgroup = fs::read_to_string("/proc/1/cgroup").unwrap_or_default();
    let in_ctr = docker
        || cgroup.contains("docker")
        || cgroup.contains("containerd")
        || Path::new("/run/systemd/container").exists();
    ProbeObservation::detected(
        in_ctr,
        "proc/sysfs",
        at,
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
        VerificationStatus::ObservedBound,
    )
}

fn probe_efi(at: &str, binding: TargetBinding) -> FirmwareProbeReport {
    let efi_dir = Path::new("/sys/firmware/efi");
    let size_path = Path::new("/sys/firmware/efi/fw_platform_size");

    let (boot_mode, fw_size, uefi_cap) = if !efi_dir.exists() {
        (
            ProbeObservation::unknown(
                "sysfs:/sys/firmware/efi",
                at,
                ObservationOrigin::LocalProbe,
                "Sin /sys/firmware/efi: sesión no EFI o filtrada; unknown≠incompatible",
            ),
            ProbeObservation::<u32>::unknown(
                "sysfs:fw_platform_size",
                at,
                ObservationOrigin::LocalProbe,
                "fw_platform_size ausente",
            ),
            ProbeObservation::unknown(
                "sysfs:uefi-capability",
                at,
                ObservationOrigin::LocalProbe,
                "Capacidad UEFI física desconocida en esta sesión",
            ),
        )
    } else {
        match read_trimmed(size_path) {
            Ok(v) => {
                let parsed = v.parse::<u32>().ok();
                match parsed {
                    Some(64) => (
                        ProbeObservation::detected(
                            CurrentBootMode::Uefi64,
                            "sysfs:fw_platform_size",
                            at,
                            ObservationOrigin::LocalProbe,
                            binding,
                            if matches!(binding, TargetBinding::CurrentHostConfirmed) {
                                VerificationStatus::ObservedBound
                            } else {
                                VerificationStatus::InsufficientEvidence
                            },
                        ),
                        ProbeObservation::detected(
                            64,
                            "sysfs:fw_platform_size",
                            at,
                            ObservationOrigin::LocalProbe,
                            binding,
                            VerificationStatus::ObservedBound,
                        ),
                        ProbeObservation::detected(
                            UefiCapability::SessionOnly,
                            "sysfs:fw_platform_size",
                            at,
                            ObservationOrigin::LocalProbe,
                            binding,
                            VerificationStatus::ObservedBound,
                        ),
                    ),
                    Some(32) => (
                        ProbeObservation::detected(
                            CurrentBootMode::Uefi32,
                            "sysfs:fw_platform_size",
                            at,
                            ObservationOrigin::LocalProbe,
                            binding,
                            VerificationStatus::ObservedBound,
                        ),
                        ProbeObservation::detected(
                            32,
                            "sysfs:fw_platform_size",
                            at,
                            ObservationOrigin::LocalProbe,
                            binding,
                            VerificationStatus::ObservedBound,
                        ),
                        ProbeObservation::detected(
                            UefiCapability::SessionOnly,
                            "sysfs:fw_platform_size",
                            at,
                            ObservationOrigin::LocalProbe,
                            binding,
                            VerificationStatus::ObservedBound,
                        ),
                    ),
                    _ => (
                        ProbeObservation::unknown(
                            "sysfs:fw_platform_size",
                            at,
                            ObservationOrigin::LocalProbe,
                            format!("fw_platform_size no interpretable: {v}"),
                        ),
                        ProbeObservation::unknown(
                            "sysfs:fw_platform_size",
                            at,
                            ObservationOrigin::LocalProbe,
                            "valor no numérico",
                        ),
                        ProbeObservation::unknown(
                            "sysfs:uefi-capability",
                            at,
                            ObservationOrigin::LocalProbe,
                            "bitness EFI desconocido",
                        ),
                    ),
                }
            }
            Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => (
                ProbeObservation::unavailable(
                    "sysfs:fw_platform_size",
                    at,
                    ObservationOrigin::LocalProbe,
                    "permiso denegado leyendo fw_platform_size",
                ),
                ProbeObservation::unavailable(
                    "sysfs:fw_platform_size",
                    at,
                    ObservationOrigin::LocalProbe,
                    "permiso denegado",
                ),
                ProbeObservation::unavailable(
                    "sysfs:uefi-capability",
                    at,
                    ObservationOrigin::LocalProbe,
                    "permiso denegado",
                ),
            ),
            Err(_) => (
                // Directorio EFI existe pero sin size: sesión EFI sin bitness.
                ProbeObservation::detected(
                    CurrentBootMode::Unknown,
                    "sysfs:/sys/firmware/efi",
                    at,
                    ObservationOrigin::LocalProbe,
                    binding,
                    VerificationStatus::InsufficientEvidence,
                ),
                ProbeObservation::unknown(
                    "sysfs:fw_platform_size",
                    at,
                    ObservationOrigin::LocalProbe,
                    "fw_platform_size ausente con EFI presente",
                ),
                ProbeObservation::detected(
                    UefiCapability::SessionOnly,
                    "sysfs:/sys/firmware/efi",
                    at,
                    ObservationOrigin::LocalProbe,
                    binding,
                    VerificationStatus::InsufficientEvidence,
                ),
            ),
        }
    };

    // Secure Boot: lectura estructurada sin escritura.
    let efivar_sb =
        Path::new("/sys/firmware/efi/efivars/SecureBoot-8be4df61-93ca-11d2-aa0d-00e098032b8c");
    let (secure_boot, efivarfs_access) = match fs::read(efivar_sb) {
        Ok(bytes) => {
            // Atributos (4 bytes) + valor.
            let state = if bytes.len() >= 5 {
                match bytes[4] {
                    1 => SecureBootState::Enabled,
                    0 => SecureBootState::Disabled,
                    _ => SecureBootState::Unknown,
                }
            } else {
                SecureBootState::Unknown
            };
            (
                ProbeObservation::detected(
                    state,
                    "efivarfs:SecureBoot",
                    at,
                    ObservationOrigin::LocalProbe,
                    binding,
                    VerificationStatus::ObservedBound,
                ),
                ProbeObservation::detected(
                    true,
                    "efivarfs",
                    at,
                    ObservationOrigin::LocalProbe,
                    binding,
                    VerificationStatus::ObservedBound,
                ),
            )
        }
        Err(e) if e.kind() == std::io::ErrorKind::PermissionDenied => (
            ProbeObservation::unavailable(
                "efivarfs:SecureBoot",
                at,
                ObservationOrigin::LocalProbe,
                "permiso denegado; cero escrituras de prueba",
            ),
            ProbeObservation::unavailable(
                "efivarfs",
                at,
                ObservationOrigin::LocalProbe,
                "permiso denegado",
            ),
        ),
        Err(_) if !efi_dir.exists() => (
            ProbeObservation::unknown(
                "efivarfs:SecureBoot",
                at,
                ObservationOrigin::LocalProbe,
                "sin EFI; Secure Boot unknown",
            ),
            ProbeObservation::unknown(
                "efivarfs",
                at,
                ObservationOrigin::LocalProbe,
                "efivarfs no montado o ausente",
            ),
        ),
        Err(_) => (
            ProbeObservation::unknown(
                "efivarfs:SecureBoot",
                at,
                ObservationOrigin::LocalProbe,
                "Secure Boot no legible",
            ),
            ProbeObservation::detected(
                Path::new("/sys/firmware/efi/efivars").exists(),
                "efivarfs",
                at,
                ObservationOrigin::LocalProbe,
                binding,
                VerificationStatus::InsufficientEvidence,
            ),
        ),
    };

    // Si no hay EFI, el modo puede ser legacy/CSM — sin concluir incapacidad UEFI física.
    let current_boot_mode = if !efi_dir.exists() {
        let mut obs = ProbeObservation::detected(
            CurrentBootMode::LegacyOrCsm,
            "sysfs:/sys/firmware/efi",
            at,
            ObservationOrigin::LocalProbe,
            TargetBinding::Unbound,
            VerificationStatus::InsufficientEvidence,
        );
        obs.diagnostic = Some(
            "Sin EFI en esta sesión (posible CSM/legacy); no se concluye falta de UEFI físico."
                .into(),
        );
        // Prefer unknown if container may hide EFI.
        obs
    } else {
        boot_mode
    };

    FirmwareProbeReport {
        current_boot_mode,
        uefi_capability: uefi_cap,
        fw_platform_size: fw_size,
        secure_boot,
        efivarfs_access,
        in_container: detect_container(at),
    }
}

/// Inspección de host: solo lectura. `target_binding` lo fija el llamador tras confirmación UI.
pub fn inspect_host_hardware(target_binding: TargetBinding) -> HostHardwareReport {
    let at = now_rfc3339ish();
    let firmware = probe_efi(&at, target_binding);
    let os = match read_trimmed(Path::new("/etc/os-release")) {
        Ok(s) => {
            let name = s
                .lines()
                .find_map(|l| l.strip_prefix("PRETTY_NAME="))
                .map(|v| v.trim_matches('"').to_string())
                .unwrap_or_else(|| "linux".into());
            ProbeObservation::detected(
                name,
                "/etc/os-release",
                &at,
                ObservationOrigin::LocalProbe,
                target_binding,
                VerificationStatus::ObservedBound,
            )
        }
        Err(_) => ProbeObservation::unknown(
            "/etc/os-release",
            &at,
            ObservationOrigin::LocalProbe,
            "os-release no legible",
        ),
    };
    let arch = ProbeObservation::detected(
        std::env::consts::ARCH.to_string(),
        "std::env::consts::ARCH",
        &at,
        ObservationOrigin::LocalProbe,
        target_binding,
        VerificationStatus::ObservedBound,
    );

    let verification_status = match target_binding {
        TargetBinding::CurrentHostConfirmed => VerificationStatus::ObservedBound,
        TargetBinding::OtherTarget => VerificationStatus::InsufficientEvidence,
        TargetBinding::Unbound => VerificationStatus::InsufficientEvidence,
    };

    HostHardwareReport {
        observed_at: at,
        origin: ObservationOrigin::LocalProbe,
        target_binding,
        verification_status,
        os,
        arch,
        firmware,
        efi_writes: 0,
    }
}
