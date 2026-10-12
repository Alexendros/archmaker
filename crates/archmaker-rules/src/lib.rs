//! Crate `archmaker-rules` — fase I5 (T-I5-01, T-I5-03 del plan MVP-0).
//!
//! AST de operadores con vocabulario cerrado, evaluador puro y las 7 reglas
//! implementables del inventario. Sin shell, sin filesystem, sin red: solo std.
//!
//! Autoridades normativas:
//! - Vocabulario cerrado: `contracts/json-schema/rule.schema.json#/$defs.operator`
//! - Fichas de reglas: `docs/07-validation/rule-inventory.md` (8 fichas)
//! - Semántica por operador: `docs/07-validation/operator-spec.md`
//! - Códigos de diagnóstico: `docs/04-interfaces/errors-events.md` (`AM-RULE-001`..`AM-RULE-009`)
//!
//! Notas de alcance:
//! - `count_gt` es legacy (solo migración): nunca se ejecuta; se normaliza a
//!   `Gt(Count(..), n)` mediante [`normalize_count_gt`] (CON-002).
//! - `target_is` está diferido a v1: el parser emite `AM-RULE-009` y nunca lo ejecuta.
//! - `RULE-GPU-001` (`AM-RULE-003`) es no implementable en MVP: su efecto `change`
//!   heredado no declara `target` (ver [`RULE_GPU_001_STATUS`]).
//! - Un operador fuera del vocabulario produce `AM-RULE-009` bloqueante al parsear
//!   (nunca se evalúa como `false` silencioso).

mod parse;

pub use parse::{
    load_rules_by_ids, parse_rule_document, parse_rules_pack, UnsupportedOperatorError,
    CANONICAL_RULES_PACK_JSON,
};

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};

// ---------------------------------------------------------------------------
// Operadores y AST
// ---------------------------------------------------------------------------

/// Vocabulario ejecutable canónico de `$defs.operator` (MVP-2).
///
/// `Count` es un término (devuelve entero, no booleano) y por eso solo aparece
/// como [`Operand`], nunca como [`Condition`] directa. `Gt` es la forma
/// normalizada del legacy `count_gt` (nunca se ejecuta el token `count_gt`).
/// `target_is` no tiene variante: el parser lo rechaza con `AM-RULE-009`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Operator {
    Eq,
    Ne,
    In,
    Count,
    Gt,
    All,
    Any,
    Not,
    Required,
    Selected,
    Provides,
}

impl Operator {
    /// Token normativo del schema (`$defs.operator`).
    pub fn as_str(self) -> &'static str {
        match self {
            Operator::Eq => "eq",
            Operator::Ne => "ne",
            Operator::In => "in",
            Operator::Count => "count",
            Operator::Gt => "gt",
            Operator::All => "all",
            Operator::Any => "any",
            Operator::Not => "not",
            Operator::Required => "required",
            Operator::Selected => "selected",
            Operator::Provides => "provides",
        }
    }
}

/// Operando tipado (`rule.schema.json#/$defs/operand`).
///
/// No existe literal string sin tipar: las opciones se comparan como
/// referencias de entidad (`vendor.kind.id`, DEC-002) y los literales como
/// `boolean`/`integer`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Operand {
    EntityRef(String),
    Bool(bool),
    Int(i64),
    Count(String),
}

/// Condición booleana cerrada (`rule.schema.json#/$defs/condition`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Condition {
    Eq(Operand, Operand),
    Ne(Operand, Operand),
    In(Operand, Vec<Operand>),
    Gt(Operand, Operand),
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Box<Condition>),
    Required(String),
    Selected(String),
    Provides(String),
}

/// Normaliza el operador legacy `count_gt(Ref, n)` a la forma ejecutable
/// `Gt(Count(Ref), n)`. El runtime nuevo nunca ejecuta `count_gt` (CON-002).
pub fn normalize_count_gt(reference: &str, threshold: i64) -> Condition {
    Condition::Gt(
        Operand::Count(reference.to_string()),
        Operand::Int(threshold),
    )
}

