use anyhow::{bail, Context, Result};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;

use crate::cli::UpdateArgs;
use crate::core::config::Config;
use crate::core::paths::get_config_path;

pub fn run(args: UpdateArgs) -> Result<()> {
    if args.name.is_none() && args.exclude.is_empty() && args.follow_symlinks.is_none() {
        bail!("Nothing to update. Provide at least one of: --name, --exclude, --follow-symlinks.");
    }

    let config_path = get_config_path()?;
    let mut config = Config::load(config_path.to_str().context("Config path not UTF-8")?)?;

    let now = OffsetDateTime::now_utc().format(&Rfc3339)?;

    let source = config.find_source_mut(&args.source).ok_or_else(|| {
        anyhow::anyhow!(
            "Source '{}' not found. Use 'jackup list' to see configured sources.",
            args.source
        )
    })?;

    if let Some(new_name) = args.name {
        source.name = new_name;
    }

    // Non-empty --exclude list replaces the existing excludes entirely.
    if !args.exclude.is_empty() {
        source.exclude = args.exclude;
    }

    if let Some(follow) = args.follow_symlinks {
        source.follow_symlinks = Some(follow);
    }

    source.updated_at = Some(now);
    let name = source.name.clone();

    config.save(&config_path)?;
    log::info!("Updated source '{}'.", name);

    Ok(())
}
