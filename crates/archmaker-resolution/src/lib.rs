//! Crate `archmaker-resolution` — fase I5 (T-I5-02 del plan MVP-0).
//!
//! Resolución determinista de selecciones efectivas, capabilities y conflictos
//! (etapa 9 del pipeline) más diagnósticos de reglas (etapa 8). Sin shell,
//! sin filesystem, sin red: solo std más `archmaker-rules`.
//!
//! - [`resolve`] es pura y determinista: mismo `(draft, catalog)` → mismo
//!   [`ResolveResult`] byte a byte, incluido [`ResolveResult::resolution_digest`].
//! - Los diagnósticos se emiten ordenados por `(path, code, source)`
//!   (`diagnostic.schema.json#/$defs/diagnosticList`).
//! - El digest es FNV-1a de 64 bits sobre la serialización canónica interna;
//!   identidad estable para MVP (el `contentDigest` SHA-256 llega con I6/core).

use std::collections::{BTreeMap, BTreeSet};

use archmaker_rules::{canonical_rules, evaluate, rule_dm_001, Diagnostic, Draft, Severity};

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

/// FNV-1a de 64 bits, hex minúscula. Determinista entre procesos y
/// plataformas para la misma entrada.
fn fnv1a64_hex(input: &str) -> String {
    let mut hash: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in input.bytes() {
        hash ^= byte as u64;
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    format!("{hash:016x}")
}

fn digest_input(
    effective: &BTreeMap<String, Vec<String>>,
    provided: &BTreeSet<String>,
    required: &BTreeSet<String>,
    diagnostics: &[Diagnostic],
) -> String {
    let mut out = String::new();
    for (step, options) in effective {
        out.push_str(step);
        out.push('=');
        out.push_str(&options.join(","));
        out.push(';');
    }
    out.push('|');
    out.push_str(&provided.iter().cloned().collect::<Vec<_>>().join(","));
    out.push('|');
    out.push_str(&required.iter().cloned().collect::<Vec<_>>().join(","));
    out.push('|');
    out.push_str(
        &diagnostics
            .iter()
            .map(|d| d.code.as_str())
            .collect::<Vec<_>>()
            .join(","),
    );
    out
}

/// Resuelve el draft contra el catálogo.
///
/// 1. Congela selecciones efectivas (ordenadas por paso; opciones ordenadas).
/// 2. Calcula capabilities aportadas/requeridas desde el catálogo.
/// 3. Aplica la derivación RULE-DM-001 (`compositor == gnome` → `dm = gdm`)
///    solo si `dm` no está fijado manualmente; ante fijación manual distinta
///    registra conflicto sin mutación silenciosa.
/// 4. Evalúa las 7 reglas canónicas sobre la vista enriquecida.
/// 5. Registra conflictos por `requires` insatisfechos.
/// 6. Ordena diagnósticos por `(path, code, source)` y calcula el digest.
pub fn resolve(draft: &Draft, catalog: &Catalog) -> ResolveResult {
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

    // Derivación RULE-DM-001 (explicable y reversible, nunca silenciosa).
    let dm_rule = rule_dm_001();
    let dm_view = Draft {
        selections: effective.clone(),
        provided: provided.clone(),
    };
    if evaluate(&dm_rule, &dm_view).is_some() {
        match effective.get("dm") {
            None => {
                effective.insert("dm".to_string(), vec!["gdm".to_string()]);
                let recomputed = capabilities_of(&effective, catalog);
                provided = recomputed.0;
                required = recomputed.1;
            }
            Some(current) if current.iter().any(|v| v == "gdm" || v.ends_with(".gdm")) => {}
            Some(_) => {
                conflicts.push(Conflict {
                    severity: Severity::Error,
                    path: "/selections/dm".to_string(),
                    message_key: "diagnostic.dmDerived".to_string(),
                    capability_id: None,
                    suggestions: vec![
                        "Set dm to gdm or change the compositor selection.".to_string()
                    ],
                });
            }
        }
    }

    // Vista enriquecida para la etapa 8 (provides evaluado en la etapa 9).
    let enriched = Draft {
        selections: effective.clone(),
        provided: provided.clone(),
    };
    let diagnostics = evaluate_all_sorted(&enriched);

    // Conflictos por requires insatisfechos.
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

    let resolution_digest = fnv1a64_hex(&digest_input(
        &effective,
        &provided,
        &required,
        &diagnostics,
    ));

    ResolveResult {
        effective_selections: effective,
        provided_capabilities: provided,
        required_capabilities: required,
        conflicts,
        diagnostics,
        resolution_digest,
    }
}

fn evaluate_all_sorted(draft: &Draft) -> Vec<Diagnostic> {
    sorted_diagnostics(archmaker_rules::evaluate_all(&canonical_rules(), draft))
}

#[cfg(test)]
mod tests {
    use super::*;

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

    #[test]
    fn gnome_derives_gdm_with_info_diagnostic() {
        let result = resolve(
            &draft(&[("compositor", &["gnome"]), ("kernel", &["linux"])]),
            &fixture_catalog(),
        );
        assert_eq!(
            result.effective_selections.get("dm"),
            Some(&vec!["gdm".to_string()])
        );
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
        );
        assert_eq!(
            result.effective_selections.get("dm"),
            Some(&vec!["sddm".to_string()])
        );
        assert!(result.conflicts.iter().any(|c| c.path == "/selections/dm"));
    }

    #[test]
    fn kernel_fs_conflict_end_to_end() {
        // btrfs seleccionado pero el catálogo solo aporta fs.ext4.
        let result = resolve(
            &draft(&[("kernel", &["linux"]), ("fs", &["btrfs"])]),
            &fixture_catalog(),
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
        let first = resolve(&draft, &catalog);
        let second = resolve(&draft, &catalog);
        assert_eq!(first, second);
        assert_eq!(first.resolution_digest, second.resolution_digest);
    }

    #[test]
    fn digest_changes_with_input() {
        let catalog = fixture_catalog();
        let a = resolve(&draft(&[("kernel", &["linux"])]), &catalog);
        let b = resolve(&draft(&[("kernel", &["linux-lts"])]), &catalog);
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
        );
        assert!(result.provided_capabilities.contains("fs.ext4"));
        assert!(result.provided_capabilities.contains("pkg.firefox"));
        // fs.ext4 aportado ⇒ sin AM-RULE-007; pkg.firefox aportado ⇒ sin AM-RULE-006.
        assert!(!result.diagnostics.iter().any(|d| d.code == "AM-RULE-007"));
        assert!(!result.diagnostics.iter().any(|d| d.code == "AM-RULE-006"));
    }
}
