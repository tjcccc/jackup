use std::fs;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::core::config::Config;
use crate::core::format::{format_datetime, format_size};
use crate::core::manifest::Manifest;
use crate::core::paths::get_config_path;
use crate::templates::{SNAPSHOTS_DIRNAME, WORKSPACE_DIRNAME};

struct Row {
    name: String,
    enabled: bool,
    last_run: String,
    file_count: String,
    archive_size: String,
}

pub fn run() -> Result<()> {
    let config_path = get_config_path()?;
    let config = Config::load(config_path.to_str().context("Config path not UTF-8")?)?;

    if config.sources.is_empty() {
        println!("No sources configured. Use 'jackup add <path>' to add sources.");
        return Ok(());
    }

    let repo_path = PathBuf::from(&config.repository_path);
    let workspace_path = repo_path.join(WORKSPACE_DIRNAME);
    let snapshots_path = repo_path.join(SNAPSHOTS_DIRNAME);

    let rows: Vec<Row> = config
        .sources
        .iter()
        .map(|source| {
            let manifest_path = workspace_path.join(format!("{}.manifest.toml", source.id));
            let snapshot_path = snapshots_path.join(format!("{}.tar.zst", source.id));
            let manifest = Manifest::load(&manifest_path).unwrap_or_default();

            let (last_run, file_count, archive_size) = match &manifest.last_run_at {
                Some(ts) => {
                    let size = fs::metadata(&snapshot_path)
                        .map(|m| format_size(m.len()))
                        .unwrap_or_else(|_| "-".to_string());
                    (format_datetime(ts), manifest.files.len().to_string(), size)
                }
                None => ("(never)".to_string(), "-".to_string(), "-".to_string()),
            };

            Row {
                name: source.name.clone(),
                enabled: source.enabled,
                last_run,
                file_count,
                archive_size,
            }
        })
        .collect();

    let name_w = rows
        .iter()
        .map(|r| r.name.len())
        .max()
        .unwrap_or(0)
        .max("Name".len());
    let last_w = rows
        .iter()
        .map(|r| r.last_run.len())
        .max()
        .unwrap_or(0)
        .max("Last Backed Up".len());
    let file_w = rows
        .iter()
        .map(|r| r.file_count.len())
        .max()
        .unwrap_or(0)
        .max("Files".len());
    let size_w = rows
        .iter()
        .map(|r| r.archive_size.len())
        .max()
        .unwrap_or(0)
        .max("Archive".len());

    println!(
        "{:<name_w$}  {:<last_w$}  {:>file_w$}  {:>size_w$}  {}",
        "Name",
        "Last Backed Up",
        "Files",
        "Archive",
        "Enabled",
        name_w = name_w,
        last_w = last_w,
        file_w = file_w,
        size_w = size_w
    );
    println!(
        "{}",
        "-".repeat(name_w + last_w + file_w + size_w + "Enabled".len() + 8)
    );

    for row in &rows {
        println!(
            "{:<name_w$}  {:<last_w$}  {:>file_w$}  {:>size_w$}  {}",
            row.name,
            row.last_run,
            row.file_count,
            row.archive_size,
            row.enabled,
            name_w = name_w,
            last_w = last_w,
            file_w = file_w,
            size_w = size_w
        );
    }

    Ok(())
}
