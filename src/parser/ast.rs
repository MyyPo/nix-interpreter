use std::collections::BTreeMap;
use std::fmt;

pub trait AstNode<'a> {
    fn get_literal(&self) -> &'a str;
}

#[derive(PartialEq)]
pub enum Expr<'a> {
    Unary(Box<UnaryExpr<'a>>),
    Binary(Box<BinaryExpr<'a>>),
    Binding(Box<BindingExpr<'a>>),
    Ident(IdentExpr<'a>),
    Literal(LiteralExpr<'a>),
    Set(SetExpr<'a>),
    List(ListExpr<'a>),
    Let(Box<LetExpr<'a>>),
    With(Box<WithExpr<'a>>),
    If(Box<IfExpr<'a>>),
    Select(Box<SelectExpr<'a>>),
    Apply(Box<ApplyExpr<'a>>),
}
impl<'a> Expr<'a> {
    pub fn new_str(s: &'a str) -> Self {
        Expr::Literal(LiteralExpr::Str(s))
    }
    pub fn new_path(s: &'a str) -> Self {
        Expr::Literal(LiteralExpr::Path(s))
    }
    pub fn new_nix_path(s: &'a str) -> Self {
        Expr::Literal(LiteralExpr::NixPath(s))
    }
    pub fn new_int(i: i32) -> Self {
        Expr::Literal(LiteralExpr::Int(i))
    }
    pub fn new_flo(f: f32) -> Self {
        Expr::Literal(LiteralExpr::Flo(f))
    }
    pub fn new_null() -> Self {
        Expr::Literal(LiteralExpr::Null())
    }

    pub fn new_add(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Add(),
        )))
    }
    pub fn new_sub(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Sub(),
        )))
    }
    pub fn new_mult(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Mult(),
        )))
    }
    pub fn new_div(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Div(),
        )))
    }

    pub fn new_compare_equals(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Equals(),
        )))
    }
    pub fn new_compare_not_equals(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::NotEquals(),
        )))
    }
    pub fn new_compare_more(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::More(),
        )))
    }
    pub fn new_compare_less(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Less(),
        )))
    }
    pub fn new_compare_more_or_equals(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::MoreOrEquals(),
        )))
    }
    pub fn new_compare_less_or_equals(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::LessOrEquals(),
        )))
    }

    pub fn new_concat(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Concat(),
        )))
    }
    pub fn new_and(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::And(),
        )))
    }
    pub fn new_or(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(left, right, BinaryExprType::Or())))
    }
    pub fn new_logical_disjunction(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Arrow(),
        )))
    }
    pub fn new_has(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Has(),
        )))
    }
    pub fn new_update(left: Expr<'a>, right: Expr<'a>) -> Self {
        Expr::Binary(Box::new(BinaryExpr::new(
            left,
            right,
            BinaryExprType::Update(),
        )))
    }

    pub fn new_logical_negation(right: Expr<'a>) -> Self {
        Expr::Unary(Box::new(UnaryExpr::new(
            right,
            UnaryExprType::LogicalNegation(),
        )))
    }
    pub fn new_arithmetic_negation(right: Expr<'a>) -> Self {
        Expr::Unary(Box::new(UnaryExpr::new(
            right,
            UnaryExprType::ArithmNegation(),
        )))
    }

    pub fn new_ident(name: &'a str) -> Self {
        Expr::Ident(IdentExpr::new(name))
    }
    pub fn new_binding(ident: IdentExpr<'a>, expr: Expr<'a>) -> Self {
        Expr::Binding(Box::new(BindingExpr::new(ident, expr)))
    }

    pub fn new_set(elems: BTreeMap<&'a str, Expr<'a>>) -> Self {
        Expr::Set(SetExpr::new(elems))
    }
    pub fn new_list(elems: Vec<Expr<'a>>) -> Self {
        Expr::List(ListExpr::new(elems))
    }

    pub fn new_let(bindings: BTreeMap<IdentExpr<'a>, Expr<'a>>, body: Expr<'a>) -> Self {
        Expr::Let(Box::new(LetExpr::new(bindings, body)))
    }
    pub fn new_with(scope: Expr<'a>, expr: Expr<'a>) -> Self {
        Expr::With(Box::new(WithExpr::new(scope, expr)))
    }
    pub fn new_if(cond: Expr<'a>, truthy: Expr<'a>, falsy: Expr<'a>) -> Self {
        Expr::If(Box::new(IfExpr::new(cond, truthy, falsy)))
    }
    pub fn new_select(set: Expr<'a>, field: IdentExpr<'a>) -> Self {
        Expr::Select(Box::new(SelectExpr::new(set, field)))
    }
    pub fn new_apply(func: Expr<'a>, arg: Expr<'a>) -> Self {
        Expr::Apply(Box::new(ApplyExpr::new(func, arg)))
    }
}
impl<'a> fmt::Debug for Expr<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expr::Binary(op) => write!(f, "{:?}", op),
            Expr::Literal(lit) => write!(f, "{:?}", lit),
            Expr::Ident(name) => write!(f, "{:?}", name),
            Expr::Let(val) => write!(
                f,
                "\nlet bindings: \n{:?}\n let body: {:?}",
                val.bindings, val.body
            ),
            Expr::With(val) => write!(f, "{:?}", val),
            Expr::List(val) => write!(f, "{:?}", val),
            Expr::Set(val) => write!(f, "{:?}", val),
            _ => write!(f, "unhandled"),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SetExpr<'a> {
    elems: std::collections::BTreeMap<&'a str, Expr<'a>>,
}
impl<'a> SetExpr<'a> {
    pub fn new(elems: BTreeMap<&'a str, Expr<'a>>) -> Self {
        Self { elems }
    }
}

