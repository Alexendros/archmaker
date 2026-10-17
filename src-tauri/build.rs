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
            "save_export_bytes",
            "inspect_host_hardware",
            "inspect_storage_devices",
            "refresh_storage_snapshot",
            "propose_storage_plan",
            "validate_storage_plan",
            "import_storage_snapshot",
        ]),
    ))
    .expect("tauri_build with AppManifest::commands");
}
