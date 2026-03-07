# DEVLOG

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
