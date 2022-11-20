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
                    let mut out = vec![];
                    program.pretty(&RcAllocator).render(100, &mut out)?;
                    println!("{}", String::from_utf8(out)?);
                }
                Err(errs) => {
                    eprintln!("Parsing failed");
                    for err in errs {
                        eprintln!("{err:?}");
                    }
                }
            }
        }
    }

    Ok(())
}
