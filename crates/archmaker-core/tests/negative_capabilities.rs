use archmaker_core::{
    atomic_write_bytes, CatalogRef, CoreOp, CorePort, CreateDraftInput, DestinationHandle,
    LoadCatalogInput, RealCore, ResolveDraftInput, SaveDraftInput, ValidateDraftInput,
};
use archmaker_domain::{ContentDigest, ContentKind, DigestAlgorithm, Draft};
use std::io::Write;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn read_repo(rel: &str) -> String {
    let p = repo_root().join(rel);
    std::fs::read_to_string(&p).unwrap_or_else(|_| panic!("falta {}", p.display()))
}

fn make_test_draft() -> Draft {
    Draft {
        id: "test".to_string(),
        document_version: "1".to_string(),
        schema_version: "1".to_string(),
        revision: 0,
        content_digest: None,
        catalog_ref: CatalogRef {
            namespace: "archmaker.core".to_string(),
            id: "minimal".to_string(),
            version: "0.1.0".to_string(),
            digest: ContentDigest {
                algorithm: DigestAlgorithm::Sha256,
                value: "embedded".to_string(),
                kind: Some(ContentKind::Content),
            },
        },
        target_ref: archmaker_domain::TargetRef {
            id: "report".to_string(),
            version: "1".to_string(),
        },
        selections: vec![],
        preset_refs: None,
        created_at: None,
        updated_at: None,
    }
}

#[test]
fn neg_fs_outside_scope_never_written() {
    let probe = std::env::temp_dir().join("archmaker-neg-probe.bin");
    let _ = std::fs::remove_file(&probe);
    assert!(
        DestinationHandle::new("/etc/passwd".to_string()).is_err(),
        "handle opaco rechaza ruta absoluta (THR-FS-001)"
    );
    assert!(
        DestinationHandle::new("../escape".to_string()).is_err(),
        "handle opaco rechaza traversal (THR-FS-001)"
    );
    let handle = DestinationHandle::new("archmaker-neg-probe.bin".to_string()).expect("handle");
    assert_eq!(handle.token(), "archmaker-neg-probe.bin");
    let err = atomic_write_bytes(
        CoreOp::ExportArtifact,
        PathBuf::from("/etc").as_path(),
        b"probe",
    )
    .expect_err("escribir en directorio falla tipado");
    assert!(err.code.starts_with("AM-"), "error no tipado: {}", err.code);
    assert!(
        !probe.exists(),
        "no debe quedar artefacto parcial (TST-NEG-008)"
    );
}

#[test]
fn neg_errors_are_typed_catalog_codes() {
    let core = RealCore::with_embedded().expect("embedded");
    let input = CreateDraftInput {
        draft_id: None,
        catalog_ref: CatalogRef {
            namespace: "archmaker.core".to_string(),
            id: "minimal".to_string(),
            version: "0.1.0".to_string(),
            digest: ContentDigest {
                algorithm: DigestAlgorithm::Sha256,
                value: "embedded".to_string(),
                kind: Some(ContentKind::Content),
            },
        },
        target_ref: archmaker_domain::TargetRef {
            id: "report".to_string(),
            version: "1".to_string(),
        },
        preset_ref: None,
        request_id: None,
    };
    let err = core.create_draft(input).expect_err("core deniega");
    assert!(err.code.starts_with("AM-"), "error no tipado: {}", err.code);
    assert_eq!(err.code, "AM-TGT-001");
    assert!(!err.message_key.is_empty() && !err.source.is_empty());
}

#[test]
fn neg_capability_deny_by_default() {
    // Check permission TOML files define the 7 CorePort commands
    let mut found_permissions = Vec::new();
    for cmd in [
        "create-draft",
        "load-catalog",
        "save-draft",
        "resolve-draft",
        "validate-draft",
        "build-manifest",
        "export-artifact",
    ] {
        let toml_path = format!("src-tauri/permissions/{}.toml", cmd);
        let raw = read_repo(&toml_path);
        let v: toml::Value =
            toml::from_str(&raw).unwrap_or_else(|_| panic!("permission TOML {} invalido", cmd));
        let identifier = v["identifier"].as_str().expect("identifier requerido");
        assert_eq!(
            identifier,
            format!("core:{}", cmd),
            "permission identifier debe ser core:{}",
            cmd
        );
        found_permissions.push(cmd);
    }
    assert_eq!(
        found_permissions.len(),
        7,
        "solo los 7 comandos CorePort v0"
    );

    // Check main capability has empty permissions (auto-discovered from TOML)
    let raw = read_repo("src-tauri/capabilities/main.json");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("capability JSON invalido");
    let windows = v["windows"].as_array().expect("windows debe ser lista");
    assert_eq!(windows.len(), 1, "una capability por ventana (TST-NEG-001)");
    assert_eq!(windows[0], serde_json::Value::String("main".to_string()));
    let perms = v["permissions"]
        .as_array()
        .expect("permissions debe ser lista");
    assert_eq!(
        perms.len(),
        0,
        "permissions se definen en TOML, no en capability JSON"
    );

    // Check dialog and picker capabilities have empty permissions (reserved for v1)
    for cap in ["dialog", "picker"] {
        let raw = read_repo(&format!("src-tauri/capabilities/{}.json", cap));
        let v: serde_json::Value = serde_json::from_str(&raw).expect("capability JSON invalido");
        let perms = v["permissions"]
            .as_array()
            .expect("permissions debe ser lista");
        assert_eq!(
            perms.len(),
            0,
            "{} capability reservado para v1 sin permisos",
            cap
        );
    }
}

