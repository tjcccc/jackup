use clap::Parser;
use anyhow::Result;
use crate::cli::Cli;

mod cli;
mod commands;
mod core;
mod templates;

fn main() -> Result<()> {
    let cli = Cli::parse();
    // let ctx = core::context::Context::bootstrap()?;
    commands::dispatch(cli.command)
}
