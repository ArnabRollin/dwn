//! This is the main file for the definitions for Dawn's (dwn's) functions and variables

use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
    process::exit,
    sync::{RwLock, RwLockReadGuard},
};

use crate::{
    lexer::{tokenize, Token, TokenModifiers, TokenTypes},
    runner::run,
};

#[derive(Clone)]
pub struct Variable {
    pub value: Token,
    pub scope: u32,
}

pub struct Metadata<'a> {
    pub line_count: usize,
    pub scope: &'a mut u32,
    pub in_scope: &'a mut bool,
    pub scope_token: &'a mut String,
    pub current_tokens: &'a mut Vec<Token>,
}

lazy_static! {
    /// The functions HashMap
    ///
    /// Examples:
    ///
    /// ```rust
    /// let functions = FUNCTIONS.read().unwrap();
    ///
    /// assert!(functions.contains_key(&"say"))
    /// ```
    pub static ref FUNCTIONS: RwLock<HashMap<&'static str, for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>>> = {
        let mut m = HashMap::new();
        m.insert("say", say as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>);
        m.insert(
            "short_say",
            short_say as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert("ask", ask as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>);
        m.insert(
            "create_var",
            create_var as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert("sum", sum as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>);
        m.insert(
            "difference",
            difference as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "product",
            product as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "quotient",
            quotient as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "forever",
            forever as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "scope",
            scope as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "if",
            if_ as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "while",
            while_ as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "until",
            until as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "eq",
            eq as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "ne",
            ne as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "gt",
            gt as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "lt",
            lt as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "add_assign",
            add_assign as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "subtract_assign",
            subtract_assign as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "multiply_assign",
            multiply_assign as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "divide_assign",
            divide_assign as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "break",
            break_ as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "lazy_eq",
            lazy_eq as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "lazy_ne",
            lazy_ne as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "int",
            int as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "float",
            float as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "vars",
            vars as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "format_array",
            format_array as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "quit",
            quit as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        m.insert(
            "exit",
            quit as for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>,
        );
        RwLock::new(m)
    };
}
lazy_static! {
    /// The variables HashMap
    ///
    /// Examples:
    ///
    /// ```rust
    /// let variables = VARIABLES.read().unwrap();
    /// assert!(variables.contains_key(&"$hello"))
    /// ```
    pub static ref VARIABLES: RwLock<HashMap<String, Variable>> = {
        let mut m = HashMap::new();
        m.insert(String::from("$hello"), Variable { value: Token {
            ty: TokenTypes::STRING,
            modifiers: vec![],
            val: "Hello, World!".to_string(),
        }, scope: 0 });

        RwLock::new(m)
    };
}

/// Get all arguments for functions
///
/// Examples:
///
/// ```rust
/// let tokens = vec![
///     Token {
///           ty: TokenTypes::VARIABLE,
///           modifiers: vec![TokenModifiers::ARGS],
///           val: "a"
///     },
///     Token {
///           ty: TokenTypes::VARIABLE,
///           modifiers: vec![],
///           val: "b"  
///     }
/// ];
/// let args = get_args(tokens, meta);
///
/// assert_eq!(
///     args,
///     vec![
///         Token {
///             ty: TokenTypes::VARIABLE,
///             modifiers: vec![TokenModifiers::ARGS],
///             val: "a"
///         }
///     ]
/// );
/// ```
fn get_args(tokens: Vec<Token>, meta: &mut Metadata) -> Vec<Token> {
    let mut args: Vec<Token> = vec![];

    for token in tokens {
        if !token.modifiers.contains(&TokenModifiers::ARGS) {
            break;
        }

        let token = match token.ty {
            TokenTypes::LITERAL => {
                let ret = run(token.val, get_funcs(), meta);
                ret
            }
            TokenTypes::VARIABLE => {
                let variables = get_variables();
                let variable = variables.get(&token.val);
                let variable = match variable {
                    Some(var) => var,
                    None => {
                        eprintln!(
                            "Error on line {}: Variable '{}' does not exist!",
                            meta.line_count + 1,
                            token.val
                        );
                        exit(1);
                    }
                };
                let val = &variable.value;
                Token {
                    ty: val.ty.clone(),
                    modifiers: val.modifiers.clone(),
                    val: val.val.to_string(),
                }
            }
            TokenTypes::NAME => {
                eprintln!(
                    "Error on line {}: Name '{}' does not exist!",
                    meta.line_count + 1,
                    token.val
                );
                exit(1);
            }
            _ => token,
        };

        args.push(token);
    }
    args
}

/// Runs a scope in Dawn (dwn)
///
/// Examples:
///
/// ```rust
/// let stat: Option<String> = run_scope(token, meta);
///
/// match stat {
///     Some(stat) => println!("Breaking scope!"),
///     None => println!("It's fine..."),
/// }
/// ```
fn run_scope(token: &Token, meta: &mut Metadata) -> Token {
    match token.ty {
        TokenTypes::SCOPE => {
            *meta.scope += 1;
            let mut ret = Token {
                ty: TokenTypes::NONE,
                modifiers: vec![],
                val: "None".to_string(),
            };

            for line in token.val.lines() {
                ret = run(line.to_string(), get_funcs(), meta);

                if (ret
                    == Token {
                        ty: TokenTypes::FUNC,
                        modifiers: vec![],
                        val: "break".to_string(),
                    })
                {
                    return ret;
                }
            }

            let mut drop_vars: Vec<String> = vec![];
            let mut variables = VARIABLES.write().unwrap();

            for (k, v) in variables.clone() {
                if v.scope == *meta.scope {
                    drop_vars.push(k);
                }
            }

            for k in drop_vars {
                variables.remove(&k);
            }

            *meta.scope -= 1;

            return ret;
        }
        _ => {
            eprintln!("Error on line {}: Expected scope!", meta.line_count + 1);
            exit(1);
        }
    }
}

fn read_array(token: &Token, meta: &mut Metadata) -> Vec<Token> {
    let mut array: Vec<Token> = vec![];

    let array_items: Vec<&str> = token.val.split('\x05').collect();

    for array_item in array_items {
        let tokens = tokenize(array_item.to_string(), meta);

        for token in tokens {
            if !token.val.is_empty() {
                array.push(token);
            }
        }
    }

    array
}

/// Gets the functions HashMap
///
/// Examples:
///
/// ```rust
/// let functions = get_funcs();
///
/// assert!(functions.contains_key(&"say"))
/// ```
pub fn get_funcs() -> RwLockReadGuard<
    'static,
    HashMap<&'static str, for<'a> fn(Vec<Token>, &'a mut Metadata) -> Result<Token, String>>,
> {
    FUNCTIONS
        .read()
        .expect("Error: Another user of this mutex panicked while holding the mutex!")
}

/// Gets the variables HashMap
///
/// Examples:
///
/// ```rust
/// let variables = get_variables();
///
/// assert!(variables.contains_key(&"$hello"))
/// ```
pub fn get_variables() -> RwLockReadGuard<'static, HashMap<String, Variable>> {
    VARIABLES
        .read()
        .expect("Error: Another user of this mutex panicked while holding the mutex!")
}

fn say(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    for arg in args {
        print!("{} ", arg.val);
    }

    println!();

    Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    })
}

