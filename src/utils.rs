use std::fs;
use std::path::{Path, PathBuf};

use crate::error::Result;

pub fn tmp_path_for(path: &Path, fallback_name: &str) -> PathBuf {
    let mut name = path
        .file_name()
        .map(|file_name| file_name.to_os_string())
        .unwrap_or_else(|| fallback_name.into());
    name.push(".tmp");
    path.with_file_name(name)
}

pub fn atomic_write<F>(path: impl AsRef<Path>, fallback_name: &str, write_fn: F) -> Result<()>
where
    F: FnOnce(&Path) -> Result<()>,
{
    let path = path.as_ref();
    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent)?;
    }

    let tmp_path = tmp_path_for(path, fallback_name);
    if tmp_path.exists() {
        fs::remove_file(&tmp_path)?;
    }

    write_fn(&tmp_path)?;

    if path.exists() {
        fs::remove_file(path)?;
    }
    fs::rename(tmp_path, path)?;
    Ok(())
}
