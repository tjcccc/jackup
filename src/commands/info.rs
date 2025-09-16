use anyhow::Context;
use crate::core::config::Config;
use crate::core::paths::{get_config_path};

pub fn run() -> anyhow::Result<()> {
    let config_path = get_config_path().context("Get config file")?;
    let config = Config::load(config_path.to_str().unwrap()).with_context(|| "Failed to load configuration.")?;
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
        if !source.excludes.is_empty() {
            println!("    Excludes:");
            for exclude in &source.excludes {
                println!("      - {}", exclude);
            }
        }
    }

    Ok(())
}