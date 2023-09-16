#[allow(clippy::module_inception)]
mod lexer;

mod chars;
mod tests_lexer;
pub mod tokens;

pub use lexer::*;
