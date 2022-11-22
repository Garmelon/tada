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

use std::io::Write;
use std::path::PathBuf;
use std::{fs, process};

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
    Parse {
        file: PathBuf,
    },
    Pretty {
        file: PathBuf,
    },
    Desugar {
        file: PathBuf,
        #[arg(long, short, default_value = "diff")]
        difftool: String,
        #[arg(long, short = 'a')]
        diffarg: Vec<String>,
    },
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

            print!("{}", pretty::pretty_to_string(program, 100));
        }

        Command::Desugar {
            file,
            difftool,
            diffarg,
        } => {
            let content = fs::read_to_string(&file)?;
            let stream = span::stream_from_str(&content);
            let mut program = parser::parser()
                .parse(stream)
                .map_err(|e| anyhow!("{e:?}"))?;

            let mut builder = tempfile::Builder::new();
            builder.suffix(".tada");

            let mut prev = builder.tempfile()?;
            prev.write_all(pretty::pretty_to_string(program.clone(), 100).as_bytes())?;
            prev.flush()?;

            loop {
                let (new_program, desugared) = program.desugar();
                program = new_program;
                if !desugared {
                    break;
                }

                let mut cur = builder.tempfile()?;
                cur.write_all(pretty::pretty_to_string(program.clone(), 100).as_bytes())?;
                cur.flush()?;

                process::Command::new(&difftool)
                    .args(&diffarg)
                    .arg(prev.path())
                    .arg(cur.path())
                    .spawn()?
                    .wait()?;

                prev = cur;
            }
        }
    }

    Ok(())
}
