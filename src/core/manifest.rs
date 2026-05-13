use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Serialize, Deserialize, Clone)]
pub struct FileEntry {
    pub path: String,
    pub mtime: i64,
    pub size: u64,
}

#[derive(Serialize, Deserialize, Default)]
pub struct Manifest {
    pub source_id: String,
    pub last_run_at: Option<String>,
    #[serde(default)]
    pub files: Vec<FileEntry>,
}

impl Manifest {
    pub fn load(path: &Path) -> Result<Self> {
        if !path.exists() {
            return Ok(Self::default());
        }
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    pub fn save(&self, path: &Path) -> Result<()> {
        let tmp = path.with_extension("tmp");
        let content = toml::to_string_pretty(self)?;
        fs::write(&tmp, content)?;
        fs::rename(&tmp, path)?;
        Ok(())
    }

    pub fn build_lookup(&self) -> HashMap<&str, &FileEntry> {
        self.files.iter().map(|e| (e.path.as_str(), e)).collect()
    }
}