// ---------------------------------------------------------------------------
// Efectos, severidad y diagnóstico
// ---------------------------------------------------------------------------

/// Efecto aplicado cuando la condición se cumple (`rule.schema.json#/$defs/action`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Effect {
    Block,
    Suggest,
    Derive { target: String, value: String },
}

/// Severidad canónica (`common.schema.json#/$defs/severity`).
/// `fatal`/`debug` están reservados y el MVP no los emite.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Error,
    Warning,
    Info,
}

impl Severity {
    pub fn as_str(self) -> &'static str {
        match self {
            Severity::Error => "error",
            Severity::Warning => "warning",
            Severity::Info => "info",
        }
    }

    /// Canónico: `error` ⇒ bloqueante; `warning`/`info` ⇒ no bloqueante.
    pub fn is_blocking(self) -> bool {
        matches!(self, Severity::Error)
    }
}

/// Diagnóstico tipado (`diagnostic.schema.json`).
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Diagnostic {
    pub code: String,
    pub severity: Severity,
    pub blocking: bool,
    pub path: String,
    pub message_key: String,
    pub source: String,
    pub rule_id: String,
    pub suggestions: Vec<String>,
}

/// Componente emisor de los diagnósticos de este crate.
pub const SOURCE: &str = "archmaker-rules";

/// Regla declarativa: condición + efecto + código canónico.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Rule {
    pub id: String,
    pub condition: Condition,
    pub effect: Effect,
    pub code: String,
}

impl Rule {
    pub fn new(
        id: impl Into<String>,
        condition: Condition,
        effect: Effect,
        code: impl Into<String>,
    ) -> Self {
        Self {
            id: id.into(),
            condition,
            effect,
            code: code.into(),
        }
    }
}

// ---------------------------------------------------------------------------
// Draft mínimo (vista de evaluación, etapa 8 del pipeline)
// ---------------------------------------------------------------------------

/// Vista mínima del draft necesaria para evaluar reglas.
///
/// - `selections`: paso/ValueRef → ids de opción seleccionadas.
///   Claves canónicas usadas por las reglas MVP: `compositor`, `dm`,
///   `kernel`, `browser`, `pkg`, `fs`.
/// - `provided`: capabilities aportadas por la resolución (etapa 9); el
///   resolutor la calcula desde el catálogo antes de evaluar.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Draft {
    pub selections: BTreeMap<String, Vec<String>>,
    pub provided: BTreeSet<String>,
}

impl Draft {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_selection(mut self, step: impl Into<String>, options: Vec<String>) -> Self {
        self.selections.insert(step.into(), options);
        self
    }

    pub fn with_provided(mut self, capabilities: Vec<String>) -> Self {
        self.provided = capabilities.into_iter().collect();
        self
    }

    /// Todas las opciones seleccionadas (aplanado, con duplicados preservados
    /// para que RULE-DUP-001 pueda detectarlos).
    pub fn all_selected(&self) -> Vec<&String> {
        self.selections.values().flatten().collect()
    }

    /// `count(Ref)`: número de selecciones efectivas de la referencia.
    /// Si `reference` es una clave de paso, cardinalidad del paso; si no,
    /// ocurrencias de esa identidad de opción en todas las selecciones.
    pub fn count_ref(&self, reference: &str) -> i64 {
        if let Some(options) = self.selections.get(reference) {
            return options.len() as i64;
        }
        if let Some(options) = self
            .selections
            .iter()
            .find(|(step, _)| id_matches(step, reference))
            .map(|(_, options)| options)
        {
            return options.len() as i64;
        }
        self.all_selected()
            .iter()
            .filter(|id| id_matches(id, reference))
            .count() as i64
    }

