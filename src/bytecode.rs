//! The bytecode compiler for Dawn (dwn)

use std::collections::HashMap;
use std::fs::{write, File};
use std::io::{BufRead, BufReader, Read};
use std::process::exit;

use crate::dwn::{get_funcs, Metadata, Variable, VARIABLES};
use crate::lexer::{tokenize, Token, TokenModifiers, TokenTypes};
use crate::runner::run_tokens;

lazy_static! {
    static ref TYPES: HashMap<&'static str, TokenTypes> = {
        let mut m = HashMap::new();
        m.insert("b", TokenTypes::BOOL);
        m.insert("fl", TokenTypes::FLOAT);
        m.insert("fu", TokenTypes::FUNC);
        m.insert("i", TokenTypes::INT);
        m.insert("l", TokenTypes::LITERAL);
        m.insert("na", TokenTypes::NAME);
        m.insert("no", TokenTypes::NONE);
        m.insert("sc", TokenTypes::SCOPE);
        m.insert("st", TokenTypes::STRING);
        m.insert("v", TokenTypes::VARIABLE);
        m.insert("a", TokenTypes::ARRAY);
        m
    };
}

/// The function used to bytecode compile files.
pub fn bytecode_compile_file(file: Option<&String>, level: Option<&String>) {
    if file.is_none() {
        eprintln!("Error: Please provide a file to compile!");
        exit(1);
    }

    let file = file.unwrap();

    let max_level = 1;
    let level = if level.is_none() {
        max_level
    } else {
        let level = level.unwrap();
        if level.to_lowercase() == "latest".to_string() {
            max_level
        } else {
            match level.parse::<i64>() {
                Ok(level) => level,
                Err(_) => {
                    eprintln!("Error: Level must be a number or 'latest' !");
                    exit(1);
                }
            }
        }
    };

    let reader =
        BufReader::new(File::open(file).expect(format!("Cannot open file `{}`", file).as_str()));

    match level {
        1 => bytec_lvl1(reader, file),
        lvl => {
            eprintln!("Error: Bytecode compiler level {lvl} has not been implemented!");
            exit(1);
        }
    }
}

fn bytec_lvl1(reader: BufReader<File>, file: &String) {
    let mut scope = 0;
    let mut in_scope = false;
    let mut scope_token = String::new();
    let mut current_tokens = vec![];

    let mut bytecode = String::new();
    bytecode.push('1');
    bytecode.push('\x04');
    bytecode.push('\n');

    for (count, line) in reader.lines().enumerate() {
        let line = remove_all_after(line.unwrap(), ';');

        if line.trim().is_empty() {
            bytecode.push('\x03');
            continue;
        }

        let tokens = tokenize(
            line.trim_end().to_string(),
            &mut Metadata {
                line_count: count,
                scope: &mut scope,
                in_scope: &mut in_scope,
                scope_token: &mut scope_token,
                current_tokens: &mut current_tokens,
            },
        );

        if (tokens.len() > 0
            && tokens[0]
                == Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "create_var".to_string(),
                })
        {
            VARIABLES.write().unwrap().insert(
                tokens[1].val.to_string(),
                Variable {
                    scope,
                    value: tokens[2].clone(),
                },
            );
        }

        for token in tokens {
            let mut type_ = "na";

            for (&key, value) in TYPES.iter() {
                if value == &token.ty {
                    type_ = key;
                    break;
                }
            }

            let mut modifiers: Vec<&str> = vec![];

            for modifier in token.modifiers {
                modifiers.push(match modifier {
                    TokenModifiers::ARGS => "a",
                })
            }

            let value = token.val;

            bytecode.push_str(type_);
            bytecode.push('\x00');

            for modifier in modifiers {
                bytecode.push_str(modifier);
                bytecode.push('\x01');
            }

            bytecode.push('\x00');
            bytecode.push_str(&value);
            bytecode.push('\x02');
        }

        bytecode.push('\x03');
    }

    let mut file_without_ext = file.split('.').collect::<Vec<&str>>();
    file_without_ext.pop();

    let mut outfile = file_without_ext.join(".");
    outfile.push_str(".light");

    match write(outfile, bytecode) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }
}

