//! Guardado nativo acotado: el destino lo elige el diálogo; no hay FS global.

use std::path::{Path, PathBuf};

use archmaker_core::{atomic_write_bytes, CoreError, CoreOp, DestinationHandle};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};

#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase", deny_unknown_fields)]
pub struct SaveExportInput {
    pub bytes: Vec<u8>,
    pub suggested_name: String,
    pub expected_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(tag = "status", rename_all = "camelCase")]
pub enum SaveExportResult {
    #[serde(rename_all = "camelCase")]
    Saved {
        file_name: String,
        bytes_written: usize,
        overwritten: bool,
    },
    Cancelled,
}

pub trait SavePicker {
    /// Devuelve la ruta elegida por el usuario, o `None` si cancela.
    fn pick_save_path(&self, suggested_name: &str) -> Option<PathBuf>;
}

/// Diálogo nativo de guardado (producción). Sin shell ni scopes FS globales.
pub struct NativeSavePicker;

impl SavePicker for NativeSavePicker {
    fn pick_save_path(&self, suggested_name: &str) -> Option<PathBuf> {
        rfd::FileDialog::new()
            .set_file_name(suggested_name)
            .set_title("Guardar perfil Forge")
            .save_file()
    }
}

pub fn validate_save_input(input: &SaveExportInput) -> Result<String, CoreError> {
    let source = "archmaker-tauri.save_export_bytes";
    if input.bytes.is_empty() {
        return Err(CoreError::schema_invalid(&format!("{source}: empty bytes")));
    }
    let digest = input.expected_digest.to_lowercase();
    if digest.len() != 64 || !digest.chars().all(|c| c.is_ascii_hexdigit()) {
        return Err(CoreError::schema_invalid(&format!(
            "{source}: expectedDigest must be sha256 hex"
        )));
    }
    let actual = hex_sha256(&input.bytes);
    if actual != digest {
        return Err(CoreError::schema_invalid(&format!(
            "{source}: bytes digest mismatch"
        )));
    }
    // Reutiliza la política de token opaco: solo basename seguro.
    let handle = DestinationHandle::new(input.suggested_name.clone())
        .map_err(|e| CoreError::schema_invalid(&format!("{source}: suggestedName: {e}")))?;
    Ok(handle.token().to_string())
}

pub fn save_export_with_picker<P: SavePicker>(
    picker: &P,
    input: SaveExportInput,
) -> Result<SaveExportResult, CoreError> {
    let suggested = validate_save_input(&input)?;
    let Some(path) = picker.pick_save_path(&suggested) else {
        return Ok(SaveExportResult::Cancelled);
    };
    write_export_at_path(&path, &input.bytes)
}

pub fn write_export_at_path(path: &Path, bytes: &[u8]) -> Result<SaveExportResult, CoreError> {
    let source = "archmaker-tauri.save_export_bytes";
    if path.as_os_str().is_empty() {
        return Err(CoreError::schema_invalid(&format!("{source}: empty path")));
    }
    // Solo se escribe la ruta que devolvió el diálogo (no una del frontend).
    let overwritten = path.exists();
    atomic_write_bytes(CoreOp::ExportArtifact, path, bytes)?;
    let file_name = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("export.bin")
        .to_string();
    Ok(SaveExportResult::Saved {
        file_name,
        bytes_written: bytes.len(),
        overwritten,
    })
}