    /// `selected(Id)`: verdadero sii la opción está seleccionada.
    pub fn is_selected(&self, id: &str) -> bool {
        self.all_selected().iter().any(|s| id_matches(s, id))
    }

    /// Opciones del grupo de paso `group` (coincidencia exacta o por sufijo).
    pub fn group_options(&self, group: &str) -> Vec<String> {
        self.selections
            .iter()
            .find(|(step, _)| *step == group || id_matches(step, group))
            .map(|(_, options)| options.clone())
            .unwrap_or_default()
    }
}

/// Igualdad de identificadores con tolerancia de cualificación:
/// `gnome` casa con `archmaker.compositor.gnome` y viceversa.
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

/// Segmento corto de un id (`archmaker.pkg.firefox` → `firefox`).
fn short_id(id: &str) -> &str {
    id.rsplit('.').next().unwrap_or(id)
}

// ---------------------------------------------------------------------------
// Evaluación
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Str(String),
    Int(i64),
    Bool(bool),
}

fn resolve_operand(operand: &Operand, draft: &Draft) -> Option<Value> {
    match operand {
        Operand::Bool(b) => Some(Value::Bool(*b)),
        Operand::Int(n) => Some(Value::Int(*n)),
        Operand::Count(reference) => Some(Value::Int(draft.count_ref(reference))),
        Operand::EntityRef(reference) => {
            if let Some(options) = draft
                .selections
                .iter()
                .find(|(step, _)| *step == reference || id_matches(step, reference))
                .map(|(_, options)| options)
            {
                if options.len() == 1 {
                    return Some(Value::Str(options[0].clone()));
                }
                return None;
            }
            Some(Value::Str(reference.clone()))
        }
    }
}

fn values_equal(left: &Value, right: &Value) -> bool {
    match (left, right) {
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Str(a), Value::Str(b)) => id_matches(a, b),
        _ => false,
    }
}

/// Evalúa una condición cerrada sobre el draft. Pares heterogéneos en
/// `Eq`/`Ne`/`Gt`/`In` se evalúan como `false` (nunca lanzan).
pub fn eval_condition(condition: &Condition, draft: &Draft) -> bool {
    match condition {
        Condition::Eq(left, right) => {
            match (resolve_operand(left, draft), resolve_operand(right, draft)) {
                (Some(l), Some(r)) => values_equal(&l, &r),
                _ => false,
            }
        }
        Condition::Ne(left, right) => {
            match (resolve_operand(left, draft), resolve_operand(right, draft)) {
                (Some(l), Some(r)) => !values_equal(&l, &r),
                _ => false,
            }
        }
        Condition::In(needle, haystack) => match resolve_operand(needle, draft) {
            Some(value) => haystack.iter().any(|candidate| {
                resolve_operand(candidate, draft).is_some_and(|other| values_equal(&value, &other))
            }),
            None => false,
        },
        Condition::Gt(left, right) => {
            match (resolve_operand(left, draft), resolve_operand(right, draft)) {
                (Some(Value::Int(a)), Some(Value::Int(b))) => a > b,
                _ => false,
            }
        }
        Condition::All(items) => items.iter().all(|c| eval_condition(c, draft)),
        Condition::Any(items) => items.iter().any(|c| eval_condition(c, draft)),
        Condition::Not(inner) => !eval_condition(inner, draft),
        Condition::Required(reference) => draft.count_ref(reference) >= 1,
        Condition::Selected(id) => draft.is_selected(id),
        Condition::Provides(capability) => draft.provided.contains(capability),
    }
}

// ---------------------------------------------------------------------------
// Catálogo de reglas (rule-inventory.md)
// ---------------------------------------------------------------------------

