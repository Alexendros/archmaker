//! Grafo de uso de discos: identidad opaca, protecciones, observed≠planned.

use std::collections::{BTreeMap, BTreeSet};

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

use crate::lsblk::{LsblkBlock, LsblkDocument};
use crate::observation::{
    ObservationOrigin, ProbeObservation, ProbeStatus, TargetBinding, VerificationStatus,
};
use archmaker_domain::{
    PartitionTableKind, PartitionTableObservation, PartitionTableObservationStatus,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ProtectionReason {
    RootFilesystem,
    EspMounted,
    SwapActive,
    LiveMedia,
    LoopDevice,
    Overlay,
    Squashfs,
    DeviceMapperMember,
    LvmMember,
    RaidMember,
    MultipathMember,
    Mounted,
    AmbiguousIdentity,
    DuplicateIdentity,
    UnsupportedTopology,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum ReplaceEligibility {
    Eligible,
    Protected,
    Ambiguous,
    Unknown,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DeviceNode {
    pub handle_token: String,
    pub parent_token: Option<String>,
    pub device_type: String,
    pub size_bytes: Option<u64>,
    pub logical_sector_bytes: Option<u64>,
    pub physical_sector_bytes: Option<u64>,
    pub transport: Option<String>,
    pub model: Option<String>,
    pub read_only: Option<bool>,
    pub removable: Option<bool>,
    pub fstype: Option<String>,
    pub mountpoints: Vec<String>,
    pub partition_table: PartitionTableObservation,
    pub protections: Vec<ProtectionReason>,
    pub replace_eligibility: ReplaceEligibility,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StorageDevicePublic {
    pub handle_token: String,
    /// Alias portable del handle (opaco; sin /dev).
    pub logical_target_ref: archmaker_domain::LogicalTargetRef,
    pub model: Option<String>,
    pub size_bytes: Option<u64>,
    pub logical_sector_bytes: Option<u64>,
    pub transport: Option<String>,
    pub replace_eligibility: ReplaceEligibility,
    pub protections: Vec<ProtectionReason>,
    pub partition_table: Option<PartitionTableKind>,
    pub partition_table_observation: PartitionTableObservation,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct DeviceUsageGraph {
    pub nodes: Vec<DeviceNode>,
    pub roots: Vec<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct StorageSnapshot {
    pub snapshot_id: String,
    pub observed_at: String,
    pub origin: ObservationOrigin,
    pub target_binding: TargetBinding,
    pub verification_status: VerificationStatus,
    pub graph: DeviceUsageGraph,
    /// Discos físicos públicos (handles opacos). Alias de planificación.
    pub devices: Vec<StorageDevicePublic>,
    /// Tabla observada por disco (ortogonal a cualquier plan).
    pub observed_tables: BTreeMap<String, PartitionTableObservation>,
    /// Huella semántica (particiones/mounts/identidad/geometría); no incluye RAM libre.
    pub semantic_fingerprint: String,
    #[serde(skip_serializing_if = "Option::is_none", default)]
    pub diagnostic: Option<String>,
}

/// Identidad privada de sesión (nunca exportable).
#[derive(Debug, Clone)]
pub struct PrivateDeviceIdentity {
    pub path: Option<String>,
    pub serial: Option<String>,
    pub wwn: Option<String>,
    pub name: Option<String>,
}

#[derive(Debug, Clone, Default)]
pub struct SessionDeviceRegistry {
    pub by_token: BTreeMap<String, PrivateDeviceIdentity>,
}

fn token_for(identity: &PrivateDeviceIdentity, salt: &str) -> String {
    let mut h = Sha256::new();
    h.update(salt.as_bytes());
    h.update(b"|");
    h.update(identity.path.as_deref().unwrap_or("").as_bytes());
    h.update(b"|");
    h.update(identity.serial.as_deref().unwrap_or("").as_bytes());
    h.update(b"|");
    h.update(identity.wwn.as_deref().unwrap_or("").as_bytes());
    h.update(b"|");
    h.update(identity.name.as_deref().unwrap_or("").as_bytes());
    format!("dev_{}", hex::encode(&h.finalize()[..16]))
}

fn parse_pttype(raw: Option<&str>) -> PartitionTableObservation {
    match raw.map(|s| s.to_ascii_lowercase()).as_deref() {
        Some("gpt") => PartitionTableObservation {
            status: PartitionTableObservationStatus::Observed,
            kind: Some(PartitionTableKind::Gpt),
        },
        Some("dos") | Some("msdos") | Some("mbr") => PartitionTableObservation {
            status: PartitionTableObservationStatus::Observed,
            kind: Some(PartitionTableKind::Mbr),
        },
        Some("") | None => PartitionTableObservation {
            status: PartitionTableObservationStatus::Unknown,
            kind: None,
        },
        Some(_) => PartitionTableObservation {
            status: PartitionTableObservationStatus::Observed,
            kind: None,
        },
    }
}

fn collect_mounts(block: &LsblkBlock, out: &mut BTreeSet<String>) {
    for m in &block.mountpoints {
        if !m.is_empty() {
            out.insert(m.clone());
        }
    }
    for child in &block.children {
        collect_mounts(child, out);
    }
}

fn collect_subtree_fstypes(block: &LsblkBlock, out: &mut BTreeSet<String>) {
    if let Some(fs) = &block.fstype {
        out.insert(fs.to_ascii_lowercase());
    }
    for child in &block.children {
        collect_subtree_fstypes(child, out);
    }
}

fn classify_protections(
    block: &LsblkBlock,
    subtree_mounts: &BTreeSet<String>,
    subtree_fstypes: &BTreeSet<String>,
    identity_ambiguous: bool,
    identity_duplicate: bool,
) -> Vec<ProtectionReason> {
    let mut reasons = Vec::new();
    let dtype = block.device_type.as_deref().unwrap_or("");
    let fstype = block.fstype.as_deref().unwrap_or("").to_ascii_lowercase();

    if identity_ambiguous {
        reasons.push(ProtectionReason::AmbiguousIdentity);
    }
    if identity_duplicate {
        reasons.push(ProtectionReason::DuplicateIdentity);
    }

    if subtree_mounts.iter().any(|m| m == "/") {
        reasons.push(ProtectionReason::RootFilesystem);
    }
    if subtree_mounts
        .iter()
        .any(|m| m == "/boot" || m == "/boot/efi" || m == "/efi" || m.starts_with("/boot/"))
    {
        reasons.push(ProtectionReason::EspMounted);
    }
    if subtree_mounts
        .iter()
        .any(|m| m.starts_with("[SWAP]") || m == "SWAP")
        || fstype == "swap"
    {
        reasons.push(ProtectionReason::SwapActive);
    }
    if !subtree_mounts.is_empty() {
        reasons.push(ProtectionReason::Mounted);
    }

    if dtype == "loop" || block.name.as_deref().unwrap_or("").starts_with("loop") {
        reasons.push(ProtectionReason::LoopDevice);
    }
    if matches!(fstype.as_str(), "squashfs" | "iso9660") {
        reasons.push(ProtectionReason::Squashfs);
        reasons.push(ProtectionReason::LiveMedia);
    }
    if fstype.contains("overlay") || subtree_mounts.iter().any(|m| m.contains("overlay")) {
        reasons.push(ProtectionReason::Overlay);
        reasons.push(ProtectionReason::LiveMedia);
    }
    if dtype == "rom"
        || subtree_mounts
            .iter()
            .any(|m| m.contains("archiso") || m.contains("casper") || m.contains("/live"))
        || (block.tran.as_deref().unwrap_or("") == "usb"
            && subtree_mounts
                .iter()
                .any(|m| m == "/" || m.contains("archiso")))
    {
        reasons.push(ProtectionReason::LiveMedia);
    }
    if subtree_fstypes.contains("linux_raid_member") {
        reasons.push(ProtectionReason::RaidMember);
    }
    if subtree_fstypes.contains("lvm2_member") {
        reasons.push(ProtectionReason::LvmMember);
    }
    if subtree_fstypes.contains("crypto_luks") {
        reasons.push(ProtectionReason::DeviceMapperMember);
    }

    if dtype == "lvm" || dtype.starts_with("lvm") {
        reasons.push(ProtectionReason::LvmMember);
    }
    if dtype == "crypt" || fstype == "crypto_LUKS" {
        reasons.push(ProtectionReason::DeviceMapperMember);
    }
    if dtype == "raid" || dtype.starts_with("raid") || dtype == "md" {
        reasons.push(ProtectionReason::RaidMember);
    }
    if dtype == "mpath" || dtype.contains("multipath") {
        reasons.push(ProtectionReason::MultipathMember);
    }
    if matches!(
        dtype,
        "raid0" | "raid1" | "raid5" | "raid6" | "raid10" | "dm" | "crypt" | "lvm"
    ) {
        reasons.push(ProtectionReason::UnsupportedTopology);
    }

    reasons.sort_by_key(|r| format!("{r:?}"));
    reasons.dedup();
    reasons
}

fn eligibility(reasons: &[ProtectionReason]) -> ReplaceEligibility {
    if reasons.iter().any(|r| {
        matches!(
            r,
            ProtectionReason::AmbiguousIdentity | ProtectionReason::DuplicateIdentity
        )
    }) {
        return ReplaceEligibility::Ambiguous;
    }
    if reasons.is_empty() {
        return ReplaceEligibility::Eligible;
    }
    ReplaceEligibility::Protected
}

fn walk(
    block: &LsblkBlock,
    parent: Option<&str>,
    snapshot_salt: &str,
    identity_counts: &BTreeMap<String, usize>,
    nodes: &mut Vec<DeviceNode>,
    registry: &mut SessionDeviceRegistry,
    roots: &mut Vec<String>,
) {
    let identity = PrivateDeviceIdentity {
        path: block.path.clone(),
        serial: block.serial.clone(),
        wwn: block.wwn.clone(),
        name: block.name.clone(),
    };
    // Duplicados por serial+wwn cuando ambos existen (path no basta para distinguir).
    let key = if identity.serial.as_ref().is_some_and(|s| !s.is_empty())
        || identity.wwn.as_ref().is_some_and(|s| !s.is_empty())
    {
        format!(
            "{}|{}",
            identity.serial.as_deref().unwrap_or(""),
            identity.wwn.as_deref().unwrap_or("")
        )
    } else {
        format!("path|{}", identity.path.as_deref().unwrap_or(""))
    };
    let duplicate = identity_counts.get(&key).copied().unwrap_or(0) > 1;
    let ambiguous = identity.serial.is_none() && identity.wwn.is_none() && identity.path.is_none();

    let token = token_for(&identity, snapshot_salt);
    registry.by_token.insert(token.clone(), identity);

    let mut subtree_mounts = BTreeSet::new();
    collect_mounts(block, &mut subtree_mounts);
    let mut subtree_fstypes = BTreeSet::new();
    collect_subtree_fstypes(block, &mut subtree_fstypes);
    let protections = classify_protections(
        block,
        &subtree_mounts,
        &subtree_fstypes,
        ambiguous,
        duplicate,
    );
    let replace_eligibility = eligibility(&protections);
    let partition_table = parse_pttype(block.pttype.as_deref());

    let node = DeviceNode {
        handle_token: token.clone(),
        parent_token: parent.map(str::to_string),
        device_type: block
            .device_type
            .clone()
            .unwrap_or_else(|| "unknown".into()),
        size_bytes: block.size,
        logical_sector_bytes: block.log_sec,
        physical_sector_bytes: block.phy_sec,
        transport: block.tran.clone(),
        model: block.model.clone(),
        read_only: block.ro,
        removable: block.rm,
        fstype: block.fstype.clone(),
        mountpoints: block.mountpoints.clone(),
        partition_table: partition_table.clone(),
        protections,
        replace_eligibility,
    };

    if parent.is_none() {
        roots.push(token.clone());
    }
    nodes.push(node);

    for child in &block.children {
        walk(
            child,
            Some(&token),
            snapshot_salt,
            identity_counts,
            nodes,
            registry,
            roots,
        );
    }
}

fn count_identities(block: &LsblkBlock, counts: &mut BTreeMap<String, usize>) {
    if block.device_type.as_deref() == Some("disk") {
        let key = if block.serial.as_ref().is_some_and(|s| !s.is_empty())
            || block.wwn.as_ref().is_some_and(|s| !s.is_empty())
        {
            format!(
                "{}|{}",
                block.serial.as_deref().unwrap_or(""),
                block.wwn.as_deref().unwrap_or("")
            )
        } else {
            format!("path|{}", block.path.as_deref().unwrap_or(""))
        };
        *counts.entry(key).or_default() += 1;
    }
    for child in &block.children {
        count_identities(child, counts);
    }
}

pub fn build_usage_graph(
    doc: &LsblkDocument,
    snapshot_id: impl Into<String>,
    observed_at: impl Into<String>,
    origin: ObservationOrigin,
    target_binding: TargetBinding,
) -> (StorageSnapshot, SessionDeviceRegistry) {
    let snapshot_id = snapshot_id.into();
    let observed_at = observed_at.into();
    let mut identity_counts = BTreeMap::new();
    for b in &doc.blockdevices {
        count_identities(b, &mut identity_counts);
    }

    let mut nodes = Vec::new();
    let mut roots = Vec::new();
    let mut registry = SessionDeviceRegistry::default();
    let salt = format!("{snapshot_id}|{observed_at}");

    for b in &doc.blockdevices {
        walk(
            b,
            None,
            &salt,
            &identity_counts,
            &mut nodes,
            &mut registry,
            &mut roots,
        );
    }

    let mut observed_tables = BTreeMap::new();
    let mut devices = Vec::new();
    for n in &nodes {
        if n.parent_token.is_none() && n.device_type == "disk" {
            observed_tables.insert(n.handle_token.clone(), n.partition_table.clone());
            devices.push(StorageDevicePublic {
                handle_token: n.handle_token.clone(),
                logical_target_ref: archmaker_domain::LogicalTargetRef {
                    id: n.handle_token.clone(),
                },
                model: n.model.clone(),
                size_bytes: n.size_bytes,
                logical_sector_bytes: n.logical_sector_bytes,
                transport: n.transport.clone(),
                replace_eligibility: n.replace_eligibility,
                protections: n.protections.clone(),
                partition_table: n.partition_table.kind,
                partition_table_observation: n.partition_table.clone(),
            });
        }
    }

    // Ningún disco preseleccionado: eligibility se calcula; la UI no marca por defecto.
    let verification_status = if nodes
        .iter()
        .any(|n| matches!(n.replace_eligibility, ReplaceEligibility::Ambiguous))
    {
        VerificationStatus::AmbiguousIdentity
    } else if nodes.is_empty() {
        VerificationStatus::InsufficientEvidence
    } else {
        match (origin, target_binding) {
            (ObservationOrigin::LocalProbe, TargetBinding::CurrentHostConfirmed) => {
                VerificationStatus::ObservedBound
            }
            (ObservationOrigin::LocalProbe, _) => VerificationStatus::InsufficientEvidence,
            (ObservationOrigin::ImportedReport, _) => VerificationStatus::ImportedUnverified,
            (ObservationOrigin::ManualDeclaration, _) => VerificationStatus::DeclaredOnly,
        }
    };

    let semantic_fingerprint = {
        let mut h = Sha256::new();
        for n in &nodes {
            h.update(n.handle_token.as_bytes());
            h.update(n.device_type.as_bytes());
            if let Some(sz) = n.size_bytes {
                h.update(sz.to_le_bytes());
            }
            if let Some(sec) = n.logical_sector_bytes {
                h.update(sec.to_le_bytes());
            }
            for m in &n.mountpoints {
                h.update(m.as_bytes());
            }
            h.update(format!("{:?}", n.partition_table.kind).as_bytes());
        }
        hex::encode(&h.finalize())
    };

    let snapshot = StorageSnapshot {
        snapshot_id,
        observed_at,
        origin,
        target_binding,
        verification_status,
        graph: DeviceUsageGraph { nodes, roots },
        devices,
        observed_tables,
        semantic_fingerprint,
        diagnostic: None,
    };
    (snapshot, registry)
}

pub fn empty_storage_observation(
    source: impl Into<String>,
    observed_at: impl Into<String>,
    origin: ObservationOrigin,
    diagnostic: impl Into<String>,
) -> ProbeObservation<StorageSnapshot> {
    ProbeObservation {
        status: ProbeStatus::Unavailable,
        value: None,
        source: source.into(),
        observed_at: observed_at.into(),
        origin,
        target_binding: TargetBinding::Unbound,
        verification_status: VerificationStatus::Unavailable,
        diagnostic: Some(diagnostic.into()),
    }
}

/// Snapshot desde JSON lsblk (fixtures / importación). Sin IO de host.
pub fn snapshot_from_lsblk_json(
    raw_json: &str,
    snapshot_id: impl Into<String>,
    observed_at: impl Into<String>,
    origin: ObservationOrigin,
    target_binding: TargetBinding,
) -> Result<(StorageSnapshot, SessionDeviceRegistry), String> {
    let doc = crate::lsblk::parse_lsblk_json(raw_json)?;
    Ok(build_usage_graph(
        &doc,
        snapshot_id,
        observed_at,
        origin,
        target_binding,
    ))
}

/// Hex mínimo sin dependencia `hex` crate.
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{b:02x}")).collect()
    }
}
