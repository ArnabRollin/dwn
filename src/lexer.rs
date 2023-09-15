use crate::dwn::HASHMAP;

#[derive(Clone, Copy, Debug)]
pub enum TokenTypes {
    FUNC,
    STRING,
}

#[derive(PartialEq, Debug)]
pub enum TokenModifiers {
    ARGS,
}

#[derive(Debug)]
pub struct Token {
    pub ty: TokenTypes,
    pub modifiers: Vec<TokenModifiers>,
    pub val: String,
}

pub fn tokenize(data: String) -> Vec<Token> {
    let mut tokens: Vec<Token> = vec![];
    let mut in_func = false;
    let mut in_string = false;
    let mut string_token = String::new();

    for word in data.split(' ') {
        if word == ";" && !in_string && !word.starts_with('"') {
            continue;
        }

        if word.starts_with('"') {
            if in_string {
                string_token.push(' ');
                tokens.push(Token {
                    ty: TokenTypes::STRING,
                    modifiers: if in_func {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: string_token.clone(),
                });

                in_string = false;
                continue;
            }

            in_string = true;

            string_token.push_str(if word == "\"" { " " } else { &word[1..] });

            if word.ends_with('"') {
                in_string = false;

                string_token.pop();
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

            continue;
        }

        if in_string {
            string_token.push(' ');
            string_token.push_str(word);

            if word.ends_with('"') {
                in_string = false;

                string_token.pop();
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

            continue;
        }

        if HASHMAP.lock().unwrap().contains_key(word) {
            tokens.push(Token {
                ty: TokenTypes::FUNC,
                modifiers: vec![],
                val: word.to_string(),
            });

            in_func = true;
        }
    }

    tokens
}
