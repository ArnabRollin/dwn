use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    process::exit,
    sync::MutexGuard,
};

use crate::{
    lexer::{Token, TokenModifiers, TokenTypes},
    runner::run,
};

use std::sync::Mutex;

lazy_static! {
    pub static ref FUNCTIONS: Mutex<HashMap<&'static str, for<'a> fn(Vec<&'a Token>) -> (Option<String>, Option<String>)>> = {
        let mut m = HashMap::new();
        m.insert(
            "say",
            say as for<'a> fn(Vec<&'a Token>) -> (Option<String>, Option<String>),
        );
        m.insert(
            "short_say",
            short_say as for<'a> fn(Vec<&'a Token>) -> (Option<String>, Option<String>),
        );
        m.insert(
            "ask",
            ask as for<'a> fn(Vec<&'a Token>) -> (Option<String>, Option<String>),
        );
        m.insert(
            "create_var",
            create_var as for<'a> fn(Vec<&'a Token>) -> (Option<String>, Option<String>),
        );

        Mutex::new(m)
    };
}
lazy_static! {
    pub static ref VARIABLES: Mutex<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert(String::from("__hello__"), String::from("Hello, World!"));

        Mutex::new(m)
    };
}

fn get_args(tokens: Vec<&Token>) -> Vec<&Token> {
    let mut args: Vec<&Token> = vec![];

    for token in tokens {
        if !token.modifiers.contains(&TokenModifiers::ARGS) {
            break;
        }

        args.push(token);
    }

    args
}

fn get_variables() -> MutexGuard<'static, HashMap<String, String>> {
    VARIABLES
        .lock()
        .expect("Error: Another user of this mutex panicked while holding the mutex!")
}

fn say(tokens: Vec<&Token>) -> (Option<String>, Option<String>) {
    let args = get_args(tokens);
    let variables = get_variables();

    for arg in args {
        match arg.ty {
            TokenTypes::VARIABLE => print!(
                "{} ",
                match variables.get(&arg.val) {
                    Some(val) => val,
                    None => return (Some(format!("Variable not found: {}", arg.val)), None),
                }
            ),
            _ => print!("{} ", arg.val),
        };
    }

    println!();

    (None, None)
}

fn short_say(tokens: Vec<&Token>) -> (Option<String>, Option<String>) {
    let args = get_args(tokens);

    for arg in args {
        print!("{}", arg.val);

        match stdout().flush() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }

    (None, None)
}

fn ask(tokens: Vec<&Token>) -> (Option<String>, Option<String>) {
    let args = get_args(tokens);

    if args.len() < 1 {
        return (Some("Not enough arguments!".to_string()), None);
    }

    let mut input = String::new();
    let prompt = &args[0].val;

    print!("{}", prompt);

    match stdout().flush() {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();

            return (Some(e), None);
        }
    }

    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();

            return (Some(e), None);
        }
    }

    (None, Some(input))
}

fn create_var(tokens: Vec<&Token>) -> (Option<String>, Option<String>) {
    let functions = FUNCTIONS
        .lock()
        .expect("Error: Another user of this mutex panicked while holding the mutex!");
    let variables = VARIABLES
        .lock()
        .expect("Error: Another user of this mutex panicked while holding the mutex!");

    let args = get_args(tokens);
    let var_name = &args[0].val;
    let var_val_code = args[1].val.clone();

    let mut variables_ = variables.clone();
    let value = match run(var_val_code, 0, functions, variables) {
        Some(val) => val,
        None => "None".to_string(),
    };

    variables_.insert(var_name.to_string(), value);

    (None, None)
}
