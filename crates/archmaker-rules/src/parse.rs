//! Parseo de documentos `Rule` (rule.schema.json) a AST ejecutable.
//!
//! - `count_gt` se normaliza a `Gt(Count(..), n)` y nunca queda como token crudo.
//! - `target_is` y cualquier operador fuera del vocabulario ejecutable producen
//!   [`UnsupportedOperatorError`] con código `AM-RULE-009` bloqueante.

use serde_json::Value;

use crate::{
    code_info, normalize_count_gt, Condition, Diagnostic, Effect, Operand, Rule, Severity, SOURCE,
};

/// Pack canónico de las 7 reglas implementables (oráculo MVP-2).
pub const CANONICAL_RULES_PACK_JSON: &str = include_str!("../fixtures/canonical-rules.json");

/// Error tipado al rechazar un operador no ejecutable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnsupportedOperatorError {
    pub code: &'static str,
    pub operator: String,
    pub path: String,
}

impl UnsupportedOperatorError {
    pub fn am_rule_009(operator: impl Into<String>, path: impl Into<String>) -> Self {
        Self {
            code: "AM-RULE-009",
            operator: operator.into(),
            path: path.into(),
        }
    }

    pub fn into_diagnostic(self) -> Diagnostic {
        let (severity, blocking, message_key) = code_info(self.code);
        Diagnostic {
            code: self.code.to_string(),
            severity,
            blocking,
            path: self.path,
            message_key: message_key.to_string(),
            source: SOURCE.to_string(),
            rule_id: String::new(),
            suggestions: vec![format!(
                "Operator '{}' is outside the executable vocabulary.",
                self.operator
            )],
        }
    }
}

type ParseResult<T> = Result<T, UnsupportedOperatorError>;

/// Parsea un documento Rule JSON. Rechaza operadores desconocidos y `target_is`.
pub fn parse_rule_document(value: &Value) -> ParseResult<Rule> {
    let obj = value
        .as_object()
        .ok_or_else(|| UnsupportedOperatorError::am_rule_009("document", "/"))?;
    let id = obj
        .get("id")
        .and_then(Value::as_str)
        .ok_or_else(|| UnsupportedOperatorError::am_rule_009("id", "/id"))?
        .to_string();
    let action = obj
        .get("action")
        .and_then(Value::as_str)
        .ok_or_else(|| UnsupportedOperatorError::am_rule_009("action", "/action"))?;
    let when = obj
        .get("when")
        .ok_or_else(|| UnsupportedOperatorError::am_rule_009("when", "/when"))?;
    let condition = parse_condition(when, "/when")?;
    let code = obj
        .get("diagnosticCode")
        .and_then(Value::as_str)
        .unwrap_or("AM-RULE-009")
        .to_string();
    let effect = parse_effect(action, obj)?;
    Ok(Rule::new(id, condition, effect, code))
}

/// Parsea un array JSON de documentos Rule.
pub fn parse_rules_pack(bytes: &str) -> ParseResult<Vec<Rule>> {
    let value: Value = serde_json::from_str(bytes)
        .map_err(|_| UnsupportedOperatorError::am_rule_009("pack", "/"))?;
    let items = value
        .as_array()
        .ok_or_else(|| UnsupportedOperatorError::am_rule_009("pack", "/"))?;
    let mut rules = Vec::with_capacity(items.len());
    for (index, item) in items.iter().enumerate() {
        let mut rule = parse_rule_document(item).map_err(|mut err| {
            err.path = format!("/{index}{}", err.path);
            err
        })?;
        // Preserve severity-derived defaults when diagnostic code maps known severities.
        let (severity, _, _) = code_info(&rule.code);
        if matches!(severity, Severity::Error) && matches!(rule.effect, Effect::Suggest) {
            // Keep declared effect; inventory is authoritative for action.
        }
        let _ = &mut rule;
        rules.push(rule);
    }
    Ok(rules)
}

/// Carga del pack solo las reglas cuyos ids aparecen en `wanted`, en ese orden.
/// No inventa reglas ausentes: un id pedido y no encontrado es error de ficha.
pub fn load_rules_by_ids(pack_json: &str, wanted: &[&str]) -> ParseResult<Vec<Rule>> {
    let pack = parse_rules_pack(pack_json)?;
    let mut out = Vec::with_capacity(wanted.len());
    for id in wanted {
        let Some(rule) = pack.iter().find(|rule| rule.id == *id).cloned() else {
            return Err(UnsupportedOperatorError::am_rule_009(
                format!("missing:{id}"),
                "/rules",
            ));
        };
        out.push(rule);
    }
    Ok(out)
}

fn parse_effect(action: &str, obj: &serde_json::Map<String, Value>) -> ParseResult<Effect> {
    match action {
        "block" => Ok(Effect::Block),
        "suggest" => Ok(Effect::Suggest),
        "derive" | "change" => {
            let target = obj
                .get("target")
                .and_then(Value::as_str)
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009(action, "/target"))?
                .to_string();
            let value = obj
                .get("allowed")
                .and_then(Value::as_array)
                .and_then(|items| items.first())
                .and_then(Value::as_str)
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009(action, "/allowed"))?
                .to_string();
            Ok(Effect::Derive { target, value })
        }
        "lock" => {
            // MVP-2: lock se modela como derive sin valor mutado; se rechaza sin target.
            Err(UnsupportedOperatorError::am_rule_009("lock", "/action"))
        }
        other => Err(UnsupportedOperatorError::am_rule_009(other, "/action")),
    }
}

