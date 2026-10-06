//! Fase I3 (T-I3-01/T-I3-02): tipos de dominio minimos.
//!
//! Autoridad semantica: Rust (ADR-0001). Derivado de
//! `contracts/json-schema/{draft,catalog,manifest,artifact,common}.schema.json`
//! y `docs/03-data/domain-model.md`. Solo tipos + serde; sin logica de
//! reglas/resolucion (eso es I5) y sin cmd/hooks/shell/pacstrap.

use std::collections::HashMap;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

pub const MANIFEST_DOMAIN_LABEL: &str = "archmaker:manifest:v1";

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
pub struct ManifestDomainLabel;

impl Serialize for ManifestDomainLabel {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(MANIFEST_DOMAIN_LABEL)
    }
}

impl<'de> Deserialize<'de> for ManifestDomainLabel {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        if value == MANIFEST_DOMAIN_LABEL {
            Ok(ManifestDomainLabel)
        } else {
            Err(serde::de::Error::custom(format!(
                "invalid domainLabel: expected {MANIFEST_DOMAIN_LABEL}"
            )))
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
    fn manifest_domain_label_is_const() {
        let mut value: serde_json::Value = serde_json::from_str(manifest_fixture()).unwrap();
        value["domainLabel"] = serde_json::json!("archmaker:other:v9");
        assert!(serde_json::from_value::<Manifest>(value).is_err());
    }
}
