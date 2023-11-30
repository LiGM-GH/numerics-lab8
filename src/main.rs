use clap::{Parser, Subcommand};
use error::SomeError;
use error_stack::Result;

mod pt1;
mod pt2;
mod error;

#[derive(Debug, Clone, Subcommand)]
enum Part {
    Pt1,
    Pt2,
}

#[derive(Debug, Parser)]
struct Args {
    #[clap(subcommand)]
    part: Part,
}

fn main() -> Result<(), SomeError> {
    let args = Args::parse();

    match args.part {
        Part::Pt1 => pt1::main(),
        Part::Pt2 => pt2::main(),
    }
}