fn short_say(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    for arg in args {
        print!("{} ", arg.val);
    }

    Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    })
}

fn ask(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 1 {
        return Err("(ask) Not enough arguments!".to_string());
    }

    let mut input = String::new();
    let prompt = &args[0].val;

    print!("{}", prompt);

    match stdout().flush() {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();

            return Err(format!("(ask) {e}"));
        }
    }

    match stdin().read_line(&mut input) {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();

            return Err(format!("(ask) {e}"));
        }
    }

    Ok(Token {
        ty: TokenTypes::STRING,
        modifiers: vec![],
        val: input.trim().to_string(),
    })
}

fn create_var(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);
    let var_name = args[0].val.to_string();
    let var_value = args[1].val.to_string();

    if args.len() < 1 {
        return Err("(let) Not enough arguments!".to_string());
    }

    match args[0].ty {
        TokenTypes::NONE => return Err(format!("(let) Cannot accept none as variable name!")),
        _ => {}
    }

    let mut variables = VARIABLES
        .write()
        .expect("Error: Another user of this mutex panicked while holding the mutex!");

    variables.insert(
        var_name,
        Variable {
            value: Token {
                ty: args[1].ty.clone(),
                modifiers: args[1].modifiers.clone(),
                val: var_value,
            },
            scope: *meta.scope,
        },
    );

    Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    })
}

