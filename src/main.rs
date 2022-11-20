use std::fs;
use std::path::PathBuf;

use ::pretty::{Pretty, RcAllocator};
use chumsky::Parser as _;
use clap::Parser;

mod ast;
mod builtin;
mod parser;
mod pretty;
mod span;
mod table;
mod value;

#[derive(Parser)]
enum Command {
    Parse { file: PathBuf },
    Pretty { file: PathBuf },
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
            let stream = span::stream_from_str(&content);
            match parser::parser().parse(stream) {
                Ok(program) => println!("Successful parse: {program:#?}"),
                Err(errs) => {
                    println!("Parsing failed");
                    for err in errs {
                        println!("{err:?}");
                    }
                }
            }
        }
        Command::Pretty { file } => {
            let content = fs::read_to_string(&file)?;
            let stream = span::stream_from_str(&content);
            match parser::parser().parse(stream) {
                Ok(program) => {
                    println!("Successful parse");
                    let doc = program.pretty(&RcAllocator);
                    let mut out = vec![];
                    doc.render(100, &mut out)?;
                    let str = String::from_utf8(out)?;
                    println!("{str}");
                }
                Err(errs) => {
                    println!("Parsing failed");
                    for err in errs {
                        println!("{err:?}");
                    }
                }
            }
        }
    }

    Ok(())
}
