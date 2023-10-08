use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    sync::{RwLock, RwLockReadGuard},
};

use crate::{
    lexer::{Token, TokenModifiers, TokenTypes},
    runner::run,
};

lazy_static! {
    pub static ref FUNCTIONS: RwLock<HashMap<&'static str, fn(Vec<Token>) -> Result<Token, String>>> = {
        let mut m = HashMap::new();
        m.insert("say", say as fn(Vec<Token>) -> Result<Token, String>);
        m.insert(
            "short_say",
            short_say as fn(Vec<Token>) -> Result<Token, String>,
        );
        m.insert("ask", ask as fn(Vec<Token>) -> Result<Token, String>);
        m.insert(
            "create_var",
            create_var as fn(Vec<Token>) -> Result<Token, String>,
        );
        m.insert("sum", sum as fn(Vec<Token>) -> Result<Token, String>);
        m.insert(
            "difference",
            difference as fn(Vec<Token>) -> Result<Token, String>,
        );
        m.insert(
            "product",
            product as fn(Vec<Token>) -> Result<Token, String>,
        );
        m.insert(
            "quotient",
            quotient as fn(Vec<Token>) -> Result<Token, String>,
        );

        RwLock::new(m)
    };
}
lazy_static! {
    pub static ref VARIABLES: RwLock<HashMap<String, String>> = {
        let mut m = HashMap::new();
        m.insert(String::from("$hello"), String::from("Hello, World!"));

        RwLock::new(m)
    };
}

fn get_args(tokens: Vec<Token>) -> Vec<Token> {
    let mut args: Vec<Token> = vec![];

    for token in tokens {
        if !token.modifiers.contains(&TokenModifiers::ARGS) {
            break;
        }

        let token = match token.ty {
            TokenTypes::LITERAL => {
                let ret = run(token.val, 0, get_funcs(), get_variables());
                ret
            }
            _ => token,
        };

        args.push(token);
    }

    args
}

fn get_funcs(
) -> RwLockReadGuard<'static, HashMap<&'static str, fn(Vec<Token>) -> Result<Token, String>>> {
    FUNCTIONS
        .read()
        .expect("Error: Another user of this mutex panicked while holding the mutex!")
}

fn get_variables() -> RwLockReadGuard<'static, HashMap<String, String>> {
    VARIABLES
        .read()
        .expect("Error: Another user of this mutex panicked while holding the mutex!")
}

fn say(tokens: Vec<Token>) -> Result<Token, String> {
    let args = get_args(tokens);
    let variables = get_variables();

    for arg in args {
        match arg.ty {
            TokenTypes::VARIABLE => print!(
                "{}",
                match variables.get(&arg.val) {
                    Some(val) => val,
                    None => return Err(format!("Variable not found: {}", arg.val)),
                }
            ),
            _ => print!("{} ", arg.val),
        };
    }

    println!();

    Ok(Token {
        ty: TokenTypes::STRING,
        modifiers: vec![],
        val: "None".to_string(),
    })
}

fn short_say(tokens: Vec<Token>) -> Result<Token, String> {
    let args = get_args(tokens);
    let variables = get_variables();

    for arg in args {
        match arg.ty {
            TokenTypes::VARIABLE => print!(
                "{}",
                match variables.get(&arg.val) {
                    Some(val) => val,
                    None => return Err(format!("Variable not found: {}", arg.val)),
                }
            ),
            _ => print!("{} ", arg.val),
        };
    }

    Ok(Token {
        ty: TokenTypes::STRING,
        modifiers: vec![],
        val: "None".to_string(),
    })
}

fn ask(tokens: Vec<Token>) -> Result<Token, String> {
    let args = get_args(tokens);

    if args.len() < 1 {
        return Err("Not enough arguments!".to_string());
    }

    let mut input = String::new();
    let prompt = &args[0].val;

    print!("{}", prompt);

    match stdout().flush() {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();

            return Err(e);
        }
    }

    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();

            return Err(e);
        }
    }

    Ok(Token {
        ty: TokenTypes::STRING,
        modifiers: vec![],
        val: input.trim().to_string(),
    })
}

fn create_var(tokens: Vec<Token>) -> Result<Token, String> {
    let args = get_args(tokens);
    let var_name = args[0].val.to_string();
    let var_value = args[1].val.to_string();

    let mut variables = VARIABLES
        .write()
        .expect("Error: Another user of this mutex panicked while holding the mutex!");

    variables.insert(var_name, var_value);

    Ok(Token {
        ty: TokenTypes::STRING,
        modifiers: vec![],
        val: "None".to_string(),
    })
}

fn sum(tokens: Vec<Token>) -> Result<Token, String> {
    let args = get_args(tokens);
    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f32>().unwrap(),
        ty => {
            return Err(format!(
                "Invalid type: Cannot use operation '+' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f32>().unwrap(),
        ty => {
            return Err(format!(
                "Invalid type: Cannot use operation '+' with type {ty:?}"
            ))
        }
    };

    let total = first + second;

    if total.fract() == 0.0 {
        let total = total as i32;

        return Ok(Token {
            ty: TokenTypes::INT,
            modifiers: vec![],
            val: total.to_string(),
        });
    } else {
        return Ok(Token {
            ty: TokenTypes::FLOAT,
            modifiers: vec![],
            val: total.to_string(),
        });
    }
}
fn difference(tokens: Vec<Token>) -> Result<Token, String> {
    let args = get_args(tokens);
    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f32>().unwrap(),
        ty => {
            return Err(format!(
                "Invalid type: Cannot use operation '-' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f32>().unwrap(),
        ty => {
            return Err(format!(
                "Invalid type: Cannot use operation '-' with type {ty:?}"
            ))
        }
    };

    let difference = first - second;

    if difference.fract() == 0.0 {
        let difference = difference as i32;

        return Ok(Token {
            ty: TokenTypes::INT,
            modifiers: vec![],
            val: difference.to_string(),
        });
    } else {
        return Ok(Token {
            ty: TokenTypes::FLOAT,
            modifiers: vec![],
            val: difference.to_string(),
        });
    }
}
fn product(tokens: Vec<Token>) -> Result<Token, String> {
    let args = get_args(tokens);
    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f32>().unwrap(),
        ty => {
            return Err(format!(
                "Invalid type: Cannot use operation '*' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f32>().unwrap(),
        ty => {
            return Err(format!(
                "Invalid type: Cannot use operation '*' with type {ty:?}"
            ))
        }
    };

    let product = first * second;

    if product.fract() == 0.0 {
        let product = product as i32;

        return Ok(Token {
            ty: TokenTypes::INT,
            modifiers: vec![],
            val: product.to_string(),
        });
    } else {
        return Ok(Token {
            ty: TokenTypes::FLOAT,
            modifiers: vec![],
            val: product.to_string(),
        });
    }
}
fn quotient(tokens: Vec<Token>) -> Result<Token, String> {
    let args = get_args(tokens);
    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f32>().unwrap(),
        ty => {
            return Err(format!(
                "Invalid type: Cannot use operation '/' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f32>().unwrap(),
        ty => {
            return Err(format!(
                "Invalid type: Cannot use operation '/' with type {ty:?}"
            ))
        }
    };

    let quotient = first / second;

    if quotient.fract() == 0.0 {
        let quotient = quotient as i32;

        return Ok(Token {
            ty: TokenTypes::INT,
            modifiers: vec![],
            val: quotient.to_string(),
        });
    } else {
        return Ok(Token {
            ty: TokenTypes::FLOAT,
            modifiers: vec![],
            val: quotient.to_string(),
        });
    }
}