fn sum(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(+) Not enough arguments!".to_string());
    }

    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(+) Invalid type: Cannot use operation '+' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(+) Invalid type: Cannot use operation '+' with type {ty:?}"
            ))
        }
    };

    let total = first + second;

    if total.fract() == 0.0 {
        let total = total as i64;

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
fn difference(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(-) Not enough arguments!".to_string());
    }

    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(-) Invalid type: Cannot use operation '-' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(-) Invalid type: Cannot use operation '-' with type {ty:?}"
            ))
        }
    };

    let difference = first - second;

    if difference.fract() == 0.0 {
        let difference = difference as i64;

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
fn product(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(*) Not enough arguments!".to_string());
    }

    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(*) Invalid type: Cannot use operation '*' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(*) Invalid type: Cannot use operation '*' with type {ty:?}"
            ))
        }
    };

    let product = first * second;

    if product.fract() == 0.0 {
        let product = product as i64;

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
fn quotient(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(/) Not enough arguments!".to_string());
    }

    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(/) Invalid type: Cannot use operation '/' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(/) Invalid type: Cannot use operation '/' with type {ty:?}"
            ))
        }
    };

    let quotient = first / second;

    if quotient.fract() == 0.0 {
        let quotient = quotient as i64;

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

fn forever(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 1 {
        return Err("(forever) Not enough arguments!".to_string());
    }

    let scope = args[0].clone();

    loop {
        let stat = run_scope(&scope, meta);

        if (stat
            == Token {
                ty: TokenTypes::FUNC,
                modifiers: vec![],
                val: "break".to_string(),
            })
        {
            break;
        }
    }

    Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    })
}

fn scope(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 1 {
        return Err("(scope) Not enough arguments!".to_string());
    }

    let scope = args[0].clone();
    let stat = run_scope(&scope, meta);

    if (stat
        == Token {
            ty: TokenTypes::FUNC,
            modifiers: vec![],
            val: "break".to_string(),
        })
    {
        return Ok(Token {
            ty: TokenTypes::NONE,
            modifiers: vec![],
            val: "None".to_string(),
        });
    }

    Ok(stat)
}

fn if_(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(if) Not enough arguments!".to_string());
    }

    let condition = args[0].clone();

    let result = match condition.ty {
        TokenTypes::BOOL => condition.val,
        ty => return Err(format!("(if) Type {ty:?} cannot be used as condition!")),
    };

    if result == "false" {
        return Ok(Token {
            ty: TokenTypes::NONE,
            modifiers: vec![],
            val: "None".to_string(),
        });
    }

    let ret = run_scope(&args[1], meta);

    Ok(ret)
}

fn eq(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(==) Not enough arguments!".to_string());
    }

    let first = &args[0];
    let second = &args[1];

    if (first.ty == second.ty) && (first.val == second.val) {
        return Ok(Token {
            ty: TokenTypes::BOOL,
            modifiers: vec![],
            val: "true".to_string(),
        });
    }

    return Ok(Token {
        ty: TokenTypes::BOOL,
        modifiers: vec![],
        val: "false".to_string(),
    });
}

fn ne(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(!=) Not enough arguments!".to_string());
    }

    let first = &args[0];
    let second = &args[1];

    if (first.ty == second.ty) && (first.val == second.val) {
        return Ok(Token {
            ty: TokenTypes::BOOL,
            modifiers: vec![],
            val: "false".to_string(),
        });
    }

    return Ok(Token {
        ty: TokenTypes::BOOL,
        modifiers: vec![],
        val: "true".to_string(),
    });
}

fn lazy_eq(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(lazy=) Not enough arguments!".to_string());
    }

    let first = &args[0].val;
    let second = &args[1].val;

    if first == second {
        return Ok(Token {
            ty: TokenTypes::BOOL,
            modifiers: vec![],
            val: "true".to_string(),
        });
    }

    return Ok(Token {
        ty: TokenTypes::BOOL,
        modifiers: vec![],
        val: "false".to_string(),
    });
}

