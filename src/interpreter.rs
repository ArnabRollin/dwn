use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use crate::dwn::{FUNCTIONS, VARIABLES};
use crate::runner::run;

pub fn interpret_file(file: Option<&String>) {
    let file_default = &String::new();
    let file = file.unwrap_or(file_default);

    if file == file_default {
        eprintln!("Error: Please provide a file to run!");
        exit(1);
    }

    let reader =
        BufReader::new(File::open(file).expect(format!("Cannot open file `{}`", file).as_str()));

    for (count, line) in reader.lines().enumerate() {
        let line = remove_all_after(line.unwrap(), ';');
        run(
            line.trim().to_string(),
            count,
            FUNCTIONS.read().unwrap(),
            VARIABLES.read().unwrap(),
        );
    }
}

fn remove_all_after(text: String, ch: char) -> String {
    text.split(ch).next().unwrap().to_string()
}
