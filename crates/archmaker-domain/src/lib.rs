//! Fase I3 (T-I3-01/T-I3-02): tipos de dominio minimos.
//!
//! Autoridad semantica: Rust (ADR-0001). Derivado de
//! `contracts/json-schema/{draft,catalog,manifest,artifact,common}.schema.json`
//! y `docs/03-data/domain-model.md`. Solo tipos + serde; sin logica de
//! reglas/resolucion (eso es I5) y sin cmd/hooks/shell/pacstrap.

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub mod catalog_providers;
pub mod schema_v2;

pub use catalog_providers::{
    onlyoffice_ids_equivalent, resolve_onlyoffice_canonical, CatalogProvider, ProviderChannel,
    ProviderStatus, ONLYOFFICE_HISTORICAL_ALIAS_ID,
};
pub use schema_v2::{
    decode_draft_schema_version, manual_reference_storage_plan, BootArtifactStrategy,
    BootCompatibilityStatus, BrowserPolicy, BrowserPolicyError, CapacityVerificationStatus,
    DeviceHandle, DraftSchemaMigrationEntry, DraftSchemaMigrationReport, DraftSchemaVersion,
    EncryptionKind, EncryptionPolicy, ExecutionStatus, FilesystemKind, FirmwareDeclaration,
    FirmwareKind, FirmwareProvenance, HardwareVerificationStatus, LogicalTargetRef,
    LuksUnlockMethod, PartitionRole, PartitionTableKind, PartitionTableObservation,
    PartitionTableObservationStatus, PlanCertifiabilityStatus, PlanValidationStatus, PlannedAction,
    PlannedPartition, SafeByteCount, SafeIntegerError, SchemaVersionError, StorageLayer,
    StorageMode, StoragePlan, VerificationStatuses, DEFAULT_BROWSER_ID_NEW_PROFILE,
    DRAFT_DOMAIN_V1, DRAFT_DOMAIN_V2, DRAFT_SCHEMA_V1, DRAFT_SCHEMA_V2, JS_MAX_SAFE_INTEGER,
    MANIFEST_DOMAIN_LABEL_V1, MANIFEST_DOMAIN_LABEL_V2, ONLYOFFICE_CANONICAL_ID,
};

