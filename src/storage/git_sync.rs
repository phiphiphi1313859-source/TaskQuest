use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub struct GitSync {
    enabled: bool,
}

impl GitSync {
    pub fn new(enabled: bool) -> Self {
        Self { enabled }
    }

    /// Auto-commit changes to git if enabled
    pub fn auto_commit(&self, data_dir: &Path, message: &str) -> Result<()> {
        if !self.enabled {
            return Ok(());
        }

        // Check if directory is a git repo
        if !data_dir.join(".git").exists() {
            return Ok(());
        }

        // Add all changes
        Command::new("git")
            .arg("-C")
            .arg(data_dir)
            .arg("add")
            .arg(".")
            .output()
            .context("Failed to run git add")?;

        // Commit changes
        Command::new("git")
            .arg("-C")
            .arg(data_dir)
            .arg("commit")
            .arg("-m")
            .arg(message)
            .output()
            .context("Failed to run git commit")?;

        Ok(())
    }

    /// Initialize git repository in data directory
    pub fn init_repo(data_dir: &Path) -> Result<()> {
        if data_dir.join(".git").exists() {
            return Ok(());
        }

        Command::new("git")
            .arg("-C")
            .arg(data_dir)
            .arg("init")
            .output()
            .context("Failed to initialize git repository")?;

        Ok(())
    }
}
