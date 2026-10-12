//! Crate `archmaker-resolution` — resolución determinista (etapas 8–9).
//!
//! - [`resolve`] es pura: mismo `(draft, catalog, rules)` → mismo [`ResolveResult`].
//! - Diagnósticos ordenados por `(path, code, source)`.
//! - Digest SHA-256 canónico (`archmaker:resolution:v1`), alineado con core.

use std::collections::{BTreeMap, BTreeSet};

use archmaker_rules::{
    evaluate, evaluate_all, rule_dm_001, Diagnostic, Draft, Effect, Rule, Severity, RULE_DM_001,
};

const RESOLUTION_DOMAIN: &str = "archmaker:resolution:v1";

// ---------------------------------------------------------------------------
// Catálogo mínimo (vista de resolución)
// ---------------------------------------------------------------------------

/// Opción del catálogo relevante para la resolución: identidad más las
/// capabilities que aporta y las que requiere.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CatalogOption {
    pub id: String,
    pub provides: Vec<String>,
    pub requires: Vec<String>,
}

impl CatalogOption {
    pub fn new(id: impl Into<String>, provides: Vec<String>, requires: Vec<String>) -> Self {
        Self {
            id: id.into(),
            provides,
            requires,
        }
    }
}

/// Catálogo indexado por id de opción (ordenado para determinismo).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Catalog {
    pub options: BTreeMap<String, CatalogOption>,
}

impl Catalog {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_option(mut self, option: CatalogOption) -> Self {
        self.options.insert(option.id.clone(), option);
        self
    }

    /// Búsqueda exacta o por sufijo (`firefox` casa con `archmaker.pkg.firefox`).
    pub fn find(&self, option_id: &str) -> Option<&CatalogOption> {
        if let Some(option) = self.options.get(option_id) {
            return Some(option);
        }
        self.options.values().find(|option| {
            option.id == option_id
                || option.id.rsplit('.').next() == Some(option_id)
                || option_id.rsplit('.').next() == Some(option.id.as_str())
        })
    }
}

// ---------------------------------------------------------------------------
// Resultado
// ---------------------------------------------------------------------------

/// Conflicto de resolución (capabilities o derivación manual).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Conflict {
    pub severity: Severity,
    pub path: String,
    pub message_key: String,
    pub capability_id: Option<String>,
    pub suggestions: Vec<String>,
}

/// Salida determinista de [`resolve`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResolveResult {
    pub effective_selections: BTreeMap<String, Vec<String>>,
    pub provided_capabilities: BTreeSet<String>,
    pub required_capabilities: BTreeSet<String>,
    pub conflicts: Vec<Conflict>,
    pub diagnostics: Vec<Diagnostic>,
    /// Hex SHA-256 del payload canónico de resolución.
    pub resolution_digest: String,
}

// ---------------------------------------------------------------------------
// Resolución
// ---------------------------------------------------------------------------

fn capabilities_of(
    selections: &BTreeMap<String, Vec<String>>,
    catalog: &Catalog,
) -> (BTreeSet<String>, BTreeSet<String>) {
    let mut provided = BTreeSet::new();
    let mut required = BTreeSet::new();
    for option_id in selections.values().flatten() {
        if let Some(option) = catalog.find(option_id) {
            provided.extend(option.provides.iter().cloned());
            required.extend(option.requires.iter().cloned());
        }
    }
    (provided, required)
}

fn sorted_diagnostics(mut diagnostics: Vec<Diagnostic>) -> Vec<Diagnostic> {
    diagnostics.sort_by(|a, b| (&a.path, &a.code, &a.source).cmp(&(&b.path, &b.code, &b.source)));
    diagnostics
}

fn id_matches(stored: &str, wanted: &str) -> bool {
    if stored == wanted {
        return true;
    }
    if stored.rsplit('.').next() == Some(wanted) {
        return true;
    }
    if wanted.rsplit('.').next() == Some(stored) {
        return true;
    }
    false
}

fn step_matches(step: &str, wanted: &str) -> bool {
    id_matches(step, wanted)
}

fn digest_input(
    effective: &BTreeMap<String, Vec<String>>,
    provided: &BTreeSet<String>,
    required: &BTreeSet<String>,
    diagnostics: &[Diagnostic],
) -> String {
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
    let value = serde_json::Value::Object(root);
    let canonical = archmaker_canonicalization::canonicalize(&value);
    archmaker_canonicalization::digest(&canonical, RESOLUTION_DOMAIN)
}