pub const RULE_COMP_001: &str = "RULE-COMP-001";
pub const RULE_DM_001: &str = "RULE-DM-001";
pub const RULE_GPU_001: &str = "RULE-GPU-001";
pub const RULE_KERNEL_001: &str = "RULE-KERNEL-001";
pub const RULE_BROWSER_001: &str = "RULE-BROWSER-001";
pub const RULE_PKG_001: &str = "RULE-PKG-001";
pub const RULE_KERNEL_002: &str = "RULE-KERNEL-002";
pub const RULE_DUP_001: &str = "RULE-DUP-001";

/// RULE-GPU-001 es no implementable en MVP: la condición es expresable
/// (`all([eq(gpu,nvidia), eq(compositor,sway)])`) pero el efecto `change`
/// heredado no declara `target`; falta evidencia primaria (SRC-002/SRC-003).
pub const RULE_GPU_001_STATUS: &str =
    "not-implementable: change effect without target; evidence-required (SRC-002/SRC-003)";

/// Verdadero sii la regla es implementable en MVP.
pub fn is_implementable(rule_id: &str) -> bool {
    rule_id != RULE_GPU_001
}

/// RULE-COMP-001: compositor único. `count_gt` legacy normalizado.
pub fn rule_comp_001() -> Rule {
    Rule::new(
        RULE_COMP_001,
        normalize_count_gt("archmaker.environment.compositor", 1),
        Effect::Block,
        "AM-RULE-001",
    )
}

/// RULE-DM-001: derivación GNOME→GDM (explicable y reversible).
pub fn rule_dm_001() -> Rule {
    Rule::new(
        RULE_DM_001,
        Condition::Eq(
            Operand::EntityRef("archmaker.environment.compositor".to_string()),
            Operand::EntityRef("archmaker.compositor.gnome".to_string()),
        ),
        Effect::Derive {
            target: "archmaker.environment.dm".to_string(),
            value: "archmaker.dm.gdm".to_string(),
        },
        "AM-RULE-002",
    )
}

/// RULE-KERNEL-001: kernel obligatorio (`not(required(kernel))` → block).
pub fn rule_kernel_001() -> Rule {
    Rule::new(
        RULE_KERNEL_001,
        Condition::Not(Box::new(Condition::Required(
            "archmaker.base.kernel".to_string(),
        ))),
        Effect::Block,
        "AM-RULE-004",
    )
}

/// RULE-BROWSER-001: navegador sugerido (`eq(count(browser),0)` → suggest).
pub fn rule_browser_001() -> Rule {
    Rule::new(
        RULE_BROWSER_001,
        Condition::Eq(
            Operand::Count("archmaker.apps.browser".to_string()),
            Operand::Int(0),
        ),
        Effect::Suggest,
        "AM-RULE-005",
    )
}

/// RULE-PKG-001: disponibilidad de paquetes; [`evaluate`] expande por elemento.
pub fn rule_pkg_001() -> Rule {
    Rule::new(
        RULE_PKG_001,
        Condition::All(vec![
            Condition::Selected("archmaker.apps.apps-core".to_string()),
            Condition::Not(Box::new(Condition::Provides("pkg.placeholder".to_string()))),
        ]),
        Effect::Suggest,
        "AM-RULE-006",
    )
}

/// RULE-KERNEL-002: conflicto kernel/filesystem; [`evaluate`] expande por `fs`.
pub fn rule_kernel_002() -> Rule {
    Rule::new(
        RULE_KERNEL_002,
        Condition::All(vec![
            Condition::Selected("archmaker.base.filesystem".to_string()),
            Condition::Not(Box::new(Condition::Provides("fs.placeholder".to_string()))),
        ]),
        Effect::Block,
        "AM-RULE-007",
    )
}

/// RULE-DUP-001: identidad duplicada; el runtime comprueba toda identidad.
pub fn rule_dup_001() -> Rule {
    Rule::new(
        RULE_DUP_001,
        normalize_count_gt("archmaker.pkg.package", 1),
        Effect::Block,
        "AM-RULE-008",
    )
}