#[derive(Debug, PartialEq)]
pub struct ListExpr<'a> {
    elems: Vec<Expr<'a>>,
}
impl<'a> ListExpr<'a> {
    pub fn new(elems: Vec<Expr<'a>>) -> Self {
        Self { elems }
    }
}

#[derive(Debug, PartialEq)]
pub struct WithExpr<'a> {
    scope: Expr<'a>,
    expr: Expr<'a>,
}
impl<'a> WithExpr<'a> {
    pub fn new(scope: Expr<'a>, expr: Expr<'a>) -> Self {
        Self { scope, expr }
    }
}

#[derive(Debug, PartialEq)]
pub struct IfExpr<'a> {
    cond: Expr<'a>,
    truthy: Expr<'a>,
    falsy: Expr<'a>,
}
impl<'a> IfExpr<'a> {
    pub fn new(cond: Expr<'a>, truthy: Expr<'a>, falsy: Expr<'a>) -> Self {
        Self {
            cond,
            truthy,
            falsy,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct SelectExpr<'a> {
    set: Expr<'a>,
    field: IdentExpr<'a>,
}
impl<'a> SelectExpr<'a> {
    pub fn new(set: Expr<'a>, field: IdentExpr<'a>) -> Self {
        Self { set, field }
    }
}

#[derive(Debug, PartialEq)]
pub struct ApplyExpr<'a> {
    func: Expr<'a>,
    arg: Expr<'a>,
}
impl<'a> ApplyExpr<'a> {
    pub fn new(func: Expr<'a>, arg: Expr<'a>) -> Self {
        Self { func, arg }
    }
}

#[derive(Debug, PartialEq)]
pub struct LetExpr<'a> {
    bindings: std::collections::BTreeMap<IdentExpr<'a>, Expr<'a>>,
    body: Expr<'a>,
}
impl<'a> LetExpr<'a> {
    pub fn new(bindings: BTreeMap<IdentExpr<'a>, Expr<'a>>, body: Expr<'a>) -> Self {
        Self { bindings, body }
    }
}

#[derive(Debug, PartialEq)]
pub enum UnaryExprType {
    LogicalNegation(),
    ArithmNegation(),
}

#[derive(Debug, PartialEq)]
pub struct UnaryExpr<'a> {
    pub right: Expr<'a>,
    pub typ: UnaryExprType,
}
impl<'a> UnaryExpr<'a> {
    pub fn new(right: Expr<'a>, typ: UnaryExprType) -> Self {
        Self { right, typ }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum BinaryExprType {
    Add(),
    Sub(),
    Equals(),
    NotEquals(),
    More(),
    Less(),
    MoreOrEquals(),
    LessOrEquals(),
    Concat(),
    And(),
    Or(),
    Arrow(),
    Has(),
    Mult(),
    Div(),
    Update(),
}

#[derive(Debug, PartialEq)]
pub struct BinaryExpr<'a> {
    pub left: Expr<'a>,
    pub right: Expr<'a>,
    pub typ: BinaryExprType,
}
impl<'a> BinaryExpr<'a> {
    pub fn new(left: Expr<'a>, right: Expr<'a>, typ: BinaryExprType) -> Self {
        Self { left, right, typ }
    }
}

#[derive(Debug, PartialEq)]
pub struct BindingExpr<'a> {
    pub ident: IdentExpr<'a>,
    pub expr: Expr<'a>,
}
impl<'a> BindingExpr<'a> {
    pub fn new(ident: IdentExpr<'a>, expr: Expr<'a>) -> Self {
        Self { ident, expr }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct IdentExpr<'a> {
    pub name: &'a str,
}
impl<'a> IdentExpr<'a> {
    pub fn new(name: &'a str) -> Self {
        Self { name }
    }
}

#[derive(Debug, PartialEq)]
pub enum LiteralExpr<'a> {
    Str(&'a str),
    Path(&'a str),
    NixPath(&'a str),
    Int(i32),
    Flo(f32),
    Bool(bool),
    Null(),
}

#[derive(Debug, PartialEq)]
pub struct IntExpr {
    val: i32,
}
impl IntExpr {
    pub fn new(val: i32) -> Self {
        Self { val }
    }
}

#[derive(Debug, PartialEq)]
pub struct FloExpr {
    val: f32,
}
impl FloExpr {
    pub fn new(val: f32) -> Self {
        Self { val }
    }
}

#[derive(Debug, PartialEq)]
pub struct StrExpr<'a> {
    val: &'a str,
}
impl<'a> StrExpr<'a> {
    pub fn new(val: &'a str) -> Self {
        Self { val }
    }
}
