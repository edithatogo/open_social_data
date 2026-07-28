# 🔒 fix: use secure temporary files in catalog persistence

### 🎯 What:
The `catalog.rs` module contained an insecure temporary file creation vulnerability. When persisting the catalog to disk (`LocalCatalog::save_atomic`), it constructed a predictable `.tmp` path based on the target filename. It then removed any existing file at that location and created a new file with that exact predictable name to write the new catalog output.

### ⚠️ Risk:
This approach introduces a Time-of-Check to Time-of-Use (TOCTOU) condition and predictability vulnerability. A malicious actor with write access to the directory could anticipate the `.tmp` file name and replace it with a symlink or alternate file, potentially resulting in unauthorized file overwrite (clobbering) or tampering with the dataset catalog when `fs::rename` is executed.

### 🛡️ Solution:
Removed the bespoke `tmp_path_for` path generation and replaced the file persistence logic using the `tempfile` crate's `NamedTempFile` api. The new implementation creates a unique, cryptographically random temporary file securely within the destination directory and atomically replaces the original file using `persist()`. This closes the race condition and uses the operating system's facilities for safe temporary file handling.
