use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;
use anyhow::{Error, Result};
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct Source {
    pub id: String,
    pub path: PathBuf,
    pub name: String,
    pub enabled: bool,
    #[serde(default, alias = "excludes")]
    pub exclude: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub follow_symlinks: Option<bool>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_at: Option<String>,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub version: i8,
    pub id: String,
    pub device: String,
    pub repository_path: String,
    pub sources: Vec<Source>,
}

impl Config {
    pub fn load(path_string: &str) -> Result<Self> {
        let path = PathBuf::from(path_string);
        if !path.exists() {
            return Err(Error::msg(format!("Config file not found at path: {:?}", path)));
        }

        let toml_content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&toml_content)?;

        Ok(Self {
            version: config.version,
            id: config.id,
            device: config.device,
            repository_path: config.repository_path,
            sources: config.sources,
        })
    }

    pub fn save<P: AsRef<Path>>(&self, path: P) -> Result<()> {
        let path = path.as_ref();
        
        if let Some(parent) = path.parent() {
            if !parent.exists() {
                return Err(Error::msg(format!("Config directory not found at path: {:?}", parent.display())));
            }
        }
        
        let tmp = path.with_extension("tmp");
        let toml_content = toml::to_string_pretty(self)?;

        // Write to temp file
        let result = (|| {
            let mut file = fs::File::create(&tmp)?;
            file.write_all(toml_content.as_bytes())?;
            file.sync_all()?;
            fs::rename(&tmp, path)?;
            Ok(())
        })();

        if result.is_err() && tmp.exists() {
            let _ = fs::remove_file(&tmp);
        }

        result
    }

    // pub fn add_source(&mut self, path_string: &str, name: Option<&str>, excludes: &[String]) -> Result<()> {
    //     let new_source = Source {
    //         id: uuid::Uuid::
    //     }
    // }
}
