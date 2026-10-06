use std::fs::{self, File};
use std::io::Write;
use std::path::Path;
use std::sync::atomic::{AtomicU64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};

use crate::error::{CoreError, Result};
use crate::port::CoreOp;

static TMP_COUNTER: AtomicU64 = AtomicU64::new(0);

pub fn atomic_write_bytes(op: CoreOp, path: &Path, bytes: &[u8]) -> Result<()> {
    let source = op.source();
    let parent = path.parent().filter(|p| !p.as_os_str().is_empty());
    if let Some(dir) = parent {
        fs::create_dir_all(dir).map_err(|_| CoreError::io_failed(source))?;
    }
    let stamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_nanos())
        .unwrap_or(0);
    let salt = TMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let tmp_name = format!(".tmp-{}-{}-{}", std::process::id(), stamp, salt);
    let tmp_path = match parent {
        Some(dir) => dir.join(tmp_name),
        None => Path::new(&tmp_name).to_path_buf(),
    };
    let write_result = (|| {
        let mut file = File::create(&tmp_path)?;
        file.write_all(bytes)?;
        file.sync_all()?;
        drop(file);
        fs::rename(&tmp_path, path)?;
        Ok::<(), std::io::Error>(())
    })();
    if write_result.is_err() {
        let _ = fs::remove_file(&tmp_path);
        return Err(CoreError::io_failed(source));
    }
    Ok(())
}
