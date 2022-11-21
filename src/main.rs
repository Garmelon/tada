#![deny(unsafe_code)]
// Rustc lint groups
#![warn(future_incompatible)]
#![warn(rust_2018_idioms)]
// Rustc lints
#![warn(noop_method_call)]
#![warn(single_use_lifetimes)]
#![warn(trivial_numeric_casts)]
#![warn(unused_crate_dependencies)]
#![warn(unused_extern_crates)]
#![warn(unused_import_braces)]
#![warn(unused_lifetimes)]
#![warn(unused_qualifications)]
// Clippy lints
#![warn(clippy::use_self)]

use std::fs;
use std::path::PathBuf;

use anyhow::anyhow;
use chumsky::Parser as _;
use clap::Parser;

mod ast;
mod builtin;
mod desugar;
mod parser;
mod pretty;
mod span;
mod table;
mod value;

#[derive(Parser)]
enum Command {
    Parse { file: PathBuf },
    Pretty { file: PathBuf },
    Desugar { file: PathBuf },
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
            let program = parser::parser()
                .parse(stream)
                .map_err(|e| anyhow!("{e:?}"))?;

            println!("{}", pretty::pretty_to_string(program, 100));
        }

        Command::Desugar { file } => {
            let content = fs::read_to_string(&file)?;
            let stream = span::stream_from_str(&content);
            let mut program = parser::parser()
                .parse(stream)
                .map_err(|e| anyhow!("{e:?}"))?;

            println!("{}", pretty::pretty_to_string(program.clone(), 100));

            loop {
                let (new_program, desugared) = program.desugar();
                program = new_program;
                if !desugared {
                    break;
                }

                println!();
                println!("================================================================================");
                println!();
                println!("{}", pretty::pretty_to_string(program.clone(), 100));
            }
        }
    }

    Ok(())
}
