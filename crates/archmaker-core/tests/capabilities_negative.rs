use archmaker_core::{
    allowed_ops, check_invoke, check_invoke_named, CoreOp, ErrorCategory, ErrorFamily, Severity,
    Window, ALL_OPS,
};

#[test]
fn allowlist_covers_all_seven_ops_exactly_once() {
    let mut seen = Vec::new();
    for window in [Window::Main, Window::Dialog, Window::Picker] {
        seen.extend_from_slice(allowed_ops(window));
    }
    seen.sort_by_key(|op| op.command_name());
    let mut expected = ALL_OPS.to_vec();
    expected.sort_by_key(|op| op.command_name());
    assert_eq!(seen, expected);
}

#[test]
fn main_window_allows_all_seven_ops() {
    for op in ALL_OPS {
        assert!(
            check_invoke(Window::Main, op).is_ok(),
            "main must allow {}",
            op.command_name()
        );
    }
}

#[test]
fn dialog_and_picker_deny_everything_by_default() {
    for window in [Window::Dialog, Window::Picker] {
        for op in ALL_OPS {
            let err = check_invoke(window, op).expect_err("must be denied");
            assert_eq!(err.code, "AM-PROTO-001");
            assert_eq!(err.family, ErrorFamily::Proto);
            assert_eq!(err.category, ErrorCategory::Protocol);
            assert_eq!(err.severity, Severity::Error);
        }
    }
}

#[test]
fn denied_error_is_typed_proto_001() {
    let err = check_invoke(Window::Dialog, CoreOp::LoadCatalog).expect_err("must be denied");
    assert_eq!(err.code, "AM-PROTO-001");
    assert_eq!(err.family, ErrorFamily::Proto);
    assert_eq!(err.category, ErrorCategory::Protocol);
    assert_eq!(err.severity, Severity::Error);
    assert!(!err.message_key.is_empty());
}

#[test]
fn unknown_command_yields_typed_proto_error() {
    let err = check_invoke_named("main", "import_draft").expect_err("must be denied");
    assert_eq!(err.code, "AM-PROTO-002");
    assert_eq!(err.family, ErrorFamily::Proto);
    assert_eq!(err.category, ErrorCategory::Protocol);
}

#[test]
fn unknown_window_yields_typed_proto_error() {
    let err = check_invoke_named("attacker", "create_draft").expect_err("must be denied");
    assert_eq!(err.code, "AM-PROTO-002");
    assert_eq!(err.family, ErrorFamily::Proto);
}

#[test]
fn named_denied_invoke_yields_proto_001() {
    let err = check_invoke_named("picker", "export_artifact").expect_err("must be denied");
    assert_eq!(err.code, "AM-PROTO-001");
}

#[test]
fn named_allowed_invoke_resolves_op() {
    assert_eq!(
        check_invoke_named("main", "build_manifest").expect("allowed"),
        CoreOp::BuildManifest
    );
    assert_eq!(
        check_invoke_named("main", "load_catalog").expect("allowed"),
        CoreOp::LoadCatalog
    );
    assert_eq!(
        check_invoke_named("main", "export_artifact").expect("allowed"),
        CoreOp::ExportArtifact
    );
}

#[test]
fn denied_error_source_identifies_window_and_command() {
    let err = check_invoke(Window::Picker, CoreOp::ResolveDraft).expect_err("must be denied");
    assert!(err.source.contains("picker"), "source: {}", err.source);
    assert!(
        err.source.contains("resolve_draft"),
        "source: {}",
        err.source
    );
    assert!(!err.code.is_empty());
}
