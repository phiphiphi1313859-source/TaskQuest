use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, File};
use std::io::Write;
use std::path::{Path, PathBuf};

pub mod git_sync;

#[allow(unused_imports)]
pub use git_sync::GitSync;

/// Get the TaskQuest data directory
pub fn get_data_dir() -> Result<PathBuf> {
    let data_dir = if let Ok(custom_dir) = std::env::var("TASKQUEST_DATA") {
        PathBuf::from(custom_dir)
    } else {
        let home = std::env::var("HOME").context("HOME environment variable not set")?;
        PathBuf::from(home).join(".taskquest")
    };

    if !data_dir.exists() {
        fs::create_dir_all(&data_dir)
            .context("Failed to create TaskQuest data directory")?;
    }

    Ok(data_dir)
}

/// Safely write data to a JSON file with atomic writes and backup
pub fn safe_write<T: Serialize>(path: &Path, data: &T) -> Result<()> {
    let tmp_path = path.with_extension("tmp");
    let bak_path = path.with_extension("bak");

    // Write to temp file
    let json = serde_json::to_string_pretty(data)
        .context("Failed to serialize data to JSON")?;

    let mut file = File::create(&tmp_path)
        .context("Failed to create temporary file")?;

    file.write_all(json.as_bytes())
        .context("Failed to write to temporary file")?;

    file.sync_all()
        .context("Failed to sync temporary file")?;

    // Backup existing file
    if path.exists() {
        fs::copy(path, &bak_path)
            .context("Failed to create backup")?;
    }

    // Atomic rename
    fs::rename(&tmp_path, path)
        .context("Failed to rename temporary file")?;

    Ok(())
}

/// Load data from a JSON file
pub fn load<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    let file = File::open(path)
        .with_context(|| format!("Failed to open file: {}", path.display()))?;

    let data = serde_json::from_reader(file)
        .with_context(|| format!("Failed to parse JSON from: {}", path.display()))?;

    Ok(data)
}

/// Try to load data, falling back to backup if main file is corrupted
pub fn load_with_backup<T: for<'de> Deserialize<'de>>(path: &Path) -> Result<T> {
    match load(path) {
        Ok(data) => Ok(data),
        Err(e) => {
            let bak_path = path.with_extension("bak");
            if bak_path.exists() {
                eprintln!("Warning: Main file corrupted, loading from backup");
                load(&bak_path).context("Failed to load from backup")
            } else {
                Err(e)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::{Deserialize, Serialize};
    use tempfile::tempdir;

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }

    #[test]
    fn test_safe_write_and_load() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.json");

        let data = TestData {
            name: "test".to_string(),
            value: 42,
        };

        // Write data
        safe_write(&file_path, &data).unwrap();

        // Load data
        let loaded: TestData = load(&file_path).unwrap();
        assert_eq!(data, loaded);

        // Verify backup was created
        let bak_path = file_path.with_extension("bak");

        // Write again to create backup
        let data2 = TestData {
            name: "test2".to_string(),
            value: 100,
        };
        safe_write(&file_path, &data2).unwrap();

        assert!(bak_path.exists());
    }
}
