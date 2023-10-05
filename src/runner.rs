use crate::lexer::{tokenize, Token, TokenTypes};
use std::process::exit;
use std::sync::RwLockReadGuard;

pub fn run(
    line: String,
    line_count: usize,
    functions: RwLockReadGuard<
        '_,
        std::collections::HashMap<&str, fn(Vec<Token>) -> (Option<String>, Option<String>)>,
    >,
    variables: RwLockReadGuard<'_, std::collections::HashMap<String, String>>,
) -> Option<String> {
    let functions_ = functions.clone();
    let tokens = tokenize(line, functions, variables);

    if tokens.len() > 0 {
        match tokens[0].ty {
            TokenTypes::FUNC => {
                let f = functions_.get(tokens[0].val.as_str());

                match f {
                    Some(f) => {
                        let mut args: Vec<Token> = vec![];
                        let mut tokens = tokens.iter();

                        tokens.next();

                        for token in tokens {
                            args.push(Token { ..token.clone() })
                        }

                        let ret = f(args);

                        let feedback = ret.0;

                        match feedback {
                            Some(err) => {
                                eprintln!("\nError on line {}: {}", line_count + 1, err);
                                exit(1);
                            }
                            None => return ret.1,
                        }
                    }
                    None => {
                        eprintln!("Error: Function {} does not exist!", tokens[0].val);
                        exit(1);
                    }
                }
            }
            _ => return None,
        }
    } else {
        return None;
    }
}
