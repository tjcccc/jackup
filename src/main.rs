mod cli;
mod commands;
mod core;
mod templates;

use std::path::PathBuf;
use clap::Parser;
use anyhow::Result;
use crate::cli::Cli;


fn main() -> Result<()> {
    println!("jackup - A simple backup tool");
    let cli = Cli::parse();
    let ctx = core::Context::bootstrap()?;
    commands::dispatch(cli, ctx);
}
