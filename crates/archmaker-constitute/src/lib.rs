//! Descarga tipada de los nombres del perfil vivo (ISO-03).
//!
//! Un solo proceso, una sola operación. No abre red, no ejecuta shell y no
//! escribe bajo `airootfs/`. El binario no acepta scripts.

use std::collections::BTreeSet;
use std::fs;
use std::io::Read;
use std::path::{Component, Path};

use sha2::{Digest, Sha256};

pub const CODE_OFFLINE: &str = "constitution-offline";
pub const CODE_OUTSIDE: &str = "constitution-name-outside-profile";
pub const CODE_DEFERRED: &str = "constitution-deferred-not-embedded";
pub const CODE_SCRIPT: &str = "constitution-script-rejected";
pub const CODE_DIGEST: &str = "constitution-digest-mismatch";
pub const CODE_OUTPUT: &str = "constitution-unsafe-output";
pub const CODE_IMAGE_ABSENT: &str = "constitution-mkarchiso-absent";
pub const CODE_IMAGE_REFUSED: &str = "constitution-mkarchiso-refused";

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConstituteError {
    pub code: &'static str,
    pub name: Option<String>,
}

impl ConstituteError {
    fn new(code: &'static str) -> Self {
        Self { code, name: None }
    }

    fn named(code: &'static str, name: impl Into<String>) -> Self {
        Self {
            code,
            name: Some(name.into()),
        }
    }