fn lazy_ne(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(lazy!=) Not enough arguments!".to_string());
    }

    let first = &args[0].val;
    let second = &args[1].val;

    if first == second {
        return Ok(Token {
            ty: TokenTypes::BOOL,
            modifiers: vec![],
            val: "false".to_string(),
        });
    }

    return Ok(Token {
        ty: TokenTypes::BOOL,
        modifiers: vec![],
        val: "true".to_string(),
    });
}

fn gt(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(>) Not enough arguments!".to_string());
    }

    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(>) Invalid type: Cannot use operation '>' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(>) Invalid type: Cannot use operation '>' with type {ty:?}"
            ))
        }
    };

    if first > second {
        return Ok(Token {
            ty: TokenTypes::BOOL,
            modifiers: vec![],
            val: "true".to_string(),
        });
    }

    return Ok(Token {
        ty: TokenTypes::BOOL,
        modifiers: vec![],
        val: "false".to_string(),
    });
}
fn lt(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 2 {
        return Err("(<) Not enough arguments!".to_string());
    }

    let first = match args[0].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[0].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(<) Invalid type: Cannot use operation '<' with type {ty:?}"
            ))
        }
    };
    let second = match args[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => args[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(<) Invalid type: Cannot use operation '<' with type {ty:?}"
            ))
        }
    };

    if first < second {
        return Ok(Token {
            ty: TokenTypes::BOOL,
            modifiers: vec![],
            val: "true".to_string(),
        });
    }

    return Ok(Token {
        ty: TokenTypes::BOOL,
        modifiers: vec![],
        val: "false".to_string(),
    });
}

fn while_(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    loop {
        let args = get_args(tokens.clone(), meta);

        if args.len() < 2 {
            return Err("(while) Not enough arguments!".to_string());
        }

        let condition = args[0].clone();
        let result = match condition.ty {
            TokenTypes::BOOL => condition.val,
            ty => return Err(format!("(while) Type {ty:?} cannot be used as condition!")),
        };

        if result == "false" {
            return Ok(Token {
                ty: TokenTypes::NONE,
                modifiers: vec![],
                val: "None".to_string(),
            });
        }

        let scope = args[1].clone();
        let stat = run_scope(&scope, meta);

        if (stat
            == Token {
                ty: TokenTypes::FUNC,
                modifiers: vec![],
                val: "break".to_string(),
            })
        {
            return Ok(Token {
                ty: TokenTypes::NONE,
                modifiers: vec![],
                val: "None".to_string(),
            });
        }
    }
}

fn until(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    loop {
        let args = get_args(tokens.clone(), meta);

        if args.len() < 2 {
            return Err("(until) Not enough arguments!".to_string());
        }

        let condition = args[0].clone();
        let result = match condition.ty {
            TokenTypes::BOOL => condition.val,
            ty => return Err(format!("(until) Type {ty:?} cannot be used as condition!")),
        };

        if result == "true" {
            return Ok(Token {
                ty: TokenTypes::NONE,
                modifiers: vec![],
                val: "None".to_string(),
            });
        }

        let scope = args[1].clone();
        let stat = run_scope(&scope, meta);

        if (stat
            == Token {
                ty: TokenTypes::FUNC,
                modifiers: vec![],
                val: "break".to_string(),
            })
        {
            return Ok(Token {
                ty: TokenTypes::NONE,
                modifiers: vec![],
                val: "None".to_string(),
            });
        }
    }
}

