# jackup

A simple backup tool for creating and managing file snapshots.

## Features

- **Initialize repository** - Create a new backup repository with configurable storage location
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

## Configuration

Config file location: `~/.jackup/config.toml`

```toml
version = 1
id = "your-uuid"
device = "your-device-name"
repository_path = "/path/to/repo"
sources = []
```

Ignore file location: `~/.jackup/.jackupignore`

Add patterns (one per line) to exclude from backups, e.g.:
```
*.DS_Store
Thumbs.db
node_modules
```

## Roadmap

- [ ] Add source directories to backup
- [ ] Create snapshots
- [ ] Restore from snapshots
- [ ] List snapshot history
- [ ] Verify backup integrity
