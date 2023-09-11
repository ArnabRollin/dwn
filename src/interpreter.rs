use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::exit;

use crate::dwn::HASHMAP;
use crate::lexer::{tokenize, Token, TokenTypes};

pub fn interpret(file: Option<&String>) {
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
        let tokens = tokenize(line);

        println!("TOkens {tokens:#?}");

        if tokens.len() > 0 {
            match tokens[0].ty {
                TokenTypes::FUNC => {
                    let functions = HASHMAP.lock().unwrap();

                    let f = functions.get(tokens[0].val.as_str());

                    match f {
                        Some(f) => {
                            let mut args: Vec<&Token> = vec![];

                            for token in &tokens[1..] {
                                args.push(token);
                            }

                            let feedback = f(args);

                            match feedback {
                                Some(err) => {
                                    eprintln!("\nError on line {}: {}", count, err);
                                    exit(1);
                                }
                                None => {}
                            }
                        }
                        None => {
                            eprintln!("Error: Function {} does not exist!", tokens[0].val);
                            exit(1);
                        }
                    }
                }
                _ => {}
            }
        }
    }
}

fn remove_all_after(text: String, ch: char) -> String {
    text.split(ch).next().unwrap().to_string()
}
