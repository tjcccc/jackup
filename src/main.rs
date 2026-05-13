use crate::cli::Cli;
use anyhow::Result;
use clap::Parser;

mod cli;
mod commands;
mod core;
mod templates;

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    let cli = Cli::parse();
    // let ctx = core::context::Context::bootstrap()?;
    commands::dispatch(cli.command)
}
