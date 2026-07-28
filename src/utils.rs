use std::path::{Path, PathBuf};

/// Generates a temporary path for a given file by appending ".tmp" to the file name.
/// If the given path does not have a file name, the provided default is used.
pub fn tmp_path_for(path: &Path, default_name: &str) -> PathBuf {
    let mut name = path
        .file_name()
        .map(|file_name| file_name.to_os_string())
        .unwrap_or_else(|| default_name.into());
    name.push(".tmp");
    path.with_file_name(name)
}