/// Etiqueta histórica v1 (ADR-0007). Preferir [`MANIFEST_DOMAIN_LABEL_V1`].
pub const MANIFEST_DOMAIN_LABEL: &str = MANIFEST_DOMAIN_LABEL_V1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigestAlgorithm {
    #[serde(rename = "sha256")]
    Sha256,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum DigestKind {
    #[serde(rename = "content")]
    Content,
    #[serde(rename = "artifact")]
    Artifact,
    #[serde(rename = "binary")]
    Binary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Digest {
    pub algorithm: DigestAlgorithm,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub kind: Option<DigestKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ContentKind {
    #[serde(rename = "content")]
    Content,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ContentDigest {
    pub algorithm: DigestAlgorithm,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub kind: Option<ContentKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinaryKind {
    #[serde(rename = "binary")]
    Binary,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BinaryDigest {
    pub algorithm: DigestAlgorithm,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub kind: Option<BinaryKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ArtifactKind {
    #[serde(rename = "artifact")]
    Artifact,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ArtifactDigest {
    pub algorithm: DigestAlgorithm,
    pub value: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub kind: Option<ArtifactKind>,
}

/// ADR-0006 MVP-1: `documentVersion` = `vNext` or `vNext.<positive-int>`.
pub fn is_document_version(value: &str) -> bool {
    if value == "vNext" {
        return true;
    }
    let Some(rest) = value.strip_prefix("vNext.") else {
        return false;
    };
    !rest.is_empty() && rest.bytes().all(|b| b.is_ascii_digit()) && !rest.starts_with('0')
}

/// ADR-0006 MVP-1: positive integer string (`schemaVersion`, `protocolVersion`).
pub fn is_integer_version(value: &str) -> bool {
    !value.is_empty() && value.bytes().all(|b| b.is_ascii_digit()) && !value.starts_with('0')
}

/// ADR-0006 MVP-1: SemVer 2.0 core (major.minor.patch, optional pre/build).
pub fn is_semver(value: &str) -> bool {
    let (core, rest) = match value.split_once(['-', '+']) {
        Some((c, r)) => (c, Some(r)),
        None => (value, None),
    };
    let mut parts = core.split('.');
    let (Some(maj), Some(min), Some(pat), None) =
        (parts.next(), parts.next(), parts.next(), parts.next())
    else {
        return false;
    };
    let ok_num = |s: &str| {
        !s.is_empty() && s.bytes().all(|b| b.is_ascii_digit()) && (s == "0" || !s.starts_with('0'))
    };
    if !(ok_num(maj) && ok_num(min) && ok_num(pat)) {
        return false;
    }
    if let Some(r) = rest {
        if r.is_empty() {
            return false;
        }
    }
    true
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CatalogProvenance {
    pub source_id: String,
    pub path: String,
    pub captured_at: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CatalogRef {
    pub namespace: String,
    pub id: String,
    pub version: String,
    pub digest: ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TargetRef {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PresetRef {
    pub namespace: String,
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Producer {
    pub id: String,
    pub version: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CapabilityRef {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SelectionOrigin {
    Manual,
    Derived,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Conflict {
    pub severity: Severity,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "capabilityId"
    )]
    pub capability_id: Option<String>,
    pub path: String,
    #[serde(rename = "messageKey")]
    pub message_key: String,
    pub suggestions: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "kind", deny_unknown_fields)]
pub enum SelectionValue {
    #[serde(rename = "single")]
    Single {
        #[serde(rename = "optionId")]
        option_id: String,
    },
    #[serde(rename = "multiple")]
    Multiple {
        #[serde(rename = "optionIds")]
        option_ids: Vec<String>,
    },
    #[serde(rename = "boolean")]
    Boolean { value: bool },
    #[serde(rename = "number")]
    Number {
        value: f64,
        #[serde(skip_serializing_if = "Option::is_none", default)]
        unit: Option<String>,
    },
    #[serde(rename = "text")]
    Text { value: String },
    #[serde(rename = "secretRef")]
    SecretRef {
        #[serde(rename = "ref")]
        reference: String,
    },
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Selection {
    #[serde(rename = "stepId")]
    pub step_id: String,
    pub value: SelectionValue,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResolvedSelection {
    #[serde(rename = "stepId")]
    pub step_id: String,
    #[serde(rename = "optionId")]
    pub option_id: String,
    pub value: SelectionValue,
    pub origin: SelectionOrigin,
    pub locked: bool,
}

/// Metadatos legados de cara (no son selecciones de catálogo ni hechos de hardware).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LegacyMetadata {
    /// Intención de cara preservada aunque el paso UI desaparezca (`archmaker.face.intent`).
    #[serde(skip_serializing_if = "Option::is_none", default, rename = "faceIntent")]
    pub face_intent: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Draft {
    pub id: String,
    #[serde(rename = "documentVersion")]
    pub document_version: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    pub revision: u64,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "contentDigest"
    )]
    pub content_digest: Option<ContentDigest>,
    #[serde(rename = "catalogRef")]
    pub catalog_ref: CatalogRef,
    #[serde(rename = "targetRef")]
    pub target_ref: TargetRef,
    pub selections: Vec<Selection>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "presetRefs"
    )]
    pub preset_refs: Option<Vec<PresetRef>>,
    #[serde(skip_serializing_if = "Option::is_none", default, rename = "createdAt")]
    pub created_at: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default, rename = "updatedAt")]
    pub updated_at: Option<String>,
    /// Plan de disco declarativo (`schemaVersion` `"2"`). Ausente en v1.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "storagePlan"
    )]
    pub storage_plan: Option<StoragePlan>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "encryptionPolicy"
    )]
    pub encryption_policy: Option<EncryptionPolicy>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "browserPolicy"
    )]
    pub browser_policy: Option<BrowserPolicy>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub verification: Option<VerificationStatuses>,
    /// Intención histórica de cifrado booleano v1; pendiente de confirmar como LUKS2.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "inheritedEncryptIntent"
    )]
    pub inherited_encrypt_intent: Option<bool>,
    /// Residuos de cara (p. ej. face.intent) preservados fuera del catálogo.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "legacyMetadata"
    )]
    pub legacy_metadata: Option<LegacyMetadata>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum StepMode {
    Grouped,
    Flat,
    Single,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Cardinality {
    One,
    Many,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CatalogOption {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub fixed: Option<bool>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "essentialReason"
    )]
    pub essential_reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub tags: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub provides: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub requires: Option<Vec<String>>,
    /// Proveedor Arch/AUR verificado o solicitud no verificada (fuente + revisión + fecha).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub provider: Option<CatalogProvider>,
    /// Familia de navegador (p. ej. Firefox/Mozilla, Chromium). No confundir con linux-zen.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "browserBase"
    )]
    pub browser_base: Option<String>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "renderingEngine"
    )]
    pub rendering_engine: Option<String>,
    /// Si esta opción es un alias histórico, apunta al id canónico.
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "canonicalId"
    )]
    pub canonical_id: Option<String>,
    /// Aliases históricos explícitos del id canónico.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub aliases: Option<Vec<String>>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Section {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    pub cardinality: Cardinality,
    pub options: Vec<CatalogOption>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Step {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    pub mode: StepMode,
    pub sections: Vec<Section>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Libpack {
    pub id: String,
    pub title: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
    pub essential: Vec<CatalogOption>,
    pub recommended: Vec<CatalogOption>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Capability {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub description: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RuleRef {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub digest: Option<ContentDigest>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Catalog {
    pub namespace: String,
    pub id: String,
    pub version: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub revision: Option<u64>,
    #[serde(rename = "documentVersion")]
    pub document_version: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "contentDigest"
    )]
    pub content_digest: Option<ContentDigest>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub provenance: Option<CatalogProvenance>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub metadata: Option<HashMap<String, String>>,
    pub steps: Vec<Step>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub libpacks: Option<Vec<Libpack>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub capabilities: Option<Vec<Capability>>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub rules: Option<Vec<RuleRef>>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ManifestDomainLabel {
    #[default]
    V1,
    V2,
}

impl ManifestDomainLabel {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::V1 => MANIFEST_DOMAIN_LABEL_V1,
            Self::V2 => MANIFEST_DOMAIN_LABEL_V2,
        }
    }

    pub fn for_schema_version(schema_version: &str) -> Result<Self, SchemaVersionError> {
        match decode_draft_schema_version(schema_version)? {
            DraftSchemaVersion::V1 => Ok(Self::V1),
            DraftSchemaVersion::V2 => Ok(Self::V2),
        }
    }
}

impl Serialize for ManifestDomainLabel {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(self.as_str())
    }
}

