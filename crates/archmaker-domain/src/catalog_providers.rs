//! Providers de catálogo y alias ONLYOFFICE (ADR-0017).
//!
//! La cardinalidad de navegadores vive en [`crate::schema_v2::BrowserPolicy`].
//! Aquí solo hechos de proveedor (paquete + fuente + revisión + fecha).

use serde::{Deserialize, Serialize};

/// Alias histórico de cara/HTML (mismo paquete AUR; no es una segunda instalación).
pub const ONLYOFFICE_HISTORICAL_ALIAS_ID: &str = "archmaker.app.onlyoffice-standalone";

/// Canal de proveedor verificado o solicitud no verificada.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderChannel {
    Core,
    Extra,
    Aur,
    None,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProviderStatus {
    Verified,
    Unverified,
}

/// Hecho de proveedor (paquete + fuente + revisión + fecha). Sin cmd/shell.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CatalogProvider {
    pub channel: ProviderChannel,
    pub package_id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub architecture: Option<String>,
    pub source_url: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub revision: Option<String>,
    /// Fecha UTC de la captura (YYYY-MM-DD).
    pub observed_at: String,
    pub status: ProviderStatus,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub note: Option<String>,
}

/// Resuelve la identidad canónica ONLYOFFICE (alias histórico → canónico).
pub fn resolve_onlyoffice_canonical(option_id: &str) -> Option<&'static str> {
    match option_id {
        crate::ONLYOFFICE_CANONICAL_ID | ONLYOFFICE_HISTORICAL_ALIAS_ID => {
            Some(crate::ONLYOFFICE_CANONICAL_ID)
        }
        _ => None,
    }
}

/// True si ambos IDs denotan la misma instalación ONLYOFFICE.
pub fn onlyoffice_ids_equivalent(a: &str, b: &str) -> bool {
    match (
        resolve_onlyoffice_canonical(a),
        resolve_onlyoffice_canonical(b),
    ) {
        (Some(x), Some(y)) => x == y,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn onlyoffice_canonical_and_alias() {
        assert_eq!(
            resolve_onlyoffice_canonical(ONLYOFFICE_HISTORICAL_ALIAS_ID),
            Some(crate::ONLYOFFICE_CANONICAL_ID)
        );
        assert!(onlyoffice_ids_equivalent(
            crate::ONLYOFFICE_CANONICAL_ID,
            ONLYOFFICE_HISTORICAL_ALIAS_ID
        ));
        assert!(!onlyoffice_ids_equivalent(
            crate::ONLYOFFICE_CANONICAL_ID,
            "archmaker.pkg.libreoffice-fresh"
        ));
    }
}
