use std::path::PathBuf;
use clap::Parser;

#[derive(Parser)]
#[command(name = "jackup", version)]
enum Commands {
    Init { repo: PathBuf },
    Run,
    Snapshot { source: PathBuf, #[arg(long)] label: Option<String> },
    List,
    Extract { snapshot: String, destination: PathBuf },
}

fn main() {
    println!("Hello, world!");
}
