//! Ayuda de boot/FS y validación de acceso a artefactos UKI en ESP.
//! LUKS2 por capas; passphrase-requested-at-install sin secretos; sin activación silenciosa.
//! Cero botón Formatear; A6 fuera.

use archmaker_domain::{FilesystemKind, FirmwareKind};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BootFsHelp {
    pub bootloader_title: String,
    pub bootloader_summary: String,
    pub bootloader_limits: Vec<String>,
    pub filesystem_title: String,
    pub filesystem_summary: String,
    pub filesystem_limits: Vec<String>,
    pub luks_summary: String,
    pub format_button_present: bool,
    pub a6_execution_available: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct UkiEspAccess {
    pub accessible: bool,
    pub strategy: String,
    pub mount_point: String,
    pub diagnostics: Vec<String>,
}

pub fn boot_filesystem_help(fs: FilesystemKind, firmware: FirmwareKind) -> BootFsHelp {
    let uefi_only = !matches!(firmware, FirmwareKind::Bios);
    BootFsHelp {
        bootloader_title: "systemd-boot + UKI en ESP".into(),
        bootloader_summary: "Gestor UEFI que carga Unified Kernel Images desde la ESP montada en /efi."
            .into(),
        bootloader_limits: vec![
            "systemd-boot es solo UEFI; no se propone sobre BIOS/MBR.".into(),
            "UKI vive en la ESP (FAT32 /efi), fuera de LUKS2.".into(),
            if uefi_only {
                "Firmware UEFI requerido para este perfil de referencia.".into()
            } else {
                "Firmware BIOS declarado: el perfil no certifica este bootloader.".into()
            },
        ],
        filesystem_title: match fs {
            FilesystemKind::Ext4 => "ext4 en raíz".into(),
            FilesystemKind::Btrfs => "btrfs en raíz".into(),
            FilesystemKind::Fat32 => "FAT32".into(),
        },
        filesystem_summary: match fs {
            FilesystemKind::Ext4 => {
                "Sistema de archivos estable para la raíz; sin snapshots integrados.".into()
            }
            FilesystemKind::Btrfs => {
                "Permite subvolúmenes y snapshots configurados; no sustituye una copia de seguridad."
                    .into()
            }
            FilesystemKind::Fat32 => "Usado en la ESP, no como raíz del sistema.".into(),
        },
        filesystem_limits: match fs {
            FilesystemKind::Btrfs => vec![
                "Los snapshots de Btrfs no son un backup off-disk.".into(),
                "Hay que configurar subvolúmenes de forma explícita.".into(),
            ],
            _ => vec!["Sin promesa de borrado seguro al formatear (A6 fuera).".into()],
        },
        luks_summary:
            "LUKS2 opcional solo en la capa de raíz; método passphrase-requested-at-install sin valor; sin TPM auto-unlock ni activación silenciosa."
                .into(),
        format_button_present: false,
        a6_execution_available: false,
    }
}

/// Valida acceso *declarativo* a artefactos UKI en ESP (sin montar ni escribir).
pub fn validate_uki_esp_access(
    uefi_gpt_plan: bool,
    esp_size_ok: bool,
    strategy_uki_on_esp: bool,
) -> UkiEspAccess {
    let mut diagnostics = Vec::new();
    if !strategy_uki_on_esp {
        diagnostics.push("Estrategia de arranque no es uki-on-esp.".into());
    }
    if !uefi_gpt_plan {
        diagnostics.push("UKI en ESP requiere plan UEFI+GPT.".into());
    }
    if !esp_size_ok {
        diagnostics.push("ESP por debajo del mínimo de política para UKI.".into());
    }
    UkiEspAccess {
        accessible: diagnostics.is_empty(),
        strategy: "uki-on-esp".into(),
        mount_point: "/efi".into(),
        diagnostics,
    }
}
