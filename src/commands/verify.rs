use std::collections::HashMap;
use std::fs;
use std::io::BufReader;
use std::path::PathBuf;

use anyhow::{Context, Result};

use crate::cli::VerifyArgs;
use crate::core::config::{Config, Source};
use crate::core::manifest::Manifest;
use crate::core::paths::get_config_path;
use crate::templates::{SNAPSHOTS_DIRNAME, WORKSPACE_DIRNAME};

enum VerifyResult {
    Ok { file_count: usize },
    Failed { missing: usize, size_mismatch: usize },
    NeverBacked,
    SnapshotMissing,
}

pub fn run(args: VerifyArgs) -> Result<()> {
    let config_path = get_config_path()?;
    let config = Config::load(config_path.to_str().context("Config path not UTF-8")?)?;

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
        None => config.sources.iter().collect(),
    };

    if sources.is_empty() {
        println!("No sources configured.");
        return Ok(());
    }

    let mut ok_count = 0usize;
    let mut fail_count = 0usize;

    for source in &sources {
        let manifest_path = workspace_path.join(format!("{}.manifest.toml", source.id));
        let snapshot_path = snapshots_path.join(format!("{}.tar.zst", source.id));

        let result = verify_source(source, &manifest_path, &snapshot_path)?;

        match result {
            VerifyResult::Ok { file_count } => {
                log::info!("[{}] OK — {} files verified", source.name, file_count);
                ok_count += 1;
            }
            VerifyResult::NeverBacked => {
                log::warn!("[{}] SKIP — never backed up", source.name);
            }
            VerifyResult::SnapshotMissing => {
                log::error!("[{}] FAIL — snapshot file missing", source.name);
                fail_count += 1;
            }
            VerifyResult::Failed { missing, size_mismatch } => {
                log::error!(
                    "[{}] FAIL — {} missing, {} size mismatch(es)",
                    source.name, missing, size_mismatch
                );
                fail_count += 1;
            }
        }
    }

    println!("\nVerified {} source(s): {} OK, {} FAILED", ok_count + fail_count, ok_count, fail_count);

    if fail_count > 0 {
        anyhow::bail!("Verification failed for {} source(s).", fail_count);
    }

    Ok(())
}

fn verify_source(
    source: &Source,
    manifest_path: &std::path::Path,
    snapshot_path: &std::path::Path,
) -> Result<VerifyResult> {
    let manifest = Manifest::load(manifest_path)?;

    if manifest.last_run_at.is_none() {
        return Ok(VerifyResult::NeverBacked);
    }

    if !snapshot_path.exists() {
        return Ok(VerifyResult::SnapshotMissing);
    }

    // Build lookup: rel_path -> expected_size
    let expected: HashMap<String, u64> =
        manifest.files.iter().map(|f| (f.path.clone(), f.size)).collect();
    let mut found: HashMap<String, bool> =
        expected.keys().map(|k| (k.clone(), false)).collect();

    let mut size_mismatch = 0usize;

    let file = fs::File::open(snapshot_path)
        .with_context(|| format!("Opening snapshot for '{}'", source.name))?;
    let decoder = zstd::Decoder::new(BufReader::new(file))
        .context("Initializing zstd decoder")?;
    let mut archive = tar::Archive::new(decoder);

    for entry in archive.entries().context("Reading archive entries")? {
        let entry = entry.context("Reading archive entry")?;
        let path = entry.path().context("Reading entry path")?;
        let rel = path.to_string_lossy().into_owned();
        let archive_size = entry.header().size().context("Reading entry size")?;

        if let Some(&expected_size) = expected.get(&rel) {
            found.insert(rel, true);
            if archive_size != expected_size {
                size_mismatch += 1;
                log::warn!(
                    "[{}] Size mismatch: {} (expected {} B, got {} B)",
                    source.name, path.display(), expected_size, archive_size
                );
            }
        }
        // Extra files in archive beyond manifest are silently ignored.
    }

    let missing = found.values().filter(|&&v| !v).count();

    if missing == 0 && size_mismatch == 0 {
        Ok(VerifyResult::Ok { file_count: expected.len() })
    } else {
        Ok(VerifyResult::Failed { missing, size_mismatch })
    }
}
