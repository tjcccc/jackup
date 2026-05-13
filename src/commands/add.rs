use anyhow::{Context, anyhow, bail};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use uuid::Uuid;

use crate::cli::AddArgs;
use crate::core::config::{Config, Source};
use crate::core::paths::{expand_tilde, get_config_path};

pub fn run(args: AddArgs) -> anyhow::Result<()> {
    let config_path = get_config_path().context("Get config file")?;
    let config_path_string = config_path
        .to_str()
        .ok_or_else(|| anyhow!("Failed to read config path"))?;
    let mut config = Config::load(config_path_string)?;

    let source_input_path = expand_tilde(&args.path)?;
    let source_path = source_input_path.canonicalize().with_context(|| {
        format!(
            "Invalid or inaccessible source path: {}",
            source_input_path.display()
        )
    })?;

    if !source_path.is_dir() {
        bail!("Source path must be a directory: {}", source_path.display());
    }

    if config.sources.iter().any(|s| s.path == source_path) {
        bail!("Source already exists: {}", source_path.display());
    }

    let source_name = args.name.unwrap_or_else(|| {
        source_path
            .file_name()
            .and_then(|name| name.to_str())
            .filter(|name| !name.is_empty())
            .map(|name| name.to_string())
            .unwrap_or_else(|| source_path.display().to_string())
    });
    let now = OffsetDateTime::now_utc()
        .format(&Rfc3339)
        .context("Failed to format timestamp")?;

    config.sources.push(Source {
        id: Uuid::new_v4().to_string(),
        path: source_path.clone(),
        name: source_name.clone(),
        enabled: true,
        exclude: args.exclude,
        follow_symlinks: Some(args.follow_symlinks),
        created_at: Some(now.clone()),
        updated_at: Some(now),
    });

    config.save(&config_path)?;
    log::info!(
        "Added source '{}' at {}",
        source_name,
        source_path.display()
    );
    Ok(())
}
