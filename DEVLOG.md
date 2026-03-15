# DEVLOG

## 2026-03-15

### Summary
Built the full backup engine: `jackup run`, `jackup status`, `jackup peek`, `jackup withdraw`, `jackup verify`, and source management commands (`remove`, `enable`, `disable`, `update`). Added shared formatting utilities and a `Manifest` type for incremental backup state.

### Implemented

- **`jackup run [--dry-run] [--force]`**
  - Walks each enabled source with `walkdir`, respects per-source `exclude` patterns and global `.jackupignore` using `globset` (gitignore semantics: patterns without `/` match at any depth)
  - Compares mtime + size against manifest to skip unchanged sources
  - Streams files directly into `<uuid>.tar.zst` via `tar::Builder<zstd::Encoder<BufWriter<File>>>`; no staging directory required
  - Atomic snapshot write: temp file + rename
  - Progress logged every 10% for large sources
  - Manifest saved atomically after successful archive write

- **`jackup status`** — table of all sources: last backed-up time, file count, archive size on disk

- **`jackup peek <source>`** — lists files in a source's backup from its manifest (no decompression)

- **`jackup withdraw <target> [--source <name>] [--dry-run]`**
  - Two-phase: build extraction plan from manifests (fast, no I/O), then extract from archives
  - Conflict resolution: if two sources map to the same output path, the newer mtime wins
  - Path mapping: Unix absolute → strip leading `/`; Windows `C:\...` → `c/...`

- **`jackup verify [--source <name>]`**
  - Opens each `.tar.zst`, reads all entry headers, checks completeness and size against manifest
  - Full streaming decompression — catches corrupt compressed data
  - Exits non-zero if any source fails

- **`jackup remove <source> [--purge] [-y]`** — removes source from config; `--purge` deletes snapshot + manifest

- **`jackup enable/disable <source>`** — toggles `enabled` flag, updates `updated_at`

- **`jackup update <source> [--name] [--exclude] [--follow-symlinks]`** — updates source metadata; `--exclude` replaces the entire exclude list

- **`src/core/manifest.rs`** — `Manifest` + `FileEntry`; serializes as `[[files]]` TOML array; atomic save

- **`src/core/format.rs`** — shared `format_size`, `format_datetime`, `format_date_unix`, `truncate`

- **`src/core/config.rs`** — added `find_source(&str)` and `find_source_mut(&str)` helpers

### Repo structure
```
workspace/
  <source-uuid>.manifest.toml   ← per-source incremental state
snapshots/
  <source-uuid>.tar.zst         ← compressed archive (one per source)
```

### Dependencies added
- `walkdir` — directory traversal with early directory pruning
- `tar` — tar archive read/write
- `zstd` — zstd compression/decompression
- `globset` — glob pattern matching for excludes

### Verified
- `cargo build` passes with zero new warnings.
- All commands visible in `jackup --help`.

### Next Candidates
- Snapshot history (keep N previous archives per source).
- Tests for manifest serialization, path mapping, conflict resolution.
- Progress bar (instead of log-line checkpoints) for large sources.

---

## 2026-03-07

### Summary
- Added `jackup add` command for source registration into `~/.jackup/config.toml`.
- Added `jackup list` command with alias `jackup ls` for table-style source listing.
- Extended source schema with symlink behavior, exclude patterns, and timestamps.
- Updated README usage and examples to match implemented CLI behavior.

### Implemented
- `jackup add <path> [--name <label>] [--exclude <pattern> ...] [--follow-symlinks=<bool>]`
  - `--follow-symlinks` default: `false`
  - default source name: folder basename
  - duplicate path protection
  - source defaults: `enabled = true`
  - source metadata: `id` (UUID v4), `created_at`, `updated_at` (RFC3339 UTC)
- `jackup list` / `jackup ls`
  - default columns: `Name`, `Path`, `Enabled`
  - `--verbose` / `--full`: show all source fields
  - `--sort name|created|updated` (default: `name`)
  - friendly datetime display in list output

### Config Schema Notes
- Canonical source exclude key is `exclude` (list of patterns).
- Backward compatibility for prior configs is preserved with `alias = "excludes"`.
- `follow_symlinks`, `created_at`, and `updated_at` are optional in deserialization for compatibility.

### Verified
- `cargo build` passes.
- `jackup add --help` shows expected options.
- Test add succeeded:
  - path: `/home/taojiachun/tempfiles/somthing_need_to_backup`
  - name: `first_backup`
  - exclude: `.txt`
- `jackup info` and `jackup list` show the added source.

### Next Candidates
- Add `jackup remove` / `jackup enable` / `jackup disable`.
- Add global snapshot settings section (e.g., default `follow_symlinks`).
- Add tests for config migrations and list sorting behavior.

### Session Workflow
- At session end, run: "update devlog".
- Keep entries concise: summary, implemented, verified, and next candidates.
