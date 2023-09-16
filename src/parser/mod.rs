#[allow(clippy::module_inception)]
mod parser;

mod ast;
mod tests_parser;

pub use crate::parser::ast::*;
pub use parser::*;
