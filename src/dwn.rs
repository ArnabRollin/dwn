use std::{
    collections::HashMap,
    io::{stdout, Write},
    process::exit,
};

use crate::lexer::{Token, TokenModifiers};

use std::sync::Mutex;

lazy_static! {
    pub static ref HASHMAP: Mutex<HashMap<&'static str, for<'a> fn(Vec<&'a Token>) -> Option<&'a str>>> = {
        let mut m = HashMap::new();
        m.insert("say", say as for<'a> fn(Vec<&'a Token>) -> Option<&'a str>);
        m.insert(
            "short_say",
            short_say as for<'a> fn(Vec<&'a Token>) -> Option<&'a str>,
        );
        m.insert("ask", ask as for<'a> fn(Vec<&'a Token>) -> Option<&'a str>);

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

fn say(tokens: Vec<&Token>) -> Option<&str> {
    let args = get_args(tokens);

    for arg in args {
        print!("{arg}");
    }

    println!();

    None
}

fn short_say(tokens: Vec<&Token>) -> Option<&str> {
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

fn ask(tokens: Vec<&Token>) -> Option<&str> {
    let args = get_args(tokens);

    if args.len() < 1 {
        return Some("Not enough arguments!");
    } else {
        return None;
    }
}
