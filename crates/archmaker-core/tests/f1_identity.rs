//! F1 — identidad de sesión: UUID de producción, create sin sobrescritura, open ≠ create.

use std::collections::HashSet;

use archmaker_core::{
    is_uuid, new_uuid_v4, new_uuid_v4_wasm_safe, CorePort, CreateDraftInput, OpenDraftInput,
    RealCore, SaveDraftInput,
};
use archmaker_domain::{CatalogRef, Draft, Selection, SelectionValue, TargetRef};

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

fn assert_uuid_v4(id: &str) {
    assert!(is_uuid(id), "no es uuid: {id}");
    let bytes = id.as_bytes();
    assert_eq!(bytes[14], b'4', "versión distinta de 4: {id}");
    assert!(
        matches!(bytes[19], b'8' | b'9' | b'a' | b'b'),
        "variante distinta de RFC 4122: {id}"
    );
}

fn create_input(id: Option<&str>) -> CreateDraftInput {
    CreateDraftInput {
        draft_id: id.map(str::to_string),
        catalog_ref: catalog_ref(),
        target_ref: draft_target(),
        preset_ref: None,
        request_id: None,
    }
}

#[test]
fn create_draft_rejects_existing_id_without_overwrite() {
    let core = RealCore::with_embedded().expect("embedded");
    let id = "0d5e6b1a-4444-4a2b-9c3d-444444444444".to_string();
    let created = core.create_draft(create_input(Some(&id))).expect("create");
    let mut edited = created.clone();
    edited.selections = vec![Selection {
        step_id: "kernel".to_string(),
        value: SelectionValue::Single {
            option_id: "archmaker.kernel.linux".to_string(),
        },
    }];
    let saved = core
        .save_draft(SaveDraftInput {
            draft: edited,
            expected_revision: 0,
            changeset: None,
            request_id: None,
        })
        .expect("save");
    assert_eq!(saved.revision, 1);

    let err = core
        .create_draft(create_input(Some(&id)))
        .expect_err("colisión tipada");
    assert_eq!(err.code, "AM-DOC-002");

    let stored = core.stored_draft(&id).expect("intact");
    assert_eq!(stored.revision, 1);
    assert_eq!(stored.selections.len(), 1);
}

#[test]
fn open_and_create_remain_distinct() {
    let core = RealCore::with_embedded().expect("embedded");
    let id = "0d5e6b1a-5555-4a2b-9c3d-555555555555".to_string();

    let missing = core
        .open_draft(OpenDraftInput {
            draft_id: id.clone(),
            seed: None,
            request_id: None,
        })
        .expect_err("open sin draft no crea");
    assert_eq!(missing.code, "AM-DOC-002");

    let created = core.create_draft(create_input(Some(&id))).expect("create");
    assert_eq!(created.revision, 0);

    let opened = core
        .open_draft(OpenDraftInput {
            draft_id: id.clone(),
            seed: None,
            request_id: None,
        })
        .expect("open tras create");
    assert_eq!(opened.id, created.id);
    assert_eq!(opened.revision, 0);

    let conflict = core
        .create_draft(create_input(Some(&id)))
        .expect_err("create no sustituye open");
    assert_eq!(conflict.code, "AM-DOC-002");
}

#[test]
fn production_uuids_unique_v4_batch() {
    let mut seen = HashSet::new();
    for _ in 0..100 {
        let id = new_uuid_v4().expect("entropía nativa");
        assert_uuid_v4(&id);
        assert!(seen.insert(id), "duplicado en lote de producción");
    }
}

#[test]
fn wasm_safe_counter_remains_test_only_path() {
    let first = new_uuid_v4_wasm_safe();
    let second = new_uuid_v4_wasm_safe();
    assert_uuid_v4(&first);
    assert_uuid_v4(&second);
    assert_ne!(first, second);

    let source = include_str!("../src/core.rs");
    assert!(
        source.contains("getrandom::fill(buffer)"),
        "producción WASM debe usar getrandom de plataforma"
    );
    assert!(
        source.contains("/// Solo tests / Prove-It:"),
        "wasm_safe debe quedar documentado como solo tests"
    );
    let prod_fn = source
        .split("pub fn new_uuid_v4()")
        .nth(1)
        .and_then(|rest| rest.split("pub fn new_uuid_v4_wasm_safe()").next())
        .expect("cuerpo new_uuid_v4");
    assert!(
        !prod_fn.contains("fill_with_counter_entropy"),
        "new_uuid_v4 de producción no debe usar el contador"
    );
}

#[test]
fn create_draft_auto_id_uses_production_uuid() {
    let core = RealCore::with_embedded().expect("embedded");
    let draft: Draft = core.create_draft(create_input(None)).expect("create");
    assert_uuid_v4(&draft.id);
}