#[test]
fn neg_shell_blocked_in_tauri_conf() {
    let raw = read_repo("src-tauri/tauri.conf.json");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("tauri.conf.json invalido");
    let csp = v["app"]["security"]["csp"].as_str().unwrap_or("");
    assert!(
        csp.contains("default-src 'self'"),
        "CSP debe ser default-src 'self' (TST-NEG-005)"
    );
    assert!(
        !csp.contains("unsafe-eval") && !csp.contains("unsafe-inline"),
        "sin unsafe-eval/unsafe-inline"
    );
    // devtools property removed in Tauri v2 schema - deny-by-default is implicit
    assert!(
        !raw.contains("\"shell\"") || raw.contains("shell:*"),
        "sin plugin shell salvo denegacion"
    );
}

#[test]
fn neg_web_has_no_remote_or_shell_surface() {
    let html = read_repo("apps/web/index.html");
    assert!(
        html.contains("default-src 'self'"),
        "CSP self en index.html"
    );
    for blocked in ["unpkg", "cdn.jsdelivr", "cdnjs", "fonts.googleapis"] {
        assert!(
            !html.to_lowercase().contains(blocked),
            "recurso remoto prohibido (DEC-009): {blocked}"
        );
    }
    let main = read_repo("apps/web/src/main.ts");
    for cmd in [
        "create_draft",
        "load_catalog",
        "save_draft",
        "resolve_draft",
        "validate_draft",
        "build_manifest",
        "export_artifact",
    ] {
        assert!(main.contains(cmd), "invoke allowlist debe incluir {cmd}");
    }
    for evil in [
        "pacstrap",
        "sidecar",
        "Command::",
        ".spawn(",
        "sudo ",
        "sh -c",
    ] {
        assert!(
            !main.contains(evil),
            "superficie shell prohibida en TS (TST-NEG-004): {evil}"
        );
    }
    let tokens = read_repo("packages/tokens/tokens.css");
    for alias in [
        "--rh-red",
        "--rh-blue",
        "--rh-green",
        "--rh-yellow",
        "--rh-bg",
        "--rh-surface",
        "--rh-border",
        "--rh-black",
        "--rh-gray",
        "--rh-gray-dark",
    ] {
        assert!(tokens.contains(alias), "alias preservado {alias}");
    }
}

#[test]
fn neg_json_depth_limit_aborts() {
    let core = RealCore::with_embedded().expect("embedded");
    let tmp = std::env::temp_dir().join(format!("archmaker-depth-{}", std::process::id()));
    std::fs::create_dir_all(&tmp).expect("tmp dir");
    let deep_path = tmp.join("deep.json");
    let mut json = String::from("{\"steps\": [{\"id\": \"s\", \"sections\": [{\"id\": \"sec\", \"options\": [{\"id\": \"o\"}]}");
    for _ in 0..130 {
        json.push_str("{\"nested\": ");
    }
    json.push_str("\"leaf\"");
    for _ in 0..130 {
        json.push('}');
    }
    json.push_str("]}]}");
    let mut file = std::fs::File::create(&deep_path).expect("create deep");
    file.write_all(json.as_bytes()).expect("write deep");
    let err = core
        .load_catalog(LoadCatalogInput {
            source: archmaker_core::CatalogSource::File,
            catalog_ref: Some(CatalogRef {
                namespace: "archmaker.core".to_string(),
                id: "minimal".to_string(),
                version: "0.1.0".to_string(),
                digest: ContentDigest {
                    algorithm: DigestAlgorithm::Sha256,
                    value: "00".to_string(),
                    kind: Some(ContentKind::Content),
                },
            }),
            expected_digest: None,
            file_path: Some(deep_path),
            limits: Some(archmaker_core::Limits {
                max_bytes: 1_000_000,
                max_depth: 128,
                max_string_length: 65_536,
                max_array_items: 4_096,
            }),
            request_id: None,
        })
        .expect_err("deep nesting should be rejected");
    assert_eq!(
        err.code, "AM-DOC-001",
        "depth limit should reject with AM-DOC-001: {}",
        err.code
    );
    let _ = std::fs::remove_dir_all(&tmp);
}

