mod lexer;

// Export just the lex function; that's all the other modules really need to
// know about for now.
pub use lexer::lex;

#[cfg(test)]
mod tests;
