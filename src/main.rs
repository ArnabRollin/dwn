#![doc = include_str!("../README.md")]
#[macro_use]
extern crate lazy_static;

use about::{about, help};
use argparser::argparse;
use framework::make_framework;
use idle::idle;
use interpreter::interpret_file;
use std::{env::args, process::exit};

mod about;
mod argparser;
mod dwn;
mod framework;
mod idle;
mod interpreter;
mod lexer;
mod runner;

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

    if arguments.options.contains(&"version".to_string())
        || arguments.flags.contains(&"v".to_string())
    {
        println!("0.9.0");
        exit(0);
    }

    match arguments.command.as_str() {
        "help" => help(arguments.arguments.get(0)),
        "run" | "r" => interpret_file(arguments.arguments.get(0)),
        "idle" => idle(),
        "framework" | "fw" => make_framework(),
        unknown_command => eprintln!("Unknown command: {}", unknown_command),
    }
}