impl<'de> Deserialize<'de> for ManifestDomainLabel {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            MANIFEST_DOMAIN_LABEL_V1 => Ok(Self::V1),
            MANIFEST_DOMAIN_LABEL_V2 => Ok(Self::V2),
            other => Err(serde::de::Error::custom(format!(
                "invalid domainLabel: {other}"
            ))),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Manifest {
    #[serde(rename = "documentVersion")]
    pub document_version: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "domainLabel")]
    pub domain_label: ManifestDomainLabel,
    #[serde(rename = "catalogRef")]
    pub catalog_ref: CatalogRef,
    #[serde(rename = "targetRef")]
    pub target_ref: TargetRef,
    pub producer: Producer,
    #[serde(rename = "resolutionDigest")]
    pub resolution_digest: ContentDigest,
    #[serde(rename = "effectiveSelections")]
    pub effective_selections: Vec<ResolvedSelection>,
    #[serde(rename = "providedCapabilities")]
    pub provided_capabilities: Vec<CapabilityRef>,
    #[serde(rename = "requiredCapabilities")]
    pub required_capabilities: Vec<CapabilityRef>,
    pub conflicts: Vec<Conflict>,
    #[serde(rename = "contentDigest")]
    pub content_digest: ContentDigest,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "storagePlan"
    )]
    pub storage_plan: Option<StoragePlan>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "encryptionPolicy"
    )]
    pub encryption_policy: Option<EncryptionPolicy>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "browserPolicy"
    )]
    pub browser_policy: Option<BrowserPolicy>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub verification: Option<VerificationStatuses>,
}

/// Constante de paso de cara (no es entidad de catálogo).
pub const FACE_INTENT_STEP_ID: &str = "archmaker.face.intent";

/// Vista pública de export: plan + verificación + metadatos legados.
///
/// No incluye snapshot de hardware ni identidades privadas (serial/WWN/path).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct PublicExportBundle {
    pub schema_version: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub storage_plan: Option<StoragePlan>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub encryption_policy: Option<EncryptionPolicy>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub browser_policy: Option<BrowserPolicy>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub verification: Option<VerificationStatuses>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub legacy_metadata: Option<LegacyMetadata>,
    #[serde(
        skip_serializing_if = "Option::is_none",
        default,
        rename = "inheritedEncryptIntent"
    )]
    pub inherited_encrypt_intent: Option<bool>,
}

