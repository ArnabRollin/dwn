//! The interpreter for Dawn (dwn)

use std::fs::File;
use std::io::{BufRead, BufReader};

use crate::dwn::{get_funcs, Metadata};
use crate::runner::run;

/// The function used to interpret files.
pub fn interpret_file(file: String) {
    let reader =
        BufReader::new(File::open(&file).expect(format!("Cannot open file `{}`", file).as_str()));

    let mut scope = 0;
    let mut in_scope = false;
    let mut scope_token = String::new();
    let mut current_tokens = vec![];

    for (count, line) in reader.lines().enumerate() {
        let line = remove_all_after(line.unwrap(), ';');

        run(
            line.trim_end().to_string(),
            get_funcs(),
            &mut Metadata {
                line_count: count,
                scope: &mut scope,
                in_scope: &mut in_scope,
                scope_token: &mut scope_token,
                current_tokens: &mut current_tokens,
            },
        );
    }
}

/// The function to remove every character in `text` after `ch` is reached (including `ch`).
///
/// Examples:
///
/// ```rust
/// let new = remove_all_after("say \"Hello!\" ; abcdefghij...".to_string(), ';');
///
/// assert_eq!(new, "say \"Hello!\" ".to_string());
/// ```
fn remove_all_after(text: String, ch: char) -> String {
    text.split(ch).next().unwrap().to_string()
}

#[test]
fn removing_all_after() {
    let new = remove_all_after("say \"Hello!\" ; abcdefghij...".to_string(), ';');
    assert_eq!(new, "say \"Hello!\" ".to_string());
}