fn hex_sha256(bytes: &[u8]) -> String {
    let digest = Sha256::digest(bytes);
    digest.iter().map(|b| format!("{b:02x}")).collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    struct ScriptedPicker {
        next: Mutex<Option<Option<PathBuf>>>,
    }

    impl ScriptedPicker {
        fn new(next: Option<PathBuf>) -> Self {
            Self {
                next: Mutex::new(Some(next)),
            }
        }
    }

    impl SavePicker for ScriptedPicker {
        fn pick_save_path(&self, _suggested_name: &str) -> Option<PathBuf> {
            self.next.lock().expect("picker").take().flatten()
        }
    }

    fn sample_input(bytes: &[u8], name: &str) -> SaveExportInput {
        SaveExportInput {
            bytes: bytes.to_vec(),
            suggested_name: name.to_string(),
            expected_digest: hex_sha256(bytes),
        }
    }

    fn tmp_file(name: &str) -> PathBuf {
        let dir =
            std::env::temp_dir().join(format!("archmaker-f5-{}-{}", std::process::id(), name));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("tmp dir");
        dir.join(name)
    }

    #[test]
    fn save_cancelled_when_picker_returns_none() {
        let picker = ScriptedPicker::new(None);
        let out =
            save_export_with_picker(&picker, sample_input(b"{}", "a.json")).expect("cancel is ok");
        assert_eq!(out, SaveExportResult::Cancelled);
    }

    #[test]
    fn save_writes_new_file_atomically() {
        let path = tmp_file("nuevo.json");
        let picker = ScriptedPicker::new(Some(path.clone()));
        let bytes = br#"{"profile":true}"#;
        let out =
            save_export_with_picker(&picker, sample_input(bytes, "nuevo.json")).expect("save");
        assert_eq!(
            out,
            SaveExportResult::Saved {
                file_name: "nuevo.json".into(),
                bytes_written: bytes.len(),
                overwritten: false,
            }
        );
        assert_eq!(std::fs::read(&path).expect("read"), bytes);
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_dir(path.parent().unwrap());
    }

    #[test]
    fn save_overwrites_existing_file() {
        let path = tmp_file("existente.json");
        std::fs::write(&path, b"old").expect("seed");
        let picker = ScriptedPicker::new(Some(path.clone()));
        let bytes = b"new-content";
        let out = save_export_with_picker(&picker, sample_input(bytes, "existente.json"))
            .expect("overwrite");
        match out {
            SaveExportResult::Saved {
                overwritten,
                bytes_written,
                ..
            } => {
                assert!(overwritten);
                assert_eq!(bytes_written, bytes.len());
            }
            SaveExportResult::Cancelled => panic!("expected saved"),
        }
        assert_eq!(std::fs::read(&path).expect("read"), bytes);
        let _ = std::fs::remove_file(&path);
        let _ = std::fs::remove_dir(path.parent().unwrap());
    }

    #[test]
    fn save_rejects_digest_mismatch_without_writing() {
        let path = tmp_file("bad-digest.json");
        let picker = ScriptedPicker::new(Some(path.clone()));
        let mut input = sample_input(b"data", "bad-digest.json");
        input.expected_digest = "0".repeat(64);
        let err = save_export_with_picker(&picker, input).expect_err("mismatch");
        assert_eq!(err.code, "AM-SCHEMA-001");
        assert!(!path.exists());
        let _ = std::fs::remove_dir(path.parent().unwrap());
    }

    #[test]
    fn save_rejects_empty_bytes() {
        let err = validate_save_input(&SaveExportInput {
            bytes: vec![],
            suggested_name: "x.json".into(),
            expected_digest: "0".repeat(64),
        })
        .expect_err("empty");
        assert_eq!(err.code, "AM-SCHEMA-001");
    }

    #[test]
    fn save_rejects_path_like_suggested_name() {
        let err = validate_save_input(&SaveExportInput {
            bytes: b"x".to_vec(),
            suggested_name: "../escape.json".into(),
            expected_digest: hex_sha256(b"x"),
        })
        .expect_err("traversal");
        assert_eq!(err.code, "AM-SCHEMA-001");
    }

    #[test]
    fn save_rejects_absolute_suggested_name() {
        let err = validate_save_input(&SaveExportInput {
            bytes: b"x".to_vec(),
            suggested_name: "/etc/passwd".into(),
            expected_digest: hex_sha256(b"x"),
        })
        .expect_err("absolute");
        assert_eq!(err.code, "AM-SCHEMA-001");
    }

    #[test]
    fn atomic_write_failure_leaves_no_partial_target() {
        let dir = std::env::temp_dir().join(format!("archmaker-f5-ro-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).expect("dir");
        let path = dir.join("target.json");
        // Archivo como "directorio padre" imposible: path bajo un fichero.
        let blocker = dir.join("blocker");
        std::fs::write(&blocker, b"x").expect("blocker");
        let nested = blocker.join("target.json");
        let err = write_export_at_path(&nested, b"payload").expect_err("must fail");
        assert_eq!(err.code, "AM-IO-001");
        assert!(!path.exists());
        let _ = std::fs::remove_dir_all(&dir);
    }
}