fn apply_dm_derivation(
    effective: &mut BTreeMap<String, Vec<String>>,
    conflicts: &mut Vec<Conflict>,
    provided: &mut BTreeSet<String>,
    required: &mut BTreeSet<String>,
    catalog: &Catalog,
    rules: &[Rule],
) {
    let dm_rule = rules
        .iter()
        .find(|rule| rule.id == RULE_DM_001)
        .cloned()
        .unwrap_or_else(rule_dm_001);
    let Effect::Derive { target, value } = &dm_rule.effect else {
        return;
    };
    let view = Draft {
        selections: effective.clone(),
        provided: provided.clone(),
    };
    if evaluate(&dm_rule, &view).is_none() {
        return;
    }
    let existing = effective
        .iter()
        .find(|(step, _)| step_matches(step, target))
        .map(|(step, options)| (step.clone(), options.clone()));
    match existing {
        None => {
            effective.insert(target.clone(), vec![value.clone()]);
            let recomputed = capabilities_of(effective, catalog);
            *provided = recomputed.0;
            *required = recomputed.1;
        }
        Some((_, current)) if current.iter().any(|v| id_matches(v, value)) => {}
        Some((step, _)) => {
            conflicts.push(Conflict {
                severity: Severity::Error,
                path: format!("/selections/{step}"),
                message_key: "diagnostic.dmDerived".to_string(),
                capability_id: None,
                suggestions: vec![format!(
                    "Set {target} to {value} or change the compositor selection."
                )],
            });
        }
    }
}

