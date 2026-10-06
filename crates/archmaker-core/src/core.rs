use std::cell::RefCell;
use std::collections::{BTreeSet, HashMap};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use archmaker_domain::{
    Artifact, ArtifactRef, BinaryDigest, BinaryKind, CapabilityRef, Catalog, CatalogRef,
    ContentDigest, ContentKind, DigestAlgorithm, Draft, Manifest, ManifestDomainLabel, ManifestRef,
    PresetRef, Producer, ResolvedSelection, SelectionOrigin, SelectionValue, TargetRef,
    MANIFEST_DOMAIN_LABEL,
};
use archmaker_resolution::{Catalog as ResolutionCatalog, CatalogOption};
use archmaker_rules::{canonical_rules, evaluate_all, Diagnostic, Draft as RulesDraft};

use crate::error::{CoreError, Result};
use crate::inputs::{
    sort_diagnostics, BuildManifestInput, CatalogSource, CreateDraftInput, ExportArtifactInput,
    LoadCatalogInput, ResolveDraftInput, ResolveResult, SaveDraftInput, ValidateDraftInput,
    ValidationResult,
};
use crate::io::atomic_write_bytes;
use crate::limits::DEFAULT_LIMITS;
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

const DRAFT_DOMAIN: &str = "archmaker:draft:v1";
const CATALOG_DOMAIN: &str = "archmaker:catalog:v1";
const RESOLUTION_DOMAIN: &str = "archmaker:resolution:v1";

const DRAFT_TARGETS: [&str; 1] = ["arch-x86_64"];
const EXPORT_TARGETS: [&str; 3] = ["archmaker-profile", "archinstall-profile", "report"];

const PRODUCER_ID: &str = "archmaker-core";
const PRODUCER_VERSION: &str = "0.1.0";
pub const PIPELINE_VERSION: &str = "0.1.0";

static UUID_COUNTER: AtomicU64 = AtomicU64::new(0);

pub struct RealCore {
    pub catalog_embedded: Catalog,
    store_dir: Option<PathBuf>,
    drafts: RefCell<HashMap<String, Draft>>,
}

impl RealCore {
    pub fn new(catalog_embedded: Catalog) -> Self {
        Self {
            catalog_embedded,
            store_dir: None,
            drafts: RefCell::new(HashMap::new()),
        }
    }

    pub fn with_embedded() -> Result<Self> {
        let source = CoreOp::LoadCatalog.source();
        let value: serde_json::Value = serde_json::from_str(EMBEDDED_CATALOG_JSON)
            .map_err(|_| CoreError::schema_invalid(source))?;
        let catalog: Catalog =
            serde_json::from_value(value).map_err(|_| CoreError::schema_invalid(source))?;
        Ok(Self::new(catalog))
    }

