//! Application-contract verifier (ADR-0011).
//!
//! Semantic authority for CLAUSE-* compliance. Does not spawn shell, pacstrap,
//! mkarchiso or require network. Image format is not admitted: CLAUSE-BOOT-001
//! cannot be satisfied.

use serde_json::Value;

const ADMITTED_FORMATS: &[&str] = &["archmaker-profile", "archinstall-profile", "report"];
const ALLOWED_OPERATOR_FORMS: &[&str] = &["gt-count", "required", "eq", "info-count"];
const REQUIRED_CLAUSE_IDS: &[&str] = &[
    "CLAUSE-BASE-001",
    "CLAUSE-MANIFEST-001",
    "CLAUSE-OPTION-001",
    "CLAUSE-RULE-001",
    "CLAUSE-BYTES-001",
    "CLAUSE-HOST-001",
    "CLAUSE-BOOT-001",
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContractVerdict {
    pub ok: bool,
    pub failures: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClauseVerdict {
    pub ok: bool,
    pub clause_id: String,
    pub reason: Option<String>,
}

fn digest_hex(obj: &Value, field: &str) -> Result<String, String> {
    obj.get(field)
        .and_then(|d| d.get("value"))
        .and_then(|v| v.as_str())
        .map(str::to_string)
        .ok_or_else(|| format!("missing {field}.value"))
}

fn string_list(obj: &Value, field: &str) -> Result<Vec<String>, String> {
    let arr = obj
        .get(field)
        .and_then(|v| v.as_array())
        .ok_or_else(|| format!("missing array {field}"))?;
    arr.iter()
        .map(|item| {
            item.as_str()
                .map(str::to_string)
                .ok_or_else(|| format!("{field} items must be strings"))
        })
        .collect()
}

fn bool_field(obj: &Value, field: &str) -> Result<bool, String> {
    obj.get(field)
        .and_then(|v| v.as_bool())
        .ok_or_else(|| format!("missing bool {field}"))
}

fn str_field<'a>(obj: &'a Value, field: &str) -> Result<&'a str, String> {
    obj.get(field)
        .and_then(|v| v.as_str())
        .ok_or_else(|| format!("missing string {field}"))
}

fn clause_ok(clause_id: &str) -> ClauseVerdict {
    ClauseVerdict {
        ok: true,
        clause_id: clause_id.to_string(),
        reason: None,
    }
}

fn clause_fail(clause_id: &str, reason: impl Into<String>) -> ClauseVerdict {
    ClauseVerdict {
        ok: false,
        clause_id: clause_id.to_string(),
        reason: Some(reason.into()),
    }
}

fn verify_base(value: &Value) -> ClauseVerdict {
    let id = "CLAUSE-BASE-001";
    if str_field(value, "id").ok() != Some(id) {
        return clause_fail(id, "id must be CLAUSE-BASE-001");
    }
    let base = match value.get("base") {
        Some(b) => b,
        None => return clause_fail(id, "missing base"),
    };
    if base.get("distro").and_then(|v| v.as_str()) != Some("arch-linux") {
        return clause_fail(id, "base.distro must be arch-linux");
    }
    if base.get("arch").and_then(|v| v.as_str()) != Some("x86_64") {
        return clause_fail(id, "base.arch must be x86_64");
    }
    if base.get("kind").and_then(|v| v.as_str()) != Some("base-iso") {
        return clause_fail(id, "base.kind must be base-iso");
    }
    match bool_field(value, "presentsAsOfficialArch") {
        Ok(false) => {}
        Ok(true) => return clause_fail(id, "presentsAsOfficialArch must be false"),
        Err(e) => return clause_fail(id, e),
    }
    match bool_field(value, "usesArchLogo") {
        Ok(false) => {}
        Ok(true) => return clause_fail(id, "usesArchLogo must be false"),
        Err(e) => return clause_fail(id, e),
    }
    match str_field(value, "publicName") {
        Ok(name) if name.to_ascii_lowercase().starts_with("arch") => {
            clause_fail(id, "publicName must not start with ARCH")
        }
        Ok(_) => clause_ok(id),
        Err(e) => clause_fail(id, e),
    }
}

