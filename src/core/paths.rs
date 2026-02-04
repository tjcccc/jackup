use std::path::{Path, PathBuf};
// use std::env;
use anyhow::{anyhow, bail, Context, Result};
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
            Err(anyhow!(
                "Config file not found at {}. Please run 'jackup init' to create a configuration file.",
                config_path.display()
            ))
            
        }
    }
}

pub fn get_ignore_path() -> Result<PathBuf> {
    let app_path = get_user_config_dir()?;
    Ok(app_path.join(IGNORE_FILENAME))
}

pub fn validate_repo_path(path: &Path) -> Result<PathBuf> {
    // Canonicalize to resolve symlinks and get absolute path
    let canonical = path.canonicalize()
        .with_context(|| format!("Invalid or inaccessible path: {}", path.display()))?;
    
    // Prevent writing to system directories
    let forbidden_prefixes = ["/etc", "/sys", "/proc", "/dev", "/boot"];
    for prefix in &forbidden_prefixes {
        if canonical.starts_with(prefix) {
            bail!("Cannot use system directory '{}' as repository", prefix);
        }
    }
    
    // Ensure it's not root
    if canonical == Path::new("/") {
        bail!("Cannot use root directory as repository");
    }
    
    Ok(canonical)
}