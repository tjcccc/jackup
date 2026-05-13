use anyhow::{Context, Result};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::cli::ToggleArgs;
use crate::core::config::Config;
use crate::core::paths::get_config_path;

pub fn run(args: ToggleArgs, enabled: bool) -> Result<()> {
    let config_path = get_config_path()?;
    let mut config = Config::load(config_path.to_str().context("Config path not UTF-8")?)?;

    let now = OffsetDateTime::now_utc().format(&Rfc3339)?;

    let source = config.find_source_mut(&args.source).ok_or_else(|| {
        anyhow::anyhow!(
            "Source '{}' not found. Use 'jackup list' to see configured sources.",
            args.source
        )
    })?;

    if source.enabled == enabled {
        let state = if enabled {
            "already enabled"
        } else {
            "already disabled"
        };
        log::info!("Source '{}' is {}.", source.name, state);
        return Ok(());
    }

    source.enabled = enabled;
    source.updated_at = Some(now);
    let name = source.name.clone();

    config.save(&config_path)?;

    let state = if enabled { "enabled" } else { "disabled" };
    log::info!("Source '{}' {}.", name, state);

    Ok(())
}
