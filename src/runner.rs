//! The runner for Dawn (dwn)

use crate::dwn::Metadata;
use crate::lexer::{tokenize, Token, TokenTypes};
use std::process::exit;
use std::sync::RwLockReadGuard;

/// The runner function
///
/// Examples:
///
/// ```rust
/// let none = run(
///     "say \"Hello World!\"".to_string(),
///     1,
///     FUNCTIONS.read().unwrap(),
///     VARIABLES.read().unwrap(),
/// );
/// assert_eq!(none.val, "None".to_string());
/// ```
pub fn run(
    line: String,
    functions: RwLockReadGuard<
        '_,
        std::collections::HashMap<
            &str,
            for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        >,
    >,
    meta: &mut Metadata,
) -> Token {
    let tokens = tokenize(line, meta);

    run_tokens(tokens, functions, meta, false).unwrap()
}

pub fn run_tokens(
    tokens: Vec<Token>,
    functions: RwLockReadGuard<
        '_,
        std::collections::HashMap<
            &str,
            for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        >,
    >,
    meta: &mut Metadata,
    capture_errors: bool,
) -> Result<Token, String> {
    let functions_ = functions.clone();

    if tokens.len() > 0 {
        match tokens[0].ty.clone() {
            TokenTypes::FUNC => {
                let fname = tokens[0].val.as_str();
                let f = functions_.get(fname);

                match f {
                    Some(f) => {
                        let mut args: Vec<Token> = vec![];
                        let mut tokens = tokens.iter();

                        tokens.next();

                        for token in tokens {
                            args.push(Token { ..token.clone() })
                        }

                        let ret = if !*meta.in_scope {
                            f(args, meta)
                        } else {
                            return Ok(Token {
                                ty: TokenTypes::NONE,
                                modifiers: vec![],
                                val: "None".to_string(),
                            });
                        };

                        match ret {
                            Ok(token) => return Ok(token),
                            Err(err) => {
                                if capture_errors {
                                    return Err(format!(
                                        "Error on line {}: {}",
                                        meta.line_count + 1,
                                        err
                                    ));
                                }
                                eprintln!("Error on line {}: {}", meta.line_count + 1, err);
                                exit(1);
                            }
                        }
                    }
                    None => {
                        if capture_errors {
                            return Err(format!(
                                "Error: Function {} does not exist!",
                                tokens[0].val
                            ));
                        }
                        eprintln!("Error: Function {} does not exist!", tokens[0].val);
                        exit(1);
                    }
                }
            }
            ty => {
                return Ok(Token {
                    ty,
                    modifiers: vec![],
                    val: tokens[0].val.to_string(),
                })
            }
        }
    } else {
        return Ok(Token {
            ty: TokenTypes::NONE,
            modifiers: vec![],
            val: "None".to_string(),
        });
    }
}

#[test]
fn line_runner() {
    use crate::dwn::get_funcs;

    let none = run(
        "say \"Hello World!\"".to_string(),
        get_funcs(),
        &mut Metadata {
            line_count: 0,
            scope: &mut 0,
            in_scope: &mut false,
            scope_token: &mut String::new(),
            current_tokens: &mut vec![],
        },
    );

    assert_eq!(none.val, "None".to_string());
}
