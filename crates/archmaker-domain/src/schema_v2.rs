//! Contratos de draft/manifiesto `schemaVersion` `"2"` (fase fundacional Forge).
//!
//! Autoridad semántica en Rust. Sin IO de host, sin ejecución en discos (A6 fuera).

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// Límite de enteros seguros compartido con JavaScript (`Number.MAX_SAFE_INTEGER`).
pub const JS_MAX_SAFE_INTEGER: u64 = 9_007_199_254_740_991;

pub const DRAFT_SCHEMA_V1: &str = "1";
pub const DRAFT_SCHEMA_V2: &str = "2";

pub const MANIFEST_DOMAIN_LABEL_V1: &str = "archmaker:manifest:v1";
pub const MANIFEST_DOMAIN_LABEL_V2: &str = "archmaker:manifest:v2";
pub const DRAFT_DOMAIN_V1: &str = "archmaker:draft:v1";
pub const DRAFT_DOMAIN_V2: &str = "archmaker:draft:v2";

/// Identidad canónica ONLYOFFICE ya presente en el catálogo (no se inventa).
pub const ONLYOFFICE_CANONICAL_ID: &str = "archmaker.pkg.onlyoffice-bin";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DraftSchemaVersion {
    V1,
    V2,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SchemaVersionError {
    Invalid(String),
    UnsupportedFuture(String),
}

impl std::fmt::Display for SchemaVersionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Invalid(v) => write!(f, "schemaVersion inválida: {v}"),
            Self::UnsupportedFuture(v) => {
                write!(f, "schemaVersion futura no soportada: {v}")
            }
        }
    }
}

impl std::error::Error for SchemaVersionError {}

/// Decodifica `schemaVersion` de draft/manifiesto. Rechaza versiones futuras (`"3"`+).
pub fn decode_draft_schema_version(value: &str) -> Result<DraftSchemaVersion, SchemaVersionError> {
    match value {
        DRAFT_SCHEMA_V1 => Ok(DraftSchemaVersion::V1),
        DRAFT_SCHEMA_V2 => Ok(DraftSchemaVersion::V2),
        other if crate::is_integer_version(other) => {
            Err(SchemaVersionError::UnsupportedFuture(other.to_string()))
        }
        other => Err(SchemaVersionError::Invalid(other.to_string())),
    }
}

/// Conteo de bytes acotado a enteros seguros JS/Rust (sin pérdida en JSON number).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SafeByteCount(u64);

impl SafeByteCount {
    pub fn new(value: u64) -> Result<Self, SafeIntegerError> {
        if value > JS_MAX_SAFE_INTEGER {
            Err(SafeIntegerError::OutOfRange(value))
        } else {
            Ok(Self(value))
        }
    }

    pub const fn get(self) -> u64 {
        self.0
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SafeIntegerError {
    OutOfRange(u64),
    NotInteger,
    Negative,
}

impl std::fmt::Display for SafeIntegerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OutOfRange(v) => write!(f, "byte count fuera de rango seguro JS: {v}"),
            Self::NotInteger => write!(f, "byte count no es entero"),
            Self::Negative => write!(f, "byte count negativo"),
        }
    }
}

impl std::error::Error for SafeIntegerError {}

impl Serialize for SafeByteCount {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u64(self.0)
    }
}

impl<'de> Deserialize<'de> for SafeByteCount {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = serde_json::Number::deserialize(deserializer)?;
        let Some(n) = value.as_u64() else {
            if value.as_i64().is_some_and(|i| i < 0) {
                return Err(serde::de::Error::custom(SafeIntegerError::Negative));
            }
            return Err(serde::de::Error::custom(SafeIntegerError::NotInteger));
        };
        Self::new(n).map_err(serde::de::Error::custom)
    }
}

/// Estrategia de artefacto de arranque: UKI en ESP (sin XBOOTLDR).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BootArtifactStrategy {
    UkiOnEsp,
}

/// Handle de sesión (no portable; no se serializa en perfil exportable).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DeviceHandle {
    pub session_token: String,
}

