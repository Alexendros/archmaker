//! Hardware observation + storage planning (Forge Plan A, A6 fuera).
//!
//! - Tipos y reglas portables: siempre disponibles (datos ya capturados).
//! - Sonda Linux (`feature = "probe"`): solo lectura; no entra en WASM/core.

pub mod compat;
pub mod graph;
pub mod help;
pub mod lsblk;
pub mod observation;
pub mod plan;
pub mod policy;

#[cfg(all(feature = "probe", target_os = "linux"))]
pub mod probe;

pub use compat::{
    evaluate_firmware_compat, CompatCode, CompatVerdict, CurrentBootMode, FirmwareCompatInput,
    FirmwareCompatReport, HostKind, SecureBootState, UefiCapability,
};
pub use graph::{
    build_usage_graph, empty_storage_observation, snapshot_from_lsblk_json, DeviceNode,
    DeviceUsageGraph, PrivateDeviceIdentity, ProtectionReason, ReplaceEligibility,
    SessionDeviceRegistry, StorageDevicePublic, StorageSnapshot,
};
pub use help::{boot_filesystem_help, validate_uki_esp_access, BootFsHelp, UkiEspAccess};
pub use lsblk::{parse_lsblk_json, LsblkBlock, LsblkDocument};
pub use observation::{
    ObservationOrigin, ProbeObservation, ProbeStatus, TargetBinding, VerificationStatus,
};
pub use plan::{
    propose_storage_plan, validate_storage_plan, PlanRequest, PlanResult, PlanningPolicy,
    PLANNING_POLICY_VERSION,
};
pub use policy::StoragePlanningLimits;
