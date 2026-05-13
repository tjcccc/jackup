use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::cli::PeekArgs;
use crate::core::config::Config;
use crate::core::format::{format_date_unix, format_datetime, format_size, truncate};
use crate::core::manifest::Manifest;
use crate::core::paths::get_config_path;
use crate::templates::WORKSPACE_DIRNAME;

pub fn run(args: PeekArgs) -> Result<()> {
    let config_path = get_config_path()?;
    let config = Config::load(config_path.to_str().context("Config path not UTF-8")?)?;

    let query = args.source.to_lowercase();
    let source = config
        .sources
        .iter()
        .find(|s| s.name.to_lowercase() == query || s.id.starts_with(&query))
        .ok_or_else(|| {
            anyhow::anyhow!(
                "Source '{}' not found. Use 'jackup list' to see configured sources.",
                args.source
            )
        })?;

    let repo_path = PathBuf::from(&config.repository_path);
    let manifest_path = repo_path
        .join(WORKSPACE_DIRNAME)
        .join(format!("{}.manifest.toml", source.id));
    let manifest = Manifest::load(&manifest_path)?;

    println!("Source:  {} ({})", source.name, source.path.display());

    match &manifest.last_run_at {
        Some(ts) => println!("Backed up: {}", format_datetime(ts)),
        None => {
            println!("Backed up: (never)");
            return Ok(());
        }
    }

    if manifest.files.is_empty() {
        println!("\n(no files)");
        return Ok(());
    }

    println!();

    const PATH_MAX: usize = 72;
    let path_w = manifest
        .files
        .iter()
        .map(|f| f.path.len())
        .max()
        .unwrap_or(0)
        .min(PATH_MAX)
        .max("Path".len());
    let size_w = manifest
        .files
        .iter()
        .map(|f| format_size(f.size).len())
        .max()
        .unwrap_or(0)
        .max("Size".len());

    println!(
        "  {:<path_w$}  {:>size_w$}  {}",
        "Path",
        "Size",
        "Modified",
        path_w = path_w,
        size_w = size_w
    );
    println!("  {}", "-".repeat(path_w + size_w + "Modified".len() + 4));

    let mut total_bytes = 0u64;
    for f in &manifest.files {
        println!(
            "  {:<path_w$}  {:>size_w$}  {}",
            truncate(&f.path, path_w),
            format_size(f.size),
            format_date_unix(f.mtime),
            path_w = path_w,
            size_w = size_w
        );
        total_bytes += f.size;
    }

    println!();
    println!(
        "  {} files, {} total (uncompressed)",
        manifest.files.len(),
        format_size(total_bytes)
    );

    Ok(())
}
