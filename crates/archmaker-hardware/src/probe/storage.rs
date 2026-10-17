//! Inventario de discos vía lsblk con argv fijo (sin sh -c).

use std::process::{Command, Stdio};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::graph::{build_usage_graph, SessionDeviceRegistry, StorageSnapshot};
use crate::lsblk::parse_lsblk_json;
use crate::observation::{ObservationOrigin, TargetBinding};

pub const DEFAULT_LSBLK_ARGS: &[&str] = &[
    "--json",
    "--bytes",
    "--tree",
    "--output",
    "NAME,PATH,TYPE,SIZE,RO,RM,TRAN,MODEL,SERIAL,WWN,LOG-SEC,PHY-SEC,PTTYPE,FSTYPE,FSVER,MOUNTPOINTS",
];

const LSBLK_MAX_BYTES: usize = 4 * 1024 * 1024;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StorageProbeError {
    Spawn(String),
    NonZeroExit(i32, String),
    OutputTooLarge,
    Parse(String),
}

impl std::fmt::Display for StorageProbeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Spawn(e) => write!(f, "no se pudo ejecutar lsblk: {e}"),
            Self::NonZeroExit(c, e) => write!(f, "lsblk exit {c}: {e}"),
            Self::OutputTooLarge => write!(f, "salida lsblk excede límite"),
            Self::Parse(e) => write!(f, "parseo lsblk: {e}"),
        }
    }
}

impl std::error::Error for StorageProbeError {}

fn now_stamp() -> String {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs().to_string())
        .unwrap_or_else(|_| "0".into())
}

fn run_lsblk() -> Result<String, StorageProbeError> {
    let output = Command::new("/usr/bin/lsblk")
        .args(DEFAULT_LSBLK_ARGS)
        .env("LC_ALL", "C")
        .stdin(Stdio::null())
        .output()
        .or_else(|_| {
            Command::new("lsblk")
                .args(DEFAULT_LSBLK_ARGS)
                .env("LC_ALL", "C")
                .stdin(Stdio::null())
                .output()
        })
        .map_err(|e| StorageProbeError::Spawn(e.to_string()))?;

    if !output.status.success() {
        return Err(StorageProbeError::NonZeroExit(
            output.status.code().unwrap_or(-1),
            String::from_utf8_lossy(&output.stderr).into_owned(),
        ));
    }
    if output.stdout.len() > LSBLK_MAX_BYTES {
        return Err(StorageProbeError::OutputTooLarge);
    }
    String::from_utf8(output.stdout).map_err(|e| StorageProbeError::Parse(e.to_string()))
}

pub fn inspect_storage_devices(
    target_binding: TargetBinding,
) -> Result<(StorageSnapshot, SessionDeviceRegistry), StorageProbeError> {
    let raw = run_lsblk()?;
    let doc = parse_lsblk_json(&raw).map_err(|e| StorageProbeError::Parse(e.to_string()))?;
    let snap_id = format!("snap-{}", now_stamp());
    Ok(build_usage_graph(
        &doc,
        snap_id,
        now_stamp(),
        ObservationOrigin::LocalProbe,
        target_binding,
    ))
}

pub fn refresh_storage_snapshot(
    previous: &StorageSnapshot,
    target_binding: TargetBinding,
) -> Result<(StorageSnapshot, SessionDeviceRegistry), StorageProbeError> {
    let (mut next, registry) = inspect_storage_devices(target_binding)?;
    if next.semantic_fingerprint != previous.semantic_fingerprint {
        next.diagnostic = Some("snapshot stale: geometría/identidad cambió tras refresco".into());
    }
    Ok((next, registry))
}
