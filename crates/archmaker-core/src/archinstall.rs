use std::collections::BTreeSet;

use archmaker_domain::{Catalog, CatalogOption, Manifest, ProviderChannel, ProviderStatus};

use crate::core::NON_REDISTRIBUTABLE_TAG;
use crate::error::{CoreError, Result};
use crate::port::CoreOp;

pub const ARCHINSTALL_CONFIG_VERSION: &str = "2.8.6";

pub struct ArchinstallConfig {
    pub value: serde_json::Value,
    pub deferred: Vec<String>,
    pub warnings: Vec<String>,
}

pub fn render_archinstall_config(
    catalog: &Catalog,
    manifest: &Manifest,
) -> Result<ArchinstallConfig> {
    let mut kernels = BTreeSet::new();
    let mut packages = BTreeSet::new();
    let mut deferred = BTreeSet::new();
    let mut warnings = Vec::new();
    let mut bootloader = None;
    let mut desktop = None;
    let mut greeter = None;

    for selection in &manifest.effective_selections {
        let option_id = selection.option_id.as_str();
        let option = find_option(catalog, option_id);
        let suffix = option_id.rsplit('.').next().unwrap_or(option_id);

        if option_id.starts_with("archmaker.kernel.") {
            match option.and_then(verified_package) {
                Some(package) => {
                    kernels.insert(package);
                }
                None if package_name_is_safe(suffix) => {
                    kernels.insert(suffix.to_string());
                }
                None => warnings.push(format!("{option_id}: kernel sin nombre de paquete válido")),
            }
        } else if option_id.starts_with("archmaker.boot.") {
            match bootloader_name(suffix) {
                Some(value) => bootloader = Some(value),
                None => warnings.push(format!(
                    "{option_id}: gestor de arranque sin valor archinstall"
                )),
            }
        } else if option_id.starts_with("archmaker.compositor.")
            || option_id.starts_with("archmaker.desktop.")
        {
            match desktop_name(suffix) {
                Some(value) => desktop = Some(value),
                None => warnings.push(format!("{option_id}: escritorio sin perfil archinstall")),
            }
        } else if option_id.starts_with("archmaker.dm.") {
            match greeter_name(suffix) {
                Some(value) => greeter = Some(value),
                None => warnings.push(format!(
                    "{option_id}: gestor de acceso sin greeter archinstall"
                )),
            }
        } else if option_id.starts_with("archmaker.filesystem.")
            || option_id.starts_with("archmaker.build.")
        {
            continue;
        } else if option.is_some_and(is_deferred) {
            match deferred_package(option) {
                Some(package) => {
                    deferred.insert(package);
                }
                None => warnings.push(format!(
                    "{option_id}: paquete no redistribuible sin nombre verificado"
                )),
            }
        } else {
            match package_from_selection(option, option_id) {
                Some(package) => {
                    packages.insert(package);
                }
                None => warnings.push(format!("{option_id}: sin paquete verificado")),
            }
        }
    }

    if kernels.is_empty() {
        warnings.push("sin kernel seleccionado: se usará linux".to_string());
        kernels.insert("linux".to_string());
    }
    warnings.sort();

    let mut config = serde_json::Map::new();
    config.insert(
        "version".to_string(),
        serde_json::Value::String(ARCHINSTALL_CONFIG_VERSION.to_string()),
    );
    config.insert(
        "script".to_string(),
        serde_json::Value::String("guided".to_string()),
    );
    config.insert(
        "hostname".to_string(),
        serde_json::Value::String("forge".to_string()),
    );
    config.insert("kernels".to_string(), serde_json::json!(kernels));
    if let Some(bootloader) = bootloader {
        config.insert(
            "bootloader_config".to_string(),
            serde_json::json!({
                "bootloader": bootloader,
                "uki": false,
                "removable": false,
            }),
        );
    }
    if let Some(desktop) = desktop {
        let mut profile = serde_json::Map::new();
        profile.insert(
            "main".to_string(),
            serde_json::Value::String("Desktop".to_string()),
        );
        profile.insert("details".to_string(), serde_json::json!([desktop]));
        let mut profile_config = serde_json::Map::new();
        profile_config.insert(
            "gfx_driver".to_string(),
            serde_json::Value::String("All open-source".to_string()),
        );
        if let Some(greeter) = greeter {
            profile_config.insert(
                "greeter".to_string(),
                serde_json::Value::String(greeter.to_string()),
            );
        }
        profile_config.insert("profile".to_string(), serde_json::Value::Object(profile));
        config.insert(
            "profile_config".to_string(),
            serde_json::Value::Object(profile_config),
        );
    }
    config.insert(
        "locale_config".to_string(),
        serde_json::json!({
            "kb_layout": "us",
            "sys_enc": "UTF-8",
            "sys_lang": "en_US.UTF-8",
        }),
    );
    config.insert(
        "audio_config".to_string(),
        serde_json::json!({ "audio": "pipewire" }),
    );
    config.insert(
        "timezone".to_string(),
        serde_json::Value::String("UTC".to_string()),
    );
    config.insert("ntp".to_string(), serde_json::Value::Bool(true));
    if !packages.is_empty() {
        config.insert("packages".to_string(), serde_json::json!(packages));
    }

    let value = serde_json::Value::Object(config);
    let bytes = archmaker_canonicalization::canonicalize(&value);
    let value = serde_json::from_slice(&bytes)
        .map_err(|_| CoreError::doc_canonical(CoreOp::ExportArtifact.source()))?;

    Ok(ArchinstallConfig {
        value,
        deferred: deferred.into_iter().collect(),
        warnings,
    })
}

