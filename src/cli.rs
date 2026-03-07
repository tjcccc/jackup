use clap::{ArgAction, Args, Parser, Subcommand};

#[derive(Parser)]
#[command(
    name="jackup", 
    version,
    about = "A simple backup tool for creating and managing file snapshots",
    long_about = "jackup helps you back up your important files and directories.\n\n\
                  Usage examples:\n  \
                  jackup init          # Initialize a new backup repository\n  \
                  jackup info          # Display current configuration\n  \
                  jackup add <path>    # Add a source directory\n  \
                  jackup backup        # Create a new snapshot (coming soon)\n  \
                  jackup restore       # Restore from snapshot (coming soon)"
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
