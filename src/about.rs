//! Help generators for Dawn (dwn).

/// Generates help about Dawn (dwn).
///
/// Examples:
/// ```rust
/// about() // Prints `Usage: dwn [options...] [command] [args...]`
/// ```
pub fn about() {
    let about = r#"
Dawn (dwn) is the interpreter and bytecode compiler for the Dawn Programming Language.

Usage:
    dwn -h | --help
    dwn -v | --version
    dwn help (<command>)
    dwn run (<path_to_file>)
    dwn bytec | bytecodec | bytc (<path_to_file>) [--level=<lvl>]
    dwn byterun | bytecoderun | bytrun (<path_to_bytecode>)
    dwn idle
    dwn framework | fw
Options:
    -h --help       Show this message and exit.
    -v --version    Show the version and exit. 
    --level=<lvl>   Set the level of the engine which bytecode compiles the file.
                    Use `latest` (case insensitive) to set to the latest engine. [default: LATEST]
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
Shows information about a command and how to use it.

Usage: dwn help (<command>)
    "#
    .trim();
    let info_run: &str = r#"
Runs a Dawn project file.

Usage: dwn run (<path_to_file>)
    "#
    .trim();
    let info_bytec: &str = r#"
Bytecode compiles a Dawn project file.

Usage:
    dwn bytec (<path_to_file>)
    dwn bytecodec (<path_to_file>)
    dwn bytc (<path_to_file>)
    dwn bytec (<path_to_file>) [--level=<lvl>]
    dwn bytecodec (<path_to_file>) [--level=<lvl>]
    dwn bytc (<path_to_file>) [--level=<lvl>]
    "#
    .trim();
    let info_byterun: &str = r#"
Runs a Dawn bytecode file.

Usage:
    dwn byterun (<path_to_file>)
    dwn bytecoderun (<path_to_file>)
    dwn bytrun (<path_to_file>)
    "#
    .trim();
    let info_idle: &str = r#"
Starts the Integrated Development and Learning Environment (IDLE)

Usage: dwn idle
    "#
    .trim();
    let info_framework: &str = r#"
Creates a framework for Dawn Programming Language extensions.

Usage:
    dwn framework
    dwn fw
    "#
    .trim();

    match command.unwrap_or(&String::new()).as_str() {
        "help" => println!("{}", info_help),
        "run" => println!("{}", info_run),
        "bytec" | "bytecodec" | "bytc" => println!("{}", info_bytec),
        "byterun" | "bytecoderun" | "bytrun" => println!("{}", info_byterun),
        "idle" => println!("{}", info_idle),
        "framework" | "fw" => println!("{}", info_framework),
        command => println!("Unknown command: {}", command),
    }
}