    pub fn with_store_dir(mut self, dir: PathBuf) -> Self {
        self.store_dir = Some(dir);
        self
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

fn read_urandom(buffer: &mut [u8]) -> std::io::Result<()> {
    File::open("/dev/urandom")?.read_exact(buffer)
}

fn fallback_random(buffer: &mut [u8]) {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or(0);
    let mut state = nanos
        .wrapping_add((std::process::id() as u128) << 64)
        .wrapping_add((UUID_COUNTER.fetch_add(1, Ordering::Relaxed) as u128) << 96)
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

pub fn new_uuid_v4() -> String {
    let mut bytes = [0u8; 16];
    if read_urandom(&mut bytes).is_err() {
        fallback_random(&mut bytes);
    }
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
    let value = serde_json::to_value(draft).map_err(|_| CoreError::doc_canonical(source))?;
    Ok(content_digest(
        &without_keys(&value, &["contentDigest"]),
        DRAFT_DOMAIN,
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
        MANIFEST_DOMAIN_LABEL,
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

fn ensure_draft_shape(draft: &Draft, source: &str) -> Result<()> {
    if !is_uuid(&draft.id) {
        return Err(CoreError::schema_invalid(source));
    }
    if draft.schema_version.is_empty() || draft.document_version.is_empty() {
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
    let resolved = archmaker_resolution::resolve(&view, &catalog);
    let digest = resolution_content_digest(
        &resolved.effective_selections,
        &resolved.provided_capabilities,
        &resolved.required_capabilities,
        &resolved.diagnostics,
    );
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
        diagnostics: resolved.diagnostics,
    })
}

pub fn render_artifact_bytes(manifest: &Manifest, target: &TargetRef) -> Result<(Vec<u8>, String)> {
    let source = CoreOp::ExportArtifact.source();
    match target.id.as_str() {
        "archmaker-profile" | "archinstall-profile" => {
            let value =
                serde_json::to_value(manifest).map_err(|_| CoreError::doc_canonical(source))?;
            Ok((
                archmaker_canonicalization::canonicalize(&value),
                "application/json".to_string(),
            ))
        }
        "report" => {
            let mut text = String::from("# ArchMaker report\n\n");
            text.push_str(&format!("domain: {MANIFEST_DOMAIN_LABEL}\n"));
            text.push_str(&format!("target: {} {}\n", target.id, target.version));
            text.push_str(&format!(
                "contentDigest: sha256:{}\n\n",
                manifest.content_digest.value
            ));
            text.push_str("## Effective selections\n\n");
            let mut rows = manifest.effective_selections.clone();
            rows.sort_by(|a, b| (&a.step_id, &a.option_id).cmp(&(&b.step_id, &b.option_id)));
            for selection in &rows {
                let origin = match selection.origin {
                    SelectionOrigin::Manual => "manual",
                    SelectionOrigin::Derived => "derived",
                };
                text.push_str(&format!(
                    "- {} = {} ({})\n",
                    selection.step_id, selection.option_id, origin
                ));
            }
            Ok((text.into_bytes(), "text/markdown".to_string()))
        }
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
            None => new_uuid_v4(),
        };
        if let Some(preset) = &input.preset_ref {
            if preset.namespace.is_empty() || preset.id.is_empty() || preset.version.is_empty() {
                return Err(CoreError::schema_invalid(source));
            }
        }
        Ok(Draft {
            id,
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
        })
    }

    fn load_catalog(&self, input: LoadCatalogInput) -> Result<Catalog> {
        let source = CoreOp::LoadCatalog.source();
        let limits = input.limits.unwrap_or(DEFAULT_LIMITS);
        let catalog = match input.source {
            CatalogSource::Embedded => self.catalog_embedded.clone(),
            CatalogSource::File => {
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
                serde_json::from_value(value).map_err(|_| CoreError::schema_invalid(source))?
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
        self.ensure_catalog_match(&input.catalog_ref, source)?;
        ensure_draft_shape(&input.draft, source)?;
        if let Some(target) = &input.target_ref {
            ensure_target(target, &union_targets(), source)?;
        }
        let catalog = resolution_catalog(&self.catalog_embedded);
        let mut view = rules_view(&input.draft);
        let (provided, _) = provided_required(&view, &catalog);
        view.provided = provided.into_iter().collect();
        let mut diagnostics = evaluate_all(&canonical_rules(), &view);
        sort_diagnostics(&mut diagnostics);
        let blocking = diagnostics.iter().any(|diagnostic| diagnostic.blocking);
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
        let mut manifest = Manifest {
            document_version: input.draft.document_version.clone(),
            schema_version: input.draft.schema_version.clone(),
            domain_label: ManifestDomainLabel,
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
        let (bytes, media_type) = render_artifact_bytes(&input.manifest, &input.target_ref)?;
        let binary = BinaryDigest {
            algorithm: DigestAlgorithm::Sha256,
            value: sha256_hex(&bytes),
            kind: Some(BinaryKind::Binary),
        };
        let dir = self
            .store_dir
            .as_ref()
            .ok_or_else(|| CoreError::io_failed(source))?;
        atomic_write_bytes(
            CoreOp::ExportArtifact,
            &dir.join(input.destination.token()),
            &bytes,
        )?;
        Ok(Artifact {
            document_version: input.manifest.document_version.clone(),
            schema_version: input.manifest.schema_version.clone(),
            artifact_ref: ArtifactRef {
                binary_digest: binary.clone(),
                media_type: media_type.clone(),
            },
            target_ref: input.target_ref,
            producer: Producer {
                id: PRODUCER_ID.to_string(),
                version: PRODUCER_VERSION.to_string(),
            },
            media_type,
            binary_digest: binary,
            manifest_ref: ManifestRef {
                content_digest: input.manifest.content_digest.clone(),
            },
            experimental: Some(input.manifest.target_ref.id == "archinstall-profile"),
            content: None,
        })
    }
}
