// use std::path::{PathBuf};
use time::{OffsetDateTime};
use anyhow::{ Result};
use crate::core::config::{Config};
use crate::templates::CONFIG_FILENAME;

pub struct Context {
    // pub repo_path: PathBuf,
    pub config: Config,
    pub now: OffsetDateTime,
}

impl Context {
    pub fn new(config: Config) -> Self {
        Self {
            // repo_path,
            config,
            now: OffsetDateTime::now_utc(),
        }
    }
    
    pub fn bootstrap() -> Result<Self> {
        let cwd = std::env::current_dir()?;
        let config_path = cwd.join(CONFIG_FILENAME);
        let config = Config::load(config_path.to_str().unwrap())?;
        Ok(Self::new(config))
    }
}