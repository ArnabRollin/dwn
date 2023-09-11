//! Help generators for Dawn (dwn).

/// Generates help about Dawn (dwn).
///
/// Examples:
/// ```rust
/// about() // Prints `Usage: dwn [options...] [command] [args...]`
/// ```
pub fn about() {
    let about = r#"
	Usage: dwn [options...] [command] [args...]

	"#
    .trim();

    println!("{}", about);
}

/// Generates help about a command.
///
/// Examples
/// ```rust
/// help("help") // Prints -->
/// /*
/// Usage: dwn help [command]
///
/// Shows information about a command and how to use it.
///  */
/// ```
pub fn help(command: Option<&String>) {
    let info_help: &str = r#"
Usage: dwn help [command]

Shows information about a command and how to use it.
"#
    .trim();
    match command.unwrap_or(&String::new()).as_str() {
        "help" => println!("{}", info_help),
        command => println!("Unknown command: {}", command),
    }
}