fn add_assign(tokens: Vec<Token>, _meta: &mut Metadata) -> Result<Token, String> {
    if tokens.len() < 2 {
        return Err("(+=) Not enough arguments!".to_string());
    }

    let first = match tokens[0].ty.clone() {
        TokenTypes::VARIABLE => &tokens[0].val,
        ty => {
            return Err(format!(
                "(+=) Invalid type: Cannot use operation '+=' with type {ty:?}"
            ))
        }
    };
    let second = match tokens[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => tokens[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(+=) Invalid type: Cannot add thing of type {ty:?} to variable"
            ))
        }
    };

    let mut variables = VARIABLES.write().unwrap();

    let variable = variables.get_mut(first);

    let variable = match variable {
        Some(v) => v,
        None => return Err(format!("(+=) Variable `{first}` not found")),
    };

    let value: f64 = match &variable.value.ty {
        TokenTypes::INT => variable.value.val.parse().unwrap(),
        TokenTypes::FLOAT => variable.value.val.parse().unwrap(),
        ty => {
            return Err(format!(
                "(+=) Invalid type: Cannot use operation '+=' with variable of type {ty:?}"
            ))
        }
    };

    let total = value + second;

    if total.fract() == 0.0 {
        let total = total as i64;

        *variable = Variable {
            value: Token {
                ty: TokenTypes::INT,
                modifiers: variable.value.modifiers.clone(),
                val: total.to_string(),
            },
            scope: variable.scope,
        };
    } else {
        *variable = Variable {
            value: Token {
                ty: TokenTypes::FLOAT,
                modifiers: variable.value.modifiers.clone(),
                val: total.to_string(),
            },
            scope: variable.scope,
        };
    }

    return Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    });
}
fn subtract_assign(tokens: Vec<Token>, _meta: &mut Metadata) -> Result<Token, String> {
    if tokens.len() < 2 {
        return Err("(-=) Not enough arguments!".to_string());
    }

    let first = match tokens[0].ty.clone() {
        TokenTypes::VARIABLE => &tokens[0].val,
        ty => {
            return Err(format!(
                "(-=) Invalid type: Cannot use operation '-=' with type {ty:?}"
            ))
        }
    };
    let second = match tokens[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => tokens[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(-=) Invalid type: Cannot subtract thing of type {ty:?} from variable"
            ))
        }
    };

    let mut variables = VARIABLES.write().unwrap();

    let variable = variables.get_mut(first);

    let variable = match variable {
        Some(v) => v,
        None => return Err(format!("(-=) Variable `{first}` not found")),
    };

    let value: f64 = match &variable.value.ty {
        TokenTypes::INT => variable.value.val.parse().unwrap(),
        TokenTypes::FLOAT => variable.value.val.parse().unwrap(),
        ty => {
            return Err(format!(
                "(-=) Invalid type: Cannot use operation '-=' with variable of type {ty:?}"
            ))
        }
    };

    let total = value - second;

    if total.fract() == 0.0 {
        let total = total as i64;

        *variable = Variable {
            value: Token {
                ty: TokenTypes::INT,
                modifiers: variable.value.modifiers.clone(),
                val: total.to_string(),
            },
            scope: variable.scope,
        };
    } else {
        *variable = Variable {
            value: Token {
                ty: TokenTypes::FLOAT,
                modifiers: variable.value.modifiers.clone(),
                val: total.to_string(),
            },
            scope: variable.scope,
        };
    }

    return Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    });
}

fn multiply_assign(tokens: Vec<Token>, _meta: &mut Metadata) -> Result<Token, String> {
    if tokens.len() < 2 {
        return Err("(*=) Not enough arguments!".to_string());
    }

    let first = match tokens[0].ty.clone() {
        TokenTypes::VARIABLE => &tokens[0].val,
        ty => {
            return Err(format!(
                "(*=) Invalid type: Cannot use operation '*=' with type {ty:?}"
            ))
        }
    };
    let second = match tokens[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => tokens[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(*=) Invalid type: Cannot multiply thing of type {ty:?} with variable"
            ))
        }
    };

    let mut variables = VARIABLES.write().unwrap();

    let variable = variables.get_mut(first);

    let variable = match variable {
        Some(v) => v,
        None => return Err(format!("(*=) Variable `{first}` not found")),
    };

    let value: f64 = match &variable.value.ty {
        TokenTypes::INT => variable.value.val.parse().unwrap(),
        TokenTypes::FLOAT => variable.value.val.parse().unwrap(),
        ty => {
            return Err(format!(
                "(*=) Invalid type: Cannot use operation '*=' with variable of type {ty:?}"
            ))
        }
    };

    let total = value * second;

    if total.fract() == 0.0 {
        let total = total as i64;

        *variable = Variable {
            value: Token {
                ty: TokenTypes::INT,
                modifiers: variable.value.modifiers.clone(),
                val: total.to_string(),
            },
            scope: variable.scope,
        };
    } else {
        *variable = Variable {
            value: Token {
                ty: TokenTypes::FLOAT,
                modifiers: variable.value.modifiers.clone(),
                val: total.to_string(),
            },
            scope: variable.scope,
        };
    }

    return Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    });
}

