use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::cli::RemoveArgs;
use crate::core::config::Config;
use crate::core::paths::get_config_path;
use crate::templates::{SNAPSHOTS_DIRNAME, WORKSPACE_DIRNAME};

pub fn run(args: RemoveArgs) -> Result<()> {
    let config_path = get_config_path()?;
    let mut config = Config::load(config_path.to_str().context("Config path not UTF-8")?)?;

    let (source_id, source_name, source_path_display) = {
        let source = config.find_source(&args.source).ok_or_else(|| {
            anyhow::anyhow!(
                "Source '{}' not found. Use 'jackup list' to see configured sources.",
                args.source
            )
        })?;
        (source.id.clone(), source.name.clone(), source.path.display().to_string())
    };

    if !args.yes && !confirm(&format!("Remove source '{}' ({})?", source_name, source_path_display))? {
        println!("Aborted.");
        return Ok(());
    }

    config.sources.retain(|s| s.id != source_id);
    config.save(&config_path)?;
    log::info!("Removed source '{}'", source_name);

    if args.purge {
        let repo_path = PathBuf::from(&config.repository_path);
        let snapshot = repo_path.join(SNAPSHOTS_DIRNAME).join(format!("{}.tar.zst", source_id));
        let manifest = repo_path.join(WORKSPACE_DIRNAME).join(format!("{}.manifest.toml", source_id));

        for path in [&snapshot, &manifest] {
            if path.exists() {
                fs::remove_file(path)
                    .with_context(|| format!("Removing {}", path.display()))?;
                log::info!("Deleted {}", path.display());
            }
        }
    } else {
        log::info!("Snapshot files kept. Use --purge to also delete them.");
    }

    Ok(())
}

fn confirm(prompt: &str) -> io::Result<bool> {
    print!("{} [y/N] ", prompt);
    io::stdout().flush()?;
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().eq_ignore_ascii_case("y"))
}
