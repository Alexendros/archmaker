pub mod capability;
pub mod contract;
pub mod core;
pub mod error;
pub mod inputs;
pub mod io;
pub mod limits;
pub mod migration;
pub mod port;

pub use capability::{
    allowed_ops, check_invoke, check_invoke_named, denied_error, unknown_command_error, Window,
    ALL_OPS,
};
pub use contract::{verify_application_contract, verify_clause, ClauseVerdict, ContractVerdict};

pub use archmaker_domain::{
    is_document_version, is_integer_version, is_semver, Artifact, ArtifactRef, BinaryDigest,
    BinaryKind, Capability, CapabilityRef, Catalog, CatalogOption, CatalogProvenance, CatalogRef,
    Conflict, ContentDigest, ContentKind, DigestAlgorithm, Draft, Libpack, Manifest,
    ManifestDomainLabel, ManifestRef, PresetRef, Producer, ResolvedSelection, Section, Selection,
    SelectionOrigin, SelectionValue, Severity as DomainSeverity, Step, TargetRef,
    MANIFEST_DOMAIN_LABEL,
};
pub use archmaker_rules::Diagnostic as RuleDiagnostic;
pub use core::{
    catalog_content_digest, draft_content_digest, is_uuid, manifest_content_digest, new_uuid_v4,
    render_artifact_bytes, RealCore, RenderedArtifact, DESKTOP_CATALOG_JSON, EMBEDDED_CATALOG_JSON,
    PIPELINE_VERSION,
};
pub use error::{CoreError, ErrorCategory, ErrorFamily, Result, Severity};
pub use inputs::{
    sort_diagnostics, BuildManifestInput, CatalogSource, ChangeSet, ChangeSetOp,
    ChangeSetOperation, ChangeSetOrigin, ChangeSetSummary, CreateDraftInput, DestinationHandle,
    ExportArtifactInput, ImportDraftInput, ImportDraftResult, LoadCatalogInput, MigrationEntry,
    MigrationFate, MigrationReport, OpenDraftInput, ResolveDraftInput, ResolveResult,
    SaveDraftInput, ValidateDraftInput, ValidationResult,
};
pub use io::atomic_write_bytes;
pub use limits::{IoClass, Limits, DEFAULT_LIMITS};
pub use port::{CoreOp, CorePort};
