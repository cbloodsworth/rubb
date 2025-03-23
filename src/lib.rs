//! Library file for Rubb.
//!
//! `lib.rs` files in `src/` let you define what parts of your library get
//! exported where.
//!
//! Usually in a `lib.rs`, WE* like to include implementation for some common
//! utility functions that are likely to be used throughout the crate -- we
//! definitely don't want to clutter it too much, though. This file acts
//! as kind of a guide for what parts of the API is public vs. library-only.

// Public API
// This API can be accessed from anywhere -- it should be simple to call and 
// not expose any details about the implementation.
pub fn run(program: String) {
    // let tokens = lexer::lex(&program);

    todo!()
}

// Library API
// Note: `pub(crate)` is similar to "package-private", meaning only modules
// in our library can access these. (main.rs cannot!)
// TODO: revert to pub(crate) gaming
pub mod lexer;
pub(crate) mod parser;
pub(crate) mod interpreter;
