use anyhow::{Result, Context, anyhow};

use std::fs;
use std::env;

use rubb::lexer::Token;
use rubb::lexer::Lexer;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        panic!("error: expected filename")
    }
    let filename = &args[1];
    let content = fs::read_to_string(filename)
        .with_context(|| format!("file not found: {filename}"))?;

    let tokens: Vec<Token> = Lexer::lex(&content)
        .map_err(anyhow::Error::from)
        .with_context(|| format!("lexer error with the following program:\n```\n{content}```"))?;

    Ok(())
}
