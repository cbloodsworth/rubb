use anyhow::Result;

use std::fs;
use std::env;

use rubb::lexer::Token;
use rubb::lexer::Lexer;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 { return; }
    let filename = args[1].clone();
    let content = fs::read_to_string(filename).expect("error: file not found");

    let tokens: Vec<Token> = Lexer::lex(&content);
}
