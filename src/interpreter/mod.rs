pub mod interpreter;

// Export everything from the parser module.
// TODO: We should only be exporting things that other modules might use,
// not exporting wholesale.
pub use interpreter::*;

#[cfg(test)]
mod tests;