/// Proyecta el draft a un paquete exportable sin snapshot privado.
pub fn public_export_bundle(draft: &Draft) -> PublicExportBundle {
    PublicExportBundle {
        schema_version: draft.schema_version.clone(),
        storage_plan: draft.storage_plan.clone(),
        encryption_policy: draft.encryption_policy.clone(),
        browser_policy: draft.browser_policy.clone(),
        verification: draft.verification.clone(),
        legacy_metadata: draft.legacy_metadata.clone(),
        inherited_encrypt_intent: draft.inherited_encrypt_intent,
    }
}

/// Migración idempotente draft `schemaVersion` `"1"` → `"2"`.
///
/// No inventa hardware, no sustituye apps (Firefox/LibreOffice/Flameshot) ni convierte
/// `encrypt: true` en política LUKS2: esa intención queda en `inheritedEncryptIntent`.
/// `archmaker.face.intent` se copia a `legacyMetadata.faceIntent` (el paso UI puede desaparecer).
pub fn migrate_draft_v1_to_v2(
    draft: Draft,
) -> Result<(Draft, DraftSchemaMigrationReport), SchemaVersionError> {
    let version = decode_draft_schema_version(&draft.schema_version)?;
    match version {
        DraftSchemaVersion::V2 => {
            let report = DraftSchemaMigrationReport {
                from_schema_version: DRAFT_SCHEMA_V2.to_string(),
                to_schema_version: DRAFT_SCHEMA_V2.to_string(),
                entries: vec![DraftSchemaMigrationEntry {
                    path: "/schemaVersion".to_string(),
                    message_key: "migration.draftSchema.alreadyV2".to_string(),
                    detail: None,
                }],
                idempotent_noop: true,
            };
            Ok((draft, report))
        }
        DraftSchemaVersion::V1 => {
            let mut entries = vec![DraftSchemaMigrationEntry {
                path: "/schemaVersion".to_string(),
                message_key: "migration.draftSchema.upgradedToV2".to_string(),
                detail: Some("schemaVersion 1 → 2".to_string()),
            }];

            let inherited_encrypt = draft.selections.iter().find_map(|selection| {
                if selection.step_id.contains("encrypt") || selection.step_id.contains("cipher") {
                    match &selection.value {
                        SelectionValue::Boolean { value } => Some(*value),
                        _ => None,
                    }
                } else {
                    None
                }
            });
            if let Some(flag) = inherited_encrypt {
                entries.push(DraftSchemaMigrationEntry {
                    path: "/inheritedEncryptIntent".to_string(),
                    message_key: "migration.draftSchema.encryptIntentPreserved".to_string(),
                    detail: Some(format!(
                        "booleano histórico {flag}; no se materializa como encryptionPolicy LUKS2"
                    )),
                });
            }

            entries.push(DraftSchemaMigrationEntry {
                path: "/storagePlan".to_string(),
                message_key: "migration.draftSchema.storageAbsent".to_string(),
                detail: Some(
                    "sin firmware/disco/capacidad inventados; storagePlan queda ausente hasta declaración manual"
                        .to_string(),
                ),
            });
            entries.push(DraftSchemaMigrationEntry {
                path: "/browserPolicy".to_string(),
                message_key: "migration.draftSchema.appsPreserved".to_string(),
                detail: Some("no se reescribe Firefox→Zen ni LibreOffice→ONLYOFFICE".to_string()),
            });

            let mut migrated = draft;
            migrated.schema_version = DRAFT_SCHEMA_V2.to_string();
            migrated.content_digest = None;
            migrated.storage_plan = None;
            migrated.encryption_policy = Some(EncryptionPolicy::none());
            migrated.browser_policy = None;
            migrated.verification = Some(VerificationStatuses::foundational_defaults());
            migrated.inherited_encrypt_intent = inherited_encrypt;

            let face_intent = migrated.selections.iter().find_map(|selection| {
                if selection.step_id == FACE_INTENT_STEP_ID {
                    match &selection.value {
                        SelectionValue::Single { option_id } => Some(option_id.clone()),
                        _ => None,
                    }
                } else {
                    None
                }
            });
            if let Some(intent_id) = face_intent {
                entries.push(DraftSchemaMigrationEntry {
                    path: "/legacyMetadata/faceIntent".to_string(),
                    message_key: "migration.draftSchema.faceIntentPreserved".to_string(),
                    detail: Some(format!(
                        "archmaker.face.intent={intent_id}; se conserva en legacyMetadata aunque el paso UI desaparezca"
                    )),
                });
                migrated.legacy_metadata = Some(LegacyMetadata {
                    face_intent: Some(intent_id),
                });
            }

            let report = DraftSchemaMigrationReport {
                from_schema_version: DRAFT_SCHEMA_V1.to_string(),
                to_schema_version: DRAFT_SCHEMA_V2.to_string(),
                entries,
                idempotent_noop: false,
            };
            Ok((migrated, report))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ArtifactRef {
    #[serde(rename = "binaryDigest")]
    pub binary_digest: BinaryDigest,
    #[serde(rename = "mediaType")]
    pub media_type: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ManifestRef {
    #[serde(rename = "contentDigest")]
    pub content_digest: ContentDigest,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Artifact {
    #[serde(rename = "documentVersion")]
    pub document_version: String,
    #[serde(rename = "schemaVersion")]
    pub schema_version: String,
    #[serde(rename = "artifactRef")]
    pub artifact_ref: ArtifactRef,
    #[serde(rename = "targetRef")]
    pub target_ref: TargetRef,
    pub producer: Producer,
    #[serde(rename = "mediaType")]
    pub media_type: String,
    #[serde(rename = "binaryDigest")]
    pub binary_digest: BinaryDigest,
    #[serde(rename = "manifestRef")]
    pub manifest_ref: ManifestRef,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub experimental: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub content: Option<serde_json::Value>,
    /// Exact renderer bytes. SHA-256 of these equals `binary_digest`.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub bytes: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft_fixture() -> &'static str {
        include_str!("../../../contracts/json-schema/examples/01-draft.valid.json")
    }

    fn manifest_fixture() -> &'static str {
        include_str!("../../../contracts/json-schema/examples/05-manifest.valid.json")
    }

    #[test]
    fn draft_round_trip() {
        let draft: Draft =
            serde_json::from_str(draft_fixture()).expect("draft fixture deserializes");
        assert_eq!(draft.selections.len(), 6);
        assert!(draft.preset_refs.as_ref().is_some_and(|v| !v.is_empty()));
        let value = serde_json::to_value(&draft).expect("draft serializes");
        let back: Draft = serde_json::from_value(value.clone()).expect("draft re-deserializes");
        assert_eq!(draft, back);
        let canonical = serde_json::to_string(&back).expect("draft to string");
        let again: Draft = serde_json::from_str(&canonical).expect("draft from string");
        assert_eq!(back, again);
    }

    #[test]
    fn manifest_round_trip() {
        let manifest: Manifest =
            serde_json::from_str(manifest_fixture()).expect("manifest fixture deserializes");
        assert_eq!(manifest.effective_selections.len(), 2);
        assert!(manifest.conflicts.is_empty());
        let value = serde_json::to_value(&manifest).expect("manifest serializes");
        assert_eq!(value["domainLabel"], MANIFEST_DOMAIN_LABEL);
        let back: Manifest =
            serde_json::from_value(value.clone()).expect("manifest re-deserializes");
        assert_eq!(manifest, back);
    }

    #[test]
    fn rejects_unknown_fields() {
        let mut value: serde_json::Value = serde_json::from_str(draft_fixture()).unwrap();
        value["unexpectedField"] = serde_json::json!("nope");
        assert!(serde_json::from_value::<Draft>(value).is_err());

        let mut manifest: serde_json::Value = serde_json::from_str(manifest_fixture()).unwrap();
        manifest["unexpectedField"] = serde_json::json!(1);
        assert!(serde_json::from_value::<Manifest>(manifest).is_err());
    }

    #[test]
    fn secret_ref_carries_only_reference() {
        let draft: Draft = serde_json::from_str(draft_fixture()).unwrap();
        let secret = draft
            .selections
            .iter()
            .find_map(|selection| match &selection.value {
                SelectionValue::SecretRef { reference } => Some(reference.clone()),
                _ => None,
            })
            .expect("draft fixture contains a secretRef selection");
        assert_eq!(secret, "secret://user/registry-token");
        let value = serde_json::to_value(&draft).unwrap();
        let secret_json = value["selections"]
            .as_array()
            .unwrap()
            .iter()
            .find(|entry| entry["value"]["kind"] == "secretRef")
            .unwrap()["value"]
            .clone();
        assert_eq!(secret_json["ref"], "secret://user/registry-token");
        assert!(secret_json.get("value").is_none());
        assert!(secret_json.get("secret").is_none());
    }

    #[test]
    fn manifest_domain_label_accepts_v1_and_v2() {
        let mut value: serde_json::Value = serde_json::from_str(manifest_fixture()).unwrap();
        value["domainLabel"] = serde_json::json!("archmaker:other:v9");
        assert!(serde_json::from_value::<Manifest>(value.clone()).is_err());
        value["domainLabel"] = serde_json::json!(MANIFEST_DOMAIN_LABEL_V2);
        // Fixture still has schemaVersion "1"; deserialization of label alone is allowed.
        assert!(
            serde_json::from_value::<ManifestDomainLabel>(serde_json::json!(
                MANIFEST_DOMAIN_LABEL_V2
            ))
            .is_ok()
        );
    }

    #[test]
    fn migrate_v1_to_v2_is_idempotent() {
        let draft: Draft =
            serde_json::from_str(draft_fixture()).expect("draft fixture deserializes");
        let (v2, report) = migrate_draft_v1_to_v2(draft).expect("migrate");
        assert_eq!(v2.schema_version, DRAFT_SCHEMA_V2);
        assert!(!report.idempotent_noop);
        assert!(v2.storage_plan.is_none());
        assert_eq!(
            v2.verification.as_ref().map(|v| v.execution_status),
            Some(ExecutionStatus::NotExecuted)
        );
        let (again, report2) = migrate_draft_v1_to_v2(v2.clone()).expect("idempotent");
        assert!(report2.idempotent_noop);
        assert_eq!(again.schema_version, DRAFT_SCHEMA_V2);
        assert_eq!(again.selections, v2.selections);
        assert_eq!(again.legacy_metadata, v2.legacy_metadata);
    }

    #[test]
    fn migrate_preserves_face_intent_in_legacy_metadata() {
        let mut draft: Draft =
            serde_json::from_str(draft_fixture()).expect("draft fixture deserializes");
        draft.selections.push(Selection {
            step_id: FACE_INTENT_STEP_ID.into(),
            value: SelectionValue::Single {
                option_id: "archmaker.intent.desarrollo".into(),
            },
        });
        let (v2, report) = migrate_draft_v1_to_v2(draft).expect("migrate");
        assert_eq!(
            v2.legacy_metadata
                .as_ref()
                .and_then(|m| m.face_intent.as_deref()),
            Some("archmaker.intent.desarrollo")
        );
        assert!(report.entries.iter().any(|e| e.path == "/legacyMetadata/faceIntent"));
        let (again, report2) = migrate_draft_v1_to_v2(v2.clone()).expect("idempotent");
        assert!(report2.idempotent_noop);
        assert_eq!(again.legacy_metadata, v2.legacy_metadata);
    }

    #[test]
    fn public_export_includes_plan_and_verification_not_private_snapshot() {
        let draft: Draft =
            serde_json::from_str(draft_fixture()).expect("draft fixture deserializes");
        let (mut v2, _) = migrate_draft_v1_to_v2(draft).expect("migrate");
        v2.storage_plan = Some(manual_reference_storage_plan());
        v2.legacy_metadata = Some(LegacyMetadata {
            face_intent: Some("archmaker.intent.desarrollo".into()),
        });
        let bundle = public_export_bundle(&v2);
        let json = serde_json::to_string(&bundle).expect("json");
        assert!(json.contains("storagePlan"));
        assert!(json.contains("verification"));
        assert!(json.contains("legacyMetadata"));
        assert!(json.contains("faceIntent"));
        // Privacidad: el export público de perfil no arrastra snapshot ni identidad de disco.
        assert!(!json.contains("snapshotId"));
        assert!(!json.contains("\"serial\""));
        assert!(!json.contains("\"wwn\""));
        assert!(!json.to_lowercase().contains("password"));
        let again = public_export_bundle(&v2);
        assert_eq!(
            serde_json::to_string(&bundle).unwrap(),
            serde_json::to_string(&again).unwrap(),
            "export público idempotente"
        );
    }

    #[test]
    fn version_axis_formats() {
        assert!(is_document_version("vNext"));
        assert!(is_document_version("vNext.1"));
        assert!(!is_document_version("1.0.0"));
        assert!(is_integer_version("1"));
        assert!(!is_integer_version("01"));
        assert!(!is_integer_version("v1"));
        assert!(is_semver("1.0.0"));
        assert!(is_semver("2026.10.0"));
        assert!(!is_semver("2026.10"));
    }
}
