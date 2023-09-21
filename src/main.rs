//! Dawn (dwn) is the language interpreter for Dawn.
#![deny(missing_docs)]
#[macro_use]
extern crate lazy_static;

use about::{about, help};
use argparser::argparse;
use idle::idle;
use interpreter::interpret_file;
use std::{env::args, process::exit};

mod about;
mod argparser;
mod dwn;
mod idle;
mod interpreter;
mod lexer;

fn main() {
    let mut args = args();

    // Skip program name
    args.next();

    let arguments = argparse(args);

    if arguments.options.is_empty()
        && arguments.flags.is_empty()
        && arguments.arguments.is_empty()
        && (arguments.command == String::new())
    {
        about();
        exit(1)
    }

    if arguments.options.contains(&"help".to_string()) || arguments.flags.contains(&"h".to_string())
    {
        about();
        exit(0);
    }

    match arguments.command.as_str() {
        "help" => help(arguments.arguments.get(0)),
        "run" | "r" => interpret_file(arguments.arguments.get(0)),
        "idle" => idle(),
        unknown_command => eprintln!("Unknown command: {}", unknown_command),
    }
}
