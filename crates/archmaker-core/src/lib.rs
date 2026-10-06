pub mod capability;
pub mod core;
pub mod error;
pub mod inputs;
pub mod io;
pub mod limits;
pub mod port;

pub use capability::{
    allowed_ops, check_invoke, check_invoke_named, denied_error, unknown_command_error, Window,
    ALL_OPS,
};

pub use archmaker_domain::{
    Artifact, ArtifactRef, BinaryDigest, BinaryKind, Capability, CapabilityRef, Catalog,
    CatalogOption, CatalogRef, Conflict, ContentDigest, ContentKind, DigestAlgorithm, Draft,
    Libpack, Manifest, ManifestDomainLabel, ManifestRef, PresetRef, Producer, ResolvedSelection,
    Section, Selection, SelectionOrigin, SelectionValue, Severity as DomainSeverity, Step,
    TargetRef, MANIFEST_DOMAIN_LABEL,
};
pub use archmaker_rules::Diagnostic as RuleDiagnostic;
pub use core::{
    catalog_content_digest, draft_content_digest, is_uuid, manifest_content_digest, new_uuid_v4,
    render_artifact_bytes, RealCore, EMBEDDED_CATALOG_JSON, PIPELINE_VERSION,
};
pub use error::{CoreError, ErrorCategory, ErrorFamily, Result, Severity};
pub use inputs::{
    sort_diagnostics, BuildManifestInput, CatalogSource, CreateDraftInput, DestinationHandle,
    ExportArtifactInput, LoadCatalogInput, ResolveDraftInput, ResolveResult, SaveDraftInput,
    ValidateDraftInput, ValidationResult,
};
pub use io::atomic_write_bytes;
pub use limits::{IoClass, Limits, DEFAULT_LIMITS};
pub use port::{CoreOp, CorePort};