fn verify_manifest(value: &Value) -> ClauseVerdict {
    let id = "CLAUSE-MANIFEST-001";
    if str_field(value, "id").ok() != Some(id) {
        return clause_fail(id, "id must be CLAUSE-MANIFEST-001");
    }
    for key in ["formatA", "formatB"] {
        match str_field(value, key) {
            Ok(fmt) if ADMITTED_FORMATS.contains(&fmt) => {}
            Ok(fmt) => return clause_fail(id, format!("{key}={fmt} is not an admitted format")),
            Err(e) => return clause_fail(id, e),
        }
    }
    let manifest = match digest_hex(value, "manifestDigest") {
        Ok(v) => v,
        Err(e) => return clause_fail(id, e),
    };
    let a = match digest_hex(value, "digestAfterFormatA") {
        Ok(v) => v,
        Err(e) => return clause_fail(id, e),
    };
    let b = match digest_hex(value, "digestAfterFormatB") {
        Ok(v) => v,
        Err(e) => return clause_fail(id, e),
    };
    if a != manifest || b != manifest {
        return clause_fail(id, "format projections must keep the same manifest digest");
    }
    clause_ok(id)
}

fn verify_option(value: &Value) -> ClauseVerdict {
    let id = "CLAUSE-OPTION-001";
    if str_field(value, "id").ok() != Some(id) {
        return clause_fail(id, "id must be CLAUSE-OPTION-001");
    }
    match string_list(value, "forbiddenTokens") {
        Ok(tokens) if tokens.is_empty() => {}
        Ok(tokens) => {
            return clause_fail(id, format!("forbiddenTokens must be empty, got {tokens:?}"))
        }
        Err(e) => return clause_fail(id, e),
    }
    match string_list(value, "embeddedBinaries") {
        Ok(bins) if bins.is_empty() => {}
        Ok(bins) => {
            return clause_fail(id, format!("embeddedBinaries must be empty, got {bins:?}"))
        }
        Err(e) => return clause_fail(id, e),
    }
    if value
        .get("loadedPackageNames")
        .and_then(|v| v.as_array())
        .is_none()
    {
        return clause_fail(id, "missing loadedPackageNames");
    }
    if value
        .get("gapOptionIds")
        .and_then(|v| v.as_array())
        .is_none()
    {
        return clause_fail(id, "missing gapOptionIds");
    }
    clause_ok(id)
}

fn verify_rule(value: &Value) -> ClauseVerdict {
    let id = "CLAUSE-RULE-001";
    if str_field(value, "id").ok() != Some(id) {
        return clause_fail(id, "id must be CLAUSE-RULE-001");
    }
    match string_list(value, "presetsApplied") {
        Ok(presets) if presets.is_empty() => {}
        Ok(presets) => {
            return clause_fail(id, format!("presets must not be applied, got {presets:?}"))
        }
        Err(e) => return clause_fail(id, e),
    }
    match bool_field(value, "silentMutation") {
        Ok(false) => {}
        Ok(true) => return clause_fail(id, "silentMutation must be false"),
        Err(e) => return clause_fail(id, e),
    }
    match str_field(value, "evaluatedOperatorForm") {
        Ok(form) if ALLOWED_OPERATOR_FORMS.contains(&form) => {}
        Ok(form) => {
            return clause_fail(
                id,
                format!("evaluatedOperatorForm={form} is not normalized"),
            )
        }
        Err(e) => return clause_fail(id, e),
    }
    if value
        .get("diagnostics")
        .and_then(|v| v.as_array())
        .is_none()
    {
        return clause_fail(id, "missing diagnostics");
    }
    clause_ok(id)
}

