use std::{
    io::{stdin, stdout, Write},
    process::exit,
};

use crate::{
    dwn::{FUNCTIONS, VARIABLES},
    runner::run,
};

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

        run(
            code.to_string(),
            count,
            FUNCTIONS.read().unwrap(),
            VARIABLES.read().unwrap(),
        );

        count += 1;
    }
}