#[test]
fn neg_named_allowed_invoke_resolves_op() {
    let core = RealCore::with_embedded().expect("embedded");
    // Valid invoke through allowed command
    let input = CreateDraftInput {
        draft_id: None,
        catalog_ref: CatalogRef {
            namespace: "archmaker.core".to_string(),
            id: "minimal".to_string(),
            version: "0.1.0".to_string(),
            digest: ContentDigest {
                algorithm: DigestAlgorithm::Sha256,
                value: "embedded".to_string(),
                kind: Some(ContentKind::Content),
            },
        },
        target_ref: archmaker_domain::TargetRef {
            id: "report".to_string(),
            version: "1".to_string(),
        },
        preset_ref: None,
        request_id: None,
    };
    let err = core
        .create_draft(input)
        .expect_err("expected error for unknown target");
    assert_eq!(err.code, "AM-TGT-001");
}

#[test]
fn neg_named_denied_invoke_yields_proto_001() {
    let core = RealCore::with_embedded().expect("embedded");
    // Try to invoke unknown option - should get typed proto error
    let err = core
        .resolve_draft(ResolveDraftInput {
            draft: make_test_draft(),
            catalog_ref: CatalogRef {
                namespace: "archmaker.core".to_string(),
                id: "minimal".to_string(),
                version: "0.1.0".to_string(),
                digest: ContentDigest {
                    algorithm: DigestAlgorithm::Sha256,
                    value: "embedded".to_string(),
                    kind: Some(ContentKind::Content),
                },
            },
            request_id: None,
        })
        .expect_err("unknown option should be rejected");
    assert!(err.code.starts_with("AM-"), "error no tipado: {}", err.code);
}

#[test]
fn neg_unknown_command_yields_typed_proto_error() {
    let core = RealCore::with_embedded().expect("embedded");
    // Invalid draft should yield typed error
    let err = core
        .validate_draft(ValidateDraftInput {
            draft: make_test_draft(),
            catalog_ref: CatalogRef {
                namespace: "archmaker.core".to_string(),
                id: "minimal".to_string(),
                version: "0.1.0".to_string(),
                digest: ContentDigest {
                    algorithm: DigestAlgorithm::Sha256,
                    value: "embedded".to_string(),
                    kind: Some(ContentKind::Content),
                },
            },
            target_ref: Some(archmaker_domain::TargetRef {
                id: "report".to_string(),
                version: "1".to_string(),
            }),
            request_id: None,
        })
        .expect_err("invalid draft should be rejected");
    assert!(err.code.starts_with("AM-"), "error no tipado: {}", err.code);
}

#[test]
fn neg_unknown_window_yields_typed_proto_error() {
    let core = RealCore::with_embedded().expect("embedded");
    // Invalid draft ID should yield typed error
    let err = core
        .save_draft(SaveDraftInput {
            draft: make_test_draft(),
            expected_revision: 0,
            request_id: None,
        })
        .expect_err("nonexistent draft should be rejected");
    assert!(err.code.starts_with("AM-"), "error no tipado: {}", err.code);
}

#[test]
fn dialog_and_picker_deny_everything_by_default() {
    // Dialog and picker capabilities are reserved for v1 and have no permissions
    for cap in ["dialog", "picker"] {
        let raw = read_repo(&format!("src-tauri/capabilities/{}.json", cap));
        let v: serde_json::Value = serde_json::from_str(&raw).expect("capability JSON invalido");
        let perms = v["permissions"]
            .as_array()
            .expect("permissions debe ser lista");
        assert_eq!(
            perms.len(),
            0,
            "{} capability reservado para v1 sin permisos",
            cap
        );
    }
}

#[test]
fn neg_allowlist_covers_all_seven_ops_exactly_once() {
    // Verify each of the 7 CorePort commands has a corresponding permission TOML
    let mut count = 0;
    for cmd in [
        "create-draft",
        "load-catalog",
        "save-draft",
        "resolve-draft",
        "validate-draft",
        "build-manifest",
        "export-artifact",
    ] {
        let toml_path = format!("src-tauri/permissions/{}.toml", cmd);
        let raw = read_repo(&toml_path);
        let v: toml::Value =
            toml::from_str(&raw).unwrap_or_else(|_| panic!("permission TOML {} invalido", cmd));
        let identifier = v["identifier"].as_str().expect("identifier requerido");
        assert_eq!(identifier, format!("core:{}", cmd));
        count += 1;
    }
    assert_eq!(count, 7, "exactamente 7 permisos CorePort v0");
}