pub fn bytecode_run(bytecode_file: Option<&String>) {
    if bytecode_file.is_none() {
        eprintln!("Error: Please provide a file to compile!");
        exit(1);
    }

    let bytecode_file = bytecode_file.unwrap();

    let mut reader = BufReader::new(
        File::open(bytecode_file).expect(format!("Cannot open file `{}`", bytecode_file).as_str()),
    );

    let mut level = String::new();
    match reader.read_line(&mut level) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }

    let level = match level.split('\x04').next() {
        Some(l) => match l.parse::<i64>() {
            Ok(l) => l,
            Err(_) => {
                eprintln!("Error: Level is not numeric in bytecode file!");
                exit(1);
            }
        },
        None => {
            eprintln!("Error: Could not determine level from bytecode file!");
            exit(1);
        }
    };

    match level {
        1 => byterun_lvl1(reader),
        lvl => {
            eprintln!("Error: Bytecode runner level {lvl} has not been implemented!");
            exit(1);
        }
    }
}

fn byterun_lvl1(mut reader: BufReader<File>) {
    let mut text = String::new();

    match reader.read_to_string(&mut text) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{e}");
            exit(1);
        }
    }

    let bytecode_lines: Vec<&str> = text.split('\x03').collect();
    let mut scope = 0;
    let mut in_scope = false;
    let mut scope_token = String::new();
    let mut current_tokens = vec![];

    for (count, bytecode_line) in bytecode_lines.iter().enumerate() {
        let tokens: Vec<&str> = bytecode_line.split('\x02').collect();
        let mut tokens_vec: Vec<Token> = vec![];

        for token in &tokens[..tokens.len() - 1] {
            let token_parts: Vec<&str> = token.split('\x00').collect();

            let mut token_parts = token_parts.iter();

            let type_part = token_parts.next();
            let modifier_part = token_parts.next();
            let value_part = token_parts.next();

            let type_ = match type_part {
                Some(ty) => {
                    match TYPES.get(ty) {
                        Some(ty) => ty,
                        None => {
                            eprintln!("(type_get no_found: >>{ty}<<) Error: Invalid format in bytecode file!");
                            exit(1);
                        }
                    }
                }
                None => {
                    eprintln!("(type_get no_part_found) Error: Invalid format in bytecode file!");
                    exit(1);
                }
            };

            let modifiers = match modifier_part {
                Some(modifier_part) => {
                    let modifier_parts: Vec<&str> = modifier_part.split('\x01').collect();
                    let mut modifiers: Vec<TokenModifiers> = vec![];

                    for modifier_part in &modifier_parts[..modifier_parts.len() - 1] {
                        modifiers.push(match modifier_part {
                            &"a" => TokenModifiers::ARGS,
                            modifier => {
                                eprintln!("(modifier_get no_found: >>{modifier}<<) Error: Invalid format in bytecode file!");
                                exit(1);
                            }
                        })
                    }

                    modifiers
                }
                None => {
                    eprintln!(
                        "(modifier_get no_part_found) Error: Invalid format in bytecode file!"
                    );
                    exit(1);
                }
            };

            let val = match value_part {
                Some(val) => val.to_string(),
                None => {
                    eprintln!("(val_get no_part_found) Error: Invalid format in bytecode file!");
                    exit(1);
                }
            };

            tokens_vec.push(Token {
                ty: type_.clone(),
                modifiers,
                val,
            });
        }

        run_tokens(
            tokens_vec,
            get_funcs(),
            &mut Metadata {
                line_count: count,
                scope: &mut scope,
                in_scope: &mut in_scope,
                scope_token: &mut scope_token,
                current_tokens: &mut current_tokens,
            },
        );
    }
}

/// The function to remove every character in `text` after `ch` is reached (including `ch`).
///
/// Examples:
///
/// ```rust
/// let new = remove_all_after("say \"Hello!\" ; abcdefghij...".to_string(), ';');
///
/// assert_eq!(new, "say \"Hello!\" ".to_string());
/// ```
fn remove_all_after(text: String, ch: char) -> String {
    text.split(ch).next().unwrap().to_string()
}

#[test]
fn removing_all_after() {
    let new = remove_all_after("say \"Hello!\" ; abcdefghij...".to_string(), ';');
    assert_eq!(new, "say \"Hello!\" ".to_string());
}