fn verify_bytes(value: &Value) -> ClauseVerdict {
    let id = "CLAUSE-BYTES-001";
    if str_field(value, "id").ok() != Some(id) {
        return clause_fail(id, "id must be CLAUSE-BYTES-001");
    }
    match str_field(value, "format") {
        Ok(fmt) if ADMITTED_FORMATS.contains(&fmt) => {}
        Ok(fmt) => return clause_fail(id, format!("format={fmt} is not admitted")),
        Err(e) => return clause_fail(id, e),
    }
    let same = match bool_field(value, "sameSelection") {
        Ok(v) => v,
        Err(e) => return clause_fail(id, e),
    };
    let export_a = match value.get("exportA") {
        Some(v) => v,
        None => return clause_fail(id, "missing exportA"),
    };
    let export_b = match value.get("exportB") {
        Some(v) => v,
        None => return clause_fail(id, "missing exportB"),
    };
    let bin_a = match digest_hex(export_a, "binaryDigest") {
        Ok(v) => v,
        Err(e) => return clause_fail(id, e),
    };
    let bin_b = match digest_hex(export_b, "binaryDigest") {
        Ok(v) => v,
        Err(e) => return clause_fail(id, e),
    };
    let man_a = match digest_hex(value, "manifestDigestA") {
        Ok(v) => v,
        Err(e) => return clause_fail(id, e),
    };
    let man_b = match digest_hex(value, "manifestDigestB") {
        Ok(v) => v,
        Err(e) => return clause_fail(id, e),
    };
    if same && (bin_a != bin_b || man_a != man_b) {
        return clause_fail(
            id,
            "sameSelection requires identical binary and manifest digests",
        );
    }
    clause_ok(id)
}

fn verify_host(value: &Value) -> ClauseVerdict {
    let id = "CLAUSE-HOST-001";
    if str_field(value, "id").ok() != Some(id) {
        return clause_fail(id, "id must be CLAUSE-HOST-001");
    }
    match string_list(value, "spawnedProcesses") {
        Ok(procs) if procs.is_empty() => {}
        Ok(procs) => {
            return clause_fail(id, format!("spawnedProcesses must be empty, got {procs:?}"))
        }
        Err(e) => return clause_fail(id, e),
    }
    match bool_field(value, "requiredNetworkForManifest") {
        Ok(false) => {}
        Ok(true) => return clause_fail(id, "requiredNetworkForManifest must be false"),
        Err(e) => return clause_fail(id, e),
    }
    match bool_field(value, "elevatedPrivileges") {
        Ok(false) => {}
        Ok(true) => return clause_fail(id, "elevatedPrivileges must be false"),
        Err(e) => return clause_fail(id, e),
    }
    match string_list(value, "rejectedOutOfProfileNames") {
        Ok(names) if !names.is_empty() => clause_ok(id),
        Ok(_) => clause_fail(id, "rejectedOutOfProfileNames must be non-empty"),
        Err(e) => clause_fail(id, e),
    }
}

fn verify_boot(value: &Value) -> ClauseVerdict {
    let id = "CLAUSE-BOOT-001";
    if str_field(value, "id").ok() != Some(id) {
        return clause_fail(id, "id must be CLAUSE-BOOT-001");
    }
    match str_field(value, "status") {
        Ok("satisfied") => return clause_fail(id, "status satisfied is not admitted yet"),
        Ok("pending" | "failed" | "not-applicable") => {}
        Ok(other) => return clause_fail(id, format!("status={other} is invalid")),
        Err(e) => return clause_fail(id, e),
    }
    match bool_field(value, "imageFormatAdmitted") {
        Ok(false) => {}
        Ok(true) => return clause_fail(id, "imageFormatAdmitted must be false"),
        Err(e) => return clause_fail(id, e),
    }
    match str_field(value, "arch") {
        Ok("x86_64") => {}
        Ok(other) => return clause_fail(id, format!("arch={other} rejected; only x86_64")),
        Err(e) => return clause_fail(id, e),
    }
    match str_field(value, "firmware") {
        Ok("uefi") => clause_ok(id),
        Ok(other) => clause_fail(id, format!("firmware={other} must be uefi")),
        Err(e) => clause_fail(id, e),
    }
}

