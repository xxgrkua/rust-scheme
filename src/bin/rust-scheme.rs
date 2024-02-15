use std::io::{self, Stdin, Stdout, Write};

use rust_scheme::{create_global_frame, interpret};

fn read(stdout: &mut Stdout, stdin: &Stdin, buffer: &mut String) -> Result<usize, io::Error> {
    print!("scm> ");
    stdout.flush()?;
    stdin.read_line(buffer)
}

fn main() {
    let mut global = create_global_frame();
    let mut buffer = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    while read(&mut stdout, &stdin, &mut buffer).is_ok() {
        match interpret(&buffer, &mut global) {
            Ok(value) => println!("{}", value),
            Err(error) => println!("Error: {}", error),
        }
        buffer.clear();
    }
}
