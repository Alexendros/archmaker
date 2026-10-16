//! Una muestra de tiempos F7-P95: inicialización del core embebido y
//! resolve + validate + build_manifest sobre el fixture de catálogo embebido.
//!
//! El harness Python (`tools/run_f7_p95_harness.py`) ejecuta este test N veces
//! (binario precompilado) y calcula P95. Este binario imprime una línea JSON
//! con `initMs` y `prepareMs` cuando se invoca con `ARCHMAKER_F7_P95_SAMPLE=1`.

use std::time::Instant;

use archmaker_core::{
    BuildManifestInput, CorePort, CreateDraftInput, RealCore, ResolveDraftInput, ValidateDraftInput,
};
use archmaker_domain::{CatalogRef, Draft, Selection, SelectionValue, TargetRef};

fn single(step: &str, option: &str) -> Selection {
    Selection {
        step_id: step.to_string(),
        value: SelectionValue::Single {
            option_id: option.to_string(),
        },
    }
}

fn draft_target() -> TargetRef {
    TargetRef {
        id: "arch-x86_64".to_string(),
        version: "2026.10.0".to_string(),
    }
}

fn export_target() -> TargetRef {
    TargetRef {
        id: "archmaker-profile".to_string(),
        version: "0.1.0".to_string(),
    }
}

fn valid_draft(core: &RealCore, catalog: CatalogRef) -> Draft {
    let mut draft = core
        .create_draft(CreateDraftInput {
            draft_id: None,
            catalog_ref: catalog,
            target_ref: draft_target(),
            preset_ref: None,
            request_id: None,
        })
        .expect("create draft");
    draft.selections = vec![
        single("kernel", "archmaker.kernel.linux"),
        single("session", "archmaker.desktop.gnome"),
    ];
    draft
}

fn sample_once() -> (f64, f64) {
    let init_start = Instant::now();
    let core = RealCore::with_embedded().expect("embedded catalog");
    let catalog = core.active_catalog_ref().expect("catalog ref");
    let init_ms = init_start.elapsed().as_secs_f64() * 1000.0;

    let draft = valid_draft(&core, catalog.clone());
    let prepare_start = Instant::now();
    let _resolved = core
        .resolve_draft(ResolveDraftInput {
            draft: draft.clone(),
            catalog_ref: catalog.clone(),
            request_id: None,
        })
        .expect("resolve");
    let _validated = core
        .validate_draft(ValidateDraftInput {
            draft: draft.clone(),
            catalog_ref: catalog.clone(),
            target_ref: None,
            request_id: None,
        })
        .expect("validate");
    let _manifest = core
        .build_manifest(BuildManifestInput {
            draft,
            catalog_ref: catalog,
            target_ref: export_target(),
            expected_resolution_digest: None,
            request_id: None,
        })
        .expect("manifest");
    let prepare_ms = prepare_start.elapsed().as_secs_f64() * 1000.0;
    (init_ms, prepare_ms)
}

#[test]
fn f7_p95_one_sample_prints_json_when_env_set() {
    let (init_ms, prepare_ms) = sample_once();
    assert!(init_ms >= 0.0);
    assert!(prepare_ms >= 0.0);
    if std::env::var_os("ARCHMAKER_F7_P95_SAMPLE").is_some() {
        println!(
            "{{\"initMs\":{init_ms:.6},\"prepareMs\":{prepare_ms:.6},\"fixture\":\"embedded-catalog\"}}"
        );
    }
}
