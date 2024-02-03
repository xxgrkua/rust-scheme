use std::io;

use rust_scheme::{create_global_frame, eval, parse, tokenize};

fn main() {
    let mut global = create_global_frame();
    let mut buffer = String::new();
    let stdin = io::stdin();
    while stdin.read_line(&mut buffer).is_ok() {
        // Trim end.
        let trimmed = buffer.trim_end();
        let mut token_buffer = match tokenize(&trimmed) {
            Ok(tokens) => tokens,
            Err(error) => {
                println!("{}", error);
                buffer.clear();
                continue;
            }
        };
        let expression = match parse(&mut token_buffer) {
            Ok(expression) => expression,
            Err(error) => {
                println!("{}", error);
                buffer.clear();
                continue;
            }
        };
        match eval(expression, &mut global, false) {
            Ok(value) => println!("{}", value),
            Err(error) => println!("{}", error),
        }
        buffer.clear();
    }
}