fn find_option<'a>(catalog: &'a Catalog, option_id: &str) -> Option<&'a CatalogOption> {
    catalog
        .steps
        .iter()
        .flat_map(|step| step.sections.iter())
        .flat_map(|section| section.options.iter())
        .find(|option| option.id == option_id)
}

fn has_tag(option: &CatalogOption, tag: &str) -> bool {
    option
        .tags
        .as_ref()
        .is_some_and(|tags| tags.iter().any(|value| value == tag))
}

fn is_deferred(option: &CatalogOption) -> bool {
    if has_tag(option, NON_REDISTRIBUTABLE_TAG) {
        return true;
    }
    match option.provider.as_ref() {
        Some(provider) => {
            provider.channel == ProviderChannel::Aur || provider.status != ProviderStatus::Verified
        }
        None => false,
    }
}

fn verified_package(option: &CatalogOption) -> Option<String> {
    let provider = option.provider.as_ref()?;
    if provider.status != ProviderStatus::Verified || provider.channel == ProviderChannel::Aur {
        return None;
    }
    package_name_is_safe(&provider.package_id).then(|| provider.package_id.clone())
}

fn deferred_package(option: Option<&CatalogOption>) -> Option<String> {
    let option = option?;
    let package = option.provider.as_ref()?.package_id.clone();
    package_name_is_safe(&package).then_some(package)
}

fn package_from_selection(option: Option<&CatalogOption>, option_id: &str) -> Option<String> {
    if let Some(option) = option {
        if let Some(package) = verified_package(option) {
            return Some(package);
        }
        if option.provider.is_some() {
            return None;
        }
    }
    let suffix = option_id.strip_prefix("archmaker.pkg.")?;
    package_name_is_safe(suffix).then(|| suffix.to_string())
}

fn package_name_is_safe(name: &str) -> bool {
    if name.is_empty() || matches!(name, "pacstrap" | "cmd") {
        return false;
    }
    let first = name.chars().next().unwrap_or_default();
    if !(first.is_ascii_lowercase() || first.is_ascii_digit()) {
        return false;
    }
    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '+' | '.' | '_' | '-'))
}

fn bootloader_name(suffix: &str) -> Option<&'static str> {
    match suffix {
        "systemd-boot" => Some("Systemd-boot"),
        "grub" => Some("Grub"),
        "limine" => Some("Limine"),
        _ => None,
    }
}

fn desktop_name(suffix: &str) -> Option<&'static str> {
    match suffix {
        "gnome" => Some("GNOME"),
        "hyprland" => Some("Hyprland"),
        "sway" => Some("Sway"),
        "cosmic" => Some("Cosmic"),
        "kde" => Some("KDE Plasma"),
        "cinnamon" => Some("Cinnamon"),
        "xfce" => Some("Xfce4"),
        "budgie" => Some("Budgie"),
        "i3" => Some("i3-wm"),
        _ => None,
    }
}

fn greeter_name(suffix: &str) -> Option<&'static str> {
    match suffix {
        "gdm" => Some("gdm"),
        "sddm" => Some("sddm"),
        _ => None,
    }
}