/// Resuelve el draft contra el catálogo aplicando las reglas dadas.
///
/// 1. Congela selecciones efectivas.
/// 2. Calcula capabilities aportadas/requeridas.
/// 3. Aplica RULE-DM-001 si está en `rules` (derivación explicable).
/// 4. Evalúa las reglas sobre la vista enriquecida.
/// 5. Registra conflictos por `requires` insatisfechos.
/// 6. Ordena diagnósticos y calcula el digest SHA-256.
pub fn resolve(draft: &Draft, catalog: &Catalog, rules: &[Rule]) -> ResolveResult {
    let mut effective: BTreeMap<String, Vec<String>> = draft
        .selections
        .iter()
        .map(|(step, options)| {
            let mut sorted = options.clone();
            sorted.sort();
            (step.clone(), sorted)
        })
        .collect();

    let mut conflicts: Vec<Conflict> = Vec::new();
    let (mut provided, mut required) = capabilities_of(&effective, catalog);

    apply_dm_derivation(
        &mut effective,
        &mut conflicts,
        &mut provided,
        &mut required,
        catalog,
        rules,
    );

    let enriched = Draft {
        selections: effective.clone(),
        provided: provided.clone(),
    };
    let diagnostics = sorted_diagnostics(evaluate_all(rules, &enriched));

    for option_id in effective.values().flatten() {
        if let Some(option) = catalog.find(option_id) {
            for requirement in &option.requires {
                if !provided.contains(requirement) {
                    conflicts.push(Conflict {
                        severity: Severity::Error,
                        path: format!("/selections/{option_id}"),
                        message_key: "diagnostic.capabilityMissing".to_string(),
                        capability_id: Some(requirement.clone()),
                        suggestions: vec![format!(
                            "Provide capability {requirement} or drop {option_id}."
                        )],
                    });
                }
            }
        }
    }
    conflicts.sort_by(|a, b| (&a.path, &a.message_key).cmp(&(&b.path, &b.message_key)));

    let resolution_digest = digest_input(&effective, &provided, &required, &diagnostics);

    ResolveResult {
        effective_selections: effective,
        provided_capabilities: provided,
        required_capabilities: required,
        conflicts,
        diagnostics,
        resolution_digest,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use archmaker_rules::canonical_rules;

    fn option(id: &str, provides: &[&str], requires: &[&str]) -> CatalogOption {
        CatalogOption::new(
            id,
            provides.iter().map(|s| s.to_string()).collect(),
            requires.iter().map(|s| s.to_string()).collect(),
        )
    }

    fn fixture_catalog() -> Catalog {
        Catalog::new()
            .with_option(option("linux", &["fs.ext4"], &[]))
            .with_option(option("gnome", &[], &[]))
            .with_option(option("gdm", &[], &[]))
            .with_option(option("firefox", &["pkg.firefox"], &[]))
            .with_option(option("ext4", &["fs.ext4"], &[]))
    }

    fn draft(selections: &[(&str, &[&str])]) -> Draft {
        let mut d = Draft::new();
        for (step, options) in selections {
            d.selections.insert(
                step.to_string(),
                options.iter().map(|s| s.to_string()).collect(),
            );
        }
        d
    }

    fn rules() -> Vec<Rule> {
        canonical_rules()
    }

    #[test]
    fn gnome_derives_gdm_with_info_diagnostic() {
        let result = resolve(
            &draft(&[("compositor", &["gnome"]), ("kernel", &["linux"])]),
            &fixture_catalog(),
            &rules(),
        );
        let dm = result.effective_selections.iter().find(|(step, _)| {
            step_matches(step, "dm") || step_matches(step, "archmaker.environment.dm")
        });
        assert!(dm.is_some());
        let (_, values) = dm.unwrap();
        assert!(values.iter().any(|v| id_matches(v, "gdm")));
        assert!(result.diagnostics.iter().any(|d| d.code == "AM-RULE-002"));
        assert!(result.conflicts.is_empty());
    }

    #[test]
    fn manual_dm_conflict_without_silent_mutation() {
        let result = resolve(
            &draft(&[
                ("compositor", &["gnome"]),
                ("dm", &["sddm"]),
                ("kernel", &["linux"]),
            ]),
            &fixture_catalog(),
            &rules(),
        );
        assert_eq!(
            result.effective_selections.get("dm"),
            Some(&vec!["sddm".to_string()])
        );
        assert!(result.conflicts.iter().any(|c| c.path == "/selections/dm"));
    }

    #[test]
    fn kernel_fs_conflict_end_to_end() {
        let result = resolve(
            &draft(&[("kernel", &["linux"]), ("fs", &["btrfs"])]),
            &fixture_catalog(),
            &rules(),
        );
        assert!(result
            .diagnostics
            .iter()
            .any(|d| d.code == "AM-RULE-007" && d.blocking));
    }

    #[test]
    fn unsatisfied_requires_becomes_conflict() {
        let catalog = fixture_catalog().with_option(option("zfs-module", &[], &["fs.zfs"]));
        let result = resolve(
            &draft(&[("kernel", &["linux"]), ("pkg", &["zfs-module"])]),
            &catalog,
            &rules(),
        );
        assert!(result
            .conflicts
            .iter()
            .any(|c| c.capability_id == Some("fs.zfs".to_string())));
        assert_eq!(
            result.required_capabilities,
            BTreeSet::from(["fs.zfs".to_string()])
        );
    }

    #[test]
    fn diagnostics_sorted_by_path_code_source() {
        let result = resolve(
            &draft(&[("compositor", &["gnome", "sway"])]),
            &fixture_catalog(),
            &rules(),
        );
        let keys: Vec<(&str, &str, &str)> = result
            .diagnostics
            .iter()
            .map(|d| (d.path.as_str(), d.code.as_str(), d.source.as_str()))
            .collect();
        let mut sorted = keys.clone();
        sorted.sort();
        assert_eq!(keys, sorted);
        assert!(result.diagnostics.iter().any(|d| d.code == "AM-RULE-001"));
        assert!(result.diagnostics.iter().any(|d| d.code == "AM-RULE-004"));
    }

    #[test]
    fn resolution_is_deterministic() {
        let draft = draft(&[
            ("compositor", &["sway", "gnome"]),
            ("kernel", &["linux"]),
            ("browser", &["firefox"]),
            ("fs", &["ext4"]),
        ]);
        let catalog = fixture_catalog();
        let first = resolve(&draft, &catalog, &rules());
        let second = resolve(&draft, &catalog, &rules());
        assert_eq!(first, second);
        assert_eq!(first.resolution_digest, second.resolution_digest);
        assert_eq!(first.resolution_digest.len(), 64);
    }

    #[test]
    fn digest_changes_with_input() {
        let catalog = fixture_catalog();
        let a = resolve(&draft(&[("kernel", &["linux"])]), &catalog, &rules());
        let b = resolve(&draft(&[("kernel", &["linux-lts"])]), &catalog, &rules());
        assert_ne!(a.resolution_digest, b.resolution_digest);
    }

    #[test]
    fn provided_capabilities_come_from_catalog() {
        let result = resolve(
            &draft(&[
                ("kernel", &["linux"]),
                ("browser", &["firefox"]),
                ("fs", &["ext4"]),
            ]),
            &fixture_catalog(),
            &rules(),
        );
        assert!(result.provided_capabilities.contains("fs.ext4"));
        assert!(result.provided_capabilities.contains("pkg.firefox"));
        assert!(!result.diagnostics.iter().any(|d| d.code == "AM-RULE-007"));
        assert!(!result.diagnostics.iter().any(|d| d.code == "AM-RULE-006"));
    }

    #[test]
    fn empty_rules_do_not_invent_canonical_set() {
        let result = resolve(
            &draft(&[("compositor", &["gnome", "sway"])]),
            &fixture_catalog(),
            &[],
        );
        assert!(result.diagnostics.is_empty());
    }
}
