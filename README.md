# jackup

A simple, personal backup tool. Backs up source directories into compressed snapshots (`.tar.zst`) and lets you extract them to any target path when you need them.

## Features

- Add source directories with per-source exclude patterns
- Incremental detection — only re-archives sources where files have changed
- Compressed snapshots via zstd (one `.tar.zst` per source)
- Cross-OS path recovery — handles both Unix and Windows source paths on extraction
- Conflict resolution on extract — newer file wins when two sources share a path

## Installation

Build from source:

```bash
cargo build --release
```

> **Note:** `cargo install jackup` is not available yet — not published to crates.io.

## Usage

### Initialize a repository

```bash
jackup init
```

Prompts for a device name and a repository path. Creates:
- `<repo>/workspace/` — manifest files (per-run state)
- `<repo>/snapshots/` — compressed backup archives
- `~/.jackup/config.toml` — configuration
- `~/.jackup/.jackupignore` — global ignore patterns

---

### Source management

```bash
jackup add <path> [--name <label>] [--exclude <pattern>] [--follow-symlinks=<bool>]
```

Registers a source directory. Options:
- `-n, --name` — display name (defaults to folder basename)
- `-e, --exclude` — glob pattern to exclude; repeatable
- `--follow-symlinks` — follow symlinks (default: `false`)

```bash
jackup remove <source> [--purge] [-y]
```

Removes a source from config. `--purge` also deletes its snapshot and manifest. `-y` skips the confirmation prompt.

```bash
jackup enable <source>
jackup disable <source>
```

Toggles whether a source is included in `jackup run`.

```bash
jackup update <source> [--name <label>] [--exclude <pattern>] [--follow-symlinks=<bool>]
```

Updates a source's metadata. `--exclude` (repeatable) replaces the entire exclude list.

---

### Viewing sources and status

```bash
jackup list [--verbose] [--sort name|created|updated]
jackup ls                   # alias
```

Lists configured sources. `--verbose` shows all fields including ID, excludes, and timestamps.

```bash
jackup info
```

Shows global configuration (device name, repository path, all sources).

```bash
jackup status
```

Shows backup health for every source: last backed-up time, file count, and archive size on disk. Sources that have never been backed up show `(never)`.

```bash
jackup peek <source>
```

Lists all files inside a source's latest backup (reads from the manifest — no decompression).

---

### Running backups

```bash
jackup run [--dry-run] [--force]
```

Backs up all enabled sources. For each source:
1. Compares current files against the manifest (mtime + size)
2. If nothing changed, skips the source
3. If changed, archives all files into `<repo>/snapshots/<uuid>.tar.zst` and updates the manifest

Options:
- `--dry-run` — show what would be backed up without writing
- `--force` — skip change detection and always re-archive

---

### Extracting backups

```bash
jackup withdraw <target> [--source <name>] [--dry-run]
```

Extracts backup files to `<target>`, preserving original path structure:

| Source path | Extracted to |
|---|---|
| `/user/jack/photos` | `<target>/user/jack/photos/...` |
| `C:\game\saves` | `<target>/c/game/saves/...` |

When multiple sources share an output path, the file with the newer mtime wins.

Options:
- `-s, --source` — extract only one source (name or ID prefix)
- `--dry-run` — preview without writing

---

### Verifying backups

```bash
jackup verify [--source <name>]
```

Opens each `.tar.zst` and checks:
- All files listed in the manifest are present in the archive
- File sizes in the archive match the manifest

Exits with a non-zero status if any source fails. Run this after `jackup run` on critical backups.

---

## Configuration

Config file: `~/.jackup/config.toml`

```toml
version = 1
id = "your-uuid"
device = "your-device-name"
repository_path = "/path/to/repo"

[[sources]]
id = "uuid"
path = "/absolute/path"
name = "source name"
enabled = true
follow_symlinks = false
exclude = ["*.tmp", "build/"]
created_at = "2026-03-15T00:00:00Z"
updated_at = "2026-03-15T00:00:00Z"
```

Ignore file: `~/.jackup/.jackupignore`

```
*.DS_Store
Thumbs.db
node_modules
```

Patterns without a `/` match at any depth (e.g. `node_modules` excludes any directory with that name).

---

## Release Checklist

1. Update `README.md` and `DEVLOG.md`.
2. Update `CHANGELOG.md` with version and date.
3. Bump version in `Cargo.toml`.
4. Run: `cargo fmt && cargo build && cargo test`
5. Commit, tag, and push: `git tag v0.x.x && git push && git push --tags`

## Roadmap

- [x] Initialize repository
- [x] Add / remove / enable / disable / update source directories
- [x] List sources with sorting and verbose output
- [x] Run backups with incremental change detection
- [x] View backup status and file listings
- [x] Extract backups with cross-OS path mapping
- [x] Verify archive integrity against manifests
- [ ] Snapshot history — keep N previous archives per source for rollback
- [ ] Restore to original paths (for non-system files)
- [ ] Tests
