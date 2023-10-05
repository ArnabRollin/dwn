use std::sync::RwLockReadGuard;

#[derive(Clone, Debug)]
pub enum TokenTypes {
    VARIABLE,
    FUNC,
    STRING,
    LITERAL,
}

#[derive(PartialEq, Debug, Clone)]
pub enum TokenModifiers {
    ARGS,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub ty: TokenTypes,
    pub modifiers: Vec<TokenModifiers>,
    pub val: String,
}

pub fn tokenize(
    data: String,
    functions: RwLockReadGuard<
        '_,
        std::collections::HashMap<&str, fn(Vec<Token>) -> (Option<String>, Option<String>)>,
    >,
    variables: RwLockReadGuard<'_, std::collections::HashMap<String, String>>,
) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut in_func = false;
    let mut in_string = false;
    let mut string_token = String::new();
    let mut in_variable_set = false;

    let mut literal = String::new();
    let mut in_literal = false;

    for raw_word in data.split(' ') {
        if in_variable_set {
            tokens.push(Token {
                ty: TokenTypes::VARIABLE,
                modifiers: vec![TokenModifiers::ARGS],
                val: raw_word.to_string(),
            });
            in_variable_set = false;
            continue;
        }
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
                    modifiers: if in_func {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: literal.clone(),
                });

                literal.clear();
            } else {
                literal.push_str(word);
            }
        }

        if word == "let" && !in_string {
            in_variable_set = true;
            tokens.push(Token {
                ty: TokenTypes::FUNC,
                modifiers: vec![],
                val: "create_var".to_string(),
            });

            in_func = true;
        }

        if word == ";" && !in_string {
            continue;
        }

        if variables.contains_key(word) {
            if !in_literal && !in_string {
                tokens.push(Token {
                    ty: TokenTypes::VARIABLE,
                    modifiers: if in_func {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: word.to_string(),
                });
            }
        }

        if word.starts_with('"') {
            if in_string {
                string_token.push(' ');
                if !in_literal {
                    tokens.push(Token {
                        ty: TokenTypes::STRING,
                        modifiers: if in_func {
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
                        modifiers: if in_func {
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
                        modifiers: if in_func {
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
        }
    }

    tokens
}
