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
        for r_obj in paren_split(word, in_string) {
            let obj = if r_obj.ends_with(')') {
                r_obj.strip_suffix(')').unwrap()
            } else {
                r_obj
            };

            if obj.starts_with('"') {
                in_string = true;

                if obj.ends_with('"') {
                    in_string = false;
                    string_token = obj[1..obj.len() - 1].to_string();

                    tokens.push(Token {
                        ty: TokenTypes::STRING,
                        modifiers: if in_func {
                            vec![TokenModifiers::ARGS]
                        } else {
                            vec![]
                        },
                        val: string_token.to_string(),
                    });
                } else {
                    string_token = obj[1..].to_string();
                }

                continue;
            }

            if in_string {
                string_token.push(' ');

                if obj.ends_with('"') {
                    in_string = false;
                    string_token.push_str(&obj[..obj.len() - 1]);

                    tokens.push(Token {
                        ty: TokenTypes::STRING,
                        modifiers: if in_func {
                            vec![TokenModifiers::ARGS]
                        } else {
                            vec![]
                        },
                        val: string_token.to_string(),
                    });

                    continue;
                } else {
                    string_token.push_str(obj);
                }
            }

            if obj.ends_with('"') {
                if in_string {
                    string_token.push(' ');
                    string_token.push_str(&obj[..obj.len() - 1]);
                }

                tokens.push(Token {
                    ty: TokenTypes::STRING,
                    modifiers: if in_func {
                        vec![TokenModifiers::ARGS]
                    } else {
                        vec![]
                    },
                    val: string_token.to_string(),
                });

                in_string = false;
            }

            if HASHMAP.lock().unwrap().contains_key(obj) {
                in_func = true;

                tokens.push(Token {
                    ty: TokenTypes::FUNC,
                    modifiers: vec![],
                    val: obj.to_string(),
                })
            }

            if r_obj.ends_with(')') {
                in_func = false;
            }
        }
    }

    tokens
}

fn paren_split(data: &str, in_string: bool) -> std::str::Split<'_, char> {
    if data.starts_with('"') || in_string {
        data.split(' ')
    } else {
        data.split('(')
    }
}
