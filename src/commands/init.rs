use std::io::{self, Write};
use std::path::PathBuf;
use anyhow::Context;
use uuid::{Uuid};
use crate::templates::{CONFIG_FILENAME, IGNORE_FILENAME, IGNORE_TEMPLATE};
use crate::core::config::Config;

fn prompt_with_default(q: &str, default: &str) -> Result<String, io::Error> {
    print!("{q}");
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let t = s.trim();
    Ok(if t.is_empty() { default.to_string() } else { t.to_string() })
}

fn expand_tilde(path: &str) -> anyhow::Result<PathBuf> {
    if let Some(stripped) = path.strip_prefix("~/") {
        let home_dir = home::home_dir().context("Could not determine home directory")?;
        Ok(home_dir.join(stripped))
    } else {
        Ok(PathBuf::from(path))
    }
}

pub fn run() -> anyhow::Result<()> {
    println!("jackup - A simple backup tool\n");
    let cwd = std::env::current_dir().context("Get current directory")?;

    let config_path = cwd.join(CONFIG_FILENAME);
    let ignore_path = cwd.join(IGNORE_FILENAME);

    if config_path.exists() {
        println!("Config file already exists at {:}", config_path.display());
        println!("Initialization skipped.");
        return Ok(());
    }

    // Get current computer device name
    let default_device = hostname::get()
        .ok()
        .and_then(|s| s.into_string().ok())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "my_computer".to_string());

    let device_name = prompt_with_default(
        &format!("Enter the device name (default: {}): ", default_device),
        &default_device
    )?;

    // Input repository path
    print!("Enter the repository path: ");
    io::stdout().flush()?;
    let mut input_repo_path = String::new();
    io::stdin().read_line(&mut input_repo_path)?;
    let repo_path = input_repo_path.trim();
    if repo_path.is_empty() {
        println!("Repository path cannot be empty. Initialization aborted.");
        return Ok(());
    }

    println!("HOME from env: {:?}", std::env::var_os("HOME"));

    // Create repository directory if it doesn't exist
    let full_repo_path = expand_tilde(repo_path)?;
    println!("full_repo_path: {}", full_repo_path.display());

    if !full_repo_path.exists() {
        std::fs::create_dir_all(&full_repo_path).with_context(|| format!("Creating repository directory at {}", full_repo_path.display()))?;
        println!("Created repository directory at {}", full_repo_path.display());
    } else {
        println!("Repository directory already exists at {}", full_repo_path.display());
        println!("Please make sure it is empty before proceeding.");
    }

    let initial_config = Config {
        version: 1,
        id: Uuid::new_v4().to_string(),
        device: device_name,
        repository_path: full_repo_path.display().to_string(),
        sources: vec![],
    };

    // Create the ignore file if it doesn't exist
    initial_config.save(&config_path)?;
    println!("Initialized new jackup repository with config at {:?}", config_path);

    if ignore_path.exists() {
        println!(".jackupignore file already exists at {:?}", ignore_path);
    } else {
        std::fs::write(&ignore_path, IGNORE_TEMPLATE)?;
        println!("Created .jackupignore file at {:?}", ignore_path);
    }

    println!("Successfully initialized.");

    Ok(())
}