/// Las 7 reglas implementables en MVP.
pub fn canonical_rules() -> Vec<Rule> {
    vec![
        rule_comp_001(),
        rule_dm_001(),
        rule_kernel_001(),
        rule_browser_001(),
        rule_pkg_001(),
        rule_kernel_002(),
        rule_dup_001(),
    ]
}

/// (código, path, remedio) por id de regla.
pub fn rule_info(rule_id: &str) -> Option<(&'static str, &'static str, &'static str)> {
    match rule_id {
        RULE_COMP_001 => Some((
            "AM-RULE-001",
            "/selections/compositor",
            "Reduce compositor to a single selection.",
        )),
        RULE_DM_001 => Some((
            "AM-RULE-002",
            "/selections/dm",
            "Pin dm to gdm in an explainable, reversible way.",
        )),
        RULE_KERNEL_001 => Some((
            "AM-RULE-004",
            "/selections/kernel",
            "Select exactly one kernel.",
        )),
        RULE_BROWSER_001 => Some((
            "AM-RULE-005",
            "/selections/browser",
            "Suggest a browser; the user may continue without one.",
        )),
        RULE_PKG_001 => Some((
            "AM-RULE-006",
            "/selections/pkg",
            "Availability cannot be verified; no compatibility is asserted.",
        )),
        RULE_KERNEL_002 => Some((
            "AM-RULE-007",
            "/selections/fs",
            "Choose a filesystem provided by the selected kernel, or change kernel.",
        )),
        RULE_DUP_001 => Some((
            "AM-RULE-008",
            "/selections",
            "Unify the duplicated definition under the global vendor.kind.id identity (DEC-002).",
        )),
        _ => None,
    }
}

/// (severidad, bloqueante, messageKey) por código (fuente: errors-events.md).
pub fn code_info(code: &str) -> (Severity, bool, &'static str) {
    match code {
        "AM-RULE-001" => (Severity::Error, true, "diagnostic.compositorCardinality"),
        "AM-RULE-002" => (Severity::Info, false, "diagnostic.dmDerived"),
        "AM-RULE-003" => (Severity::Error, true, "diagnostic.gpuCompositorConflict"),
        "AM-RULE-004" => (Severity::Error, true, "diagnostic.kernelCardinality"),
        "AM-RULE-005" => (Severity::Warning, false, "diagnostic.browserMissing"),
        "AM-RULE-006" => (Severity::Warning, false, "diagnostic.packageUnverified"),
        "AM-RULE-007" => (Severity::Error, true, "diagnostic.kernelFilesystemConflict"),
        "AM-RULE-008" => (Severity::Error, true, "diagnostic.duplicateIdentity"),
        "AM-RULE-009" => (Severity::Error, true, "diagnostic.operatorUnsupported"),
        _ => (Severity::Error, true, "diagnostic.operatorUnsupported"),
    }
}

fn diagnostic_for(rule: &Rule) -> Diagnostic {
    let (severity, blocking, message_key) = code_info(&rule.code);
    let path = rule_info(&rule.id)
        .map(|(_, path, _)| path.to_string())
        .unwrap_or_default();
    let suggestions = rule_info(&rule.id)
        .map(|(_, _, remedy)| vec![remedy.to_string()])
        .unwrap_or_default();
    Diagnostic {
        code: rule.code.clone(),
        severity,
        blocking,
        path,
        message_key: message_key.to_string(),
        source: SOURCE.to_string(),
        rule_id: rule.id.clone(),
        suggestions,
    }
}

fn group_options_any(draft: &Draft, keys: &[&str]) -> Vec<String> {
    for key in keys {
        let options = draft.group_options(key);
        if !options.is_empty() {
            return options;
        }
    }
    Vec::new()
}

