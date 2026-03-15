use crate::cli::Command;
use anyhow::Result;

pub mod add;
pub mod info;
pub mod init;
pub mod list;
pub mod peek;
pub mod remove;
pub mod run;
pub mod status;
pub mod toggle;
pub mod update;
pub mod verify;
pub mod withdraw;

pub fn dispatch(cmd: Command) -> Result<()> {
    match cmd {
        Command::Init => init::run(),
        Command::Info => info::run(),
        Command::Add(args) => add::run(args),
        Command::List(args) => list::run(args),
        Command::Run(args) => run::run(args),
        Command::Status => status::run(),
        Command::Peek(args) => peek::run(args),
        Command::Withdraw(args) => withdraw::run(args),
        Command::Remove(args) => remove::run(args),
        Command::Enable(args) => toggle::run(args, true),
        Command::Disable(args) => toggle::run(args, false),
        Command::Update(args) => update::run(args),
        Command::Verify(args) => verify::run(args),
    }
}
