mod lexer;
mod parser;
mod runtime;

#[derive(Debug)]
struct Ast<'a> {
    root: &'a str,
}
struct AstGiver;
impl<'a> AstGiver {
    fn give_ast() -> Ast<'a> {
        Ast { root: "xx" }
    }
}