    pub fn to_json(&self) -> String {
        let mut body = serde_json::json!({
            "ok": false,
            "code": self.code,
        });
        if let Some(name) = &self.name {
            body["name"] = serde_json::Value::String(name.clone());
        }
        body.to_string()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct JournalEntry {
    pub name: String,
    pub sha256: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LiveProfilePackages {
    pub install: BTreeSet<String>,
    pub deferred: BTreeSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FetchOutcome {
    pub journal: Vec<JournalEntry>,
}

pub fn parse_live_profile(bytes: &[u8]) -> Result<LiveProfilePackages, ConstituteError> {
    let value: serde_json::Value =
        serde_json::from_slice(bytes).map_err(|_| ConstituteError::new(CODE_SCRIPT))?;
    if value.get("script").is_some()
        || value.get("cmd").is_some()
        || value.get("pacstrap").is_some()
    {
        return Err(ConstituteError::new(CODE_SCRIPT));
    }
    let packages = value
        .get("files")
        .and_then(|files| files.get("packages.x86_64"))
        .and_then(|text| text.as_str())
        .ok_or_else(|| ConstituteError::new(CODE_SCRIPT))?;
    let mut install = BTreeSet::new();
    for line in packages.lines() {
        let name = line.trim();
        if name.is_empty() {
            continue;
        }
        if !package_name_ok(name) {
            return Err(ConstituteError::new(CODE_SCRIPT));
        }
        install.insert(name.to_string());
    }
    if install.is_empty() {
        return Err(ConstituteError::new(CODE_SCRIPT));
    }
    let mut deferred = BTreeSet::new();
    if let Some(list) = value.get("deferredFetch").and_then(|v| v.as_array()) {
        for item in list {
            let Some(name) = item.as_str() else {
                return Err(ConstituteError::new(CODE_SCRIPT));
            };
            if !package_name_ok(name) {
                return Err(ConstituteError::new(CODE_SCRIPT));
            }
            deferred.insert(name.to_string());
        }
    }
    for name in &deferred {
        install.remove(name);
    }
    Ok(LiveProfilePackages { install, deferred })
}

pub fn fetch_packages(
    profile: &LiveProfilePackages,
    names: &[String],
    fixture_dir: &Path,
    out_dir: &Path,
) -> Result<FetchOutcome, ConstituteError> {
    if path_contains_airootfs(out_dir) || path_contains_airootfs(fixture_dir) {
        return Err(ConstituteError::new(CODE_OUTPUT));
    }
    if !fixture_dir.is_dir() {
        return Err(ConstituteError::new(CODE_OFFLINE));
    }
    let requested: Vec<String> = if names.is_empty() {
        profile.install.iter().cloned().collect()
    } else {
        names.to_vec()
    };
    let mut staged: Vec<(String, Vec<u8>, Option<String>)> = Vec::with_capacity(requested.len());
    for name in &requested {
        if !package_name_ok(name) {
            return Err(ConstituteError::named(CODE_OUTSIDE, name));
        }
        if profile.deferred.contains(name) {
            return Err(ConstituteError::named(CODE_DEFERRED, name));
        }
        if !profile.install.contains(name) {
            return Err(ConstituteError::named(CODE_OUTSIDE, name));
        }
        let source = fixture_dir.join(name);
        if !source.is_file() {
            return Err(ConstituteError::named(CODE_OFFLINE, name));
        }
        let bytes = read_regular_file(&source)?;
        let cited = match read_sidecar_digest(fixture_dir, name)? {
            Some(expected) => {
                let actual = sha256_hex(&bytes);
                if actual != expected {
                    return Err(ConstituteError::named(CODE_DIGEST, name));
                }
                Some(expected)
            }
            None => None,
        };
        staged.push((name.clone(), bytes, cited));
    }
    fs::create_dir_all(out_dir).map_err(|_| ConstituteError::new(CODE_OUTPUT))?;
    let mut journal = Vec::with_capacity(staged.len());
    for (name, bytes, sha256) in staged {
        let dest = out_dir.join(&name);
        if path_contains_airootfs(&dest) {
            return Err(ConstituteError::new(CODE_OUTPUT));
        }
        fs::write(&dest, bytes).map_err(|_| ConstituteError::new(CODE_OUTPUT))?;
        journal.push(JournalEntry { name, sha256 });
    }
    let journal_body = serde_json::to_string(&journal_json(&journal))
        .map_err(|_| ConstituteError::new(CODE_OUTPUT))?;
    fs::write(out_dir.join("journal.json"), journal_body)
        .map_err(|_| ConstituteError::new(CODE_OUTPUT))?;
    Ok(FetchOutcome { journal })
}

/// La cara pide la imagen con esta operación. No recibe un directorio ni un script.
/// Sin `mkarchiso` en el path de búsqueda no crea ficheros. Si el binario está,
/// tampoco lo ejecuta: el WebView no es el sitio de la corrida.
pub fn request_live_image(search_path: &str) -> ConstituteError {
    if find_mkarchiso(search_path).is_none() {
        ConstituteError::new(CODE_IMAGE_ABSENT)
    } else {
        ConstituteError::new(CODE_IMAGE_REFUSED)
    }
}

fn find_mkarchiso(search_path: &str) -> Option<std::path::PathBuf> {
    for dir in search_path.split(':') {
        if dir.is_empty() {
            continue;
        }
        let candidate = Path::new(dir).join("mkarchiso");
        if candidate.is_file() {
            return Some(candidate);
        }
    }
    None
}

pub fn run_args(args: &[String]) -> Result<String, ConstituteError> {
    match args.first().map(String::as_str) {
        Some("image") => {
            if args.len() != 1 {
                return Err(ConstituteError::new(CODE_SCRIPT));
            }
            let search = std::env::var("PATH").unwrap_or_default();
            return Err(request_live_image(&search));
        }
        Some("fetch") => {}
        _ => return Err(ConstituteError::new(CODE_SCRIPT)),
    }
    let mut profile_path: Option<&str> = None;
    let mut fixture_path: Option<&str> = None;
    let mut out_path: Option<&str> = None;
    let mut names: Vec<String> = Vec::new();
    let mut iter = args.iter().skip(1);
    while let Some(arg) = iter.next() {
        match arg.as_str() {
            "--profile" => {
                profile_path = Some(
                    iter.next()
                        .ok_or_else(|| ConstituteError::new(CODE_SCRIPT))?
                        .as_str(),
                );
            }
            "--fixture" => {
                fixture_path = Some(
                    iter.next()
                        .ok_or_else(|| ConstituteError::new(CODE_SCRIPT))?
                        .as_str(),
                );
            }
            "--out" => {
                out_path = Some(
                    iter.next()
                        .ok_or_else(|| ConstituteError::new(CODE_SCRIPT))?
                        .as_str(),
                );
            }
            "--name" => {
                let name = iter
                    .next()
                    .ok_or_else(|| ConstituteError::new(CODE_SCRIPT))?;
                names.push(name.clone());
            }
            "--script" | "--cmd" | "-c" | "sh" => {
                return Err(ConstituteError::new(CODE_SCRIPT));
            }
            _ => return Err(ConstituteError::new(CODE_SCRIPT)),
        }
    }
    let (Some(profile_path), Some(fixture_path), Some(out_path)) =
        (profile_path, fixture_path, out_path)
    else {
        return Err(ConstituteError::new(CODE_SCRIPT));
    };
    let profile_bytes = fs::read(profile_path).map_err(|_| ConstituteError::new(CODE_OFFLINE))?;
    let profile = parse_live_profile(&profile_bytes)?;
    let outcome = fetch_packages(
        &profile,
        &names,
        Path::new(fixture_path),
        Path::new(out_path),
    )?;
    Ok(serde_json::json!({
        "ok": true,
        "journal": journal_json(&outcome.journal),
    })
    .to_string())
}

fn journal_json(journal: &[JournalEntry]) -> Vec<serde_json::Value> {
    journal
        .iter()
        .map(|entry| {
            let mut item = serde_json::json!({ "name": entry.name });
            if let Some(digest) = &entry.sha256 {
                item["sha256"] = serde_json::Value::String(digest.clone());
            }
            item
        })
        .collect()
}

fn package_name_ok(name: &str) -> bool {
    let mut chars = name.chars();
    match chars.next() {
        Some(first) if first.is_ascii_lowercase() || first.is_ascii_digit() => {}
        _ => return false,
    }
    name.chars()
        .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '+' | '.' | '_' | '-'))
        && !name.contains("pacstrap")
        && !name.contains("cmd")
}

fn path_contains_airootfs(path: &Path) -> bool {
    path.components().any(|component| match component {
        Component::Normal(part) => part == "airootfs",
        _ => false,
    })
}

fn read_regular_file(path: &Path) -> Result<Vec<u8>, ConstituteError> {
    let mut file = fs::File::open(path).map_err(|_| ConstituteError::new(CODE_OFFLINE))?;
    let meta = file
        .metadata()
        .map_err(|_| ConstituteError::new(CODE_OFFLINE))?;
    if !meta.is_file() {
        return Err(ConstituteError::new(CODE_OFFLINE));
    }
    let mut bytes = Vec::new();
    file.read_to_end(&mut bytes)
        .map_err(|_| ConstituteError::new(CODE_OFFLINE))?;
    Ok(bytes)
}

fn read_sidecar_digest(fixture_dir: &Path, name: &str) -> Result<Option<String>, ConstituteError> {
    let path = fixture_dir.join(format!("{name}.sha256"));
    if !path.exists() {
        return Ok(None);
    }
    let text = fs::read_to_string(&path).map_err(|_| ConstituteError::new(CODE_OFFLINE))?;
    let digest = text.split_whitespace().next().unwrap_or("").trim();
    if digest.len() != 64 || !digest.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(ConstituteError::named(CODE_DIGEST, name));
    }
    Ok(Some(digest.to_ascii_lowercase()))
}

fn sha256_hex(bytes: &[u8]) -> String {
    hex::encode(Sha256::digest(bytes))
}

#[cfg(test)]
mod source_guard {
    #[test]
    fn library_does_not_spawn_a_process_or_open_the_network() {
        let src = include_str!("lib.rs");
        let spawn = format!("{}{}", "Command", "::");
        let net = format!("{}{}", "Tcp", "Stream");
        let url = format!("{}{}", "https", "://");
        assert!(!src.contains(&spawn));
        assert!(!src.contains(&net));
        assert!(!src.contains(&url));
    }
}
