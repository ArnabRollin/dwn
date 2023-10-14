//! This file is used to run the IDLE for Dawn (dwn)

use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use crate::{
    dwn::{get_funcs, Metadata},
    runner::run,
};

/// The IDLE function
pub fn idle() {
    let mut count: usize = 1;
    let mut scope = 0;
    let mut in_scope = false;
    let mut scope_token = String::new();
    let mut current_tokens = vec![];

    loop {
        let mut code = String::new();

        print!("> ");
        match stdout().flush() {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        }

        match stdin().read_line(&mut code) {
            Ok(_) => {}
            Err(e) => {
                eprintln!("{e}");
                exit(1);
            }
        }

        let code = code.trim();

        if code.to_lowercase() == "quit" {
            break;
        }

        run(
            code.to_string(),
            get_funcs(),
            &mut Metadata {
                line_count: count,
                scope: &mut scope,
                in_scope: &mut in_scope,
                scope_token: &mut scope_token,
                current_tokens: &mut current_tokens,
            },
        );

        count += 1;
    }
}
