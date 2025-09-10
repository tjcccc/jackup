// use std::path::PathBuf;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name="jackup", version)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    Init,
    Info
}
