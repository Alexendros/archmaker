//! Strict ADR-0011 application-contract compliance tests.
//!
//! Expects `archmaker_core::contract` (verify_application_contract / verify_clause).
//! These tests stay red until that module exists and enforces clause semantics.

use archmaker_core::{verify_application_contract, verify_clause, ClauseVerdict, ContractVerdict};
use serde_json::Value;

fn parse_fixture(raw: &str) -> Value {
    serde_json::from_str(raw).expect("fixture deserializes as JSON")
}

fn assert_clause_accepted(clause_id: &str, value: &Value) {
    let verdict = verify_clause(clause_id, value).expect("verify_clause must succeed on valid");
    assert_eq!(verdict.clause_id, clause_id);
    assert!(
        verdict.ok,
        "CLAUSE {clause_id} must accept valid fixture; reason={:?}",
        verdict.reason
    );
}

fn assert_clause_rejected(clause_id: &str, value: &Value) {
    match verify_clause(clause_id, value) {
        Ok(ClauseVerdict {
            ok,
            clause_id: id,
            reason,
        }) => {
            assert_eq!(id, clause_id);
            assert!(
                !ok,
                "CLAUSE {clause_id} must reject invalid fixture; reason={reason:?}"
            );
        }
        Err(err) => {
            assert!(
                !err.is_empty(),
                "CLAUSE {clause_id} rejection Err must carry a message"
            );
        }
    }
}

fn digest_value<'a>(obj: &'a Value, field: &str) -> &'a str {
    obj[field]["value"]
        .as_str()
        .unwrap_or_else(|| panic!("{field}.value must be a string"))
}

fn string_array<'a>(obj: &'a Value, field: &str) -> Vec<&'a str> {
    obj[field]
        .as_array()
        .unwrap_or_else(|| panic!("{field} must be an array"))
        .iter()
        .map(|item| {
            item.as_str()
                .unwrap_or_else(|| panic!("{field} items must be strings"))
        })
        .collect()
}

const ALLOWED_OPERATOR_FORMS: &[&str] = &["gt-count", "required", "eq", "info-count"];

#[test]
fn application_contract_accepts_valid_and_rejects_invalid() {
    let valid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/24-application-contract.valid.json"
    ));
    let invalid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/24-application-contract.invalid.json"
    ));

    let ContractVerdict { ok, failures } =
        verify_application_contract(&valid).expect("valid contract verifies");
    assert!(
        ok,
        "valid application contract must pass; failures={failures:?}"
    );
    assert!(failures.is_empty(), "ok contract must report no failures");

    match verify_application_contract(&invalid) {
        Ok(ContractVerdict { ok, failures }) => {
            assert!(!ok, "invalid application contract must not be ok");
            assert!(
                !failures.is_empty(),
                "invalid contract must list at least one failure"
            );
        }
        Err(err) => {
            assert!(!err.is_empty(), "invalid contract Err must carry a message");
        }
    }
}

#[test]
fn clause_base_001_accepts_valid_and_rejects_invalid() {
    let valid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/25-clause-base.valid.json"
    ));
    let invalid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/25-clause-base.invalid.json"
    ));

    assert_eq!(valid["presentsAsOfficialArch"], false);
    assert_eq!(valid["usesArchLogo"], false);
    let valid_name = valid["publicName"].as_str().expect("publicName");
    assert!(
        !valid_name.to_ascii_lowercase().starts_with("arch"),
        "valid publicName must not start with ARCH"
    );

    assert_eq!(invalid["presentsAsOfficialArch"], true);
    assert_eq!(invalid["usesArchLogo"], true);
    let invalid_name = invalid["publicName"].as_str().expect("publicName");
    assert!(
        invalid_name.to_ascii_lowercase().starts_with("arch"),
        "invalid fixture must violate ARCH publicName rule"
    );

    assert_clause_accepted("CLAUSE-BASE-001", &valid);
    assert_clause_rejected("CLAUSE-BASE-001", &invalid);
}

#[test]
fn clause_manifest_001_digest_stable_across_formats() {
    let valid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/26-clause-manifest.valid.json"
    ));
    let invalid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/26-clause-manifest.invalid.json"
    ));

    let valid_manifest = digest_value(&valid, "manifestDigest");
    let valid_a = digest_value(&valid, "digestAfterFormatA");
    let valid_b = digest_value(&valid, "digestAfterFormatB");
    assert_eq!(valid_a, valid_manifest);
    assert_eq!(valid_b, valid_manifest);
    assert_eq!(valid_a, valid_b);

    let invalid_manifest = digest_value(&invalid, "manifestDigest");
    let invalid_a = digest_value(&invalid, "digestAfterFormatA");
    let invalid_b = digest_value(&invalid, "digestAfterFormatB");
    assert!(
        invalid_a != invalid_b || invalid_a != invalid_manifest || invalid_b != invalid_manifest,
        "invalid fixture must break digest equality across formats"
    );

    assert_clause_accepted("CLAUSE-MANIFEST-001", &valid);
    assert_clause_rejected("CLAUSE-MANIFEST-001", &invalid);
}

