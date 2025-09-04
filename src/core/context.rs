use std::path::{PathBuf};
use time::{OffsetDateTime};
use anyhow::{ Result};
use crate::core::config::{Config};


pub struct Context {
    pub repo_path: PathBuf,
    pub config: Config,
    pub now: OffsetDateTime,
}

impl Context {
    pub fn new(repo_path: PathBuf, config: Config) -> Self {
        Self {
            repo_path,
            config,
            now: OffsetDateTime::now_utc(),
        }
    }
    
    pub fn bootstrap() -> Result<Self> {
        let repo_path = std::env::current_dir()?;
        let config = Config::load()?;
        Ok(Self::new(repo_path, config))
    }
}