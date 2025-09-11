use std::path::PathBuf;
use std::env;
use std::io::Error;
use anyhow::{Context, Result};
use crate::templates::{CONFIG_FILENAME, IGNORE_FILENAME};

pub fn get_application_path() -> Result<PathBuf> {
    let exe = env::current_exe()?;
    let exe = exe.canonicalize().unwrap_or(exe);
    Ok(exe.parent().context("Get the application directory")?.to_path_buf())
}

pub fn get_config_path() -> Result<PathBuf, Error> {
    let app_path = get_application_path();
    let config_path = app_path.join(CONFIG_FILENAME);
    match config_path {
        Ok() => config_path,
        Err(e) => {
            println!("Config file not found at {:}", config_path.display());
            println!("Please run 'jackup init' to create a configuration file.");
            println!("Error: {e}");
        }
    }
}

pub fn get_ignore_path() -> Result<PathBuf> {
    let app_path = get_application_path()?;
    Ok(app_path.join(IGNORE_FILENAME))
}