use archmaker_core::{atomic_write_bytes, CoreOp, DestinationHandle};
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
    use archmaker_core::{CatalogRef, CreateDraftInput, TargetRef};
    use archmaker_core::{ContentDigest, ContentKind, CorePort, DigestAlgorithm, RealCore};
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
        target_ref: TargetRef {
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
    let raw = read_repo("src-tauri/capabilities/main.json");
    let v: serde_json::Value = serde_json::from_str(&raw).expect("capability JSON invalido");
    let windows = v["windows"].as_array().expect("windows debe ser lista");
    assert_eq!(windows.len(), 1, "una capability por ventana (TST-NEG-001)");
    assert_eq!(windows[0], serde_json::Value::String("main".to_string()));
    let perms: Vec<String> = v["permissions"]
        .as_array()
        .expect("permissions debe ser lista")
        .iter()
        .map(|p| p.as_str().unwrap_or_default().to_string())
        .collect();
    assert_eq!(perms.len(), 7, "solo los 7 comandos CorePort v0");
    for cmd in [
        "core:create-draft",
        "core:load-catalog",
        "core:save-draft",
        "core:resolve-draft",
        "core:validate-draft",
        "core:build-manifest",
        "core:export-artifact",
    ] {
        assert!(perms.contains(&cmd.to_string()), "falta permiso {cmd}");
    }
    let denied = v["denied"].as_array().expect("denied debe ser lista");
    let denied_s: Vec<&str> = denied.iter().map(|d| d.as_str().unwrap_or("")).collect();
    assert!(denied_s.contains(&"shell:*"), "shell:* debe estar denegado");
    assert!(
        denied_s.iter().any(|d| d.contains("updater")),
        "updater debe estar aislado por capability"
    );
    assert!(
        !perms.iter().any(|p| p.starts_with("shell")),
        "ningun permiso shell en MVP"
    );
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
    assert_eq!(
        v["app"]["security"]["devtools"],
        serde_json::Value::Bool(false),
        "devtools desactivadas en release"
    );
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
    use archmaker_core::{CatalogRef, CorePort, RealCore};
    use archmaker_core::{ContentDigest, ContentKind, DigestAlgorithm};
    use std::io::Write;
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
        .load_catalog(archmaker_core::LoadCatalogInput {
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
