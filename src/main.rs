//! Dawn (dwn) is the language interpreter for Dawn.
#![deny(missing_docs)]
#[macro_use]
extern crate lazy_static;

use about::{about, help};
use argparser::argparse;
use interpreter::interpret;
use std::{env::args, process::exit};

mod about;
mod argparser;
mod dwn;
mod interpreter;
mod lexer;

fn main() {
    let mut args = args();

    // Skip program name
    args.next();

    let arguments = argparse(args);

    if arguments.options.is_empty() && arguments.flags.is_empty() && arguments.arguments.is_empty()
    {
        about();
        exit(1)
    }

    if arguments.options.contains(&"help".to_string()) || arguments.flags.contains(&"h".to_string())
    {
        about();
    }

    match arguments.command.as_str() {
        "help" => help(arguments.arguments.get(0)),
        "run" | "r" => interpret(arguments.arguments.get(0)),
        unknown_command => eprintln!("Unknown command: {}", unknown_command),
    }
}
