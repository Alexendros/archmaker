//! Reproducción F0 de C-01..C-07 sobre el comportamiento actual.
//! Estas pruebas fijan el hallazgo. La fase que lo corrija tiene que
//! actualizar la aserción; no son el arreglo.

use std::path::PathBuf;

use archmaker_core::{
    is_uuid, new_uuid_v4_wasm_safe, render_artifact_bytes, BuildManifestInput, CorePort,
    CreateDraftInput, DestinationHandle, ExportArtifactInput, RealCore, ResolveDraftInput,
    SaveDraftInput,
};
use archmaker_domain::{CatalogRef, Draft, Selection, SelectionValue, TargetRef};
use sha2::{Digest, Sha256};

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
}

fn read_repo(rel: &str) -> String {
    let path = repo_root().join(rel);
    std::fs::read_to_string(&path).unwrap_or_else(|_| panic!("falta {}", path.display()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    Sha256::digest(bytes)
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}

fn catalog_ref() -> CatalogRef {
    RealCore::with_embedded()
        .expect("embedded")
        .active_catalog_ref()
        .expect("active catalog ref")
}

fn draft_target() -> TargetRef {
    TargetRef {
        id: "arch-x86_64".to_string(),
        version: "2026.10.0".to_string(),
    }
}

fn export_target(id: &str) -> TargetRef {
    TargetRef {
        id: id.to_string(),
        version: "0.1.0".to_string(),
    }
}

fn single(step: &str, option: &str) -> Selection {
    Selection {
        step_id: step.to_string(),
        value: SelectionValue::Single {
            option_id: option.to_string(),
        },
    }
}

fn assert_uuid_v4(id: &str) {
    assert!(is_uuid(id), "no es uuid: {id}");
    let bytes = id.as_bytes();
    assert_eq!(bytes[14], b'4', "versión distinta de 4: {id}");
    assert!(
        matches!(bytes[19], b'8' | b'9' | b'a' | b'b'),
        "variante distinta de RFC 4122: {id}"
    );
}

fn valid_draft(core: &RealCore) -> Draft {
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: Some("0d5e6b1a-2222-4a2b-9c3d-222222222222".to_string()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create");
    draft.selections = vec![
        single("kernel", "archmaker.kernel.linux"),
        single("session", "archmaker.desktop.gnome"),
    ];
    draft
}

#[test]
fn p5_independent_core_instances_share_no_draft_store() {
    // P5: instancias de perfil independientes — mismo id en cores distintos no colisiona.
    let a = RealCore::with_embedded().expect("core-a");
    let b = RealCore::with_embedded().expect("core-b");
    let id = "0d5e6b1a-4444-4a2b-9c3d-444444444444".to_string();
    let draft_a = a
        .create_draft(CreateDraftInput {
            draft_id: Some(id.clone()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create a");
    let draft_b = b
        .create_draft(CreateDraftInput {
            draft_id: Some(id.clone()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create b independiente");
    assert_eq!(draft_a.id, draft_b.id);
    assert!(a.stored_draft(&id).is_some());
    assert!(b.stored_draft(&id).is_some());
    // Dentro de una misma instancia sí hay colisión tipada.
    let err = a
        .create_draft(CreateDraftInput {
            draft_id: Some(id),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect_err("colisión intra-instancia");
    assert_eq!(err.code, "AM-DOC-002");
}

#[test]
fn c01_draft_create_rejects_existing_id() {
    // F1: create_draft ya no sobrescribe; colisión tipada AM-DOC-002.
    let core = RealCore::with_embedded().expect("embedded");
    let id = "0d5e6b1a-3333-4a2b-9c3d-333333333333".to_string();
    let created = core
        .create_draft(CreateDraftInput {
            draft_id: Some(id.clone()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create");
    let mut edited = created.clone();
    edited.selections = vec![single("kernel", "archmaker.kernel.linux")];
    let saved = core
        .save_draft(SaveDraftInput {
            draft: edited,
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect("save");
    assert_eq!(saved.revision, 1);
    let opened = core
        .open_draft(archmaker_core::OpenDraftInput {
            draft_id: id.clone(),
            seed: None,
            request_id: None,
        })
        .expect("open distinto de create");
    assert_eq!(opened.revision, 1);
    assert_eq!(opened.selections.len(), 1);

    let err = core
        .create_draft(CreateDraftInput {
            draft_id: Some(id.clone()),
            catalog_ref: catalog_ref(),
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect_err("create no pisa id almacenado");
    assert_eq!(err.code, "AM-DOC-002");
    let stored = core.stored_draft(&id).expect("sigue almacenado");
    assert_eq!(stored.revision, 1);
    assert_eq!(stored.selections.len(), 1);
}

#[test]
fn c01_wasm_production_uses_platform_entropy() {
    // F1: producción WASM → getrandom; contador solo en new_uuid_v4_wasm_safe (tests).
    let source = read_repo("crates/archmaker-core/src/core.rs");
    assert!(
        source.contains("static UUID_COUNTER: AtomicU64 = AtomicU64::new(0);"),
        "el contador de tests sigue naciendo en 0"
    );
    assert!(
        source.contains("getrandom::fill(buffer)"),
        "producción WASM debe pedir entropía de plataforma"
    );
    let first = new_uuid_v4_wasm_safe();
    let second = new_uuid_v4_wasm_safe();
    assert_uuid_v4(&first);
    assert_uuid_v4(&second);
    assert_ne!(first, second);
    assert_eq!(first, "b8178f96-e05d-4c4c-9574-864fca3fdb7d");
}

#[test]
fn c02_export_binary_digest_matches_renderer_bytes() {
    let core = RealCore::with_embedded().expect("embedded");
    let draft = valid_draft(&core);
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft,
            catalog_ref: catalog_ref(),
            target_ref: export_target("report"),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("manifest report");
    let rendered = render_artifact_bytes(&manifest, &export_target("report")).expect("render");
    let artifact = core
        .export_artifact(ExportArtifactInput {
            manifest,
            target_ref: export_target("report"),
            destination: DestinationHandle::new("report.md".to_string()).expect("token"),
            request_id: None,
        })
        .expect("export");
    assert_eq!(artifact.binary_digest.value, sha256_hex(&rendered.bytes));
    assert_eq!(rendered.media_type, "text/markdown");
    let dto = serde_json::to_value(&artifact).expect("dto");
    let dto_bytes = dto
        .get("bytes")
        .and_then(|v| v.as_array())
        .expect("el DTO Artifact lleva bytes");
    assert_eq!(dto_bytes.len(), rendered.bytes.len());
    let transported = artifact.bytes.as_ref().expect("bytes en Artifact");
    assert_eq!(transported.as_slice(), rendered.bytes.as_slice());
    let content = artifact.content.expect("content");
    assert!(content.is_string(), "report viaja como cadena JSON");
    let quoted = serde_json::to_string_pretty(&content).expect("stringify");
    assert_ne!(
        quoted.as_bytes(),
        rendered.bytes.as_slice(),
        "JSON.stringify de la cadena no es el texto del informe"
    );
    let delivery = read_repo("apps/web/src/export-delivery.ts");
    assert!(delivery.contains("prepareDelivery"));
    assert!(delivery.contains("bytes-missing"));
    assert!(!delivery.contains("artifact.content ??"));
}

#[test]
fn c03_resolve_hostname_encrypt_preserved_as_intention() {
    // F3 corrige C-03: hostname y cifrado (intención) llegan al manifiesto.
    let face = read_repo("apps/web/src/draft-for-core.ts");
    assert!(face.contains("STEP_IDS.intent"));
    assert!(
        !face.contains("STEP_IDS.hostname") && !face.contains("STEP_IDS.encrypt"),
        "draftForCore no quita hostname ni encrypt"
    );
    let app = read_repo("apps/web/src/App.tsx");
    assert!(
        app.contains("no cifra") || app.contains("intención"),
        "la cara no afirma cifrado aplicado"
    );

    let core = RealCore::with_embedded().expect("embedded");
    let mut draft = valid_draft(&core);
    draft.selections.push(Selection {
        step_id: "archmaker.face.hostname".to_string(),
        value: SelectionValue::Text {
            value: "arch-forge".to_string(),
        },
    });
    draft.selections.push(Selection {
        step_id: "archmaker.face.encrypt".to_string(),
        value: SelectionValue::Boolean { value: true },
    });
    let resolved = core
        .resolve_draft(ResolveDraftInput {
            draft: draft.clone(),
            catalog_ref: catalog_ref(),
            request_id: None,
        })
        .expect("text y boolean sin option id no fallan el catálogo");
    assert!(resolved
        .effective_selections
        .iter()
        .any(|s| s.step_id == "archmaker.face.hostname"
            && matches!(&s.value, SelectionValue::Text { value } if value == "arch-forge")));
    assert!(resolved.effective_selections.iter().any(|s| {
        s.step_id == "archmaker.face.encrypt"
            && matches!(&s.value, SelectionValue::Boolean { value: true })
    }));
    let manifest = core
        .build_manifest(BuildManifestInput {
            draft,
            catalog_ref: catalog_ref(),
            target_ref: export_target("archmaker-profile"),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("el manifiesto se construye");
    let json = serde_json::to_string(&manifest).expect("json");
    assert!(
        json.contains("arch-forge"),
        "el hostname llega al manifiesto"
    );
    assert!(
        json.contains("archmaker.face.encrypt"),
        "el step de cifrado (intención) llega al manifiesto"
    );
    assert!(
        !json.to_lowercase().contains("encrypted") && !json.contains("cifrado aplicado"),
        "el manifiesto no afirma cifrado aplicado al disco"
    );
}

#[test]
fn c04_import_checks_file_size_before_read_with_consent() {
    // F4 corrige C-04: file.size antes de leer; sin confirmBlockingLosses hardcodeado.
    let consented = read_repo("apps/web/src/consented-import.ts");
    assert!(consented.contains("IMPORT_MAX_BYTES = 2_000_000"));
    assert!(consented.contains("file.size"));
    assert!(consented.contains("checkImportFileSize"));
    assert!(consented.contains("confirmBlockingLosses: false"));
    assert!(consented.contains("analyzeImportFile"));
    assert!(consented.contains("applyConsentedImport"));
    let size_fn = consented
        .find("export function checkImportFileSize")
        .expect("checkImportFileSize");
    let read_fn = consented
        .find("export async function readImportBytes")
        .expect("readImportBytes");
    assert!(
        size_fn < read_fn,
        "la comprobación de tamaño precede a la lectura"
    );

    let app = read_repo("apps/web/src/App.tsx");
    let start = app
        .find("async function onImportFile")
        .expect("onImportFile");
    let end = app[start..]
        .find("async function onConfirmImport")
        .map(|offset| start + offset)
        .expect("onConfirmImport");
    let analyze_body = &app[start..end];
    assert!(
        !analyze_body.contains("confirmBlockingLosses: true"),
        "el análisis no confirma pérdidas bloqueantes"
    );
    assert!(analyze_body.contains("analyzeImportFile"));
    assert!(app.contains("onConfirmImport"));
    assert!(app.contains("Informe de importación"));
}

#[test]
fn c05_persist_has_typed_save_states_and_serialized_writes() {
    // F4 corrige C-05: estados guardando/guardado/error y cola serializada.
    let app = read_repo("apps/web/src/App.tsx");
    assert!(app.contains("guardando"));
    assert!(app.contains("guardado"));
    assert!(app.contains("persistStatus"));
    assert!(app.contains("onRetryPersist"));
    assert!(app.contains("Descargar copia"));
    let persistence = read_repo("apps/web/src/draft-persistence.ts");
    assert!(persistence.contains("indexedDB.open"));
    assert!(persistence.contains("PersistStatus"));
    assert!(persistence.contains("\"saving\""));
    assert!(persistence.contains("\"saved\""));
    assert!(persistence.contains("\"error\""));
    assert!(persistence.contains("writeChain"));
    assert!(persistence.contains("exportDraftCopy"));
    assert!(persistence.contains("retryPersistDraftLocal"));
}

#[test]
fn c06_export_capability_scoped_save_only() {
    // F5: guardado nativo acotado; sin dialog.json/picker.json ni FS global.
    let app = read_repo("apps/web/src/App.tsx");
    assert!(app.contains("deliverArtifact("));
    let delivery = read_repo("apps/web/src/export-delivery.ts");
    assert!(delivery.contains("save_export_bytes"));
    assert!(delivery.contains("confirmedSaveOutcome") || delivery.contains("file-saved"));
    let caps = repo_root().join("src-tauri/capabilities");
    assert!(!caps.join("dialog.json").exists());
    assert!(!caps.join("picker.json").exists());
    let main = read_repo("src-tauri/capabilities/main.json");
    assert!(main.contains("allow-save-export-bytes"));
    assert!(!main.contains("dialog:allow-open"));
    let port = read_repo("apps/web/src/create-core-port.ts");
    let catch_at = port.rfind("catch (e)").expect("catch wasm");
    assert!(
        !port[catch_at..].contains("MockCorePort"),
        "el fallo WASM no cae a mock"
    );
}

#[test]
fn c07_face_no_longer_invents_base_or_sway_warn() {
    // F6: C-07 corregido — sin base 650 ni aviso Sway/kernel inventado en UI.
    let app = read_repo("apps/web/src/App.tsx");
    assert!(
        !app.contains("acc + p.size, 650"),
        "la suma no debe añadir base fija 650"
    );
    assert!(
        !app.contains("Sway con este kernel puede parpadear en algunos GPUs"),
        "no debe haber aviso Sway/kernel sin diagnóstico Rust"
    );
    assert!(
        app.contains("acc + p.size, 0"),
        "la suma debe partir de cero (solo paquetes de catálogo)"
    );
    let core = read_repo("crates/archmaker-core/src/core.rs");
    assert!(!core.contains("Sway con este kernel"));
}

#[test]
fn c0_draft_corpus_fixture_digests() {
    let dir = repo_root().join("crates/archmaker-core/tests/fixtures/closure");
    let index: serde_json::Value = serde_json::from_str(
        &std::fs::read_to_string(dir.join("fixture-digests.json")).expect("índice"),
    )
    .expect("json");
    assert!(index.get("sha256").is_none());
    let files = index["files"].as_object().expect("files");
    assert_eq!(files.len(), 3);
    for (name, meta) in files {
        let bytes = std::fs::read(dir.join(name)).unwrap_or_else(|_| panic!("falta {name}"));
        assert_eq!(
            bytes.len() as u64,
            meta["sizeBytes"].as_u64().expect("size")
        );
        assert_eq!(sha256_hex(&bytes), meta["sha256"].as_str().expect("sha"));
        match meta["class"].as_str().expect("class") {
            "positive" => {
                let value: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
                assert_eq!(value["class"], "positive");
            }
            "negative" => {
                let value: serde_json::Value = serde_json::from_slice(&bytes).expect("json");
                assert!(value.get("sourceSha").is_none());
            }
            "binary" => {
                assert!(serde_json::from_slice::<serde_json::Value>(&bytes).is_err());
            }
            other => panic!("clase {other}"),
        }
    }
}