/// Verify one clause document against ADR-0011 semantics.
pub fn verify_clause(clause_id: &str, value: &Value) -> Result<ClauseVerdict, String> {
    let verdict = match clause_id {
        "CLAUSE-BASE-001" => verify_base(value),
        "CLAUSE-MANIFEST-001" => verify_manifest(value),
        "CLAUSE-OPTION-001" => verify_option(value),
        "CLAUSE-RULE-001" => verify_rule(value),
        "CLAUSE-BYTES-001" => verify_bytes(value),
        "CLAUSE-HOST-001" => verify_host(value),
        "CLAUSE-BOOT-001" => verify_boot(value),
        other => {
            return Err(format!("unknown clause id {other}"));
        }
    };
    Ok(verdict)
}

fn validate_contract_shape(value: &Value) -> Result<(), Vec<String>> {
    let mut failures = Vec::new();
    if value
        .get("documentVersion")
        .and_then(|v| v.as_str())
        .is_none()
    {
        failures.push("missing documentVersion".into());
    }
    if value
        .get("schemaVersion")
        .and_then(|v| v.as_str())
        .is_none()
    {
        failures.push("missing schemaVersion".into());
    }
    if value.get("contractId").and_then(|v| v.as_str()).is_none() {
        failures.push("missing contractId".into());
    }
    if digest_hex(value, "manifestDigest").is_err() {
        failures.push("missing manifestDigest.value".into());
    }
    match value.get("base") {
        Some(base) => {
            if base.get("distro").and_then(|v| v.as_str()) != Some("arch-linux") {
                failures.push("base.distro must be arch-linux".into());
            }
            if base.get("arch").and_then(|v| v.as_str()) != Some("x86_64") {
                failures.push("base.arch must be x86_64".into());
            }
            if base.get("kind").and_then(|v| v.as_str()) != Some("base-iso") {
                failures.push("base.kind must be base-iso".into());
            }
        }
        None => failures.push("missing base".into()),
    }
    match value.get("format").and_then(|v| v.as_str()) {
        Some(fmt) if ADMITTED_FORMATS.contains(&fmt) => {}
        Some(fmt) => failures.push(format!("format={fmt} is not admitted (DEC-001)")),
        None => failures.push("missing format".into()),
    }
    if let Some(name) = value.get("publicName").and_then(|v| v.as_str()) {
        if name.to_ascii_lowercase().starts_with("arch") {
            failures.push("publicName must not start with ARCH".into());
        }
    }
    match value.get("clauses").and_then(|v| v.as_array()) {
        Some(clauses) if clauses.len() == 7 => {
            let ids: Vec<&str> = clauses
                .iter()
                .filter_map(|c| c.get("id").and_then(|v| v.as_str()))
                .collect();
            for required in REQUIRED_CLAUSE_IDS {
                if !ids.contains(required) {
                    failures.push(format!("missing clause {required}"));
                }
            }
            for clause in clauses {
                if let Some("CLAUSE-BOOT-001") = clause.get("id").and_then(|v| v.as_str()) {
                    if clause.get("status").and_then(|v| v.as_str()) == Some("satisfied") {
                        failures.push("CLAUSE-BOOT-001 cannot be satisfied yet".into());
                    }
                }
            }
        }
        Some(clauses) => failures.push(format!(
            "clauses must have exactly 7 entries, got {}",
            clauses.len()
        )),
        None => failures.push("missing clauses".into()),
    }
    if failures.is_empty() {
        Ok(())
    } else {
        Err(failures)
    }
}

/// Verify a full application-contract document.
pub fn verify_application_contract(value: &Value) -> Result<ContractVerdict, String> {
    match validate_contract_shape(value) {
        Ok(()) => Ok(ContractVerdict {
            ok: true,
            failures: Vec::new(),
        }),
        Err(failures) => Ok(ContractVerdict {
            ok: false,
            failures,
        }),
    }
}
