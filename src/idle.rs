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
    let mut count = 1;

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

        let mut scope = 0;

        run(
            code.to_string(),
            get_funcs(),
            &mut Metadata {
                line_count: count,
                scope: &mut scope,
            },
        );

        count += 1;
    }
}
