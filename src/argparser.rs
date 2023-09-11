//! Argument parser for Dawn (dwn).

use std::env::Args;

/// Parses the arguments.
///
/// Examples:
///
/// ```rust
/// let mut args = std::env::args();
/// args.next();
/// let args = argparse(args);
///
/// assert_eq!(args.options, vec![String::from("dog")])
/// assert_eq!(args.flags, vec![String::from("f")])
/// assert_eq!(args.arguments, vec![String::from("dig")])
/// assert_eq!(args.command, String::from("lawn"))
/// ```
/// ```console
/// % cargo run -- --dog -f lawn dig
/// ```

pub fn argparse(mut args: Args) -> Arguments {
    let mut options: Vec<String> = vec![];
    let mut flags: Vec<String> = vec![];
    let mut arguments: Vec<String> = vec![];

    while let Some(arg) = args.next() {
        if arg.starts_with("--") {
            options.push(arg.trim_start_matches("--").to_string());
        } else if arg.starts_with("-") {
            flags.extend(arg.chars().skip(1).map(|c| c.to_string()));
        } else {
            arguments.push(arg);
        }
    }

    let command = if arguments.len() > 0 {
        arguments.remove(0)
    } else {
        String::new()
    };

    Arguments {
        options,
        flags,
        command,
        arguments,
    }
}

/// Arguments Struct.
///
/// Examples:
/// ```rust
/// Arguments {
///     options: vec![String::from("dog")],
///     flags: vec![String::from("f")],
///     command: String::from("lawn"),
///     arguments: vec![String::from("dig")],
/// }
/// ```
pub struct Arguments {
    pub options: Vec<String>,
    pub flags: Vec<String>,
    pub command: String,
    pub arguments: Vec<String>,
}
