use archmaker_wasm::guarded_invoke;
use serde_json::Value;

const MINIMAL_DRAFT_INPUT: &str = r#"{"draftId":null,"catalogRef":{"namespace":"archmaker.core","id":"minimal","version":"0.1.0","digest":{"algorithm":"sha256","value":"00"}},"targetRef":{"id":"arch-x86_64","version":"1"},"requestId":null}"#;

fn typed_err(json: &str) -> Value {
    serde_json::from_str(json).expect("invoke errors are typed JSON")
}

#[test]
fn denied_invoke_returns_typed_proto_001() {
    let err = typed_err(
        &guarded_invoke("dialog", "export_artifact", MINIMAL_DRAFT_INPUT)
            .expect_err("dialog must not export"),
    );
    assert_eq!(err["code"], "AM-PROTO-001");
    assert_eq!(err["family"], "AM-PROTO");
    assert_eq!(err["category"], "protocol");
    assert_eq!(err["severity"], "error");
}

#[test]
fn allowed_invoke_passes_capability_gate() {
    let err = typed_err(
        &guarded_invoke("main", "create_draft", r#"{"draftId":null}"#)
            .expect_err("invalid input still fails, but past the gate"),
    );
    assert_ne!(err["code"], "AM-PROTO-001");
    assert_ne!(err["code"], "AM-PROTO-002");
    assert_eq!(err["code"], "AM-SCHEMA-001");
}

#[test]
fn unknown_command_returns_typed_proto_002() {
    let err = typed_err(&guarded_invoke("main", "import_draft", "{}").expect_err("out of v0"));
    assert_eq!(err["code"], "AM-PROTO-002");
    assert_eq!(err["family"], "AM-PROTO");
}

#[test]
fn unknown_window_returns_typed_proto_002() {
    let err = typed_err(
        &guarded_invoke("attacker", "create_draft", MINIMAL_DRAFT_INPUT)
            .expect_err("unknown window"),
    );
    assert_eq!(err["code"], "AM-PROTO-002");
}

#[test]
fn malformed_json_returns_typed_schema_error() {
    let err =
        typed_err(&guarded_invoke("main", "create_draft", "{not json").expect_err("malformed"));
    assert_eq!(err["code"], "AM-SCHEMA-001");
    assert_eq!(err["family"], "AM-SCHEMA");
}
