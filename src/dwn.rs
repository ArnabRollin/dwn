use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    process::exit,
};

use crate::lexer::{Token, TokenModifiers};

use std::sync::Mutex;

lazy_static! {
    pub static ref HASHMAP: Mutex<HashMap<&'static str, for<'a> fn(Vec<&'a Token>) -> Option<String>>> = {
        let mut m = HashMap::new();
        m.insert("say", say as for<'a> fn(Vec<&'a Token>) -> Option<String>);
        m.insert(
            "short_say",
            short_say as for<'a> fn(Vec<&'a Token>) -> Option<String>,
        );
        m.insert("ask", ask as for<'a> fn(Vec<&'a Token>) -> Option<String>);

        Mutex::new(m)
    };
}

fn get_args(tokens: Vec<&Token>) -> Vec<String> {
    let mut args: Vec<String> = vec![];

    for token in tokens {
        if !token.modifiers.contains(&TokenModifiers::ARGS) {
            break;
        }

        let token_val = &token.val;
        let token_val = token_val.to_owned();

        args.push(token_val);
    }

    args
}

fn say(tokens: Vec<&Token>) -> Option<String> {
    let args = get_args(tokens);

    for arg in args {
        print!("{arg} ");
    }

    println!();

    None
}

fn short_say(tokens: Vec<&Token>) -> Option<String> {
    let args = get_args(tokens);

    for arg in args {
        print!("{arg}");
        match stdout().flush() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{}", e);
                exit(1);
            }
        }
    }

    None
}

fn ask(tokens: Vec<&Token>) -> Option<String> {
    let args = get_args(tokens);

    if args.len() < 1 {
        return Some("Not enough arguments!".to_string());
    }

    let mut input = String::new();
    let prompt = &args[0];

    print!("{prompt}");

    match stdout().flush() {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();

            return Some(e);
        }
    }

    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();

            return Some(e);
        }
    }

    None
}
