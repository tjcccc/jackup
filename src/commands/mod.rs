use crate::cli::Command;
// use crate::core::context::Context;
use anyhow::Result;

pub mod init;
pub mod info;
pub mod add;
pub mod list;

pub fn dispatch(cmd: Command) -> Result<()> {
    match cmd {
        Command::Init => init::run(),
        Command::Info => info::run(),
        Command::Add(args) => add::run(args),
        Command::List(args) => list::run(args),
    }
}
