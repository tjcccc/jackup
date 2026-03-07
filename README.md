# jackup

A simple backup tool for creating and managing file snapshots.

## Features

- **Initialize repository** - Create a new backup repository with configurable storage location
- **Add backup sources** - Register source directories in config with optional per-source excludes
- **Configuration management** - Store backup settings in `~/.jackup/config.toml`
- **Path validation** - Supports `~` expansion and prevents accidental use of system directories
- **Ignore patterns** - Custom `.jackupignore` file to exclude files from backups

## Installation

```bash
cargo install jackup
```

Or build from source:

```bash
cargo build --release
```

## Usage

### Initialize a new repository

```bash
jackup init
```

This will:
1. Ask for a device name (defaults to your hostname)
2. Ask for a repository path where snapshots will be stored
3. Create the necessary directory structure:
   - `<repo_path>/workspace/` - Working directory for backups
   - `<repo_path>/snapshots/` - Storage for backup snapshots
4. Create config file at `~/.jackup/config.toml`
5. Create `.jackupignore` file at `~/.jackup/.jackupignore`

### View configuration

```bash
jackup info
```

Displays current configuration including:
- Jackup ID
- Device name
- Repository path
- Configured sources (if any)

### Add a source directory

```bash
jackup add <path> [--name <label>] [--exclude <pattern>] --follow-symlinks=<bool>
```

Example:

```bash
jackup add ~/Pictures --name "my photos" --exclude "*.tmp" --exclude "build/" --follow-symlinks=false
```

Options:
- `-n, --name <NAME>` set display name
- `-e, --exclude <PATTERN>` repeat to add excludes for this source
- `--follow-symlinks=<bool>` follow symlinks for this source (default: `false`)

### List sources

```bash
jackup list
jackup ls
```

Default output is a table with:
- `Name`
- `Path`
- `Enabled`

Verbose/full output:

```bash
jackup list --verbose
jackup list --full
```

Sorting:

```bash
jackup list --sort name
jackup list --sort created
jackup list --sort updated
```

## Configuration

Config file location: `~/.jackup/config.toml`

```toml
version = 1
id = "your-uuid"
device = "your-device-name"
repository_path = "/path/to/repo"
sources = []
```

Source entries are stored as:

```toml
[[sources]]
id = "uuid"
path = "/absolute/path"
name = "source name"
enabled = true
follow_symlinks = false
exclude = ["*.tmp", "build/"]
created_at = "2026-03-07T12:34:56Z"
updated_at = "2026-03-07T12:34:56Z"
```

Ignore file location: `~/.jackup/.jackupignore`

Add patterns (one per line) to exclude from backups, e.g.:
```
*.DS_Store
Thumbs.db
node_modules
```

## Roadmap

- [x] Add source directories to backup
- [ ] Create snapshots
- [ ] Restore from snapshots
- [ ] List snapshot history
- [ ] Verify backup integrity
