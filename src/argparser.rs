//! Argument parser for Dawn (dwn).

use std::{collections::HashMap, env::Args};

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
    let mut variables: HashMap<String, String> = HashMap::new();

    while let Some(arg) = args.next() {
        if arg.starts_with("--") {
            let varsplit: Vec<&str> = arg.split('=').collect();

            if varsplit[..varsplit.len() - 1].is_empty() {
                options.extend(
                    arg.trim_start_matches("--")
                        .split('-')
                        .map(|s| s.to_string()),
                );
            } else {
                variables.insert(
                    varsplit[0].trim_start_matches("--").to_string(),
                    varsplit[1].to_string(),
                );
            }
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
        variables,
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
///     variables: vec![],
/// }
/// ```
pub struct Arguments {
    pub options: Vec<String>,
    pub flags: Vec<String>,
    pub command: String,
    pub arguments: Vec<String>,
    pub variables: HashMap<String, String>,
}

#[test]
fn argument_parser() {
    fn argparse(args: Vec<String>) -> Arguments {
        let mut options: Vec<String> = vec![];
        let mut flags: Vec<String> = vec![];
        let mut arguments: Vec<String> = vec![];
        let mut variables: HashMap<String, String> = HashMap::new();

        let mut args = args.iter();

        while let Some(arg) = args.next() {
            if arg.starts_with("--") {
                let varsplit: Vec<&str> = arg.split('=').collect();

                if varsplit[..varsplit.len() - 1].is_empty() {
                    options.extend(
                        arg.trim_start_matches("--")
                            .split('-')
                            .map(|s| s.to_string()),
                    );
                } else {
                    variables.insert(
                        varsplit[0].trim_start_matches("--").to_string(),
                        varsplit[1].to_string(),
                    );
                }
            } else if arg.starts_with("-") {
                flags.extend(arg.chars().skip(1).map(|c| c.to_string()));
            } else {
                arguments.push(arg.to_string());
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
            variables,
        }
    }
    let args = argparse(vec![
        "klein".to_string(),
        "--c-c-c".to_string(),
        "-dd".to_string(),
        "cey".to_string(),
        "--level=1".to_string(),
    ]);

    let mut variables_assert = HashMap::new();
    variables_assert.insert("level".to_string(), "1".to_string());

    assert_eq!(args.command, "klein".to_string());
    assert_eq!(args.arguments, vec!["cey".to_string()]);
    assert_eq!(
        args.options,
        vec!["c".to_string(), "c".to_string(), "c".to_string()]
    );
    assert_eq!(args.flags, vec!["d".to_string(), "d".to_string()]);
    assert_eq!(args.variables, variables_assert);
}
