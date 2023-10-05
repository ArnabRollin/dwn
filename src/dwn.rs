use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    sync::{RwLock, RwLockReadGuard},
};

use crate::{
    lexer::{Token, TokenModifiers, TokenTypes},
    runner::run,
};

lazy_static! {
    pub static ref FUNCTIONS: RwLock<HashMap<&'static str, fn(Vec<Token>) -> (Option<String>, Option<String>)>> = {
        let mut m = HashMap::new();
        m.insert(
            "say",
            say as fn(Vec<Token>) -> (Option<String>, Option<String>),
        );
        m.insert(
            "short_say",
            short_say as fn(Vec<Token>) -> (Option<String>, Option<String>),
        );
        m.insert(
            "ask",
            ask as fn(Vec<Token>) -> (Option<String>, Option<String>),
        );
        m.insert(
            "create_var",
            create_var as fn(Vec<Token>) -> (Option<String>, Option<String>),
        );

        RwLock::new(m)
    };
}
lazy_static! {
    pub static ref VARIABLES: RwLock<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert(String::from("$hello"), String::from("Hello, World!"));

        RwLock::new(m)
    };
}

fn get_args(tokens: Vec<Token>) -> Vec<Token> {
    let mut args: Vec<Token> = vec![];

    for token in tokens {
        if !token.modifiers.contains(&TokenModifiers::ARGS) {
            break;
        }

        let token = match token.ty {
            TokenTypes::LITERAL => {
                let token = Token {
                    ty: TokenTypes::STRING,
                    modifiers: vec![TokenModifiers::ARGS],
                    val: match run(token.val, 0, get_funcs(), get_variables()) {
                        Some(val) => val,
                        None => "None".to_string(),
                    },
                };
                token
            }
            _ => token,
        };

        args.push(token);
    }

    args
}

fn get_funcs() -> RwLockReadGuard<
    'static,
    HashMap<&'static str, fn(Vec<Token>) -> (Option<String>, Option<String>)>,
> {
    FUNCTIONS
        .read()
        .expect("Error: Another user of this mutex panicked while holding the mutex!")
}

fn get_variables() -> RwLockReadGuard<'static, HashMap<String, String>> {
    VARIABLES
        .read()
        .expect("Error: Another user of this mutex panicked while holding the mutex!")
}

fn say(tokens: Vec<Token>) -> (Option<String>, Option<String>) {
    let args = get_args(tokens);
    let variables = get_variables();

    for arg in args {
        match arg.ty {
            TokenTypes::VARIABLE => print!(
                "{}",
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

fn short_say(tokens: Vec<Token>) -> (Option<String>, Option<String>) {
    let args = get_args(tokens);
    let variables = get_variables();

    for arg in args {
        match arg.ty {
            TokenTypes::VARIABLE => print!(
                "{}",
                match variables.get(&arg.val) {
                    Some(val) => val,
                    None => return (Some(format!("Variable not found: {}", arg.val)), None),
                }
            ),
            _ => print!("{} ", arg.val),
        };
    }

    (None, None)
}

fn ask(tokens: Vec<Token>) -> (Option<String>, Option<String>) {
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

    (None, Some(input.trim().to_string()))
}

fn create_var(tokens: Vec<Token>) -> (Option<String>, Option<String>) {
    let args = get_args(tokens);
    let var_name = args[0].val.to_string();
    let var_value = args[1].val.to_string();

    let mut variables = VARIABLES
        .write()
        .expect("Error: Another user of this mutex panicked while holding the mutex!");

    variables.insert(var_name, var_value);

    (None, None)
}
