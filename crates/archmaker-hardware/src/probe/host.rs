//! Contexto de host (nativo / VM / contenedor). Solo lectura.

use std::fs;
use std::path::Path;

use serde::{Deserialize, Serialize};

use crate::compat::HostKind;
use crate::observation::{ObservationOrigin, ProbeObservation, TargetBinding, VerificationStatus};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct HostProbeResult {
    pub kind: ProbeObservation<HostKind>,
    pub os_release_id: ProbeObservation<String>,
    pub machine: ProbeObservation<String>,
}

pub fn probe_host_context(observed_at: &str) -> HostProbeResult {
    let kind_value = detect_host_kind();
    let kind = ProbeObservation::detected(
        kind_value,
        "proc+sys-container-heuristics",
        observed_at,
        ObservationOrigin::LocalProbe,
        TargetBinding::Unbound,
        VerificationStatus::ObservedBound,
    );

    let os_release_id = match read_os_release_id() {
        Some(id) => ProbeObservation::detected(
            id,
            "/etc/os-release",
            observed_at,
            ObservationOrigin::LocalProbe,
            TargetBinding::Unbound,
            VerificationStatus::ObservedBound,
        ),
        None => ProbeObservation::unknown(
            "/etc/os-release",
            observed_at,
            ObservationOrigin::LocalProbe,
            "os-release no legible",
        ),
    };

    let machine = match fs::read_to_string("/proc/sys/kernel/osrelease") {
        Ok(v) => ProbeObservation::detected(
            v.trim().to_string(),
            "/proc/sys/kernel/osrelease",
            observed_at,
            ObservationOrigin::LocalProbe,
            TargetBinding::Unbound,
            VerificationStatus::ObservedBound,
        ),
        Err(_) => ProbeObservation::unknown(
            "/proc/sys/kernel/osrelease",
            observed_at,
            ObservationOrigin::LocalProbe,
            "kernel release no legible",
        ),
    };

    HostProbeResult {
        kind,
        os_release_id,
        machine,
    }
}

fn detect_host_kind() -> HostKind {
    if Path::new("/.dockerenv").exists()
        || Path::new("/run/.containerenv").exists()
        || env_has_container()
        || cgroup_looks_container()
    {
        return HostKind::Container;
    }
    if dmi_is_vm() {
        return HostKind::VirtualMachine;
    }
    if Path::new("/sys/firmware/efi").exists() || Path::new("/sys/devices").exists() {
        HostKind::NativeLinux
    } else {
        HostKind::Restricted
    }
}

fn env_has_container() -> bool {
    std::env::var_os("container").is_some() || std::env::var_os("CONTAINER").is_some()
}

fn cgroup_looks_container() -> bool {
    fs::read_to_string("/proc/1/cgroup")
        .map(|s| {
            s.contains("docker")
                || s.contains("containerd")
                || s.contains("kubepods")
                || s.contains("libpod")
        })
        .unwrap_or(false)
}

fn dmi_is_vm() -> bool {
    let paths = [
        "/sys/class/dmi/id/product_name",
        "/sys/class/dmi/id/sys_vendor",
    ];
    for p in paths {
        if let Ok(s) = fs::read_to_string(p) {
            let l = s.to_ascii_lowercase();
            if l.contains("virtualbox")
                || l.contains("vmware")
                || l.contains("qemu")
                || l.contains("kvm")
                || l.contains("xen")
                || l.contains("hyper-v")
                || l.contains("bochs")
            {
                return true;
            }
        }
    }
    false
}

fn read_os_release_id() -> Option<String> {
    let raw = fs::read_to_string("/etc/os-release").ok()?;
    for line in raw.lines() {
        if let Some(rest) = line.strip_prefix("ID=") {
            return Some(rest.trim_matches('"').to_string());
        }
    }
    None
}