#[test]
fn clause_option_001_rejects_cmd_pacstrap_and_embedded_binaries() {
    let valid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/27-clause-option.valid.json"
    ));
    let invalid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/27-clause-option.invalid.json"
    ));

    assert!(string_array(&valid, "forbiddenTokens").is_empty());
    assert!(string_array(&valid, "embeddedBinaries").is_empty());

    let forbidden = string_array(&invalid, "forbiddenTokens");
    let embedded = string_array(&invalid, "embeddedBinaries");
    assert!(
        !forbidden.is_empty() || !embedded.is_empty(),
        "invalid fixture must carry forbiddenTokens or embeddedBinaries"
    );
    assert!(
        forbidden
            .iter()
            .any(|token| *token == "cmd" || *token == "pacstrap")
            || !embedded.is_empty(),
        "invalid fixture must expose cmd/pacstrap tokens or embedded binaries"
    );

    assert_clause_accepted("CLAUSE-OPTION-001", &valid);
    assert_clause_rejected("CLAUSE-OPTION-001", &invalid);
}

#[test]
fn clause_rule_001_rejects_presets_and_silent_mutation() {
    let valid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/28-clause-rule.valid.json"
    ));
    let invalid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/28-clause-rule.invalid.json"
    ));

    assert!(string_array(&valid, "presetsApplied").is_empty());
    assert_eq!(valid["silentMutation"], false);
    let valid_op = valid["evaluatedOperatorForm"]
        .as_str()
        .expect("evaluatedOperatorForm");
    assert!(
        ALLOWED_OPERATOR_FORMS.contains(&valid_op),
        "valid operator form {valid_op} must be one of {ALLOWED_OPERATOR_FORMS:?}"
    );

    assert!(!string_array(&invalid, "presetsApplied").is_empty());
    assert_eq!(invalid["silentMutation"], true);
    let invalid_op = invalid["evaluatedOperatorForm"]
        .as_str()
        .expect("evaluatedOperatorForm");
    assert!(
        !ALLOWED_OPERATOR_FORMS.contains(&invalid_op),
        "invalid operator form {invalid_op} must be outside {ALLOWED_OPERATOR_FORMS:?}"
    );

    assert_clause_accepted("CLAUSE-RULE-001", &valid);
    assert_clause_rejected("CLAUSE-RULE-001", &invalid);
}

#[test]
fn clause_bytes_001_idempotent_export_same_selection() {
    let valid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/29-clause-bytes.valid.json"
    ));
    let invalid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/29-clause-bytes.invalid.json"
    ));

    assert_eq!(valid["sameSelection"], true);
    assert_eq!(
        digest_value(&valid["exportA"], "binaryDigest"),
        digest_value(&valid["exportB"], "binaryDigest")
    );
    assert_eq!(
        digest_value(&valid, "manifestDigestA"),
        digest_value(&valid, "manifestDigestB")
    );

    assert_eq!(invalid["sameSelection"], true);
    assert_ne!(
        digest_value(&invalid["exportA"], "binaryDigest"),
        digest_value(&invalid["exportB"], "binaryDigest")
    );

    assert_clause_accepted("CLAUSE-BYTES-001", &valid);
    assert_clause_rejected("CLAUSE-BYTES-001", &invalid);
}

#[test]
fn clause_host_001_no_spawn_no_root_no_network_for_manifest() {
    let valid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/30-clause-host.valid.json"
    ));
    let invalid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/30-clause-host.invalid.json"
    ));

    assert!(string_array(&valid, "spawnedProcesses").is_empty());
    assert_eq!(valid["requiredNetworkForManifest"], false);
    assert_eq!(valid["elevatedPrivileges"], false);
    assert!(!string_array(&valid, "rejectedOutOfProfileNames").is_empty());

    assert!(!string_array(&invalid, "spawnedProcesses").is_empty());
    assert_eq!(invalid["requiredNetworkForManifest"], true);
    assert_eq!(invalid["elevatedPrivileges"], true);
    assert!(string_array(&invalid, "rejectedOutOfProfileNames").is_empty());

    assert_clause_accepted("CLAUSE-HOST-001", &valid);
    assert_clause_rejected("CLAUSE-HOST-001", &invalid);
}

#[test]
fn clause_boot_001_rejects_satisfied_until_image_admitted() {
    let valid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/31-clause-boot.valid.json"
    ));
    let invalid = parse_fixture(include_str!(
        "../../../contracts/json-schema/examples/31-clause-boot.invalid.json"
    ));

    assert_ne!(valid["status"], "satisfied");
    assert_eq!(valid["imageFormatAdmitted"], false);
    assert_eq!(valid["arch"], "x86_64");

    assert_eq!(invalid["status"], "satisfied");
    assert_eq!(invalid["imageFormatAdmitted"], true);
    assert_ne!(invalid["arch"], "x86_64");

    assert_clause_accepted("CLAUSE-BOOT-001", &valid);
    assert_clause_rejected("CLAUSE-BOOT-001", &invalid);
}
