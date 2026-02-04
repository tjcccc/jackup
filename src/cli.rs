// use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name="jackup", 
    version,
    about = "A simple backup tool for creating and managing file snapshots",
    long_about = "jackup helps you back up your important files and directories.\n\n\
                  Usage examples:\n  \
                  jackup init          # Initialize a new backup repository\n  \
                  jackup info          # Display current configuration\n  \
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
    Info
}
