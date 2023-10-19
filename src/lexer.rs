//! The lexer for Dawn (dwn)

use std::process::exit;

use crate::dwn::{get_funcs, Metadata, VARIABLES};

/// The token types.
#[derive(Clone, Debug, PartialEq)]
pub enum TokenTypes {
    VARIABLE,
    FUNC,
    STRING,
    LITERAL,
    INT,
    FLOAT,
    NAME,
    SCOPE,
    NONE,
    BOOL,
}

/// The token modifiers.
#[derive(PartialEq, Debug, Clone)]
pub enum TokenModifiers {
    ARGS,
}

/// The token struct.
#[derive(Debug, Clone, PartialEq)]
pub struct Token {
    pub ty: TokenTypes,
    pub modifiers: Vec<TokenModifiers>,
    pub val: String,
}

/// The tokenizer function.
///
/// Examples:
///
/// ```rust
/// let tokens = tokenize(
///     "say \"Hello World\"".to_string(),
///     FUNCTIONS.read().unwrap(),
///     VARIABLES.read().unwrap(),
/// );
///
/// assert_eq!(
///     tokens,
///     vec![
///         Token {
///             ty: TokenTypes::FUNC,
///             modifiers: vec![],
///             val: "say".to_string()
///         },
///         Token {
///             ty: TokenTypes::STRING,
///             modifiers: vec![TokenModifiers::ARGS],
///             val: "Hello World".to_string()
///         },
///     ]
/// )
/// ```
pub fn tokenize(data: String, meta: &mut Metadata) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut in_func = false;
    let mut in_string = false;
    let mut string_token = String::new();
    let mut in_variable_set = false;
    let mut in_operator = false;
    let mut in_compare = false;

    let mut literal = String::new();
    let mut in_literal = false;

    let functions = get_funcs();
    let variables = VARIABLES.read().unwrap();

    if *meta.in_scope {
        if data.starts_with('}') {
            *meta.in_scope = false;
            let scope_token = meta.scope_token.to_string();
            meta.scope_token.clear();

            let mut tokens: Vec<Token> = vec![];
            for t in meta.current_tokens.clone() {
                tokens.push(t);
            }

            match meta.current_tokens[0].ty {
                TokenTypes::FUNC => tokens.push(Token {
                    ty: TokenTypes::SCOPE,
                    modifiers: vec![TokenModifiers::ARGS],
                    val: scope_token,
                }),
                _ => {
                    eprintln!(
                        "Error on line {}: No function found to run scope!",
                        meta.line_count
                    );
                    exit(1);
                }
            }
            return tokens;
        } else {
            meta.scope_token.push_str(&data);
            meta.scope_token.push('\n');
            return vec![];
        }
    }

    for raw_word in data.split(' ') {
        let word = if raw_word.starts_with('(') && !in_string {
            in_literal = true;
            &raw_word[1..]
        } else {
            raw_word
        };

        if in_literal {
            if raw_word.ends_with(')') {
                if in_string {
                    if raw_word.strip_suffix(')').unwrap().ends_with('"') {
                        in_string = false;
                    }
                }
            }

            if !raw_word.starts_with('(') {
                literal.push(' ');
            }

            if raw_word.ends_with(')') && !in_string {
                in_literal = false;
                literal.push_str(&word[..word.len() - 1]);

                tokens.push(Token {
                    ty: TokenTypes::LITERAL,
                    modifiers: if in_func || in_compare {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: literal.clone(),
                });

                literal.clear();

                continue;
            } else {
                literal.push_str(word);
                continue;
            }
        }

        if word == "None" && !in_string {
            if !in_literal {
                tokens.push(Token {
                    ty: TokenTypes::NONE,
                    modifiers: if in_func || in_compare {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: "None".to_string(),
                });
            }
            continue;
        }
        if word == "true" && !in_string {
            if !in_literal {
                tokens.push(Token {
                    ty: TokenTypes::BOOL,
                    modifiers: if in_func || in_compare {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: "true".to_string(),
                });
            }
            continue;
        }
        if word == "false" && !in_string {
            if !in_literal {
                tokens.push(Token {
                    ty: TokenTypes::BOOL,
                    modifiers: if in_func || in_compare {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: "false".to_string(),
                });
            }
            continue;
        }

        if word == "let" && !in_string {
            in_variable_set = true;
            tokens.push(Token {
                ty: TokenTypes::FUNC,
                modifiers: vec![],
                val: "create_var".to_string(),
            });

            in_func = true;

            continue;
        }

        if word == ";" || word == "=" && !in_string {
            continue;
        }

        if variables.contains_key(word) {
            if !in_literal && !in_string {
                tokens.push(Token {
                    ty: TokenTypes::VARIABLE,
                    modifiers: if in_func || in_operator || in_compare {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: word.to_string(),
                });
            }

            continue;
        }

        if word == "==" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No first value for comparison operator '==' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "eq".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_compare = true;
            };

            continue;
        }
        if word == ">" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No first value for comparison operator '>' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "gt".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }
        if word == "<" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No first value for comparison operator '<' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "lt".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }

        if word == "+" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No first number for operator '+' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                match first.ty {
                    TokenTypes::INT | TokenTypes::FLOAT | TokenTypes::VARIABLE => {}
                    _ => {
                        eprintln!(
                            "Error on line {}: No first number for operator '+' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                }

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "sum".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }
        if word == "-" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No first number for operator '-' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                match first.ty {
                    TokenTypes::INT | TokenTypes::FLOAT | TokenTypes::VARIABLE => {}
                    _ => {
                        eprintln!(
                            "Error on line {}: No first number for operator '-' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                }

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "difference".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }
        if word == "*" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No first number for operator '*' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                match first.ty {
                    TokenTypes::INT | TokenTypes::FLOAT | TokenTypes::VARIABLE => {}
                    _ => {
                        eprintln!(
                            "Error on line {}: No first number for operator '*' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                }

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "product".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }
        if word == "/" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No first number for operator '/' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                match first.ty {
                    TokenTypes::INT | TokenTypes::FLOAT | TokenTypes::VARIABLE => {}
                    _ => {
                        eprintln!(
                            "Error on line {}: No first number for operator '/' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                }

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "quotient".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }

        if word == "+=" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No variable for operator '+=' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                match first.ty {
                    TokenTypes::VARIABLE => {}
                    _ => {
                        eprintln!(
                            "Error on line {}: No variable for operator '+=' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                }

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "add_assign".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }

        if word == "-=" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No variable for operator '-=' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                match first.ty {
                    TokenTypes::VARIABLE => {}
                    _ => {
                        eprintln!(
                            "Error on line {}: No variable for operator '-=' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                }

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "subtract_assign".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }

        if word == "*=" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No variable for operator '*=' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                match first.ty {
                    TokenTypes::VARIABLE => {}
                    _ => {
                        eprintln!(
                            "Error on line {}: No variable for operator '*=' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                }

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "multiply_assign".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }

        if word == "/=" && !in_string {
            if !in_literal {
                let first = tokens.pop();

                let first = match first {
                    Some(token) => token,
                    None => {
                        eprintln!(
                            "Error on line {}: No variable for operator '/=' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                };

                match first.ty {
                    TokenTypes::VARIABLE => {}
                    _ => {
                        eprintln!(
                            "Error on line {}: No variable for operator '/=' !",
                            meta.line_count
                        );
                        exit(1);
                    }
                }

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: "divide_assign".to_string(),
                });

                tokens.push(Token {
                    modifiers: vec![TokenModifiers::ARGS],
                    ..first
                });

                in_operator = true;
            };

            continue;
        }

        if word.parse::<i32>().is_ok() && !in_string {
            if !in_literal {
                tokens.push(Token {
                    ty: TokenTypes::INT,
                    modifiers: if in_func || in_operator || in_compare {
                        if in_operator {
                            in_operator = false;
                        }

                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: word.to_string(),
                })
            }

            continue;
        }

        if word.parse::<f32>().is_ok() && !in_string {
            if !in_literal {
                tokens.push(Token {
                    ty: TokenTypes::FLOAT,
                    modifiers: if in_func || in_operator || in_compare {
                        if in_operator {
                            in_operator = false;
                        }

                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: word.to_string(),
                })
            }

            continue;
        }

        if word == "{" && !in_string {
            *meta.scope += 1;
            *meta.in_scope = true;

            *meta.current_tokens = tokens.clone();

            continue;
        }

        if word.starts_with('"') {
            if in_string {
                string_token.push(' ');
                if !in_literal {
                    tokens.push(Token {
                        ty: TokenTypes::STRING,
                        modifiers: if in_func || in_compare {
                            vec![TokenModifiers::ARGS]
                        } else {
                            vec![]
                        },
                        val: string_token.clone(),
                    });
                }

                in_string = false;
                continue;
            }

            in_string = true;

            string_token.push_str(if word == "\"" { " " } else { &word[1..] });

            if word.ends_with('"') {
                in_string = false;

                string_token.pop();
                if !in_literal {
                    tokens.push(Token {
                        ty: TokenTypes::STRING,
                        modifiers: if in_func || in_compare {
                            vec![TokenModifiers::ARGS]
                        } else {
                            vec![]
                        },
                        val: string_token.clone(),
                    });
                }

                string_token.clear();
            }

            continue;
        }

        if in_string {
            string_token.push(' ');
            string_token.push_str(word);

            if word.ends_with('"') {
                in_string = false;

                string_token.pop();
                if !in_literal {
                    tokens.push(Token {
                        ty: TokenTypes::STRING,
                        modifiers: if in_func || in_compare {
                            vec![TokenModifiers::ARGS]
                        } else {
                            vec![]
                        },
                        val: string_token.clone(),
                    });
                }

                string_token.clear();
            }

            continue;
        }

        if functions.contains_key(word) {
            if !in_literal {
                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: word.to_string(),
                });
            }

            in_func = true;

            continue;
        }

        if in_variable_set {
            tokens.push(Token {
                ty: TokenTypes::STRING,
                modifiers: vec![TokenModifiers::ARGS],
                val: word.to_string(),
            });
            in_variable_set = false;
            continue;
        }

        if !in_string {
            if !in_literal {
                tokens.push(Token {
                    ty: TokenTypes::NAME,
                    modifiers: if in_func || in_compare {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: word.to_string(),
                });
            }

            continue;
        }
    }

    tokens
}

#[test]
fn tokenizer() {
    let tokens = tokenize(
        "say \"Hello World\"".to_string(),
        &mut Metadata {
            line_count: 0,
            scope: &mut 0,
            in_scope: &mut false,
            scope_token: &mut String::new(),
            current_tokens: &mut vec![],
        },
    );

    assert_eq!(
        tokens,
        vec![
            Token {
                ty: TokenTypes::FUNC,
                modifiers: vec![],
                val: "say".to_string()
            },
            Token {
                ty: TokenTypes::STRING,
                modifiers: vec![TokenModifiers::ARGS],
                val: "Hello World".to_string()
            },
        ]
    )
}
