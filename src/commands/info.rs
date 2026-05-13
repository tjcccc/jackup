use crate::core::config::Config;
use crate::core::paths::get_config_path;
use anyhow::{Context, anyhow};

pub fn run() -> anyhow::Result<()> {
    let config_path = get_config_path().context("Get config file")?;
    let config = Config::load(
        config_path
            .to_str()
            .ok_or_else(|| anyhow!("Failed to load configuration."))?,
    )?;
    // println!("Configuration Information:");
    // println!("--------------------------");
    // println!("Version: {}", config.version);
    println!("Jackup Id: {}", config.id);
    println!("Device: {}", config.device);
    println!("Repository Path: {}", config.repository_path);
    println!("Sources:");
    for source in &config.sources {
        println!("  - ID: {}", source.id);
        println!("    Name: {}", source.name);
        println!("    Path: {}", source.path.display());
        println!("    Enabled: {}", source.enabled);
        if let Some(follow_symlinks) = source.follow_symlinks {
            println!("    Follow Symlinks: {}", follow_symlinks);
        }
        if !source.exclude.is_empty() {
            println!("    Exclude:");
            for exclude in &source.exclude {
                println!("      - {}", exclude);
            }
        }
        if let Some(created_at) = &source.created_at {
            println!("    Created At: {}", created_at);
        }
        if let Some(updated_at) = &source.updated_at {
            println!("    Updated At: {}", updated_at);
        }
    }

    Ok(())
}
