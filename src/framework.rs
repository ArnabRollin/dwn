use std::{fs, process::exit};

use crate::dwn::{FUNCTIONS, VARIABLES};

pub fn make_framework() {
    let mut text = String::from("funcs:\n");

    for func in FUNCTIONS
        .read()
        .expect("Error: Could not access functions!")
        .keys()
    {
        text.push_str(func);
        text.push('\n');
    }

    text.push(';');
    text.push('\n');

    text.push_str("vars:\n");
    for var in VARIABLES
        .read()
        .expect("Error: Could not access functions!")
        .keys()
    {
        text.push_str(var);
        text.push('\n');
    }

    text.push(';');
    text.push('\n');

    text.push_str("keywordOther:\n");
    text.push_str("let\n");

    text.push(';');
    text.push('\n');

    match fs::write("framework.fw", text) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}
