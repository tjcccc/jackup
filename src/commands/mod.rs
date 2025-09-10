use crate::cli::Command;
// use crate::core::context::Context;
use anyhow::Result;

pub mod init;
mod info;

pub fn dispatch(cmd: Command) -> Result<()> {
    match cmd {
        Command::Init => init::run(),
        Command::Info => info::run(),
    }
}