use std::fs;
use std::io::BufWriter;
use std::path::{Path, PathBuf};
use std::time::UNIX_EPOCH;

use anyhow::{Context, Result};
use globset::{Glob, GlobSet, GlobSetBuilder};
use time::OffsetDateTime;
use time::format_description::well_known::Rfc3339;
use walkdir::WalkDir;

use crate::cli::RunArgs;
use crate::core::config::{Config, Source};
use crate::core::format::format_size;
use crate::core::manifest::{FileEntry, Manifest};
use crate::core::paths::{get_config_path, get_ignore_path};
use crate::templates::{SNAPSHOTS_DIRNAME, WORKSPACE_DIRNAME};

struct FileInfo {
    abs_path: PathBuf,
    rel_path: String,
    mtime: i64,
    size: u64,
}

enum BackupResult {
    Updated { file_count: usize, bytes: u64 },
    Skipped,
}

pub fn run(args: RunArgs) -> Result<()> {
    let config_path = get_config_path()?;
    let config = Config::load(config_path.to_str().context("Config path not UTF-8")?)?;

    let repo_path = PathBuf::from(&config.repository_path);
    let snapshots_path = repo_path.join(SNAPSHOTS_DIRNAME);
    let manifests_path = repo_path.join(WORKSPACE_DIRNAME);

    fs::create_dir_all(&manifests_path).context("Creating workspace directory")?;

    let global_ignores = load_global_ignores()?;

    let enabled_sources: Vec<&Source> = config.sources.iter().filter(|s| s.enabled).collect();

    if enabled_sources.is_empty() {
        log::warn!("No enabled sources. Use 'jackup add <path>' to add sources.");
        return Ok(());
    }

    let mut backed_up = 0usize;
    let mut skipped = 0usize;
    let mut failed = 0usize;

    for source in enabled_sources {
        let label = &source.name;
        match backup_source(source, &snapshots_path, &manifests_path, &global_ignores, args.dry_run, args.force) {
            Ok(BackupResult::Updated { file_count, bytes }) => {
                if args.dry_run {
                    log::info!("[{}] Dry run: {} files would be archived", label, file_count);
                } else {
                    log::info!("[{}] {} files archived ({})", label, file_count, format_size(bytes));
                }
                backed_up += 1;
            }
            Ok(BackupResult::Skipped) => {
                log::info!("[{}] Up to date, skipped", label);
                skipped += 1;
            }
            Err(e) => {
                log::error!("[{}] Error: {:#}", label, e);
                failed += 1;
            }
        }
    }

    if args.dry_run {
        println!("\nDry run: {} would update, {} up to date, {} error(s)", backed_up, skipped, failed);
    } else {
        println!("\nDone: {} backed up, {} skipped, {} error(s)", backed_up, skipped, failed);
    }

    Ok(())
}

fn backup_source(
    source: &Source,
    snapshots_path: &Path,
    manifests_path: &Path,
    global_ignores: &[String],
    dry_run: bool,
    force: bool,
) -> Result<BackupResult> {
    if !source.path.exists() {
        anyhow::bail!("Source path does not exist: {}", source.path.display());
    }

    let manifest_path = manifests_path.join(format!("{}.manifest.toml", source.id));
    let snapshot_path = snapshots_path.join(format!("{}.tar.zst", source.id));

    let manifest = Manifest::load(&manifest_path)?;
    let lookup = manifest.build_lookup();

    let excludes = build_globset(source.exclude.iter().chain(global_ignores.iter()))?;
    let files = walk_source(source, &excludes)?;

    // Skipped when manifest file count matches AND no file has changed mtime/size.
    let any_changed = force
        || files.len() != manifest.files.len()
        || files.iter().any(|f| {
            lookup
                .get(f.rel_path.as_str())
                .map_or(true, |e| e.mtime != f.mtime || e.size != f.size)
        });

    if !any_changed {
        return Ok(BackupResult::Skipped);
    }

    let total_bytes: u64 = files.iter().map(|f| f.size).sum();

    if dry_run {
        return Ok(BackupResult::Updated { file_count: files.len(), bytes: total_bytes });
    }

    archive_files(&files, &snapshot_path, source.follow_symlinks.unwrap_or(false), &source.name)?;

    let new_manifest = Manifest {
        source_id: source.id.clone(),
        last_run_at: Some(OffsetDateTime::now_utc().format(&Rfc3339)?),
        files: files
            .iter()
            .map(|f| FileEntry { path: f.rel_path.clone(), mtime: f.mtime, size: f.size })
            .collect(),
    };
    new_manifest.save(&manifest_path)?;

    Ok(BackupResult::Updated { file_count: files.len(), bytes: total_bytes })
}

