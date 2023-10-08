use crate::lexer::{tokenize, Token, TokenTypes};
use std::process::exit;
use std::sync::RwLockReadGuard;

pub fn run(
    line: String,
    line_count: usize,
    functions: RwLockReadGuard<
        '_,
        std::collections::HashMap<&str, fn(Vec<Token>) -> Result<Token, String>>,
    >,
    variables: RwLockReadGuard<'_, std::collections::HashMap<String, String>>,
) -> Token {
    let functions_ = functions.clone();
    let tokens = tokenize(line, functions, variables);

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

                        let ret = f(args);

                        match ret {
                            Ok(token) => return token,
                            Err(err) => {
                                eprintln!("\nError on line {}: {}", line_count + 1, err);
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
    use crate::dwn::FUNCTIONS;
    use crate::dwn::VARIABLES;

    let none = run(
        "say \"Hello World!\"".to_string(),
        1,
        FUNCTIONS.read().unwrap(),
        VARIABLES.read().unwrap(),
    );

    assert_eq!(none.val, "None".to_string());
}
