# CHANGELOG

All notable changes to this project are documented in this file.

This changelog is derived from git commit history and uses Semantic Versioning.
Until practical backup/restore features are complete, versions stay in `0.x.x`.

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
