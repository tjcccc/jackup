use std::path::PathBuf;
// use std::env;
use anyhow::{anyhow, Context, Result};
use crate::templates::{CONFIG_DIRNAME, CONFIG_FILENAME, IGNORE_FILENAME};

pub fn expand_tilde(path: &str) -> anyhow::Result<PathBuf> {
    if let Some(stripped) = path.strip_prefix("~/") {
        let home_dir = home::home_dir().context("Could not determine home directory")?;
        Ok(home_dir.join(stripped))
    } else {
        Ok(PathBuf::from(path))
    }
}

pub fn get_user_config_dir() -> Result<PathBuf> {
    let home_dir = home::home_dir().context("Could not determine home directory")?;
    let config_dir = home_dir.join(CONFIG_DIRNAME);
    if !config_dir.exists() {
        std::fs::create_dir_all(&config_dir).context("Create config directory")?;
        // If in the Windows environment, set the directory attribute to hidden
        // #[cfg(target_os = "windows")]
        // {
        //     use std::os::windows::fs::OpenOptionsExt;
        //     use std::fs::OpenOptions;
        //     use winapi::um::winbase::FILE_ATTRIBUTE_HIDDEN;
        //     let _ = OpenOptions::new()
        //         .create(true)
        //         .write(true)
        //         .attributes(FILE_ATTRIBUTE_HIDDEN)
        //         .open(&config_dir);
        // }
    }
    Ok(config_dir)
}

// pub fn get_application_path() -> Result<PathBuf> {
//     let exe = env::current_exe()?;
//     let exe = exe.canonicalize().unwrap_or(exe);
//     Ok(exe.parent().context("Get the application directory")?.to_path_buf())
// }

pub fn get_config_path() -> Result<PathBuf> {
    let config_dir_path = get_user_config_dir()?;
    let config_path = config_dir_path.join(CONFIG_FILENAME);
    match config_path.exists() {
        true => Ok(config_path),
        false => {
            eprintln!("Config file not found at {:}", config_path.display());
            eprintln!("Please run 'jackup init' to create a configuration file.");
            Err(anyhow!("Config file not exists"))
        }
    }
}

pub fn get_ignore_path() -> Result<PathBuf> {
    let app_path = get_user_config_dir()?;
    Ok(app_path.join(IGNORE_FILENAME))
}