fn walk_source(source: &Source, excludes: &GlobSet) -> Result<Vec<FileInfo>> {
    let follow_symlinks = source.follow_symlinks.unwrap_or(false);
    let source_path = &source.path;
    let mut files = Vec::new();

    let walker = WalkDir::new(source_path)
        .follow_links(follow_symlinks)
        .into_iter()
        .filter_entry(|entry| {
            if entry.depth() == 0 {
                return true;
            }
            let rel = entry.path().strip_prefix(source_path).unwrap_or(entry.path());
            !excludes.is_match(rel)
        });

    for entry in walker {
        let entry = entry.with_context(|| format!("Walking {}", source_path.display()))?;
        if !entry.file_type().is_file() {
            continue;
        }

        let abs_path = entry.path().to_path_buf();
        let rel = abs_path
            .strip_prefix(source_path)
            .with_context(|| format!("Stripping prefix from {}", abs_path.display()))?;
        let rel_path = rel.to_string_lossy().into_owned();

        let metadata = fs::metadata(&abs_path)
            .with_context(|| format!("Reading metadata for {}", abs_path.display()))?;
        let mtime = metadata
            .modified()
            .ok()
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .map(|d| d.as_secs() as i64)
            .unwrap_or(0);
        let size = metadata.len();

        files.push(FileInfo { abs_path, rel_path, mtime, size });
    }

    Ok(files)
}

fn archive_files(files: &[FileInfo], snapshot_path: &Path, follow_symlinks: bool, label: &str) -> Result<()> {
    let tmp_path = snapshot_path.with_extension("tmp");
    let total = files.len();
    // Report progress every 10% or every 200 files, whichever fires more often.
    let checkpoint = if total == 0 { 1 } else { (total / 10).max(200).min(total) };

    let result = (|| -> Result<()> {
        let file = fs::File::create(&tmp_path)
            .with_context(|| format!("Creating snapshot at {}", tmp_path.display()))?;
        let encoder = zstd::Encoder::new(BufWriter::new(file), 3)
            .context("Initializing zstd encoder")?;
        let mut tar = tar::Builder::new(encoder);
        tar.follow_symlinks(follow_symlinks);

        for (i, f) in files.iter().enumerate() {
            tar.append_path_with_name(&f.abs_path, &f.rel_path)
                .with_context(|| format!("Archiving {}", f.abs_path.display()))?;
            if (i + 1) % checkpoint == 0 && i + 1 < total {
                log::info!("[{}] archiving {}/{} files...", label, i + 1, total);
            }
        }

        let encoder = tar.into_inner().context("Finalizing tar archive")?;
        encoder.finish().context("Finishing zstd compression")?;
        Ok(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&tmp_path);
        return result;
    }

    fs::rename(&tmp_path, snapshot_path).with_context(|| {
        format!("Moving snapshot into place at {}", snapshot_path.display())
    })?;

    Ok(())
}

fn build_globset<'a>(patterns: impl Iterator<Item = &'a String>) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for raw in patterns {
        let p = raw.trim().trim_end_matches('/');
        if p.is_empty() || p.starts_with('#') {
            continue;
        }
        // Patterns without a path separator match at any depth (gitignore semantics).
        let normalized = if !p.contains('/') {
            format!("**/{}", p)
        } else if let Some(stripped) = p.strip_prefix('/') {
            stripped.to_string()
        } else {
            p.to_string()
        };
        builder.add(
            Glob::new(&normalized)
                .with_context(|| format!("Invalid glob pattern: {}", raw))?,
        );
    }
    Ok(builder.build()?)
}

fn load_global_ignores() -> Result<Vec<String>> {
    let ignore_path = get_ignore_path()?;
    if !ignore_path.exists() {
        return Ok(vec![]);
    }
    let content = fs::read_to_string(&ignore_path)?;
    Ok(content
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty() && !l.starts_with('#'))
        .collect())
}

