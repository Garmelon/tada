use std::fs;
use std::path::PathBuf;

use clap::Parser;

mod builtin;
mod table;
mod value;

#[derive(Parser)]
enum Command {
    Parse { file: PathBuf },
}

#[derive(Parser)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

/// Foo
fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    match args.command {
        Command::Parse { file } => {
            let content = fs::read_to_string(&file)?;
            print!("{content}");
            if !content.ends_with('\n') {
                println!();
            }
        }
    }

    Ok(())
}