/// RULE-PKG-001 expandida: dispara si algún paquete del grupo apps/pkg no tiene
/// su capability `pkg.<id>` en lo aportado.
fn eval_pkg_availability(draft: &Draft) -> bool {
    group_options_any(draft, &["pkg", "archmaker.apps.apps-core", "apps-core"])
        .iter()
        .any(|option| {
            let capability = format!("pkg.{}", short_id(option));
            !draft.provided.contains(&capability)
        })
}

/// RULE-KERNEL-002 expandida: dispara si algún `fs` seleccionado no tiene su
/// capability `fs.<id>` en lo aportado por el kernel.
fn eval_kernel_fs_conflict(draft: &Draft) -> bool {
    group_options_any(draft, &["fs", "archmaker.base.filesystem", "filesystem"])
        .iter()
        .any(|option| {
            let capability = format!("fs.{}", short_id(option));
            !draft.provided.contains(&capability)
        })
}

/// RULE-DUP-001 expandida: dispara si alguna identidad de opción aparece más
/// de una vez en las selecciones (CON-004, DEC-002).
fn eval_duplicate_identity(draft: &Draft) -> bool {
    let mut seen = BTreeSet::new();
    for id in draft.all_selected() {
        if !seen.insert(id.clone()) {
            return true;
        }
    }
    false
}

/// Evalúa una regla sobre el draft. `Some(Diagnostic)` si la condición se
/// cumple (la regla dispara); `None` en caso contrario. Función pura y
/// determinista: mismo input → mismo output.
pub fn evaluate(rule: &Rule, draft: &Draft) -> Option<Diagnostic> {
    let fired = match rule.id.as_str() {
        RULE_PKG_001 => eval_pkg_availability(draft),
        RULE_KERNEL_002 => eval_kernel_fs_conflict(draft),
        RULE_DUP_001 => eval_duplicate_identity(draft),
        _ => eval_condition(&rule.condition, draft),
    };
    fired.then(|| diagnostic_for(rule))
}

