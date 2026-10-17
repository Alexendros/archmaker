//! Sonda Linux de solo lectura. Feature `probe` + `target_os = "linux"`.
//! Cero escrituras en discos/EFI/efivarfs.

mod firmware;
mod host;
mod storage;

pub use firmware::{inspect_host_hardware, FirmwareProbeReport, HostHardwareReport};
pub use host::{probe_host_context, HostProbeResult};
pub use storage::{inspect_storage_devices, refresh_storage_snapshot, StorageProbeError};
