mod parser;

// Export everything from the parser module.
// TODO: We should only be exporting things that other modules might use,
// not exporting wholesale.
pub use parser::*;

#[cfg(test)]
mod tests;