/// Referencia lógica portable del destino (opaca; sin `/dev/...`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LogicalTargetRef {
    pub id: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum StorageMode {
    NewDisk,
    ReplaceDisk,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FirmwareKind {
    Uefi,
    Bios,
    Unspecified,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum FirmwareProvenance {
    Observed,
    Declared,
    Unknown,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct FirmwareDeclaration {
    pub kind: FirmwareKind,
    pub provenance: FirmwareProvenance,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PartitionTableKind {
    Gpt,
    Mbr,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PartitionTableObservationStatus {
    Observed,
    Declared,
    Unknown,
    Unavailable,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PartitionTableObservation {
    pub status: PartitionTableObservationStatus,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub kind: Option<PartitionTableKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PartitionRole {
    Esp,
    System,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum FilesystemKind {
    Fat32,
    Ext4,
    Btrfs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum LuksUnlockMethod {
    PassphraseRequestedAtInstall,
}

/// Capa de almacenamiento ordenada (ESP: FAT32→/efi; sistema: [LUKS2?]→FS→/).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "kebab-case", deny_unknown_fields)]
pub enum StorageLayer {
    Fat32 {
        #[serde(rename = "mountPoint")]
        mount_point: String,
    },
    Luks2 {
        unlock: LuksUnlockMethod,
    },
    Filesystem {
        #[serde(rename = "fs")]
        fs: FilesystemKind,
        #[serde(rename = "mountPoint")]
        mount_point: String,
    },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlannedAction {
    Create,
    Replace,
    KeepDeclared,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PlannedPartition {
    pub role: PartitionRole,
    pub size_bytes: SafeByteCount,
    pub layers: Vec<StorageLayer>,
    pub planned_action: PlannedAction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlanCertifiabilityStatus {
    Certifiable,
    NotCertifiable,
    CapacityUnverified,
    VerificationPending,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StoragePlan {
    pub mode: StorageMode,
    pub firmware: FirmwareDeclaration,
    /// Tabla planificada (intención).
    pub partition_table: PartitionTableKind,
    /// Tabla observada si existe; ortogonal a la planificada.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub observed_partition_table: Option<PartitionTableObservation>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub logical_target_ref: Option<LogicalTargetRef>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub declared_capacity_bytes: Option<SafeByteCount>,
    pub partitions: Vec<PlannedPartition>,
    pub boot_artifact_strategy: BootArtifactStrategy,
    pub intent_acknowledged: bool,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub loss_risk_acknowledged: Option<bool>,
    pub plan_status: PlanCertifiabilityStatus,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum EncryptionKind {
    None,
    Luks2,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EncryptionPolicy {
    pub kind: EncryptionKind,
    /// Solo raíz cuando `kind == luks2`; ESP queda fuera.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub unlock: Option<LuksUnlockMethod>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub covers_mount_point: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub initramfs_note: Option<String>,
}

impl EncryptionPolicy {
    pub fn none() -> Self {
        Self {
            kind: EncryptionKind::None,
            unlock: None,
            covers_mount_point: None,
            initramfs_note: None,
        }
    }
}

/// Cardinalidad de navegadores: selección múltiple + exactamente un default.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BrowserPolicy {
    pub selected_browser_ids: Vec<String>,
    pub default_browser_id: String,
}

impl BrowserPolicy {
    pub fn validate(&self) -> Result<(), BrowserPolicyError> {
        if self.selected_browser_ids.is_empty() {
            return Err(BrowserPolicyError::EmptySelection);
        }
        if self.default_browser_id.is_empty() {
            return Err(BrowserPolicyError::EmptyDefault);
        }
        if !self
            .selected_browser_ids
            .iter()
            .any(|id| id == &self.default_browser_id)
        {
            return Err(BrowserPolicyError::DefaultNotSelected);
        }
        let mut sorted = self.selected_browser_ids.clone();
        sorted.sort();
        sorted.dedup();
        if sorted.len() != self.selected_browser_ids.len() {
            return Err(BrowserPolicyError::DuplicateIds);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserPolicyError {
    EmptySelection,
    EmptyDefault,
    DefaultNotSelected,
    DuplicateIds,
}

impl std::fmt::Display for BrowserPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptySelection => write!(f, "selectedBrowserIds vacío"),
            Self::EmptyDefault => write!(f, "defaultBrowserId vacío"),
            Self::DefaultNotSelected => {
                write!(f, "defaultBrowserId no pertenece a selectedBrowserIds")
            }
            Self::DuplicateIds => write!(f, "selectedBrowserIds con duplicados"),
        }
    }
}

impl std::error::Error for BrowserPolicyError {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum PlanValidationStatus {
    NotValidated,
    Valid,
    Invalid,
    Pending,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum HardwareVerificationStatus {
    NotVerified,
    Verified,
    Pending,
    Unavailable,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BootCompatibilityStatus {
    Unknown,
    Compatible,
    Conditional,
    Incompatible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum CapacityVerificationStatus {
    Unverified,
    Sufficient,
    Insufficient,
    Unknown,
}

/// Ejecución en discos: siempre `not-executed` mientras A6 esté fuera.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ExecutionStatus {
    NotExecuted,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VerificationStatuses {
    pub plan_validation_status: PlanValidationStatus,
    pub hardware_verification_status: HardwareVerificationStatus,
    pub boot_compatibility_status: BootCompatibilityStatus,
    pub capacity_verification_status: CapacityVerificationStatus,
    pub execution_status: ExecutionStatus,
}

impl VerificationStatuses {
    pub fn foundational_defaults() -> Self {
        Self {
            plan_validation_status: PlanValidationStatus::NotValidated,
            hardware_verification_status: HardwareVerificationStatus::NotVerified,
            boot_compatibility_status: BootCompatibilityStatus::Unknown,
            capacity_verification_status: CapacityVerificationStatus::Unverified,
            execution_status: ExecutionStatus::NotExecuted,
        }
    }
}

/// Informe de migración draft v1 → v2 (idempotente).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DraftSchemaMigrationReport {
    pub from_schema_version: String,
    pub to_schema_version: String,
    pub entries: Vec<DraftSchemaMigrationEntry>,
    pub idempotent_noop: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DraftSchemaMigrationEntry {
    pub path: String,
    pub message_key: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub detail: Option<String>,
}

/// Plan ESP+raíz mínimo sin inventar hardware observado.
pub fn manual_reference_storage_plan() -> StoragePlan {
    StoragePlan {
        mode: StorageMode::NewDisk,
        firmware: FirmwareDeclaration {
            kind: FirmwareKind::Uefi,
            provenance: FirmwareProvenance::Declared,
        },
        partition_table: PartitionTableKind::Gpt,
        observed_partition_table: Some(PartitionTableObservation {
            status: PartitionTableObservationStatus::Unknown,
            kind: None,
        }),
        logical_target_ref: None,
        declared_capacity_bytes: None,
        partitions: vec![
            PlannedPartition {
                role: PartitionRole::Esp,
                size_bytes: SafeByteCount::new(2 * 1024 * 1024 * 1024).expect("2 GiB safe"),
                layers: vec![StorageLayer::Fat32 {
                    mount_point: "/efi".to_string(),
                }],
                planned_action: PlannedAction::Create,
            },
            PlannedPartition {
                role: PartitionRole::System,
                size_bytes: SafeByteCount::new(0).expect("0 safe"),
                layers: vec![StorageLayer::Filesystem {
                    fs: FilesystemKind::Ext4,
                    mount_point: "/".to_string(),
                }],
                planned_action: PlannedAction::Create,
            },
        ],
        boot_artifact_strategy: BootArtifactStrategy::UkiOnEsp,
        intent_acknowledged: true,
        loss_risk_acknowledged: None,
        plan_status: PlanCertifiabilityStatus::VerificationPending,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn decodes_v1_v2_rejects_future() {
        assert_eq!(
            decode_draft_schema_version("1").unwrap(),
            DraftSchemaVersion::V1
        );
        assert_eq!(
            decode_draft_schema_version("2").unwrap(),
            DraftSchemaVersion::V2
        );
        assert!(matches!(
            decode_draft_schema_version("3"),
            Err(SchemaVersionError::UnsupportedFuture(_))
        ));
        assert!(matches!(
            decode_draft_schema_version("v2"),
            Err(SchemaVersionError::Invalid(_))
        ));
    }

    #[test]
    fn safe_byte_count_bounds() {
        assert!(SafeByteCount::new(JS_MAX_SAFE_INTEGER).is_ok());
        assert!(SafeByteCount::new(JS_MAX_SAFE_INTEGER + 1).is_err());
        let json = format!("{JS_MAX_SAFE_INTEGER}");
        let parsed: SafeByteCount = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.get(), JS_MAX_SAFE_INTEGER);
        assert!(
            serde_json::from_str::<SafeByteCount>(&format!("{}", JS_MAX_SAFE_INTEGER + 1)).is_err()
        );
        assert!(serde_json::from_str::<SafeByteCount>("-1").is_err());
    }

    #[test]
    fn browser_policy_cardinality() {
        let ok = BrowserPolicy {
            selected_browser_ids: vec!["archmaker.app.firefox".into(), "archmaker.app.zen".into()],
            default_browser_id: "archmaker.app.zen".into(),
        };
        assert!(ok.validate().is_ok());
        let bad = BrowserPolicy {
            selected_browser_ids: vec!["archmaker.app.firefox".into()],
            default_browser_id: "archmaker.app.zen".into(),
        };
        assert!(matches!(
            bad.validate(),
            Err(BrowserPolicyError::DefaultNotSelected)
        ));
    }

    #[test]
    fn onlyoffice_canonical_id_is_stable() {
        assert_eq!(ONLYOFFICE_CANONICAL_ID, "archmaker.pkg.onlyoffice-bin");
    }

    #[test]
    fn uki_strategy_serializes() {
        let value = serde_json::to_value(BootArtifactStrategy::UkiOnEsp).unwrap();
        assert_eq!(value, serde_json::json!("uki-on-esp"));
    }

    #[test]
    fn execution_status_is_not_executed() {
        let statuses = VerificationStatuses::foundational_defaults();
        assert_eq!(statuses.execution_status, ExecutionStatus::NotExecuted);
        let value = serde_json::to_value(statuses.execution_status).unwrap();
        assert_eq!(value, serde_json::json!("not-executed"));
    }
}
