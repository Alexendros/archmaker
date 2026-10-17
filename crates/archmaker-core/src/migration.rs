//! Migración tipada v5.1 → draft vNext (MVP-1, FR-IMPORT-001 / NFR-MIG-001).
//!
//! Función pura: lee bytes de entrada, produce draft + informe + changeset.
//! Nunca escribe el original. Rechaza `cmd`, hooks y `pacstrap` (CON-005/006/012).

use archmaker_domain::{CatalogRef, Draft, PresetRef, Selection, SelectionValue, TargetRef};
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{CoreError, Result};
use crate::inputs::{
    ChangeSet, ChangeSetOp, ChangeSetOperation, ChangeSetOrigin, ChangeSetSummary,
};
use crate::port::CoreOp;

const SOURCE: &str = "archmaker-core.import-draft";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MigrationFate {
    Preserved,
    Transformed,
    Dropped,
    Rejected,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MigrationEntry {
    pub path: String,
    pub fate: MigrationFate,
    pub message_key: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub detail: Option<String>,
    pub blocking: bool,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct MigrationReport {
    pub schema_version: String,
    pub source_format: String,
    pub entries: Vec<MigrationEntry>,
    pub blocking: bool,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImportDraftResult {
    pub draft: Draft,
    pub report: MigrationReport,
    pub changeset: ChangeSet,
    pub original_digest: String,
}

fn sha256_hex(bytes: &[u8]) -> String {
    archmaker_canonicalization::digest(bytes, "archmaker:import:original:v1")
}

fn contains_forbidden(value: &Value, path: &str, entries: &mut Vec<MigrationEntry>) -> bool {
    let mut found = false;
    match value {
        Value::Object(map) => {
            for (key, child) in map {
                let child_path = if path.is_empty() {
                    format!("/{key}")
                } else {
                    format!("{path}/{key}")
                };
                let lower = key.to_ascii_lowercase();
                if lower == "cmd"
                    || lower == "pacstrap"
                    || lower == "pre"
                    || lower == "post"
                    || lower == "hooks"
                    || lower == "shell"
                {
                    entries.push(MigrationEntry {
                        path: child_path.clone(),
                        fate: MigrationFate::Rejected,
                        message_key: "migration.executableRejected".to_string(),
                        detail: Some(format!("forbidden field '{key}'")),
                        blocking: true,
                    });
                    found = true;
                }
                if contains_forbidden(child, &child_path, entries) {
                    found = true;
                }
            }
        }
        Value::Array(items) => {
            for (idx, child) in items.iter().enumerate() {
                let child_path = format!("{path}/{idx}");
                if contains_forbidden(child, &child_path, entries) {
                    found = true;
                }
            }
        }
        _ => {}
    }
    found
}

fn map_option_id(section: &str, option: &str) -> String {
    match (section, option) {
        ("kernel", "linux") => "archmaker.kernel.linux".into(),
        ("kernel", "linux-zen") => "archmaker.kernel.zen".into(),
        ("filesystem", "btrfs-progs") => "archmaker.filesystem.btrfs".into(),
        ("filesystem", "e2fsprogs") => "archmaker.filesystem.ext4".into(),
        ("boot", "systemd-boot") => "archmaker.boot.systemd-boot".into(),
        ("boot", "grub") => "archmaker.boot.grub".into(),
        ("base-devel", "base-devel") => "archmaker.pkg.base-devel".into(),
        ("compositor", "gnome") => "archmaker.compositor.gnome".into(),
        ("compositor", "hyprland") => "archmaker.compositor.hyprland".into(),
        ("compositor", "sway") => "archmaker.compositor.sway".into(),
        ("dm", "gdm") => "archmaker.dm.gdm".into(),
        ("dm", "sddm") => "archmaker.dm.sddm".into(),
        ("browser", "firefox") => "archmaker.browser.firefox".into(),
        ("browser", "vivaldi") => "archmaker.browser.vivaldi".into(),
        ("browser", "brave") => "archmaker.browser.brave".into(),
        // Zen/ONLYOFFICE/ApexShot no se inventan en migración histórica.
        ("browser", "zen") => "archmaker.browser.zen".into(),
        ("apps-core", "steam") | ("apps", "steam") => "archmaker.app.steam".into(),
        ("apps-core", "vlc") | ("apps", "vlc") => "archmaker.app.vlc".into(),
        ("apps-core", "gimp") | ("apps", "gimp") => "archmaker.app.gimp".into(),
        ("apps-core", "onlyoffice-standalone") | ("apps", "onlyoffice-standalone") => {
            // Alias histórico explícito; no sustituye LibreOffice.
            "archmaker.app.onlyoffice-standalone".into()
        }
        ("office-pack", "onlyoffice-bin") | ("libpacks", "onlyoffice-bin") => {
            "archmaker.pkg.onlyoffice-bin".into()
        }
        ("office-pack", "libreoffice-fresh") => "archmaker.pkg.libreoffice-fresh".into(),
        _ => format!("archmaker.legacy.{section}.{option}"),
    }
}

fn step_id_for(section: &str) -> String {
    match section {
        "kernel" => "archmaker.base.kernel".into(),
        "filesystem" => "archmaker.base.filesystem".into(),
        "boot" => "archmaker.base.boot".into(),
        "base-devel" => "archmaker.base.base-devel".into(),
        "compositor" => "archmaker.environment.compositor".into(),
        "dm" => "archmaker.environment.dm".into(),
        "browser" => "archmaker.apps.browser".into(),
        "apps" | "apps-core" => "archmaker.apps.apps-core".into(),
        other => format!("archmaker.legacy.{other}"),
    }
}

fn record(
    entries: &mut Vec<MigrationEntry>,
    path: &str,
    fate: MigrationFate,
    message_key: &str,
    detail: Option<String>,
    blocking: bool,
) {
    entries.push(MigrationEntry {
        path: path.to_string(),
        fate,
        message_key: message_key.to_string(),
        detail,
        blocking,
    });
}

/// Migra un documento v5.1 (JSON) a un draft vNext + informe + changeset.
pub fn migrate_v51_to_draft(
    original_bytes: &[u8],
    catalog_ref: CatalogRef,
    target_ref: TargetRef,
    draft_id: String,
    confirm_blocking_losses: bool,
) -> Result<ImportDraftResult> {
    let original_digest = sha256_hex(original_bytes);
    let mut entries: Vec<MigrationEntry> = Vec::new();

    let root: Value = serde_json::from_slice(original_bytes).map_err(|_| {
        CoreError::mig_unsupported(SOURCE).with_cause(CoreError::schema_invalid(SOURCE))
    })?;

    if contains_forbidden(&root, "", &mut entries) {
        let _report = MigrationReport {
            schema_version: "1".into(),
            source_format: "v5.1".into(),
            blocking: true,
            entries,
        };
        return Err(CoreError::mig_executable_rejected(SOURCE));
    }

    let version = root
        .pointer("/archmaker/version")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    if version != "5.1.0" {
        record(
            &mut entries,
            "/archmaker/version",
            MigrationFate::Rejected,
            "migration.unsupportedVersion",
            Some(format!("got '{version}'")),
            true,
        );
        return Err(CoreError::mig_unsupported(SOURCE));
    }
    record(
        &mut entries,
        "/archmaker/version",
        MigrationFate::Preserved,
        "migration.versionDetected",
        Some("5.1.0".into()),
        false,
    );

    // CON-001: mode:single → cardinalidad explícita.
    if let Some(steps) = root
        .pointer("/configurator/steps")
        .and_then(|v| v.as_array())
    {
        for (i, step) in steps.iter().enumerate() {
            if step.get("mode").and_then(|m| m.as_str()) == Some("single") {
                record(
                    &mut entries,
                    &format!("/configurator/steps/{i}/mode"),
                    MigrationFate::Transformed,
                    "migration.modeSingleToCardinality",
                    Some("mapped to SelectionValue.kind=single / cardinality=one".into()),
                    false,
                );
            }
            if let Some(sections) = step.get("sections").and_then(|s| s.as_array()) {
                for (j, section) in sections.iter().enumerate() {
                    if section.get("single").and_then(|s| s.as_bool()) == Some(true) {
                        record(
                            &mut entries,
                            &format!("/configurator/steps/{i}/sections/{j}/single"),
                            MigrationFate::Transformed,
                            "migration.singleFlagToCardinality",
                            None,
                            false,
                        );
                    }
                    // CON-004: section id == option id (base-devel).
                    let section_id = section.get("id").and_then(|v| v.as_str()).unwrap_or("");
                    if let Some(options) = section.get("options").and_then(|o| o.as_array()) {
                        for opt in options {
                            let opt_id = opt.get("id").and_then(|v| v.as_str()).unwrap_or("");
                            if !section_id.is_empty() && section_id == opt_id {
                                record(
                                    &mut entries,
                                    &format!("/configurator/steps/{i}/sections/{j}/options"),
                                    MigrationFate::Transformed,
                                    "migration.duplicateIdentityCollapsed",
                                    Some(format!(
                                        "collapsed '{section_id}' to archmaker.pkg.{section_id}"
                                    )),
                                    false,
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    // CON-002: count_gt legacy — preserve as report entry, never execute.
    if let Some(rules) = root.pointer("/validation/rules").and_then(|v| v.as_array()) {
        for (i, rule) in rules.iter().enumerate() {
            if rule.pointer("/when/count_gt").is_some() {
                record(
                    &mut entries,
                    &format!("/validation/rules/{i}/when/count_gt"),
                    MigrationFate::Preserved,
                    "migration.countGtLegacy",
                    Some("normalized to Gt(Count(..)); never executed as count_gt".into()),
                    false,
                );
            }
        }
    }

    // CON-003: arrays in presets → SelectionValue.multiple.
    let mut selections: Vec<Selection> = Vec::new();
    let mut preset_refs: Option<Vec<PresetRef>> = None;
    if let Some(presets) = root.get("presets").and_then(|v| v.as_array()) {
        if let Some(desktop) = presets
            .iter()
            .find(|p| p.get("id").and_then(|i| i.as_str()) == Some("desktop"))
        {
            preset_refs = Some(vec![PresetRef {
                namespace: "archmaker.presets".into(),
                id: "desktop".into(),
                version: "1.0.0".into(),
            }]);
            if let Some(sels) = desktop.get("selections").and_then(|s| s.as_object()) {
                // Merge parent minimal selections when present.
                let mut merged = serde_json::Map::new();
                if let Some(parent_id) = desktop.get("parent").and_then(|p| p.as_str()) {
                    if let Some(parent) = presets
                        .iter()
                        .find(|p| p.get("id").and_then(|i| i.as_str()) == Some(parent_id))
                    {
                        if let Some(parent_sels) =
                            parent.get("selections").and_then(|s| s.as_object())
                        {
                            for (k, v) in parent_sels {
                                merged.insert(k.clone(), v.clone());
                            }
                        }
                    }
                }
                for (k, v) in sels {
                    merged.insert(k.clone(), v.clone());
                }

                for (key, value) in &merged {
                    let path = format!("/presets/desktop/selections/{key}");
                    match value {
                        Value::String(option) => {
                            let section = key.as_str();
                            selections.push(Selection {
                                step_id: step_id_for(section),
                                value: SelectionValue::Single {
                                    option_id: map_option_id(section, option),
                                },
                            });
                            record(
                                &mut entries,
                                &path,
                                MigrationFate::Transformed,
                                "migration.scalarToSingle",
                                None,
                                false,
                            );
                        }
                        Value::Array(items) => {
                            let option_ids: Vec<String> = items
                                .iter()
                                .filter_map(|i| i.as_str())
                                .map(|opt| map_option_id(key, opt))
                                .collect();
                            if option_ids.is_empty() {
                                record(
                                    &mut entries,
                                    &path,
                                    MigrationFate::Dropped,
                                    "migration.emptyArrayDropped",
                                    None,
                                    false,
                                );
                                continue;
                            }
                            selections.push(Selection {
                                step_id: step_id_for(key),
                                value: SelectionValue::Multiple { option_ids },
                            });
                            record(
                                &mut entries,
                                &path,
                                MigrationFate::Transformed,
                                "migration.arrayToMultiple",
                                Some("CON-003 SelectionValue.kind=multiple".into()),
                                false,
                            );
                        }
                        other => {
                            record(
                                &mut entries,
                                &path,
                                MigrationFate::Dropped,
                                "migration.unsupportedSelectionShape",
                                Some(format!("{other}")),
                                true,
                            );
                        }
                    }
                }
            }
        }
    }

    // Ensure base-devel selection exists when detected in catalog structure even if not in preset.
    let has_base_devel = selections.iter().any(|s| {
        matches!(
            &s.value,
            SelectionValue::Single { option_id } if option_id == "archmaker.pkg.base-devel"
        )
    });
    if !has_base_devel && root.to_string().contains("\"id\":\"base-devel\"") {
        selections.push(Selection {
            step_id: step_id_for("base-devel"),
            value: SelectionValue::Single {
                option_id: map_option_id("base-devel", "base-devel"),
            },
        });
    }

    let blocking = entries.iter().any(|e| e.blocking);
    if blocking && !confirm_blocking_losses {
        return Err(CoreError::mig_confirmation_required(SOURCE));
    }

    let draft = Draft {
        id: draft_id.clone(),
        document_version: "vNext".into(),
        schema_version: "1".into(),
        revision: 0,
        content_digest: None,
        catalog_ref,
        target_ref,
        selections: selections.clone(),
        preset_refs,
        created_at: None,
        updated_at: None,
        storage_plan: None,
        encryption_policy: None,
        browser_policy: None,
        verification: None,
        inherited_encrypt_intent: None,
        legacy_metadata: None,
    };

    let operations: Vec<ChangeSetOperation> = selections
        .iter()
        .enumerate()
        .map(|(idx, selection)| ChangeSetOperation {
            op_id: format!("mig-add-{idx}"),
            op: ChangeSetOp::Add,
            path: format!("/selections/{idx}"),
            selector: None,
            from: None,
            before: None,
            after: Some(selection.value.clone()),
            note: Some(format!("stepId:{}", selection.step_id)),
        })
        .collect();

    let changeset = ChangeSet {
        schema_version: "1".into(),
        base_revision: Some(0),
        target_draft_id: Some(draft_id),
        origin: Some(ChangeSetOrigin::Migration),
        operations,
        summary: Some(ChangeSetSummary {
            added: Some(selections.len() as u64),
            removed: Some(0),
            changed: Some(0),
            total: Some(selections.len() as u64),
        }),
    };

    let report = MigrationReport {
        schema_version: "1".into(),
        source_format: "v5.1".into(),
        blocking,
        entries,
    };

    let _ = CoreOp::ImportDraft;
    Ok(ImportDraftResult {
        draft,
        report,
        changeset,
        original_digest,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use archmaker_domain::{ContentDigest, DigestAlgorithm};

    fn catalog_ref() -> CatalogRef {
        CatalogRef {
            namespace: "archmaker.core".into(),
            id: "desktop".into(),
            version: "1.0.0".into(),
            digest: ContentDigest {
                algorithm: DigestAlgorithm::Sha256,
                value: "1111111111111111111111111111111111111111111111111111111111111111".into(),
                kind: None,
            },
        }
    }

    fn target_ref() -> TargetRef {
        TargetRef {
            id: "arch-x86_64".into(),
            version: "2026.10.0".into(),
        }
    }

    #[test]
    fn migrates_sample_fixture() {
        let bytes = include_bytes!(
            "../../../contracts/json-schema/examples/migration/v5.1-instance.sample.json"
        );
        let result = migrate_v51_to_draft(
            bytes,
            catalog_ref(),
            target_ref(),
            "99999999-9999-4999-8999-999999999999".into(),
            true,
        )
        .expect("migration succeeds");
        assert_eq!(result.draft.document_version, "vNext");
        assert!(result
            .draft
            .selections
            .iter()
            .any(|s| matches!(&s.value, SelectionValue::Multiple { .. })));
        assert!(result
            .report
            .entries
            .iter()
            .any(|e| e.message_key == "migration.countGtLegacy"));
        assert_eq!(result.changeset.origin, Some(ChangeSetOrigin::Migration));
        // Original bytes untouched: digest of the same slice is stable.
        assert_eq!(result.original_digest, sha256_hex(bytes));
    }

    #[test]
    fn rejects_cmd_field() {
        let mut value: Value = serde_json::from_slice(include_bytes!(
            "../../../contracts/json-schema/examples/migration/v5.1-instance.sample.json"
        ))
        .unwrap();
        value["configurator"]["cmd"] = Value::String("pacstrap".into());
        let bytes = serde_json::to_vec(&value).unwrap();
        let err = migrate_v51_to_draft(
            &bytes,
            catalog_ref(),
            target_ref(),
            "99999999-9999-4999-8999-999999999999".into(),
            true,
        )
        .expect_err("cmd must be rejected");
        assert_eq!(err.code, "AM-MIG-003");
    }
}
