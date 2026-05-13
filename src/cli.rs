use clap::{ArgAction, Args, Parser, Subcommand, ValueEnum};

#[derive(Parser)]
#[command(
    name = "jackup",
    version,
    about = "A simple backup tool for creating and managing file snapshots",
    long_about = "jackup helps you back up your important files and directories.\n\n\
                  Usage examples:\n  \
                  jackup init              # Initialize a new backup repository\n  \
                  jackup add ~/Documents   # Add a source directory\n  \
                  jackup run               # Back up all enabled sources\n  \
                  jackup status            # Show backup health for all sources\n  \
                  jackup withdraw ~/dest   # Extract backup to a target directory"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Info,
    Add(AddArgs),
    #[command(alias = "ls")]
    List(ListArgs),
    Run(RunArgs),
    /// Show backup status for all configured sources
    Status,
    /// List files inside a source's latest backup
    Peek(PeekArgs),
    /// Extract backup files into a target directory, preserving original path structure
    Withdraw(WithdrawArgs),
    /// Remove a configured source
    Remove(RemoveArgs),
    /// Enable a source so it is included in backups
    Enable(ToggleArgs),
    /// Disable a source so it is excluded from backups
    Disable(ToggleArgs),
    /// Update a source's name, excludes, or symlink setting
    Update(UpdateArgs),
    /// Verify that snapshot archives match their manifests
    Verify(VerifyArgs),
}

#[derive(Args)]
pub struct AddArgs {
    /// Source directory path to add
    pub path: String,
    /// Optional display name for this source
    #[arg(short, long)]
    pub name: Option<String>,
    /// Exclude glob-like patterns for this source; can be repeated
    #[arg(short = 'e', long = "exclude")]
    pub exclude: Vec<String>,
    /// Follow symlinks while scanning this source
    #[arg(long, default_value_t = false, action = ArgAction::Set)]
    pub follow_symlinks: bool,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, ValueEnum)]
pub enum SortField {
    Name,
    Created,
    Updated,
}

#[derive(Args)]
pub struct ListArgs {
    /// Show all source fields
    #[arg(long, visible_alias = "full")]
    pub verbose: bool,
    /// Sort sources by field
    #[arg(long, value_enum, default_value_t = SortField::Name)]
    pub sort: SortField,
}

#[derive(Args)]
pub struct RunArgs {
    /// Show what would be backed up without writing anything
    #[arg(long)]
    pub dry_run: bool,
    /// Force backup even if no changes are detected
    #[arg(long, short = 'f')]
    pub force: bool,
}

#[derive(Args)]
pub struct PeekArgs {
    /// Source name or ID (prefix) to inspect
    pub source: String,
}

#[derive(Args)]
pub struct WithdrawArgs {
    /// Directory to extract backup files into
    pub target: String,
    /// Extract only a specific source (name or ID prefix); defaults to all enabled sources
    #[arg(long, short = 's')]
    pub source: Option<String>,
    /// Show what would be extracted without writing anything
    #[arg(long)]
    pub dry_run: bool,
}

#[derive(Args)]
pub struct RemoveArgs {
    /// Source name or ID prefix to remove
    pub source: String,
    /// Also delete the snapshot and manifest files
    #[arg(long)]
    pub purge: bool,
    /// Skip the confirmation prompt
    #[arg(long, short = 'y')]
    pub yes: bool,
}

#[derive(Args)]
pub struct ToggleArgs {
    /// Source name or ID prefix
    pub source: String,
}

#[derive(Args)]
pub struct UpdateArgs {
    /// Source name or ID prefix to update
    pub source: String,
    /// New display name
    #[arg(long, short = 'n')]
    pub name: Option<String>,
    /// Replace the entire exclude list (repeatable); omit to leave excludes unchanged
    #[arg(long, short = 'e')]
    pub exclude: Vec<String>,
    /// Change follow-symlinks behavior
    #[arg(long)]
    pub follow_symlinks: Option<bool>,
}

#[derive(Args)]
pub struct VerifyArgs {
    /// Source name or ID prefix to verify; omit to verify all sources
    pub source: Option<String>,
}
