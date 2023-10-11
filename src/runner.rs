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
    let functions_ = functions.clone();
    let tokens = tokenize(line, meta);

    if tokens.len() > 0 {
        match tokens[0].ty.clone() {
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

                        let ret = if !*meta.in_scope {
                            f(args, meta)
                        } else {
                            return Token {
                                ty: TokenTypes::STRING,
                                modifiers: vec![],
                                val: "None".to_string(),
                            };
                        };

                        match ret {
                            Ok(token) => return token,
                            Err(err) => {
                                eprintln!("\nError on line {}: {}", meta.line_count + 1, err);
                                exit(1);
                            }
                        }
                    }
                    None => {
                        eprintln!("Error: Function {} does not exist!", tokens[0].val);
                        exit(1);
                    }
                }
            }
            _ => {
                return Token {
                    ty: TokenTypes::STRING,
                    modifiers: vec![],
                    val: "None".to_string(),
                }
            }
        }
    } else {
        return Token {
            ty: TokenTypes::STRING,
            modifiers: vec![],
            val: "None".to_string(),
        };
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
            in_func: &mut false,
            func_token: &mut String::new(),
        },
    );

    assert_eq!(none.val, "None".to_string());
}
