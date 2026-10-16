fn main() {
    tauri_build::try_build(tauri_build::Attributes::new().app_manifest(
        tauri_build::AppManifest::new().commands(&[
            "create_draft",
            "load_catalog",
            "save_draft",
            "open_draft",
            "resolve_draft",
            "validate_draft",
            "build_manifest",
            "export_artifact",
            "import_draft",
        ]),
    ))
    .expect("tauri_build with AppManifest::commands");
}
