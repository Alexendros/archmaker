use crate::error::{CoreError, Result};
use crate::port::CoreOp;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Window {
    Main,
    Dialog,
    Picker,
}

impl Window {
    pub fn as_str(self) -> &'static str {
        match self {
            Window::Main => "main",
            Window::Dialog => "dialog",
            Window::Picker => "picker",
        }
    }

    pub fn parse(label: &str) -> Option<Window> {
        match label {
            "main" => Some(Window::Main),
            "dialog" => Some(Window::Dialog),
            "picker" => Some(Window::Picker),
            _ => None,
        }
    }
}

impl CoreOp {
    pub fn command_name(self) -> &'static str {
        match self {
            CoreOp::CreateDraft => "create_draft",
            CoreOp::LoadCatalog => "load_catalog",
            CoreOp::SaveDraft => "save_draft",
            CoreOp::OpenDraft => "open_draft",
            CoreOp::ResolveDraft => "resolve_draft",
            CoreOp::ValidateDraft => "validate_draft",
            CoreOp::BuildManifest => "build_manifest",
            CoreOp::ExportArtifact => "export_artifact",
            CoreOp::ImportDraft => "import_draft",
        }
    }

    pub fn parse_command(name: &str) -> Option<CoreOp> {
        match name {
            "create_draft" => Some(CoreOp::CreateDraft),
            "load_catalog" => Some(CoreOp::LoadCatalog),
            "save_draft" => Some(CoreOp::SaveDraft),
            "open_draft" => Some(CoreOp::OpenDraft),
            "resolve_draft" => Some(CoreOp::ResolveDraft),
            "validate_draft" => Some(CoreOp::ValidateDraft),
            "build_manifest" => Some(CoreOp::BuildManifest),
            "export_artifact" => Some(CoreOp::ExportArtifact),
            "import_draft" => Some(CoreOp::ImportDraft),
            _ => None,
        }
    }

    pub fn permission_id(self) -> &'static str {
        match self {
            CoreOp::CreateDraft => "PERM-CMD-CREATE-DRAFT",
            CoreOp::LoadCatalog => "PERM-CMD-LOAD-CATALOG",
            CoreOp::SaveDraft => "PERM-CMD-SAVE-DRAFT",
            CoreOp::OpenDraft => "PERM-CMD-OPEN-DRAFT",
            CoreOp::ResolveDraft => "PERM-CMD-RESOLVE-DRAFT",
            CoreOp::ValidateDraft => "PERM-CMD-VALIDATE-DRAFT",
            CoreOp::BuildManifest => "PERM-CMD-BUILD-MANIFEST",
            CoreOp::ExportArtifact => "PERM-CMD-EXPORT-ARTIFACT",
            CoreOp::ImportDraft => "PERM-CMD-IMPORT-DRAFT",
        }
    }
}

pub const ALL_OPS: [CoreOp; 9] = [
    CoreOp::CreateDraft,
    CoreOp::LoadCatalog,
    CoreOp::SaveDraft,
    CoreOp::OpenDraft,
    CoreOp::ResolveDraft,
    CoreOp::ValidateDraft,
    CoreOp::BuildManifest,
    CoreOp::ExportArtifact,
    CoreOp::ImportDraft,
];

pub fn allowed_ops(window: Window) -> &'static [CoreOp] {
    match window {
        Window::Main => &[
            CoreOp::CreateDraft,
            CoreOp::LoadCatalog,
            CoreOp::SaveDraft,
            CoreOp::OpenDraft,
            CoreOp::ResolveDraft,
            CoreOp::ValidateDraft,
            CoreOp::BuildManifest,
            CoreOp::ExportArtifact,
            CoreOp::ImportDraft,
        ],
        Window::Dialog => &[],
        Window::Picker => &[],
    }
}

pub fn denied_error(window: Window, op: CoreOp) -> CoreError {
    CoreError::proto_denied(&format!(
        "archmaker-invoke.{}.{}",
        window.as_str(),
        op.command_name()
    ))
}

pub fn unknown_command_error(window_label: &str, command: &str) -> CoreError {
    CoreError::proto_unknown(&format!("archmaker-invoke.{window_label}.{command}"))
}

pub fn check_invoke(window: Window, op: CoreOp) -> Result<()> {
    if allowed_ops(window).contains(&op) {
        Ok(())
    } else {
        Err(denied_error(window, op))
    }
}

pub fn check_invoke_named(window_label: &str, command: &str) -> Result<CoreOp> {
    let op = CoreOp::parse_command(command)
        .ok_or_else(|| unknown_command_error(window_label, command))?;
    let window =
        Window::parse(window_label).ok_or_else(|| unknown_command_error(window_label, command))?;
    check_invoke(window, op)?;
    Ok(op)
}
