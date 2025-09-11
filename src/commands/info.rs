use anyhow::Context;
use crate::templates::{CONFIG_FILENAME};
use crate::core::config::Config;
use crate::core::files::{get_config_path};

// fn check_config_path() {
//     let config_path = get_config_path().context("Get config file").unwrap();
//     if !config_path.exists() {
//         println!("Config file not found at {:}", config_path.display());
//         println!("Please run 'jackup init' to create a configuration file.");
//         std::process::exit(1);
//     }
// }

fn load_config() -> Config {
    // let cwd = std::env::current_dir().context("Get current directory").unwrap();
    // let config_path = cwd.join(CONFIG_FILENAME);
    let config_path = get_config_path().context("Get config file");
    if config_path.is_err() {
        std::process::exit(1);
    }
    let config_path = config_path.unwrap();
    Config::load(config_path.to_str().unwrap()).expect("Failed to load configuration.")
}

pub fn run() -> anyhow::Result<()> {
    let config = load_config();

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