fn parse_condition(value: &Value, path: &str) -> ParseResult<Condition> {
    let obj = value
        .as_object()
        .ok_or_else(|| UnsupportedOperatorError::am_rule_009("condition", path))?;
    if obj.len() != 1 {
        let key = obj.keys().next().cloned().unwrap_or_else(|| "empty".into());
        return Err(UnsupportedOperatorError::am_rule_009(key, path));
    }
    let (op, payload) = obj.iter().next().expect("len == 1");
    match op.as_str() {
        "all" => Ok(Condition::All(parse_condition_list(payload, path)?)),
        "any" => Ok(Condition::Any(parse_condition_list(payload, path)?)),
        "not" => Ok(Condition::Not(Box::new(parse_condition(
            payload,
            &format!("{path}/not"),
        )?))),
        "eq" => {
            let (left, right) = parse_operand_pair(payload, path)?;
            Ok(Condition::Eq(left, right))
        }
        "ne" => {
            let (left, right) = parse_operand_pair(payload, path)?;
            Ok(Condition::Ne(left, right))
        }
        "in" => {
            let items = payload
                .as_array()
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009("in", path))?;
            if items.len() != 2 {
                return Err(UnsupportedOperatorError::am_rule_009("in", path));
            }
            let needle = parse_operand(&items[0], &format!("{path}/in/0"))?;
            let haystack = items[1]
                .as_array()
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009("in", path))?
                .iter()
                .enumerate()
                .map(|(i, item)| parse_operand(item, &format!("{path}/in/1/{i}")))
                .collect::<ParseResult<Vec<_>>>()?;
            if haystack.is_empty() {
                return Err(UnsupportedOperatorError::am_rule_009("in", path));
            }
            Ok(Condition::In(needle, haystack))
        }
        "required" => {
            let reference = payload
                .as_str()
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009("required", path))?
                .to_string();
            Ok(Condition::Required(reference))
        }
        "selected" => {
            let id = payload
                .as_str()
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009("selected", path))?
                .to_string();
            Ok(Condition::Selected(id))
        }
        "provides" => {
            let capability = payload
                .as_str()
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009("provides", path))?
                .to_string();
            Ok(Condition::Provides(capability))
        }
        "count_gt" => {
            let items = payload
                .as_array()
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009("count_gt", path))?;
            if items.len() != 2 {
                return Err(UnsupportedOperatorError::am_rule_009("count_gt", path));
            }
            let reference = items[0]
                .as_str()
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009("count_gt", path))?;
            let threshold = items[1]
                .as_i64()
                .ok_or_else(|| UnsupportedOperatorError::am_rule_009("count_gt", path))?;
            Ok(normalize_count_gt(reference, threshold))
        }
        "count" => Err(UnsupportedOperatorError::am_rule_009("count", path)), // count is a term, not a boolean condition root
        "target_is" => Err(UnsupportedOperatorError::am_rule_009("target_is", path)),
        other => Err(UnsupportedOperatorError::am_rule_009(other, path)),
    }
}

fn parse_condition_list(value: &Value, path: &str) -> ParseResult<Vec<Condition>> {
    let items = value
        .as_array()
        .ok_or_else(|| UnsupportedOperatorError::am_rule_009("list", path))?;
    if items.is_empty() {
        return Err(UnsupportedOperatorError::am_rule_009("list", path));
    }
    items
        .iter()
        .enumerate()
        .map(|(i, item)| parse_condition(item, &format!("{path}/{i}")))
        .collect()
}

fn parse_operand_pair(value: &Value, path: &str) -> ParseResult<(Operand, Operand)> {
    let items = value
        .as_array()
        .ok_or_else(|| UnsupportedOperatorError::am_rule_009("pair", path))?;
    if items.len() != 2 {
        return Err(UnsupportedOperatorError::am_rule_009("pair", path));
    }
    Ok((
        parse_operand(&items[0], &format!("{path}/0"))?,
        parse_operand(&items[1], &format!("{path}/1"))?,
    ))
}

fn parse_operand(value: &Value, path: &str) -> ParseResult<Operand> {
    if let Some(flag) = value.as_bool() {
        return Ok(Operand::Bool(flag));
    }
    if let Some(number) = value.as_i64() {
        return Ok(Operand::Int(number));
    }
    if let Some(text) = value.as_str() {
        return Ok(Operand::EntityRef(text.to_string()));
    }
    if let Some(obj) = value.as_object() {
        if obj.len() == 1 {
            if let Some(count_ref) = obj.get("count").and_then(Value::as_str) {
                return Ok(Operand::Count(count_ref.to_string()));
            }
            let key = obj
                .keys()
                .next()
                .cloned()
                .unwrap_or_else(|| "operand".into());
            return Err(UnsupportedOperatorError::am_rule_009(key, path));
        }
    }
    Err(UnsupportedOperatorError::am_rule_009("operand", path))
}
