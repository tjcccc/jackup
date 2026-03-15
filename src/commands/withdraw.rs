use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs;
use std::io::{self, BufReader};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};

use crate::cli::WithdrawArgs;
use crate::core::config::{Config, Source};
use crate::core::format::format_size;
use crate::core::manifest::Manifest;
use crate::core::paths::{expand_tilde, get_config_path};
use crate::templates::{SNAPSHOTS_DIRNAME, WORKSPACE_DIRNAME};

struct PlanEntry {
    source_id: String,
    mtime: i64,
}

pub fn run(args: WithdrawArgs) -> Result<()> {
    let config_path = get_config_path()?;
    let config = Config::load(config_path.to_str().context("Config path not UTF-8")?)?;

    let target = expand_tilde(&args.target)?;
    let repo_path = PathBuf::from(&config.repository_path);
    let workspace_path = repo_path.join(WORKSPACE_DIRNAME);
    let snapshots_path = repo_path.join(SNAPSHOTS_DIRNAME);

    let sources: Vec<&Source> = match &args.source {
        Some(query) => {
            let q = query.to_lowercase();
            let found = config
                .sources
                .iter()
                .find(|s| s.name.to_lowercase() == q || s.id.starts_with(&q))
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Source '{}' not found. Use 'jackup list' to see configured sources.",
                        query
                    )
                })?;
            vec![found]
        }
        None => config.sources.iter().filter(|s| s.enabled).collect(),
    };

    if sources.is_empty() {
        log::warn!("No enabled sources to withdraw.");
        return Ok(());
    }

    // Phase 1: Read all manifests and build the extraction plan.
    // Conflicts (same output path from multiple sources) are resolved by keeping
    // the entry with the newer mtime. No decompression happens here.
    let mut plan: HashMap<PathBuf, PlanEntry> = HashMap::new();
    let mut conflict_count = 0usize;

    for source in &sources {
        let manifest_path = workspace_path.join(format!("{}.manifest.toml", source.id));
        let manifest = Manifest::load(&manifest_path)?;

        if manifest.last_run_at.is_none() {
            log::warn!("[{}] Never backed up, skipping.", source.name);
            continue;
        }

        let mapped_base = map_source_path(&source.path);

        for file in &manifest.files {
            let output_rel = mapped_base.join(&file.path);

            match plan.entry(output_rel) {
                Entry::Occupied(mut e) => {
                    conflict_count += 1;
                    if file.mtime > e.get().mtime {
                        *e.get_mut() = PlanEntry { source_id: source.id.clone(), mtime: file.mtime };
                    }
                }
                Entry::Vacant(e) => {
                    e.insert(PlanEntry { source_id: source.id.clone(), mtime: file.mtime });
                }
            }
        }
    }

    if plan.is_empty() {
        println!("Nothing to withdraw (no sources have been backed up yet).");
        return Ok(());
    }

    if args.dry_run {
        println!("Dry run: {} files would be extracted to {}", plan.len(), target.display());
        if conflict_count > 0 {
            println!("  {} conflict(s) would be resolved by keeping the newer file", conflict_count);
        }
        return Ok(());
    }

    // Phase 2: Extract. For each source, open its archive and extract only the
    // files where this source won conflict resolution.
    fs::create_dir_all(&target)
        .with_context(|| format!("Creating target directory: {}", target.display()))?;

    let mut extracted = 0usize;
    let mut extracted_bytes = 0u64;

    for source in &sources {
        let snapshot_path = snapshots_path.join(format!("{}.tar.zst", source.id));
        if !snapshot_path.exists() {
            continue;
        }

        let mapped_base = map_source_path(&source.path);

        let file = fs::File::open(&snapshot_path)
            .with_context(|| format!("Opening snapshot for '{}'", source.name))?;
        let decoder = zstd::Decoder::new(BufReader::new(file))
            .context("Initializing zstd decoder")?;
        let mut archive = tar::Archive::new(decoder);

        for entry in archive.entries().context("Reading archive entries")? {
            let mut entry = entry.context("Reading archive entry")?;
            let entry_path = entry.path().context("Reading entry path")?.into_owned();
            let output_rel = mapped_base.join(&entry_path);

            let won = plan.get(&output_rel).map_or(false, |e| e.source_id == source.id);
            if !won {
                continue;
            }

            let output_path = target.join(&output_rel);
            if let Some(parent) = output_path.parent() {
                fs::create_dir_all(parent)
                    .with_context(|| format!("Creating directory: {}", parent.display()))?;
            }

            let mut out_file = fs::File::create(&output_path)
                .with_context(|| format!("Creating file: {}", output_path.display()))?;
            let bytes = io::copy(&mut entry, &mut out_file)
                .with_context(|| format!("Writing: {}", output_path.display()))?;

            extracted += 1;
            extracted_bytes += bytes;
        }

        log::info!("[{}] Done", source.name);
    }

    println!("Withdrawn {} files ({}) to {}", extracted, format_size(extracted_bytes), target.display());
    if conflict_count > 0 {
        println!("  {} conflict(s) resolved — newer file kept in each case", conflict_count);
    }

    Ok(())
}

/// Map a source path (possibly from another OS) to a relative path suitable
/// for nesting under the target directory.
///
/// - Unix absolute `/user/jack/photos` → `user/jack/photos`
/// - Windows `C:\game\saves` or `C:/game/saves` → `c/game/saves`
/// - Relative paths are used as-is.
fn map_source_path(path: &Path) -> PathBuf {
    let s = path.to_string_lossy();
    let bytes = s.as_bytes();

    // Windows drive letter: "C:\" or "C:/"
    if bytes.len() >= 3
        && bytes[0].is_ascii_alphabetic()
        && bytes[1] == b':'
        && (bytes[2] == b'\\' || bytes[2] == b'/')
    {
        let drive = (bytes[0] as char).to_ascii_lowercase().to_string();
        let rest = s[3..].replace('\\', "/");
        return PathBuf::from(format!("{}/{}", drive, rest));
    }

    // Unix absolute path: strip leading "/"
    if s.starts_with('/') {
        return PathBuf::from(&s[1..]);
    }

    // Relative path: use as-is
    PathBuf::from(s.as_ref())
}
