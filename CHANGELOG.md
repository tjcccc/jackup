# CHANGELOG

All notable changes to this project are documented in this file.

This changelog uses Semantic Versioning.

## [1.0.0] - 2026-03-15

### Added
- `jackup run [--dry-run] [--force]` — backs up all enabled sources into `.tar.zst` snapshots with incremental change detection (mtime + size via manifest)
- `jackup status` — table showing last backed-up time, file count, and archive size per source
- `jackup peek <source>` — lists files inside a source's backup from its manifest (no decompression)
- `jackup withdraw <target> [--source <name>] [--dry-run]` — extracts backups preserving original path structure; handles Unix and Windows paths; resolves conflicts by newer mtime
- `jackup verify [<source>]` — verifies archive integrity against manifest (completeness + size check via full streaming decompression)
- `jackup remove <source> [--purge] [-y]` — removes a source; `--purge` deletes snapshot + manifest
- `jackup enable <source>` / `jackup disable <source>` — toggles source participation in backups
- `jackup update <source> [--name] [--exclude] [--follow-symlinks]` — edits source metadata
- `src/core/manifest.rs` — `Manifest` type tracking per-file mtime + size; atomic TOML save
- `src/core/format.rs` — shared `format_size`, `format_datetime`, `format_date_unix`, `truncate`
- Progress logging during `jackup run` (every 10% for large sources)

### Changed
- README fully rewritten to document all commands
- DEVLOG updated with 2026-03-15 session

## [0.3.0] - 2026-03-07

### Added
- `jackup add` command to register backup sources in `~/.jackup/config.toml`.
- `jackup list` command (alias: `jackup ls`) to display sources in table format.
- Source-level fields:
  - `exclude` patterns
  - `follow_symlinks`
  - `created_at` / `updated_at`

### Changed
- README updated with `add` and `list` usage examples.

### Docs
- Added `DEVLOG.md` for session-level development notes.

## [0.2.1] - 2026-03-02

### Changed
- README improvements and usage documentation refresh.

## [0.2.0] - 2026-02-04

### Changed
- Internal refactor and dependency updates around CLI/config/path handling.

## [0.1.2] - 2025-10-12

### Changed
- Improved interactive input handling in `init` flow.

## [0.1.1] - 2025-09-16

### Changed
- Path handling improvements (`core/paths`) and related command updates.

## [0.1.0] - 2025-09-10

### Added
- Initial usable CLI foundations:
  - `jackup init`
  - `jackup info`
- Config model and template/constants setup.

## [0.0.1] - 2025-09-03

### Added
- Project bootstrap (`Cargo.toml`, initial `src`, license, ignore files, README).
