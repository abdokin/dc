mod lexer;
mod parser;
mod utils;

use std::{process::exit, env};

use lexer::Token;
use utils::read_file;

use crate::lexer::Lexer;
fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        eprintln!("Usage: {} <file_path>", args[0]);
        exit(1);
    }

    let file_path = &args[1];
    let content = read_file(file_path);

    let mut lexer = Lexer::new(&content, &file_path);

    loop {
        let token = lexer.next_token();
        if token == Token::EndOfFile {
            break;
        }
        println!("{:?}", token);
    }
}