fn divide_assign(tokens: Vec<Token>, _meta: &mut Metadata) -> Result<Token, String> {
    if tokens.len() < 2 {
        return Err("(/=) Not enough arguments!".to_string());
    }

    let first = match tokens[0].ty.clone() {
        TokenTypes::VARIABLE => &tokens[0].val,
        ty => {
            return Err(format!(
                "(/=) Invalid type: Cannot use operation '/=' with type {ty:?}"
            ))
        }
    };
    let second = match tokens[1].ty.clone() {
        TokenTypes::INT | TokenTypes::FLOAT => tokens[1].val.parse::<f64>().unwrap(),
        ty => {
            return Err(format!(
                "(/=) Invalid type: Variable cannot be divided by thing of type {ty:?}"
            ))
        }
    };

    let mut variables = VARIABLES.write().unwrap();

    let variable = variables.get_mut(first);

    let variable = match variable {
        Some(v) => v,
        None => return Err(format!("(/=) Variable `{first}` not found")),
    };

    let value: f64 = match &variable.value.ty {
        TokenTypes::INT => variable.value.val.parse().unwrap(),
        TokenTypes::FLOAT => variable.value.val.parse().unwrap(),
        ty => {
            return Err(format!(
                "(/=) Invalid type: Cannot use operation '/=' with variable of type {ty:?}"
            ))
        }
    };

    let total = value / second;

    if total.fract() == 0.0 {
        let total = total as i64;

        *variable = Variable {
            value: Token {
                ty: TokenTypes::INT,
                modifiers: variable.value.modifiers.clone(),
                val: total.to_string(),
            },
            scope: variable.scope,
        };
    } else {
        *variable = Variable {
            value: Token {
                ty: TokenTypes::FLOAT,
                modifiers: variable.value.modifiers.clone(),
                val: total.to_string(),
            },
            scope: variable.scope,
        };
    }

    return Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    });
}

fn break_(_tokens: Vec<Token>, _meta: &mut Metadata) -> Result<Token, String> {
    return Ok(Token {
        ty: TokenTypes::FUNC,
        modifiers: vec![],
        val: "break".to_string(),
    });
}

fn int(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    let convertable = &args[0].val;
    match convertable.parse::<i64>() {
        Ok(int) => int,
        Err(_) => return Err("(int) Could not convert value to integer".to_string()),
    };

    return Ok(Token {
        ty: TokenTypes::INT,
        modifiers: vec![],
        val: convertable.to_string(),
    });
}
fn float(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    let convertable = &args[0].val;
    match convertable.parse::<f64>() {
        Ok(int) => int,
        Err(_) => return Err("(float) Could not convert value to float".to_string()),
    };

    return Ok(Token {
        ty: TokenTypes::FLOAT,
        modifiers: vec![],
        val: convertable.to_string(),
    });
}

fn vars(_tokens: Vec<Token>, _meta: &mut Metadata) -> Result<Token, String> {
    let variables = VARIABLES.read().unwrap();

    for (k, v) in variables.iter() {
        println!("{}: <{:?}>{}", k, v.value.ty, v.value.val);
    }

    return Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    });
}

fn format_array(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    if args.len() < 1 {
        return Err("(format_array) Not enough arguments!".to_string());
    }

    let array = read_array(&args[0], meta);
    println!("{:?}", array);

    for token in array {
        println!("{}", token.val);
    }

    Ok(Token {
        ty: TokenTypes::NONE,
        modifiers: vec![],
        val: "None".to_string(),
    })
}

fn quit(tokens: Vec<Token>, meta: &mut Metadata) -> Result<Token, String> {
    let args = get_args(tokens, meta);

    let code: i32 = if args.len() > 0 {
        match args[0].val.parse() {
            Ok(n) => n,
            Err(_) => {
                return Err("(quit / exit) Error code is not a number".to_string());
            }
        }
    } else {
        0
    };

    exit(code);
}
