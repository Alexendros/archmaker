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

/// Default de perfil nuevo (catálogo `archmaker.browser.zen`).
pub const DEFAULT_BROWSER_ID_NEW_PROFILE: &str = "archmaker.browser.zen";

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

/// Cardinalidad de navegadores: varios seleccionados + a lo sumo un default.
///
/// Perfil sin navegador: selección vacía y `defaultBrowserId = null` (permitido;
/// RULE-BROWSER-001 sugiere, no bloquea). Nunca default huérfano.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BrowserPolicy {
    pub selected_browser_ids: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub default_browser_id: Option<String>,
}

impl BrowserPolicy {
    pub fn none() -> Self {
        Self {
            selected_browser_ids: Vec::new(),
            default_browser_id: None,
        }
    }

    pub fn new_profile_zen() -> Self {
        Self {
            selected_browser_ids: vec![DEFAULT_BROWSER_ID_NEW_PROFILE.to_string()],
            default_browser_id: Some(DEFAULT_BROWSER_ID_NEW_PROFILE.to_string()),
        }
    }

    pub fn validate(&self) -> Result<(), BrowserPolicyError> {
        let mut sorted = self.selected_browser_ids.clone();
        sorted.sort();
        sorted.dedup();
        if sorted.len() != self.selected_browser_ids.len() {
            return Err(BrowserPolicyError::DuplicateIds);
        }
        match &self.default_browser_id {
            None => Ok(()),
            Some(id) if id.is_empty() => Err(BrowserPolicyError::EmptyDefault),
            Some(id) => {
                if self.selected_browser_ids.iter().any(|s| s == id) {
                    Ok(())
                } else {
                    Err(BrowserPolicyError::OrphanDefault)
                }
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BrowserPolicyError {
    EmptyDefault,
    OrphanDefault,
    DuplicateIds,
}

impl std::fmt::Display for BrowserPolicyError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EmptyDefault => write!(f, "defaultBrowserId vacío (usar null)"),
            Self::OrphanDefault => {
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
    declared_storage_plan_from_intent(StorageMode::NewDisk, true, None)
}

/// Materializa un plan declarativo a partir del modo y las confirmaciones de cara.
///
/// La estructura (UEFI declarado, GPT, ESP+raíz, UKI) es autoridad de dominio.
/// `planStatus` nunca pasa a `certifiable` aquí: hace falta validación de plan
/// (fuera de la generación del manifiesto de catálogo).
pub fn declared_storage_plan_from_intent(
    mode: StorageMode,
    intent_acknowledged: bool,
    loss_risk_acknowledged: Option<bool>,
) -> StoragePlan {
    let incomplete = !intent_acknowledged
        || (mode == StorageMode::ReplaceDisk && loss_risk_acknowledged != Some(true));
    let action = match mode {
        StorageMode::NewDisk => PlannedAction::Create,
        StorageMode::ReplaceDisk => PlannedAction::Replace,
    };
    StoragePlan {
        mode,
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
                planned_action: action,
            },
            PlannedPartition {
                role: PartitionRole::System,
                size_bytes: SafeByteCount::new(0).expect("0 safe"),
                layers: vec![StorageLayer::Filesystem {
                    fs: FilesystemKind::Ext4,
                    mount_point: "/".to_string(),
                }],
                planned_action: action,
            },
        ],
        boot_artifact_strategy: BootArtifactStrategy::UkiOnEsp,
        intent_acknowledged,
        loss_risk_acknowledged: match mode {
            StorageMode::ReplaceDisk => loss_risk_acknowledged,
            StorageMode::NewDisk => None,
        },
        plan_status: if incomplete {
            PlanCertifiabilityStatus::NotCertifiable
        } else {
            PlanCertifiabilityStatus::VerificationPending
        },
    }
}

/// Política LUKS2 declarativa (sin secretos; no implica cifrado ejecutado).
pub fn encryption_policy_from_intent(encrypt: bool) -> EncryptionPolicy {
    if encrypt {
        EncryptionPolicy {
            kind: EncryptionKind::Luks2,
            unlock: Some(LuksUnlockMethod::PassphraseRequestedAtInstall),
            covers_mount_point: Some("/".to_string()),
            initramfs_note: None,
        }
    } else {
        EncryptionPolicy::none()
    }
}

/// Reconstruye campos v2 de cara: plan desde intención, cifrado/navegadores desde selections.
pub fn project_declared_v2_fields(draft: &mut crate::Draft) {
    if decode_draft_schema_version(&draft.schema_version) != Ok(DraftSchemaVersion::V2) {
        return;
    }
    if let Some(plan) = draft.storage_plan.take() {
        draft.storage_plan = Some(declared_storage_plan_from_intent(
            plan.mode,
            plan.intent_acknowledged,
            plan.loss_risk_acknowledged,
        ));
    }

    let encrypt = draft.selections.iter().find_map(|selection| {
        if selection.step_id.contains("encrypt") || selection.step_id.contains("cipher") {
            match &selection.value {
                crate::SelectionValue::Boolean { value } => Some(*value),
                _ => None,
            }
        } else {
            None
        }
    });
    if let Some(flag) = encrypt {
        draft.encryption_policy = Some(encryption_policy_from_intent(flag));
        draft.inherited_encrypt_intent = Some(flag);
    } else if draft.encryption_policy.is_none() {
        draft.encryption_policy = Some(EncryptionPolicy::none());
    }

    let browser_ids: Vec<String> = draft
        .selections
        .iter()
        .find_map(|selection| match &selection.value {
            crate::SelectionValue::Multiple { option_ids }
                if selection.step_id.contains("apps") || selection.step_id.contains("browser") =>
            {
                let ids: Vec<String> = option_ids
                    .iter()
                    .filter(|id| id.starts_with("archmaker.browser."))
                    .cloned()
                    .collect();
                Some(ids)
            }
            _ => None,
        })
        .unwrap_or_default();
    if !browser_ids.is_empty() || draft.browser_policy.is_some() {
        let default = draft
            .browser_policy
            .as_ref()
            .and_then(|p| p.default_browser_id.clone())
            .filter(|id| browser_ids.iter().any(|b| b == id))
            .or_else(|| {
                if browser_ids
                    .iter()
                    .any(|b| b == DEFAULT_BROWSER_ID_NEW_PROFILE)
                {
                    Some(DEFAULT_BROWSER_ID_NEW_PROFILE.to_string())
                } else {
                    browser_ids.first().cloned()
                }
            });
        draft.browser_policy = Some(BrowserPolicy {
            selected_browser_ids: browser_ids,
            default_browser_id: default,
        });
    }

    match &mut draft.verification {
        Some(v) => {
            v.execution_status = ExecutionStatus::NotExecuted;
            if draft
                .storage_plan
                .as_ref()
                .is_none_or(|p| p.plan_status != PlanCertifiabilityStatus::Certifiable)
            {
                v.plan_validation_status = PlanValidationStatus::NotValidated;
            }
        }
        None => {
            draft.verification = Some(VerificationStatuses::foundational_defaults());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn declared_plan_incomplete_replace_is_not_certifiable() {
        let plan = declared_storage_plan_from_intent(StorageMode::ReplaceDisk, true, Some(false));
        assert_eq!(plan.plan_status, PlanCertifiabilityStatus::NotCertifiable);
        assert_eq!(plan.partitions[0].planned_action, PlannedAction::Replace);
    }

    #[test]
    fn declared_plan_complete_new_disk_is_verification_pending() {
        let plan = declared_storage_plan_from_intent(StorageMode::NewDisk, true, None);
        assert_eq!(
            plan.plan_status,
            PlanCertifiabilityStatus::VerificationPending
        );
        assert_ne!(plan.plan_status, PlanCertifiabilityStatus::Certifiable);
    }

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
            selected_browser_ids: vec![
                "archmaker.browser.firefox".into(),
                "archmaker.browser.zen".into(),
            ],
            default_browser_id: Some("archmaker.browser.zen".into()),
        };
        assert!(ok.validate().is_ok());
        let bad = BrowserPolicy {
            selected_browser_ids: vec!["archmaker.browser.firefox".into()],
            default_browser_id: Some("archmaker.browser.zen".into()),
        };
        assert!(matches!(
            bad.validate(),
            Err(BrowserPolicyError::OrphanDefault)
        ));
        assert!(BrowserPolicy::none().validate().is_ok());
        assert!(BrowserPolicy::new_profile_zen().validate().is_ok());
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
