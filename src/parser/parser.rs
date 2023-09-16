use crate::lexer::tokens::{
    AdditiveOperator, ArithmComparison, LogicalComparison, MultiplicativeOperator, TokenType,
};
use crate::lexer::TokenStream;
use crate::parser::ast::{
    BinaryExpr, BindingExpr, Expr, FloExpr, IdentExpr, IntExpr, LetExpr, LiteralExpr, StrExpr,
    UnaryExpr,
};
use std::collections::BTreeMap;
use std::iter::Peekable;
use std::slice::Iter;

pub trait Parser<'a> {
    fn parse(&mut self) -> Ast;
}
pub type Ast<'a> = Expr<'a>;

pub struct AstParser<'a> {
    toks: &'a TokenStream<'a>,
    iter: Peekable<Iter<'a, TokenType<'a>>>,
}
impl<'a> Parser<'a> for AstParser<'a> {
    fn parse(&mut self) -> Ast {
        self.parse_expr()
    }
}

impl<'a> AstParser<'a> {
    pub fn new(toks: &'a TokenStream<'a>) -> Self {
        Self {
            toks,
            iter: toks.iter().peekable(),
        }
    }

    fn parse_expr(&mut self) -> Expr<'a> {
        match self.iter.peek() {
            Some(tok) => match tok {
                TokenType::Let => self.parse_let(),
                TokenType::With => self.parse_with(),
                TokenType::If => self.parse_if(),
                _ => self.parse_arrow(),
            },
            _ => unimplemented!(),
        }
    }

    fn parse_arrow(&mut self) -> Expr<'a> {
        let mut left = self.parse_or();

        while let Some(TokenType::LogImpl) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_or();
            left = Expr::new_or(left, right);
        }

        left
    }

    fn parse_or(&mut self) -> Expr<'a> {
        let mut left = self.parse_and();

        while let Some(TokenType::Or) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_and();
            left = Expr::new_or(left, right);
        }

        left
    }

    fn parse_and(&mut self) -> Expr<'a> {
        let mut left = self.parse_logical_comparison();

        while let Some(TokenType::And) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_logical_comparison();
            left = Expr::new_and(left, right);
        }

        left
    }

    fn parse_logical_comparison(&mut self) -> Expr<'a> {
        let mut left = self.parse_arithm_comparison();

        while let Some(TokenType::LogicalComparison(op)) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_arithm_comparison();
            match op {
                LogicalComparison::CompareEquals => left = Expr::new_compare_equals(left, right),
                LogicalComparison::CompareNotEquals => {
                    left = Expr::new_compare_not_equals(left, right)
                }
            }
        }

        left
    }

    fn parse_arithm_comparison(&mut self) -> Expr<'a> {
        let mut left = self.parse_update();

        while let Some(TokenType::ArithmComparison(op)) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_update();
            match op {
                ArithmComparison::More => left = Expr::new_compare_more(left, right),
                ArithmComparison::Less => left = Expr::new_compare_less(left, right),
                ArithmComparison::MoreOrEquals => {
                    left = Expr::new_compare_more_or_equals(left, right)
                }
                ArithmComparison::LessOrEquals => {
                    left = Expr::new_compare_less_or_equals(left, right)
                }
            }
        }

        left
    }

    fn parse_update(&mut self) -> Expr<'a> {
        let mut left = self.parse_not();

        while let Some(TokenType::Update) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_not();
            left = Expr::new_update(left, right);
        }

        left
    }

    fn parse_not(&mut self) -> Expr<'a> {
        if let Some(TokenType::LogicalNegation) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_additive();
            return Expr::new_logical_negation(right);
        }

        self.parse_additive()
    }

    fn parse_additive(&mut self) -> Expr<'a> {
        let mut left = self.parse_multiplicative();

        while let Some(TokenType::AdditiveOperator(op)) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_multiplicative();
            match op {
                AdditiveOperator::Add => left = Expr::new_add(left, right),
                AdditiveOperator::Sub => left = Expr::new_sub(left, right),
            }
        }

        left
    }

    fn parse_multiplicative(&mut self) -> Expr<'a> {
        let mut left = self.parse_concat();

        while let Some(TokenType::MultiplicativeOperator(op)) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_concat();
            match op {
                MultiplicativeOperator::Mult => left = Expr::new_mult(left, right),
                MultiplicativeOperator::Div => left = Expr::new_div(left, right),
            }
        }

        left
    }

    fn parse_concat(&mut self) -> Expr<'a> {
        let mut left = self.parse_arithm_negation();

        while let Some(TokenType::Concat) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_arithm_negation();
            left = Expr::new_concat(left, right);
        }

        left
    }

    fn parse_has(&mut self) -> Expr<'a> {
        let mut left = self.parse_application();

        while let Some(TokenType::Has) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_application();
            left = Expr::new_has(left, right);
        }

        left
    }

    fn parse_arithm_negation(&mut self) -> Expr<'a> {
        if let Some(TokenType::ArithmNegation) = self.iter.peek() {
            self.iter.next();
            let right = self.parse_has();
            return Expr::new_arithmetic_negation(right);
        }

        self.parse_has()
    }

    // TODO
    fn parse_application(&mut self) -> Expr<'a> {
        let mut expr = self.parse_selection();

        if let Expr::Ident(_) | Expr::With(_) | Expr::Let(_) | Expr::Select(_) | Expr::Apply(_) =
            expr
        {
            while let Some(
                TokenType::Ident(_)
                | TokenType::OpenParen
                | TokenType::OpenBrace
                | TokenType::OpenSquare
                | TokenType::Int(_)
                | TokenType::Flo(_)
                | TokenType::Path(_)
                | TokenType::NixPath(_),
            ) = self.iter.peek()
            {
                self.iter.next();
                let arg = self.parse_selection();
                expr = Expr::new_apply(expr, arg);
            }
        }

        if let Expr::Apply(_) = expr {
            match self.iter.peek() {
                Some(TokenType::Semicolon | TokenType::Has) => (),
                None => (),
                _ => panic!("Failed to parse function application"),
            }
        }

        expr
    }

    fn parse_selection(&mut self) -> Expr<'a> {
        let mut obj = self.parse_term();

        if let Expr::Ident(_) | Expr::Set(_) = obj {
            while let Some(TokenType::Access) = self.iter.peek() {
                self.iter.next();

                if let Some(TokenType::Ident(field_name)) = self.iter.next() {
                    let field = IdentExpr::new(field_name);
                    obj = Expr::new_select(obj, field);
                } else {
                    panic!("expected ident after dot")
                }
            }
        }

        obj
    }

    fn parse_term(&mut self) -> Expr<'a> {
        match self.iter.next() {
            Some(tok) => match *tok {
                TokenType::Ident(val) => Expr::new_ident(val),
                TokenType::StrLiteral(val) => Expr::new_str(val),
                TokenType::Path(val) => Expr::new_path(val),
                TokenType::OpenSquare => self.parse_list(),
                TokenType::Int(val) => Expr::new_int(val),
                TokenType::Flo(val) => Expr::new_flo(val),
                TokenType::OpenBrace => self.parse_set(),
                TokenType::OpenParen => {
                    let grouped_expr = self.parse_expr();
                    if let Some(TokenType::CloseParen) = self.iter.next() {
                        grouped_expr
                    } else {
                        panic!("expected closing paren for grouping");
                    }
                }
                _ => panic!("Unexpected token type parsing a primitive value: {:?}", tok),
            },
            _ => panic!("Unexpected EOF when parsing a primitive value"),
        }
    }

    fn parse_let(&mut self) -> Expr<'a> {
        self.iter.next();

        let mut bindings: BTreeMap<IdentExpr<'a>, Expr<'a>> = BTreeMap::new();

        while let Some(tok) = self.iter.peek() {
            if **tok == TokenType::In {
                self.iter.next();
                return Expr::new_let(bindings, self.parse_expr());
            }

            let binding = self.parse_binding();
            bindings.insert(binding.ident, binding.expr);
        }

        panic!("failed to find in inside of let expr");
    }

    fn parse_with(&mut self) -> Expr<'a> {
        self.iter.next();

        if let Some(tok) = self.iter.next() {
            let scope = match tok {
                TokenType::Ident(ident_name) => Expr::new_ident(ident_name),
                TokenType::OpenBrace => self.parse_set(),
                _ => unimplemented!(),
            };
            if let Some(tok) = self.iter.next() {
                if *tok != TokenType::Semicolon {
                    unimplemented!();
                }

                return Expr::new_with(scope, self.parse_expr());
            }
        };
        unimplemented!();
    }

    fn parse_binding(&mut self) -> BindingExpr<'a> {
        if let Some(tok) = self.iter.next() {
            if let TokenType::Ident(ident_name) = tok {
                if let Some(tok) = self.iter.next() {
                    if let TokenType::Assign = tok {
                        let ident = IdentExpr::new(ident_name);
                        let binding_expr = self.parse_expr();
                        self.iter.next();
                        return BindingExpr::new(ident, binding_expr);
                    } else {
                        panic!("unexpected non-assign parsing binding");
                    }
                }
            } else {
                panic!("unexpected non-ident parsing binding: {:?}", tok);
            }
        }
        panic!("unexpected EOF parsing binding");
    }

    fn parse_set(&mut self) -> Expr<'a> {
        let mut elems: BTreeMap<&'a str, Expr<'a>> = BTreeMap::new();
        while let Some(&tok) = self.iter.peek() {
            if *tok == TokenType::CloseBrace {
                return Expr::new_set(elems);
            }

            let binding = self.parse_binding();

            elems.insert(binding.ident.name, binding.expr);
        }
        unimplemented!();
    }

    fn parse_list(&mut self) -> Expr<'a> {
        let mut elems: Vec<Expr<'a>> = vec![];
        while let Some(tok) = self.iter.peek() {
            if **tok == TokenType::CloseSquare {
                self.iter.next();
                return Expr::new_list(elems);
            }

            let elem = self.parse_term();

            elems.push(elem);
        }
        unimplemented!();
    }

    fn parse_if(&mut self) -> Expr<'a> {
        self.iter.next();
        let condition = self.parse_expr();
        if let Some(tok) = self.iter.next() {
            if *tok != TokenType::Then {
                panic!(
                    "if expression must contain then, but encountered: {:?}",
                    tok
                );
            }
        }
        let truthy = self.parse_expr();
        if let Some(tok) = self.iter.next() {
            if *tok != TokenType::Else {
                panic!(
                    "if expression must contain else, but encountered: {:?}",
                    tok
                );
            }
        }
        let falsy = self.parse_expr();
        Expr::new_if(condition, truthy, falsy)
    }
}
