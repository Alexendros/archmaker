use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
#[cfg(not(target_family = "wasm"))]
use std::fs::File;
#[cfg(not(target_family = "wasm"))]
use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
#[cfg(not(target_family = "wasm"))]
use std::time::{SystemTime, UNIX_EPOCH};

use archmaker_domain::{
    decode_draft_schema_version, is_document_version, is_integer_version, is_semver, Artifact,
    ArtifactRef, BinaryDigest, BinaryKind, CapabilityRef, Catalog, CatalogRef, ContentDigest,
    ContentKind, DigestAlgorithm, Draft, DraftSchemaVersion, Manifest, ManifestDomainLabel,
    ManifestRef, PresetRef, Producer, ResolvedSelection, Selection, SelectionOrigin,
    SelectionValue, TargetRef, DRAFT_DOMAIN_V1, DRAFT_DOMAIN_V2,
};
use archmaker_resolution::{Catalog as ResolutionCatalog, CatalogOption};
use archmaker_rules::{
    load_rules_by_ids, Diagnostic, Draft as RulesDraft, Rule, CANONICAL_RULES_PACK_JSON,
};

use crate::error::{CoreError, Result};
use crate::inputs::{
    sort_diagnostics, BuildManifestInput, CatalogSource, ChangeSet, ChangeSetOp, CreateDraftInput,
    ExportArtifactInput, ImportDraftInput, ImportDraftResult, LoadCatalogInput, OpenDraftInput,
    ResolveDraftInput, ResolveResult, SaveDraftInput, ValidateDraftInput, ValidationResult,
};
use crate::io::atomic_write_bytes;
use crate::limits::DEFAULT_LIMITS;
use crate::migration::migrate_v51_to_draft;
use crate::port::{CoreOp, CorePort};

fn check_json_depth(bytes: &[u8], max_depth: u32) -> Result<()> {
    let mut depth: u32 = 0;
    let mut in_string = false;
    let mut escape = false;
    for &b in bytes {
        if escape {
            escape = false;
            continue;
        }
        if in_string {
            if b == b'\\' {
                escape = true;
            } else if b == b'"' {
                in_string = false;
            }
            continue;
        }
        match b {
            b'{' | b'[' => {
                depth += 1;
                if depth > max_depth {
                    return Err(CoreError::doc_limit(CoreOp::LoadCatalog.source()));
                }
            }
            b'}' | b']' => {
                if depth == 0 {
                    return Err(CoreError::doc_limit(CoreOp::LoadCatalog.source()));
                }
                depth -= 1;
            }
            b'"' => in_string = true,
            _ => {}
        }
    }
    Ok(())
}

pub const EMBEDDED_CATALOG_JSON: &str = include_str!("../fixtures/embedded-catalog.json");
/// Product catalog (matrix / Forge face ids). Used by Tauri and WASM session cores.
pub const DESKTOP_CATALOG_JSON: &str = include_str!("../fixtures/local-catalog-mvp1.json");

const CATALOG_DOMAIN: &str = "archmaker:catalog:v1";
const RESOLUTION_DOMAIN: &str = "archmaker:resolution:v1";

fn draft_domain_for(schema_version: &str) -> Result<&'static str> {
    match decode_draft_schema_version(schema_version)
        .map_err(|_| CoreError::schema_invalid(CoreOp::SaveDraft.source()))?
    {
        DraftSchemaVersion::V1 => Ok(DRAFT_DOMAIN_V1),
        DraftSchemaVersion::V2 => Ok(DRAFT_DOMAIN_V2),
    }
}

const DRAFT_TARGETS: [&str; 1] = ["arch-x86_64"];
const EXPORT_TARGETS: [&str; 3] = ["archmaker-profile", "archinstall-profile", "report"];

const PRODUCER_ID: &str = "archmaker-core";
const PRODUCER_VERSION: &str = "0.1.0";
pub const PIPELINE_VERSION: &str = "0.1.0";

static UUID_COUNTER: AtomicU64 = AtomicU64::new(0);

