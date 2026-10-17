//! Parser de salida `lsblk --json` (solo lectura). Sin ejecución.

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LsblkDocument {
    #[serde(default)]
    pub blockdevices: Vec<LsblkBlock>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LsblkBlock {
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub path: Option<String>,
    #[serde(default, rename = "type")]
    pub device_type: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    #[serde(default)]
    pub ro: Option<bool>,
    #[serde(default)]
    pub rm: Option<bool>,
    #[serde(default)]
    pub tran: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default)]
    pub serial: Option<String>,
    #[serde(default)]
    pub wwn: Option<String>,
    #[serde(default, rename = "log-sec")]
    pub log_sec: Option<u64>,
    #[serde(default, rename = "phy-sec")]
    pub phy_sec: Option<u64>,
    #[serde(default)]
    pub pttype: Option<String>,
    #[serde(default)]
    pub fstype: Option<String>,
    #[serde(default)]
    pub fsver: Option<String>,
    /// lsblk puede emitir string o array según versión.
    #[serde(default, deserialize_with = "deserialize_mountpoints")]
    pub mountpoints: Vec<String>,
    #[serde(default)]
    pub children: Vec<LsblkBlock>,
}

fn deserialize_mountpoints<'de, D>(deserializer: D) -> Result<Vec<String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Raw {
        Many(Vec<Option<String>>),
        One(Option<String>),
        Absent,
    }
    let raw = Option::<Raw>::deserialize(deserializer)?;
    Ok(match raw {
        None | Some(Raw::Absent) => Vec::new(),
        Some(Raw::One(v)) => v.into_iter().collect(),
        Some(Raw::Many(vs)) => vs.into_iter().flatten().collect(),
    })
}

pub fn parse_lsblk_json(raw: &str) -> Result<LsblkDocument, String> {
    serde_json::from_str(raw).map_err(|e| format!("lsblk JSON inválido: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_minimal_tree() {
        let doc = parse_lsblk_json(
            r#"{
              "blockdevices": [{
                "name": "sda",
                "path": "/dev/sda",
                "type": "disk",
                "size": 1000000000,
                "children": [{
                  "name": "sda1",
                  "path": "/dev/sda1",
                  "type": "part",
                  "fstype": "vfat",
                  "mountpoints": ["/boot"]
                }]
              }]
            }"#,
        )
        .unwrap();
        assert_eq!(doc.blockdevices.len(), 1);
        assert_eq!(doc.blockdevices[0].children[0].mountpoints, vec!["/boot"]);
    }
}
