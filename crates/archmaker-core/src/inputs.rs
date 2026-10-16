use std::path::PathBuf;

use archmaker_domain::{
    CapabilityRef, CatalogProvenance, CatalogRef, Conflict, ContentDigest, Draft, Manifest,
    PresetRef, ResolvedSelection, SelectionValue, TargetRef,
};
use archmaker_rules::Diagnostic;
use serde::{Deserialize, Deserializer, Serialize};

use crate::limits::Limits;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct CreateDraftInput {
    pub draft_id: Option<String>,
    pub catalog_ref: CatalogRef,
    pub target_ref: TargetRef,
    pub preset_ref: Option<PresetRef>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CatalogSource {
    Embedded,
    File,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct LoadCatalogInput {
    pub source: CatalogSource,
    pub catalog_ref: Option<CatalogRef>,
    pub expected_digest: Option<ContentDigest>,
    pub file_path: Option<PathBuf>,
    /// Required for `CatalogSource::File` (MVP-1 packaged provenance). Optional for Embedded.
    pub expected_provenance: Option<CatalogProvenance>,
    pub limits: Option<Limits>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeSetOp {
    Add,
    Remove,
    Replace,
    Move,
    Copy,
    Test,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChangeSetOrigin {
    Compare,
    Preset,
    Migration,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChangeSetOperation {
    pub op_id: String,
    pub op: ChangeSetOp,
    pub path: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub selector: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub from: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub before: Option<SelectionValue>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub after: Option<SelectionValue>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub note: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChangeSetSummary {
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub added: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub removed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub changed: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub total: Option<u64>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ChangeSet {
    pub schema_version: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub base_revision: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub target_draft_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub origin: Option<ChangeSetOrigin>,
    pub operations: Vec<ChangeSetOperation>,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub summary: Option<ChangeSetSummary>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct SaveDraftInput {
    pub draft: Draft,
    pub expected_revision: u64,
    /// Optional discrete changeset applied under the revision precondition (MVP-1).
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub changeset: Option<ChangeSet>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct OpenDraftInput {
    pub draft_id: String,
    /// When the store has no copy (browser session), adapters may pass the
    /// previously persisted draft to rehydrate memory without bumping revision.
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub seed: Option<Draft>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImportDraftInput {
    /// Exact bytes of the legacy document. Never written back.
    pub source_bytes: Vec<u8>,
    pub catalog_ref: CatalogRef,
    pub target_ref: TargetRef,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub draft_id: Option<String>,
    /// When true, blocking losses may still produce a draft (FR-IMPORT-001 confirmation).
    pub confirm_blocking_losses: bool,
    pub request_id: Option<String>,
}

pub use crate::migration::{ImportDraftResult, MigrationEntry, MigrationFate, MigrationReport};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResolveDraftInput {
    pub draft: Draft,
    pub catalog_ref: CatalogRef,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ValidateDraftInput {
    pub draft: Draft,
    pub catalog_ref: CatalogRef,
    pub target_ref: Option<TargetRef>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct BuildManifestInput {
    pub draft: Draft,
    pub catalog_ref: CatalogRef,
    pub target_ref: TargetRef,
    pub expected_resolution_digest: Option<ContentDigest>,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ExportArtifactInput {
    pub manifest: Manifest,
    pub target_ref: TargetRef,
    pub destination: DestinationHandle,
    pub request_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DestinationHandle {
    token: String,
}

impl<'de> Deserialize<'de> for DestinationHandle {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        struct Raw {
            token: String,
        }
        let raw = Raw::deserialize(deserializer)?;
        DestinationHandle::new(raw.token).map_err(serde::de::Error::custom)
    }
}

impl DestinationHandle {
    pub fn new(token: String) -> Result<Self, String> {
        if token.is_empty() || token.len() > 128 {
            return Err("destination token must be 1..=128 chars".to_string());
        }
        if token.as_bytes().contains(&0) {
            return Err("destination token must not contain NUL".to_string());
        }
        if token == "." || token == ".." {
            return Err("destination token must not be . or ..".to_string());
        }
        if !token
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-' || c == '_')
        {
            return Err("destination token must be [A-Za-z0-9._-]".to_string());
        }
        if token.starts_with('.') {
            return Err("destination token must not be hidden".to_string());
        }
        Ok(Self { token })
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ResolveResult {
    pub resolution_digest: ContentDigest,
    pub effective_selections: Vec<ResolvedSelection>,
    pub provided_capabilities: Vec<CapabilityRef>,
    pub required_capabilities: Vec<CapabilityRef>,
    pub conflicts: Vec<Conflict>,
    pub diagnostics: Vec<Diagnostic>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ValidationResult {
    pub diagnostics: Vec<Diagnostic>,
    pub blocking: bool,
    pub pipeline_version: String,
}

pub fn sort_diagnostics(diagnostics: &mut [Diagnostic]) {
    diagnostics.sort_by(|a, b| (&a.path, &a.code, &a.source).cmp(&(&b.path, &b.code, &b.source)));
}
