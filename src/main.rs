use std::fs;
use std::path::PathBuf;

use chumsky::Parser as _;
use clap::Parser;

mod ast;
mod builtin;
// mod parser;
mod span;
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
            // match parser::parser().parse(&content as &str) {
            //     Ok(lit) => println!("Successful parse: {lit:#?}"),
            //     Err(errs) => {
            //         println!("Parsing failed");
            //         for err in errs {
            //             println!("{err:?}");
            //         }
            //     }
            // }
        }
    }

    Ok(())
}