/// Evalúa todas las reglas dadas y devuelve los diagnósticos emitidos.
pub fn evaluate_all(rules: &[Rule], draft: &Draft) -> Vec<Diagnostic> {
    rules.iter().filter_map(|r| evaluate(r, draft)).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn draft_with(step: &str, options: &[&str]) -> Draft {
        Draft::new().with_selection(step, options.iter().map(|s| s.to_string()).collect())
    }

    #[test]
    fn comp_001_single_compositor_passes() {
        assert!(evaluate(&rule_comp_001(), &draft_with("compositor", &["gnome"])).is_none());
    }

    #[test]
    fn comp_001_two_compositors_block() {
        let diag = evaluate(
            &rule_comp_001(),
            &draft_with("compositor", &["gnome", "sway"]),
        )
        .expect("must fire");
        assert_eq!(diag.code, "AM-RULE-001");
        assert!(diag.blocking);
        assert_eq!(diag.severity, Severity::Error);
    }

    #[test]
    fn dm_001_gnome_derives_gdm_info() {
        let diag =
            evaluate(&rule_dm_001(), &draft_with("compositor", &["gnome"])).expect("must fire");
        assert_eq!(diag.code, "AM-RULE-002");
        assert_eq!(diag.severity, Severity::Info);
        assert!(!diag.blocking);
    }

    #[test]
    fn dm_001_non_gnome_quiet() {
        assert!(evaluate(&rule_dm_001(), &draft_with("compositor", &["sway"])).is_none());
    }

    #[test]
    fn kernel_001_present_passes() {
        assert!(evaluate(&rule_kernel_001(), &draft_with("kernel", &["linux"])).is_none());
    }

    #[test]
    fn kernel_001_absent_blocks() {
        let diag = evaluate(&rule_kernel_001(), &Draft::new()).expect("must fire");
        assert_eq!(diag.code, "AM-RULE-004");
        assert!(diag.blocking);
    }

    #[test]
    fn browser_001_present_passes() {
        assert!(evaluate(&rule_browser_001(), &draft_with("browser", &["firefox"])).is_none());
    }

    #[test]
    fn browser_001_absent_suggests() {
        let diag = evaluate(&rule_browser_001(), &Draft::new()).expect("must fire");
        assert_eq!(diag.code, "AM-RULE-005");
        assert_eq!(diag.severity, Severity::Warning);
        assert!(!diag.blocking);
    }

    #[test]
    fn pkg_001_provided_passes() {
        let draft = draft_with("pkg", &["firefox"]).with_provided(vec!["pkg.firefox".to_string()]);
        assert!(evaluate(&rule_pkg_001(), &draft).is_none());
    }

    #[test]
    fn pkg_001_unprovided_warns() {
        let diag = evaluate(&rule_pkg_001(), &draft_with("pkg", &["firefox"])).expect("must fire");
        assert_eq!(diag.code, "AM-RULE-006");
        assert!(!diag.blocking);
    }

    #[test]
    fn pkg_001_empty_group_quiet() {
        assert!(evaluate(&rule_pkg_001(), &Draft::new()).is_none());
    }

    #[test]
    fn kernel_002_provided_fs_passes() {
        let draft = draft_with("fs", &["ext4"]).with_provided(vec!["fs.ext4".to_string()]);
        assert!(evaluate(&rule_kernel_002(), &draft).is_none());
    }

    #[test]
    fn kernel_002_unprovided_fs_blocks() {
        let diag = evaluate(&rule_kernel_002(), &draft_with("fs", &["btrfs"])).expect("must fire");
        assert_eq!(diag.code, "AM-RULE-007");
        assert!(diag.blocking);
    }

    #[test]
    fn dup_001_unique_passes() {
        let draft = draft_with("pkg", &["base-devel", "firefox"]);
        assert!(evaluate(&rule_dup_001(), &draft).is_none());
    }

    #[test]
    fn dup_001_base_devel_twice_blocks() {
        let draft = draft_with("pkg", &["base-devel", "base-devel"]);
        let diag = evaluate(&rule_dup_001(), &draft).expect("must fire");
        assert_eq!(diag.code, "AM-RULE-008");
        assert!(diag.blocking);
    }

    #[test]
    fn normalize_count_gt_rewrites_legacy_operator() {
        let cond = normalize_count_gt("compositor", 1);
        assert_eq!(
            cond,
            Condition::Gt(Operand::Count("compositor".to_string()), Operand::Int(1))
        );
        assert!(eval_condition(
            &cond,
            &draft_with("compositor", &["a", "b"])
        ));
        assert!(!eval_condition(&cond, &draft_with("compositor", &["a"])));
    }

    #[test]
    fn heterogeneous_eq_is_false_never_panics() {
        let cond = Condition::Eq(Operand::Int(1), Operand::Bool(true));
        assert!(!eval_condition(&cond, &Draft::new()));
        let gt = Condition::Gt(
            Operand::EntityRef("missing-step".to_string()),
            Operand::Int(0),
        );
        assert!(!eval_condition(&gt, &Draft::new()));
    }

    #[test]
    fn gpu_001_is_not_implementable() {
        assert!(!is_implementable(RULE_GPU_001));
        assert!(RULE_GPU_001_STATUS.contains("SRC-002"));
        for rule in canonical_rules() {
            assert!(is_implementable(&rule.id), "{}", rule.id);
        }
        assert_eq!(canonical_rules().len(), 7);
    }

    #[test]
    fn operator_vocabulary_tokens() {
        let tokens: Vec<&str> = [
            Operator::Eq,
            Operator::Ne,
            Operator::In,
            Operator::Count,
            Operator::Gt,
            Operator::All,
            Operator::Any,
            Operator::Not,
            Operator::Required,
            Operator::Selected,
            Operator::Provides,
        ]
        .iter()
        .map(|op| op.as_str())
        .collect();
        assert_eq!(
            tokens,
            vec![
                "eq", "ne", "in", "count", "gt", "all", "any", "not", "required", "selected",
                "provides"
            ]
        );
    }

    #[test]
    fn any_ne_in_operators_evaluate() {
        let draft = draft_with("compositor", &["gnome"]);
        assert!(eval_condition(
            &Condition::Any(vec![
                Condition::Eq(
                    Operand::EntityRef("compositor".into()),
                    Operand::EntityRef("sway".into()),
                ),
                Condition::Eq(
                    Operand::EntityRef("compositor".into()),
                    Operand::EntityRef("gnome".into()),
                ),
            ]),
            &draft
        ));
        assert!(eval_condition(
            &Condition::Ne(
                Operand::EntityRef("compositor".into()),
                Operand::EntityRef("sway".into()),
            ),
            &draft
        ));
        assert!(eval_condition(
            &Condition::In(
                Operand::EntityRef("compositor".into()),
                vec![
                    Operand::EntityRef("sway".into()),
                    Operand::EntityRef("gnome".into()),
                ],
            ),
            &draft
        ));
    }

    #[test]
    fn evaluation_is_deterministic() {
        let draft = draft_with("compositor", &["gnome", "sway"]);
        let rule = rule_comp_001();
        assert_eq!(evaluate(&rule, &draft), evaluate(&rule, &draft));
    }

    #[test]
    fn catalog_pack_matches_oracle_canonical_rules() {
        let loaded = load_rules_by_ids(
            CANONICAL_RULES_PACK_JSON,
            &[
                RULE_COMP_001,
                RULE_DM_001,
                RULE_KERNEL_001,
                RULE_BROWSER_001,
                RULE_PKG_001,
                RULE_KERNEL_002,
                RULE_DUP_001,
            ],
        )
        .expect("pack loads");
        let oracle = canonical_rules();
        assert_eq!(loaded.len(), oracle.len());
        for (sheet, fixed) in loaded.iter().zip(oracle.iter()) {
            assert_eq!(sheet.id, fixed.id);
            assert_eq!(sheet.code, fixed.code);
            assert_eq!(sheet.effect, fixed.effect);
            let draft = draft_with("compositor", &["gnome", "sway"]);
            assert_eq!(
                evaluate(sheet, &draft).map(|d| d.code),
                evaluate(fixed, &draft).map(|d| d.code),
                "oracle mismatch for {}",
                sheet.id
            );
        }
    }

    #[test]
    fn unknown_and_target_is_emit_am_rule_009() {
        let unknown = serde_json::json!({
            "id": "RULE-TEST-001",
            "documentVersion": "vNext",
            "schemaVersion": "1",
            "when": { "bogus": true },
            "action": "block",
            "diagnosticCode": "AM-RULE-001"
        });
        let err = parse_rule_document(&unknown).expect_err("unknown op");
        assert_eq!(err.code, "AM-RULE-009");
        let target_is = serde_json::json!({
            "id": "RULE-TEST-002",
            "documentVersion": "vNext",
            "schemaVersion": "1",
            "when": { "target_is": "archinstall" },
            "action": "block",
            "diagnosticCode": "AM-RULE-001"
        });
        let err = parse_rule_document(&target_is).expect_err("target_is deferred");
        assert_eq!(err.code, "AM-RULE-009");
    }

    #[test]
    fn count_gt_normalizes_and_never_stays_raw() {
        let doc = serde_json::json!({
            "id": "RULE-COMP-001",
            "documentVersion": "vNext",
            "schemaVersion": "1",
            "when": { "count_gt": ["archmaker.environment.compositor", 1] },
            "action": "block",
            "diagnosticCode": "AM-RULE-001"
        });
        let rule = parse_rule_document(&doc).expect("legacy normalizes");
        assert_eq!(
            rule.condition,
            Condition::Gt(
                Operand::Count("archmaker.environment.compositor".into()),
                Operand::Int(1)
            )
        );
    }
}
