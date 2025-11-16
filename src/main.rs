use std::env;

mod interpreter;
mod lexer;

use interpreter::Interpreter;
use lexer::{Lexer, Token};

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = args.get(1).expect("Please provide a file path");

    let contents = match std::fs::read_to_string(file_path) {
        Ok(contents) => contents,
        Err(e) => {
            eprintln!("Error reading file {}: {}", file_path, e);
            return;
        }
    };

    let mut lexer = Lexer::new(contents);

    let mut tokens = Vec::new();

    loop {
        let token = lexer.next_token();
        match token {
            Token::EOF => break,
            _ => tokens.push(token),
        }
    }

    let mut interpreter = Interpreter::new();

    interpreter.interpret(tokens);
}
