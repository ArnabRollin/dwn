#![doc = include_str!("../README.md")]
#[macro_use]
extern crate lazy_static;

use bytecode::{bytecode_compile_file, bytecode_run};
use clap::{Parser, Subcommand};
use framework::make_framework;
use idle::idle;
use interpreter::interpret_file;

mod bytecode;
mod dwn;
mod framework;
mod idle;
mod interpreter;
mod lexer;
mod runner;

/// Dawn (`dwn`) is the interpreter and bytecode compiler for the Dawn Programming Language.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Runs a Dawn project file.
    #[command(alias = "r")]
    Run { file: String },
    /// Bytecode compiles a Dawn project file.
    Bytec { file: String, level: Option<i32> },
    /// Runs a Dawn bytecode file.
    Byterun { file: String },
    /// Starts the Integrated Development and Learning Environment (IDLE)
    Idle,
    /// Creates a framework for Dawn Programming Language extensions.
    #[command(alias = "fw")]
    Framework,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Commands::Run { file } => interpret_file(file),
        Commands::Bytec { file, level } => bytecode_compile_file(file, level.unwrap_or(-1)),
        Commands::Byterun { file } => bytecode_run(file),
        Commands::Idle => idle(),
        Commands::Framework => make_framework(),
    }
}