#[cfg(test)]
thread_local! {
    static FORCE_UUID_ENTROPY_FAILURE: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

/// Test hook: fuerza fallo cerrado de entropía en `new_uuid_v4` (sin panic).
#[cfg(test)]
pub fn test_force_uuid_entropy_failure(fail: bool) {
    FORCE_UUID_ENTROPY_FAILURE.with(|flag| flag.set(fail));
}

pub struct RealCore {
    pub catalog_embedded: Catalog,
    /// Reglas activas cargadas desde fichas del catálogo (nunca inventadas en silencio).
    pub active_rules: Vec<Rule>,
    store_dir: Option<PathBuf>,
    drafts: RefCell<HashMap<String, Draft>>,
}

fn rules_from_catalog(catalog: &Catalog) -> Result<Vec<Rule>> {
    let source = CoreOp::LoadCatalog.source();
    let Some(refs) = catalog.rules.as_ref() else {
        return Ok(Vec::new());
    };
    if refs.is_empty() {
        return Ok(Vec::new());
    }
    let ids: Vec<&str> = refs.iter().map(|rule_ref| rule_ref.id.as_str()).collect();
    load_rules_by_ids(CANONICAL_RULES_PACK_JSON, &ids)
        .map_err(|_| CoreError::schema_invalid(source))
}

impl RealCore {
    pub fn new(catalog_embedded: Catalog) -> Self {
        let active_rules = rules_from_catalog(&catalog_embedded).unwrap_or_default();
        Self {
            catalog_embedded,
            active_rules,
            store_dir: None,
            drafts: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_embedded() -> Result<Self> {
        Self::from_catalog_json(EMBEDDED_CATALOG_JSON)
    }

    /// Product runtime catalog (`local-catalog-mvp1.json`).
    pub fn with_desktop() -> Result<Self> {
        Self::from_catalog_json(DESKTOP_CATALOG_JSON)
    }

    fn from_catalog_json(json: &str) -> Result<Self> {
        let source = CoreOp::LoadCatalog.source();
        let value: serde_json::Value =
            serde_json::from_str(json).map_err(|_| CoreError::schema_invalid(source))?;
        let catalog: Catalog =
            serde_json::from_value(value).map_err(|_| CoreError::schema_invalid(source))?;
        let active_rules = rules_from_catalog(&catalog)?;
        Ok(Self {
            catalog_embedded: catalog,
            active_rules,
            store_dir: None,
            drafts: RefCell::new(HashMap::new()),
        })
    }

    pub fn with_store_dir(mut self, dir: PathBuf) -> Self {
        self.store_dir = Some(dir);
        self
    }

    /// CatalogRef for the active embedded/product catalog, with content digest.
    pub fn active_catalog_ref(&self) -> Result<CatalogRef> {
        let c = &self.catalog_embedded;
        Ok(CatalogRef {
            namespace: c.namespace.clone(),
            id: c.id.clone(),
            version: c.version.clone(),
            digest: catalog_content_digest(c)?,
        })
    }

    pub fn catalog(&self) -> &Catalog {
        &self.catalog_embedded
    }

    pub fn stored_draft(&self, id: &str) -> Option<Draft> {
        self.drafts.borrow().get(id).cloned()
    }

    fn ensure_catalog_match(&self, catalog_ref: &CatalogRef, source: &str) -> Result<()> {
        let embedded = &self.catalog_embedded;
        if catalog_ref.namespace != embedded.namespace
            || catalog_ref.id != embedded.id
            || catalog_ref.version != embedded.version
        {
            return Err(CoreError::cat_not_found(source));
        }
        if catalog_ref.digest.algorithm != DigestAlgorithm::Sha256 {
            return Err(CoreError::cat_digest_mismatch(source));
        }
        let actual = catalog_content_digest(embedded)?;
        if catalog_ref.digest.value.to_lowercase() != actual.value {
            return Err(CoreError::cat_digest_mismatch(source));
        }
        Ok(())
    }
}

pub fn is_uuid(value: &str) -> bool {
    let bytes = value.as_bytes();
    if bytes.len() != 36 {
        return false;
    }
    for (index, byte) in bytes.iter().enumerate() {
        if index == 8 || index == 13 || index == 18 || index == 23 {
            if *byte != b'-' {
                return false;
            }
        } else if !byte.is_ascii_hexdigit() {
            return false;
        }
    }
    true
}

#[cfg(not(target_family = "wasm"))]
fn read_urandom(buffer: &mut [u8]) -> std::io::Result<()> {
    File::open("/dev/urandom")?.read_exact(buffer)
}

/// Entropy usable on wasm32-unknown-unknown (no `SystemTime`, no `/dev/urandom`).
fn counter_entropy_seed() -> u128 {
    (UUID_COUNTER.load(Ordering::Relaxed) as u128).wrapping_mul(0x9e3779b97f4a7c15)
}

fn fill_with_counter_entropy(buffer: &mut [u8]) {
    let mut state = counter_entropy_seed()
        .wrapping_add((UUID_COUNTER.fetch_add(1, Ordering::Relaxed) as u128) << 64)
        | 0x9e3779b97f4a7c15;
    for chunk in buffer.chunks_mut(8) {
        state ^= state >> 12;
        state = state.wrapping_mul(0x2545f4914f6cdd1d);
        state ^= state >> 41;
        let bytes = state.to_le_bytes();
        let take = chunk.len().min(8);
        chunk.copy_from_slice(&bytes[..take]);
    }
}

#[cfg(not(target_family = "wasm"))]
fn fallback_random(buffer: &mut [u8]) {
    // Native-only clock mix. wasm32 must never call SystemTime (panics).
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_else(|_| counter_entropy_seed());
    let mut state = nanos
        .wrapping_add((UUID_COUNTER.fetch_add(1, Ordering::Relaxed) as u128) << 64)
        | 0x9e3779b97f4a7c15;
    for chunk in buffer.chunks_mut(8) {
        state ^= state >> 12;
        state = state.wrapping_mul(0x2545f4914f6cdd1d);
        state ^= state >> 41;
        let bytes = state.to_le_bytes();
        let take = chunk.len().min(8);
        chunk.copy_from_slice(&bytes[..take]);
    }
}

fn fill_production_entropy(buffer: &mut [u8]) -> Result<()> {
    #[cfg(test)]
    {
        if FORCE_UUID_ENTROPY_FAILURE.with(|flag| flag.get()) {
            return Err(CoreError::io_failed("uuid.v4"));
        }
    }
    #[cfg(not(target_family = "wasm"))]
    {
        if read_urandom(buffer).is_err() {
            fallback_random(buffer);
        }
        Ok(())
    }
    #[cfg(target_family = "wasm")]
    {
        // Plataforma (Crypto.getRandomValues vía getrandom/wasm_js). Sin contador ni panic.
        getrandom::fill(buffer).map_err(|_| CoreError::io_failed("uuid.v4"))
    }
}

fn uuid_from_bytes(mut bytes: [u8; 16]) -> String {
    bytes[6] = (bytes[6] & 0x0f) | 0x40;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    format!(
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0],
        bytes[1],
        bytes[2],
        bytes[3],
        bytes[4],
        bytes[5],
        bytes[6],
        bytes[7],
        bytes[8],
        bytes[9],
        bytes[10],
        bytes[11],
        bytes[12],
        bytes[13],
        bytes[14],
        bytes[15]
    )
}

/// UUID v4 de producción: urandom (nativo) o entropía de plataforma (WASM). Fallo tipado, sin panic.
pub fn new_uuid_v4() -> Result<String> {
    let mut bytes = [0u8; 16];
    fill_production_entropy(&mut bytes)?;
    Ok(uuid_from_bytes(bytes))
}

/// Solo tests / Prove-It: contador sin host entropy. La producción WASM usa `new_uuid_v4`.
pub fn new_uuid_v4_wasm_safe() -> String {
    let mut bytes = [0u8; 16];
    fill_with_counter_entropy(&mut bytes);
    uuid_from_bytes(bytes)
}
fn sha256_hex(bytes: &[u8]) -> String {
    use sha2::{Digest, Sha256};
    let mut hasher = Sha256::new();
    hasher.update(bytes);
    let digest = hasher.finalize();
    let mut out = String::with_capacity(64);
    for byte in digest {
        out.push_str(&format!("{byte:02x}"));
    }
    out
}

fn content_digest(value: &serde_json::Value, domain: &str) -> ContentDigest {
    let canonical = archmaker_canonicalization::canonicalize(value);
    let hex = archmaker_canonicalization::digest(&canonical, domain);
    ContentDigest {
        algorithm: DigestAlgorithm::Sha256,
        value: hex,
        kind: Some(ContentKind::Content),
    }
}

fn without_keys(value: &serde_json::Value, excluded: &[&str]) -> serde_json::Value {
    match value {
        serde_json::Value::Object(map) => {
            let kept: serde_json::Map<String, serde_json::Value> = map
                .iter()
                .filter(|(key, _)| !excluded.contains(&key.as_str()))
                .map(|(key, val)| (key.clone(), val.clone()))
                .collect();
            serde_json::Value::Object(kept)
        }
        other => other.clone(),
    }
}

pub fn draft_content_digest(draft: &Draft) -> Result<ContentDigest> {
    let source = CoreOp::SaveDraft.source();
    let domain = draft_domain_for(&draft.schema_version)?;
    let value = serde_json::to_value(draft).map_err(|_| CoreError::doc_canonical(source))?;
    Ok(content_digest(
        &without_keys(&value, &["contentDigest"]),
        domain,
    ))
}

pub fn catalog_content_digest(catalog: &Catalog) -> Result<ContentDigest> {
    let source = CoreOp::LoadCatalog.source();
    let value = serde_json::to_value(catalog).map_err(|_| CoreError::doc_canonical(source))?;
    Ok(content_digest(
        &without_keys(&value, &["contentDigest"]),
        CATALOG_DOMAIN,
    ))
}

pub fn manifest_content_digest(manifest: &Manifest) -> Result<ContentDigest> {
    let source = CoreOp::BuildManifest.source();
    let value = serde_json::to_value(manifest).map_err(|_| CoreError::doc_canonical(source))?;
    Ok(content_digest(
        &without_keys(&value, &["contentDigest"]),
        manifest.domain_label.as_str(),
    ))
}

fn resolution_content_digest(
    effective: &std::collections::BTreeMap<String, Vec<String>>,
    provided: &BTreeSet<String>,
    required: &BTreeSet<String>,
    diagnostics: &[Diagnostic],
) -> ContentDigest {
    let mut root = serde_json::Map::new();
    let effective_value: serde_json::Map<String, serde_json::Value> = effective
        .iter()
        .map(|(step, options)| {
            (
                step.clone(),
                serde_json::Value::Array(
                    options
                        .iter()
                        .map(|option| serde_json::Value::String(option.clone()))
                        .collect(),
                ),
            )
        })
        .collect();
    root.insert(
        "effective".to_string(),
        serde_json::Value::Object(effective_value),
    );
    let set_to_json = |set: &BTreeSet<String>| {
        serde_json::Value::Array(
            set.iter()
                .map(|item| serde_json::Value::String(item.clone()))
                .collect(),
        )
    };
    root.insert("provided".to_string(), set_to_json(provided));
    root.insert("required".to_string(), set_to_json(required));
    root.insert(
        "diagnostics".to_string(),
        serde_json::Value::Array(
            diagnostics
                .iter()
                .map(|diagnostic| serde_json::Value::String(diagnostic.code.clone()))
                .collect(),
        ),
    );
    content_digest(&serde_json::Value::Object(root), RESOLUTION_DOMAIN)
}

fn entity_ids(value: &SelectionValue) -> Vec<String> {
    match value {
        SelectionValue::Single { option_id } => vec![option_id.clone()],
        SelectionValue::Multiple { option_ids } => {
            let mut ids = option_ids.clone();
            ids.sort();
            ids.dedup();
            ids
        }
        SelectionValue::Boolean { .. }
        | SelectionValue::Number { .. }
        | SelectionValue::Text { .. }
        | SelectionValue::SecretRef { .. } => Vec::new(),
    }
}

/// Face scalar steps that the product preserves into the sealed manifest as intention.
const FACE_HOSTNAME_STEP: &str = "archmaker.face.hostname";
const FACE_ENCRYPT_STEP: &str = "archmaker.face.encrypt";

fn face_exclusion_diagnostic(step_id: &str, kind: &str) -> Diagnostic {
    Diagnostic {
        code: "AM-FACE-001".to_string(),
        severity: archmaker_rules::Severity::Warning,
        blocking: false,
        path: format!("/selections/{step_id}"),
        message_key: "diagnostic.faceSelectionExcluded".to_string(),
        source: "archmaker-core".to_string(),
        rule_id: "FACE-EXCLUDE-001".to_string(),
        suggestions: vec![format!(
            "selection kind '{kind}' on step '{step_id}' is excluded from the sealed profile"
        )],
    }
}

/// Preserve supported face scalars (hostname text, encrypt boolean intention) and
/// emit a visible exclusion diagnostic for unsupported scalar kinds.
fn append_face_scalar_selections(
    draft: &Draft,
    effective: &mut Vec<ResolvedSelection>,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for selection in &draft.selections {
        match &selection.value {
            SelectionValue::Text { value } if selection.step_id == FACE_HOSTNAME_STEP => {
                if value.trim().is_empty() {
                    continue;
                }
                effective.push(ResolvedSelection {
                    step_id: selection.step_id.clone(),
                    option_id: FACE_HOSTNAME_STEP.to_string(),
                    value: SelectionValue::Text {
                        value: value.clone(),
                    },
                    origin: SelectionOrigin::Manual,
                    locked: false,
                });
            }
            SelectionValue::Boolean { value } if selection.step_id == FACE_ENCRYPT_STEP => {
                effective.push(ResolvedSelection {
                    step_id: selection.step_id.clone(),
                    option_id: FACE_ENCRYPT_STEP.to_string(),
                    value: SelectionValue::Boolean { value: *value },
                    origin: SelectionOrigin::Manual,
                    locked: false,
                });
            }
            SelectionValue::Number { .. } => {
                diagnostics.push(face_exclusion_diagnostic(&selection.step_id, "number"));
            }
            SelectionValue::SecretRef { .. } => {
                diagnostics.push(face_exclusion_diagnostic(&selection.step_id, "secretRef"));
            }
            SelectionValue::Text { .. }
            | SelectionValue::Boolean { .. }
            | SelectionValue::Single { .. }
            | SelectionValue::Multiple { .. } => {}
        }
    }
}

fn rules_view(draft: &Draft) -> RulesDraft {
    let mut view = RulesDraft::new();
    for selection in &draft.selections {
        view.selections
            .insert(selection.step_id.clone(), entity_ids(&selection.value));
    }
    view
}

fn resolution_catalog(catalog: &Catalog) -> ResolutionCatalog {
    let mut resolved = ResolutionCatalog::new();
    for step in &catalog.steps {
        for section in &step.sections {
            for option in &section.options {
                resolved = resolved.with_option(CatalogOption::new(
                    option.id.clone(),
                    option.provides.clone().unwrap_or_default(),
                    option.requires.clone().unwrap_or_default(),
                ));
            }
        }
    }
    resolved
}

fn provided_required(
    view: &RulesDraft,
    catalog: &ResolutionCatalog,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut provided = BTreeSet::new();
    let mut required = BTreeSet::new();
    for option_id in view.all_selected() {
        if let Some(option) = catalog.find(option_id) {
            provided.extend(option.provides.iter().cloned());
            required.extend(option.requires.iter().cloned());
        }
    }
    (provided, required)
}

fn ensure_target(target: &TargetRef, allowed: &[&str], source: &str) -> Result<()> {
    if target.id.is_empty() || target.version.is_empty() {
        return Err(CoreError::tgt_unsupported(source));
    }
    if !allowed.contains(&target.id.as_str()) {
        return Err(CoreError::tgt_unsupported(source));
    }
    Ok(())
}

fn apply_changeset(draft: &mut Draft, changeset: &ChangeSet, source: &str) -> Result<()> {
    if let Some(base) = changeset.base_revision {
        if base != draft.revision {
            return Err(CoreError::doc_conflict(source));
        }
    }
    if let Some(target_id) = &changeset.target_draft_id {
        if target_id != &draft.id {
            return Err(CoreError::schema_invalid(source));
        }
    }
    for op in &changeset.operations {
        match op.op {
            ChangeSetOp::Add => {
                let Some(after) = &op.after else {
                    return Err(CoreError::schema_invalid(source));
                };
                let step_id = op
                    .note
                    .as_deref()
                    .and_then(|n| n.strip_prefix("stepId:"))
                    .ok_or_else(|| CoreError::schema_invalid(source))?
                    .to_string();
                if step_id.is_empty() {
                    return Err(CoreError::schema_invalid(source));
                }
                draft.selections.push(Selection {
                    step_id,
                    value: after.clone(),
                });
            }
            ChangeSetOp::Replace => {
                let Some(after) = &op.after else {
                    return Err(CoreError::schema_invalid(source));
                };
                let Some(index) = op
                    .path
                    .strip_prefix("/selections/")
                    .and_then(|rest| rest.strip_suffix("/value"))
                    .and_then(|idx| idx.parse::<usize>().ok())
                else {
                    return Err(CoreError::schema_invalid(source));
                };
                let Some(selection) = draft.selections.get_mut(index) else {
                    return Err(CoreError::schema_invalid(source));
                };
                if let Some(before) = &op.before {
                    if &selection.value != before {
                        return Err(CoreError::doc_conflict(source));
                    }
                }
                selection.value = after.clone();
            }
            ChangeSetOp::Remove => {
                let Some(index) = op
                    .path
                    .strip_prefix("/selections/")
                    .and_then(|idx| idx.parse::<usize>().ok())
                else {
                    return Err(CoreError::schema_invalid(source));
                };
                if index >= draft.selections.len() {
                    return Err(CoreError::schema_invalid(source));
                }
                if let Some(before) = &op.before {
                    if &draft.selections[index].value != before {
                        return Err(CoreError::doc_conflict(source));
                    }
                }
                draft.selections.remove(index);
            }
            ChangeSetOp::Test => {
                let Some(index) = op
                    .path
                    .strip_prefix("/selections/")
                    .and_then(|rest| rest.strip_suffix("/value"))
                    .and_then(|idx| idx.parse::<usize>().ok())
                else {
                    return Err(CoreError::schema_invalid(source));
                };
                let Some(selection) = draft.selections.get(index) else {
                    return Err(CoreError::schema_invalid(source));
                };
                if op.before.as_ref() != Some(&selection.value) {
                    return Err(CoreError::doc_conflict(source));
                }
            }
            ChangeSetOp::Move | ChangeSetOp::Copy => {
                return Err(CoreError::schema_invalid(source));
            }
        }
    }
    Ok(())
}

fn ensure_draft_shape(draft: &Draft, source: &str) -> Result<()> {
    if !is_uuid(&draft.id) {
        return Err(CoreError::schema_invalid(source));
    }
    if !is_document_version(&draft.document_version) {
        return Err(CoreError::schema_invalid(source));
    }
    let schema = decode_draft_schema_version(&draft.schema_version)
        .map_err(|_| CoreError::schema_invalid(source))?;
    if !is_semver(&draft.catalog_ref.version)
        || !is_semver(&draft.target_ref.version)
        || draft
            .preset_refs
            .as_ref()
            .is_some_and(|refs| refs.iter().any(|r| !is_semver(&r.version)))
    {
        return Err(CoreError::schema_invalid(source));
    }
    if draft.selections.len() > 4096 {
        return Err(CoreError::doc_limit(source));
    }
    if draft
        .preset_refs
        .as_ref()
        .is_some_and(|refs| refs.len() > 256)
    {
        return Err(CoreError::doc_limit(source));
    }
    for selection in &draft.selections {
        if selection.step_id.is_empty() {
            return Err(CoreError::schema_invalid(source));
        }
        match &selection.value {
            SelectionValue::Single { option_id } => {
                if option_id.is_empty() {
                    return Err(CoreError::schema_invalid(source));
                }
            }
            SelectionValue::Multiple { option_ids } => {
                if option_ids.is_empty()
                    || option_ids.len() > 4096
                    || option_ids.iter().any(|id| id.is_empty())
                {
                    return Err(CoreError::schema_invalid(source));
                }
                let mut sorted = option_ids.clone();
                sorted.sort();
                sorted.dedup();
                if sorted.len() != option_ids.len() {
                    return Err(CoreError::schema_invalid(source));
                }
            }
            SelectionValue::Boolean { .. } => {}
            SelectionValue::Number { value, .. } => {
                if !value.is_finite() {
                    return Err(CoreError::schema_invalid(source));
                }
            }
            SelectionValue::Text { value } => {
                if value.len() > 4096 {
                    return Err(CoreError::schema_invalid(source));
                }
            }
            SelectionValue::SecretRef { reference } => {
                if reference.is_empty() {
                    return Err(CoreError::schema_invalid(source));
                }
            }
        }
    }
    match schema {
        DraftSchemaVersion::V1 => {
            if draft.storage_plan.is_some()
                || draft.encryption_policy.is_some()
                || draft.browser_policy.is_some()
                || draft.verification.is_some()
                || draft.inherited_encrypt_intent.is_some()
            {
                return Err(CoreError::schema_invalid(source));
            }
        }
        DraftSchemaVersion::V2 => {
            if let Some(policy) = &draft.browser_policy {
                policy
                    .validate()
                    .map_err(|_| CoreError::schema_invalid(source))?;
            }
            if let Some(verification) = &draft.verification {
                if verification.execution_status != archmaker_domain::ExecutionStatus::NotExecuted {
                    return Err(CoreError::schema_invalid(source));
                }
            }
            if let Some(plan) = &draft.storage_plan {
                if plan.boot_artifact_strategy != archmaker_domain::BootArtifactStrategy::UkiOnEsp {
                    return Err(CoreError::schema_invalid(source));
                }
                if plan.mode == archmaker_domain::StorageMode::ReplaceDisk
                    && plan.loss_risk_acknowledged != Some(true)
                {
                    return Err(CoreError::schema_invalid(source));
                }
            }
        }
    }
    Ok(())
}

fn ensure_catalog_shape(catalog: &Catalog, source: &str, max_items: usize) -> Result<()> {
    if catalog.namespace.is_empty()
        || catalog.id.is_empty()
        || catalog.version.is_empty()
        || catalog.schema_version.is_empty()
        || catalog.document_version.is_empty()
    {
        return Err(CoreError::schema_invalid(source));
    }
    if !is_document_version(&catalog.document_version)
        || !is_integer_version(&catalog.schema_version)
        || !is_semver(&catalog.version)
    {
        return Err(CoreError::schema_invalid(source));
    }
    if catalog.steps.is_empty() || catalog.steps.len() > 1024 {
        return Err(CoreError::schema_invalid(source));
    }
    let mut options = 0usize;
    for step in &catalog.steps {
        if step.id.is_empty() || step.title.is_empty() || step.sections.is_empty() {
            return Err(CoreError::schema_invalid(source));
        }
        for section in &step.sections {
            if section.id.is_empty() || section.title.is_empty() || section.options.is_empty() {
                return Err(CoreError::schema_invalid(source));
            }
            for option in &section.options {
                if option.id.is_empty() || option.title.is_empty() {
                    return Err(CoreError::schema_invalid(source));
                }
                options += 1;
                if options > max_items {
                    return Err(CoreError::doc_limit(source));
                }
            }
        }
    }
    Ok(())
}

fn map_severity(severity: archmaker_rules::Severity) -> archmaker_domain::Severity {
    match severity {
        archmaker_rules::Severity::Error => archmaker_domain::Severity::Error,
        archmaker_rules::Severity::Warning => archmaker_domain::Severity::Warning,
        archmaker_rules::Severity::Info => archmaker_domain::Severity::Info,
    }
}

struct InternalResolution {
    digest: ContentDigest,
    effective: Vec<ResolvedSelection>,
    provided: Vec<CapabilityRef>,
    required: Vec<CapabilityRef>,
    conflicts: Vec<archmaker_domain::Conflict>,
    diagnostics: Vec<Diagnostic>,
}

fn resolve_internal(
    core: &RealCore,
    draft: &Draft,
    catalog_ref: &CatalogRef,
    source: &str,
) -> Result<InternalResolution> {
    core.ensure_catalog_match(catalog_ref, source)?;
    ensure_draft_shape(draft, source)?;
    let catalog = resolution_catalog(&core.catalog_embedded);
    let mut view = rules_view(draft);
    for selection in &draft.selections {
        for option_id in entity_ids(&selection.value) {
            if catalog.find(&option_id).is_none() {
                return Err(CoreError::res_unresolved(source));
            }
        }
    }
    let (provided, _) = provided_required(&view, &catalog);
    view.provided = provided.into_iter().collect();
    let resolved = archmaker_resolution::resolve(&view, &catalog, &core.active_rules);
    let digest = ContentDigest {
        algorithm: DigestAlgorithm::Sha256,
        value: resolved.resolution_digest.clone(),
        kind: Some(ContentKind::Content),
    };
    // Parity check: core digest helper must match resolution crate (NFR-DET-001).
    let expected = resolution_content_digest(
        &resolved.effective_selections,
        &resolved.provided_capabilities,
        &resolved.required_capabilities,
        &resolved.diagnostics,
    );
    debug_assert_eq!(digest.value, expected.value);
    let input_pairs: BTreeSet<(String, String)> = draft
        .selections
        .iter()
        .flat_map(|selection| {
            entity_ids(&selection.value)
                .into_iter()
                .map(|option| (selection.step_id.clone(), option))
        })
        .collect();
    let mut effective = Vec::new();
    for (step, options) in &resolved.effective_selections {
        for option in options {
            let derived = !input_pairs.contains(&(step.clone(), option.clone()));
            effective.push(ResolvedSelection {
                step_id: step.clone(),
                option_id: option.clone(),
                value: SelectionValue::Single {
                    option_id: option.clone(),
                },
                origin: if derived {
                    SelectionOrigin::Derived
                } else {
                    SelectionOrigin::Manual
                },
                locked: derived,
            });
        }
    }
    let mut diagnostics = resolved.diagnostics;
    append_face_scalar_selections(draft, &mut effective, &mut diagnostics);
    effective.sort_by(|a, b| (&a.step_id, &a.option_id).cmp(&(&b.step_id, &b.option_id)));
    let provided_refs = resolved
        .provided_capabilities
        .iter()
        .map(|id| CapabilityRef {
            id: id.clone(),
            title: None,
        })
        .collect();
    let required_refs = resolved
        .required_capabilities
        .iter()
        .map(|id| CapabilityRef {
            id: id.clone(),
            title: None,
        })
        .collect();
    let conflicts = resolved
        .conflicts
        .iter()
        .map(|conflict| archmaker_domain::Conflict {
            severity: map_severity(conflict.severity),
            capability_id: conflict.capability_id.clone(),
            path: conflict.path.clone(),
            message_key: conflict.message_key.clone(),
            suggestions: conflict.suggestions.clone(),
        })
        .collect();
    Ok(InternalResolution {
        digest,
        effective,
        provided: provided_refs,
        required: required_refs,
        conflicts,
        diagnostics,
    })
}

/// Bytes + media type + adapter payload for a sealed manifest (ADR-0003 / DEC-001).
#[derive(Debug, Clone, PartialEq)]
pub struct RenderedArtifact {
    pub bytes: Vec<u8>,
    pub media_type: String,
    pub content: serde_json::Value,
}

fn ordered_effective_selections(manifest: &Manifest) -> Vec<ResolvedSelection> {
    let mut rows = manifest.effective_selections.clone();
    rows.sort_by(|a, b| (&a.step_id, &a.option_id).cmp(&(&b.step_id, &b.option_id)));
    rows
}

fn render_archmaker_profile(manifest: &Manifest) -> Result<RenderedArtifact> {
    let source = CoreOp::ExportArtifact.source();
    let value = serde_json::to_value(manifest).map_err(|_| CoreError::doc_canonical(source))?;
    Ok(RenderedArtifact {
        bytes: archmaker_canonicalization::canonicalize(&value),
        media_type: "application/json".to_string(),
        content: value,
    })
}

/// Experimental archinstall adapter: versioned envelope, never the raw manifest (ADR-0003).
fn render_archinstall_profile(manifest: &Manifest, target: &TargetRef) -> Result<RenderedArtifact> {
    let source = CoreOp::ExportArtifact.source();
    let selections = ordered_effective_selections(manifest);
    let envelope = serde_json::json!({
        "adapter": "archinstall-profile",
        "version": target.version,
        "manifestRef": {
            "contentDigest": {
                "algorithm": "sha256",
                "value": manifest.content_digest.value,
                "kind": "content",
            }
        },
        "effectiveSelections": selections,
    });
    // Reject executable / shell-shaped keys if they ever appear in the envelope.
    if let Some(obj) = envelope.as_object() {
        for banned in ["cmd", "hooks", "pacstrap", "shell", "script", "commands"] {
            if obj.contains_key(banned) {
                return Err(CoreError::tgt_unsupported(source));
            }
        }
    }
    Ok(RenderedArtifact {
        bytes: archmaker_canonicalization::canonicalize(&envelope),
        media_type: "application/json".to_string(),
        content: envelope,
    })
}

fn render_report(manifest: &Manifest, target: &TargetRef) -> Result<RenderedArtifact> {
    let mut text = String::from("# ArchMaker report\n\n");
    text.push_str(&format!("domain: {}\n", manifest.domain_label.as_str()));
    text.push_str(&format!("target: {} {}\n", target.id, target.version));
    text.push_str(&format!(
        "contentDigest: sha256:{}\n\n",
        manifest.content_digest.value
    ));
    text.push_str("## Effective selections\n\n");
    for selection in &ordered_effective_selections(manifest) {
        let origin = match selection.origin {
            SelectionOrigin::Manual => "manual",
            SelectionOrigin::Derived => "derived",
        };
        text.push_str(&format!(
            "- {} = {} ({})\n",
            selection.step_id, selection.option_id, origin
        ));
    }
    Ok(RenderedArtifact {
        bytes: text.as_bytes().to_vec(),
        media_type: "text/markdown".to_string(),
        content: serde_json::Value::String(text),
    })
}

pub fn render_artifact_bytes(manifest: &Manifest, target: &TargetRef) -> Result<RenderedArtifact> {
    let source = CoreOp::ExportArtifact.source();
    match target.id.as_str() {
        "archmaker-profile" => render_archmaker_profile(manifest),
        "archinstall-profile" => render_archinstall_profile(manifest, target),
        "report" => render_report(manifest, target),
        _ => Err(CoreError::tgt_unsupported(source)),
    }
}

fn union_targets() -> Vec<&'static str> {
    let mut targets = DRAFT_TARGETS.to_vec();
    targets.extend(EXPORT_TARGETS);
    targets
}

impl CorePort for RealCore {
    fn create_draft(&self, input: CreateDraftInput) -> Result<Draft> {
        let source = CoreOp::CreateDraft.source();
        self.ensure_catalog_match(&input.catalog_ref, source)?;
        ensure_target(&input.target_ref, &DRAFT_TARGETS, source)?;
        let id = match input.draft_id {
            Some(given) => {
                if !is_uuid(&given) {
                    return Err(CoreError::schema_invalid(source));
                }
                given
            }
            None => new_uuid_v4()?,
        };
        if let Some(preset) = &input.preset_ref {
            if preset.namespace.is_empty() || preset.id.is_empty() || preset.version.is_empty() {
                return Err(CoreError::schema_invalid(source));
            }
        }
        let draft = Draft {
            id: id.clone(),
            document_version: "vNext".to_string(),
            schema_version: "1".to_string(),
            revision: 0,
            content_digest: None,
            catalog_ref: input.catalog_ref,
            target_ref: input.target_ref,
            selections: Vec::new(),
            preset_refs: input.preset_ref.map(|preset: PresetRef| vec![preset]),
            created_at: None,
            updated_at: None,
            storage_plan: None,
            encryption_policy: None,
            browser_policy: None,
            verification: None,
            inherited_encrypt_intent: None,
        };
        match self.drafts.borrow_mut().entry(id) {
            std::collections::hash_map::Entry::Vacant(slot) => {
                slot.insert(draft.clone());
            }
            std::collections::hash_map::Entry::Occupied(_) => {
                return Err(CoreError::doc_conflict(source));
            }
        }
        Ok(draft)
    }

    fn load_catalog(&self, input: LoadCatalogInput) -> Result<Catalog> {
        let source = CoreOp::LoadCatalog.source();
        let limits = input.limits.unwrap_or(DEFAULT_LIMITS);
        let catalog = match input.source {
            CatalogSource::Embedded => self.catalog_embedded.clone(),
            CatalogSource::File => {
                let expected_prov = input
                    .expected_provenance
                    .as_ref()
                    .ok_or_else(|| CoreError::cat_provenance_missing(source))?;
                let path = input
                    .file_path
                    .as_ref()
                    .ok_or_else(|| CoreError::doc_limit(source))?;
                let metadata = std::fs::metadata(path).map_err(|_| CoreError::io_failed(source))?;
                if metadata.len() > limits.max_bytes {
                    return Err(CoreError::doc_limit(source));
                }
                let bytes = std::fs::read(path).map_err(|_| CoreError::io_failed(source))?;
                if bytes.len() as u64 > limits.max_bytes {
                    return Err(CoreError::doc_limit(source));
                }
                check_json_depth(&bytes, limits.max_depth)?;
                let value: serde_json::Value =
                    serde_json::from_slice(&bytes).map_err(|_| CoreError::doc_limit(source))?;
                if value
                    .get("steps")
                    .and_then(|steps| steps.as_array())
                    .is_some_and(|steps| steps.len() > limits.max_array_items)
                {
                    return Err(CoreError::doc_limit(source));
                }
                let catalog: Catalog =
                    serde_json::from_value(value).map_err(|_| CoreError::schema_invalid(source))?;
                let Some(actual_prov) = &catalog.provenance else {
                    return Err(CoreError::cat_provenance_missing(source));
                };
                if actual_prov.source_id != expected_prov.source_id
                    || actual_prov.path != expected_prov.path
                    || actual_prov.captured_at != expected_prov.captured_at
                {
                    return Err(CoreError::cat_provenance_missing(source));
                }
                catalog
            }
        };
        ensure_catalog_shape(&catalog, source, limits.max_array_items)?;
        if let Some(catalog_ref) = &input.catalog_ref {
            if catalog_ref.namespace != catalog.namespace
                || catalog_ref.id != catalog.id
                || catalog_ref.version != catalog.version
            {
                return Err(CoreError::cat_not_found(source));
            }
        }
        if let Some(expected) = &input.expected_digest {
            if expected.algorithm != DigestAlgorithm::Sha256 {
                return Err(CoreError::cat_digest_mismatch(source));
            }
            let actual = catalog_content_digest(&catalog)?;
            if expected.value.to_lowercase() != actual.value {
                return Err(CoreError::cat_digest_mismatch(source));
            }
        }
        Ok(catalog)
    }

    fn save_draft(&self, input: SaveDraftInput) -> Result<Draft> {
        let source = CoreOp::SaveDraft.source();
        let mut draft = input.draft;
        ensure_draft_shape(&draft, source)?;
        self.ensure_catalog_match(&draft.catalog_ref, source)?;
        ensure_target(&draft.target_ref, &union_targets(), source)?;
        let stored = self.drafts.borrow().get(&draft.id).cloned();
        match stored {
            Some(current) => {
                if current.revision != input.expected_revision {
                    return Err(CoreError::doc_conflict(source));
                }
            }
            None => {
                if input.expected_revision != 0 {
                    return Err(CoreError::doc_conflict(source));
                }
            }
        }
        if let Some(changeset) = &input.changeset {
            if changeset
                .base_revision
                .is_some_and(|b| b != input.expected_revision)
            {
                return Err(CoreError::doc_conflict(source));
            }
            apply_changeset(&mut draft, changeset, source)?;
            ensure_draft_shape(&draft, source)?;
        }
        draft.revision = input.expected_revision + 1;
        draft.updated_at = None;
        draft.content_digest = Some(draft_content_digest(&draft)?);
        if let Some(dir) = &self.store_dir {
            let bytes =
                serde_json::to_vec_pretty(&draft).map_err(|_| CoreError::doc_canonical(source))?;
            let path = dir.join(format!("{}.json", draft.id));
            atomic_write_bytes(CoreOp::SaveDraft, &path, &bytes)?;
        }
        self.drafts
            .borrow_mut()
            .insert(draft.id.clone(), draft.clone());
        Ok(draft)
    }

    fn open_draft(&self, input: OpenDraftInput) -> Result<Draft> {
        let source = CoreOp::OpenDraft.source();
        if !is_uuid(&input.draft_id) {
            return Err(CoreError::schema_invalid(source));
        }
        if let Some(draft) = self.drafts.borrow().get(&input.draft_id).cloned() {
            return Ok(draft);
        }
        if let Some(seed) = input.seed {
            if seed.id != input.draft_id {
                return Err(CoreError::schema_invalid(source));
            }
            ensure_draft_shape(&seed, source)?;
            self.ensure_catalog_match(&seed.catalog_ref, source)?;
            self.drafts
                .borrow_mut()
                .insert(seed.id.clone(), seed.clone());
            return Ok(seed);
        }
        let Some(dir) = &self.store_dir else {
            return Err(CoreError::doc_conflict(source));
        };
        let path = dir.join(format!("{}.json", input.draft_id));
        let bytes = std::fs::read(&path).map_err(|_| CoreError::io_failed(source))?;
        let draft: Draft =
            serde_json::from_slice(&bytes).map_err(|_| CoreError::schema_invalid(source))?;
        if draft.id != input.draft_id {
            return Err(CoreError::schema_invalid(source));
        }
        ensure_draft_shape(&draft, source)?;
        self.ensure_catalog_match(&draft.catalog_ref, source)?;
        self.drafts
            .borrow_mut()
            .insert(draft.id.clone(), draft.clone());
        Ok(draft)
    }

    fn resolve_draft(&self, input: ResolveDraftInput) -> Result<ResolveResult> {
        let source = CoreOp::ResolveDraft.source();
        let internal = resolve_internal(self, &input.draft, &input.catalog_ref, source)?;
        Ok(ResolveResult {
            resolution_digest: internal.digest,
            effective_selections: internal.effective,
            provided_capabilities: internal.provided,
            required_capabilities: internal.required,
            conflicts: internal.conflicts,
            diagnostics: internal.diagnostics,
        })
    }

    fn validate_draft(&self, input: ValidateDraftInput) -> Result<ValidationResult> {
        let source = CoreOp::ValidateDraft.source();
        if let Some(target) = &input.target_ref {
            ensure_target(target, &union_targets(), source)?;
        }
        // Same pipeline as resolve/buildManifest (stages 6–9): integrity, rules, resolution.
        let internal = resolve_internal(self, &input.draft, &input.catalog_ref, source)?;
        let mut diagnostics = internal.diagnostics;
        sort_diagnostics(&mut diagnostics);
        let blocking = !internal.conflicts.is_empty()
            || diagnostics.iter().any(|diagnostic| diagnostic.blocking);
        Ok(ValidationResult {
            diagnostics,
            blocking,
            pipeline_version: PIPELINE_VERSION.to_string(),
        })
    }

    fn build_manifest(&self, input: BuildManifestInput) -> Result<Manifest> {
        let source = CoreOp::BuildManifest.source();
        ensure_target(&input.target_ref, &EXPORT_TARGETS, source)?;
        let internal = resolve_internal(self, &input.draft, &input.catalog_ref, source)?;
        if !internal.conflicts.is_empty() {
            return Err(CoreError::res_blocked(source));
        }
        if internal
            .diagnostics
            .iter()
            .any(|diagnostic| diagnostic.blocking)
        {
            return Err(CoreError::res_blocked(source));
        }
        if internal.effective.is_empty() {
            return Err(CoreError::res_unresolved(source));
        }
        if let Some(expected) = &input.expected_resolution_digest {
            if expected.value.to_lowercase() != internal.digest.value {
                return Err(CoreError::res_blocked(source));
            }
        }
        let domain_label = ManifestDomainLabel::for_schema_version(&input.draft.schema_version)
            .map_err(|_| CoreError::schema_invalid(source))?;
        let mut manifest = Manifest {
            document_version: input.draft.document_version.clone(),
            schema_version: input.draft.schema_version.clone(),
            domain_label,
            catalog_ref: input.draft.catalog_ref.clone(),
            target_ref: input.target_ref,
            producer: Producer {
                id: PRODUCER_ID.to_string(),
                version: PRODUCER_VERSION.to_string(),
            },
            resolution_digest: internal.digest,
            effective_selections: internal.effective,
            provided_capabilities: internal.provided,
            required_capabilities: internal.required,
            conflicts: Vec::new(),
            content_digest: ContentDigest {
                algorithm: DigestAlgorithm::Sha256,
                value: String::new(),
                kind: Some(ContentKind::Content),
            },
            storage_plan: input.draft.storage_plan.clone(),
            encryption_policy: input.draft.encryption_policy.clone(),
            browser_policy: input.draft.browser_policy.clone(),
            verification: input.draft.verification.clone(),
        };
        let digest = manifest_content_digest(&manifest)?;
        manifest.content_digest = digest;
        Ok(manifest)
    }

    fn export_artifact(&self, input: ExportArtifactInput) -> Result<Artifact> {
        let source = CoreOp::ExportArtifact.source();
        ensure_target(&input.target_ref, &EXPORT_TARGETS, source)?;
        if input.manifest.target_ref.id != input.target_ref.id {
            return Err(CoreError::tgt_unsupported(source));
        }
        if !input.manifest.conflicts.is_empty() {
            return Err(CoreError::res_blocked(source));
        }
        let recomputed = manifest_content_digest(&input.manifest)?;
        if recomputed.value != input.manifest.content_digest.value {
            return Err(CoreError::doc_canonical(source));
        }
        let rendered = render_artifact_bytes(&input.manifest, &input.target_ref)?;
        let binary = BinaryDigest {
            algorithm: DigestAlgorithm::Sha256,
            value: sha256_hex(&rendered.bytes),
            kind: Some(BinaryKind::Binary),
        };
        if let Some(dir) = &self.store_dir {
            atomic_write_bytes(
                CoreOp::ExportArtifact,
                &dir.join(input.destination.token()),
                &rendered.bytes,
            )?;
        }
        let experimental = if input.target_ref.id == "archinstall-profile" {
            Some(true)
        } else {
            None
        };
        Ok(Artifact {
            document_version: input.manifest.document_version.clone(),
            schema_version: input.manifest.schema_version.clone(),
            artifact_ref: ArtifactRef {
                binary_digest: binary.clone(),
                media_type: rendered.media_type.clone(),
            },
            target_ref: input.target_ref,
            producer: Producer {
                id: PRODUCER_ID.to_string(),
                version: PRODUCER_VERSION.to_string(),
            },
            media_type: rendered.media_type,
            binary_digest: binary,
            manifest_ref: ManifestRef {
                content_digest: input.manifest.content_digest.clone(),
            },
            experimental,
            content: Some(rendered.content),
            bytes: Some(rendered.bytes),
        })
    }

    fn import_draft(&self, input: ImportDraftInput) -> Result<ImportDraftResult> {
        let source = CoreOp::ImportDraft.source();
        self.ensure_catalog_match(&input.catalog_ref, source)?;
        ensure_target(&input.target_ref, &DRAFT_TARGETS, source)?;
        let draft_id = match input.draft_id {
            Some(given) => {
                if !is_uuid(&given) {
                    return Err(CoreError::schema_invalid(source));
                }
                given
            }
            None => new_uuid_v4()?,
        };
        let result = migrate_v51_to_draft(
            &input.source_bytes,
            input.catalog_ref,
            input.target_ref,
            draft_id,
            input.confirm_blocking_losses,
        )?;
        ensure_draft_shape(&result.draft, source)?;
        Ok(result)
    }
}

#[cfg(test)]
mod identity_unit_tests {
    use super::*;

    #[test]
    fn entropy_failure_is_typed_without_panic() {
        test_force_uuid_entropy_failure(true);
        let err = new_uuid_v4().expect_err("entropía forzada");
        test_force_uuid_entropy_failure(false);
        assert_eq!(err.code, "AM-IO-001");
        assert_eq!(err.source, "uuid.v4");
    }

    #[test]
    fn production_uuid_is_v4_after_entropy_ok() {
        test_force_uuid_entropy_failure(false);
        let id = new_uuid_v4().expect("entropía disponible en nativo");
        assert!(is_uuid(&id));
        assert_eq!(id.as_bytes()[14], b'4');
    }
}
