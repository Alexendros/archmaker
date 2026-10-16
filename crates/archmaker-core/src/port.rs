use archmaker_domain::{Artifact, Catalog, Draft, Manifest};

use crate::error::Result;
use crate::inputs::{
    BuildManifestInput, CreateDraftInput, ExportArtifactInput, ImportDraftInput, ImportDraftResult,
    LoadCatalogInput, OpenDraftInput, ResolveDraftInput, ResolveResult, SaveDraftInput,
    ValidateDraftInput, ValidationResult,
};
use crate::limits::IoClass;

pub trait CorePort {
    fn create_draft(&self, input: CreateDraftInput) -> Result<Draft>;
    fn load_catalog(&self, input: LoadCatalogInput) -> Result<Catalog>;
    fn save_draft(&self, input: SaveDraftInput) -> Result<Draft>;
    /// Rehydrate a previously saved draft (memory or store_dir) without bumping revision.
    fn open_draft(&self, input: OpenDraftInput) -> Result<Draft>;
    fn resolve_draft(&self, input: ResolveDraftInput) -> Result<ResolveResult>;
    fn validate_draft(&self, input: ValidateDraftInput) -> Result<ValidationResult>;
    fn build_manifest(&self, input: BuildManifestInput) -> Result<Manifest>;
    fn export_artifact(&self, input: ExportArtifactInput) -> Result<Artifact>;
    /// MVP-1 additive: import legacy v5.1 bytes into a new draft (FR-IMPORT-001).
    fn import_draft(&self, input: ImportDraftInput) -> Result<ImportDraftResult>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CoreOp {
    CreateDraft,
    LoadCatalog,
    SaveDraft,
    OpenDraft,
    ResolveDraft,
    ValidateDraft,
    BuildManifest,
    ExportArtifact,
    ImportDraft,
}

impl CoreOp {
    pub fn io_class(&self) -> IoClass {
        match self {
            CoreOp::CreateDraft
            | CoreOp::ResolveDraft
            | CoreOp::ValidateDraft
            | CoreOp::BuildManifest
            | CoreOp::ImportDraft => IoClass::Pure,
            CoreOp::LoadCatalog | CoreOp::OpenDraft => IoClass::Mixed,
            CoreOp::SaveDraft | CoreOp::ExportArtifact => IoClass::Io,
        }
    }

    pub fn source(&self) -> &'static str {
        match self {
            CoreOp::CreateDraft => "archmaker-core.create-draft",
            CoreOp::LoadCatalog => "archmaker-core.load-catalog",
            CoreOp::SaveDraft => "archmaker-core.save-draft",
            CoreOp::OpenDraft => "archmaker-core.open-draft",
            CoreOp::ResolveDraft => "archmaker-core.resolve-draft",
            CoreOp::ValidateDraft => "archmaker-core.validate-draft",
            CoreOp::BuildManifest => "archmaker-core.build-manifest",
            CoreOp::ExportArtifact => "archmaker-core.export-artifact",
            CoreOp::ImportDraft => "archmaker-core.import-draft",
        }
    }
}
