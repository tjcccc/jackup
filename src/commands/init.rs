use std::fmt::format;
use std::io::{self, Write};
use anyhow::Context;
use crate::templates::{CONFIG_TEMPLATE, CONFIG_FILENAME, IGNORE_TEMPLATE, IGNORE_FILENAME};
use crate::core::config::Config;

fn prompt_with_default(q: &str, default: &str) -> Result<String, io::Error> {
    print!("{}: ", q);
    io::stdout().flush()?;
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    let t = s.trim();
    Ok(if t.is_empty() { default.to_string() } else { t.to_string() })
}

pub fn run() -> anyhow::Result<()> {
    let cwd = std::env::current_dir().context("Get current directory")?;

    let config_path = cwd.join(CONFIG_FILENAME);
    let ignore_path = cwd.join(IGNORE_FILENAME);

    if config_path.exists() {
        println!("Config file already exists at {:}", config_path.display());
        println!("Initialization skipped.");
        return Ok(());
    }

    if ignore_path.exists() {
        println!(".jackupignore file already exists at {:?}", ignore_path);
    } else {
        std::fs::write(&ignore_path, IGNORE_TEMPLATE)?;
        println!("Created .jackupignore file at {:?}", ignore_path);
    }

    // Get current computer device name
    let default_device = hostname::get()
        .ok()
        .and_then(|s| s.into_string().ok())
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| "my_computer".to_string());

    // Input device name
    // print!("Enter the device name for this repository (default: {:?}): ", default_device);
    // io::stdout().flush()?;
    // let mut input_device_name = String::new();
    // io::stdin().read_line(&mut input_device_name)?;
    // let device_name = if input_device_name.is_empty() { default_device } else {input_device_name.trim().to_string()};
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

    let initial_config = Config {
        version: 1,
        device: device_name,
        repository_path: repo_path.to_string(),
        sources: vec![],
    };

    initial_config.save(&config_path)?;
    println!("Initialized new jackup repository with config at {:?}", config_path);

    Ok(())
}