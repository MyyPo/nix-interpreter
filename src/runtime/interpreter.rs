use anyhow::{anyhow, bail, Error, Result};
use std::path::PathBuf;

use crate::parser::{
    Ast, BinaryExpr, BinaryExprType, Expr, IdentExpr, LiteralExpr, UnaryExpr, UnaryExprType,
};
use crate::runtime::env::{Env, Value};

#[derive(Debug)]
pub struct Interpreter<'a, 'b> {
    ast: &'a Ast<'a>,
    env: Env<'b>,
}
impl<'a, 'b> Interpreter<'a, 'b> {
    fn evaluate(&'a self, e: &'a Expr<'a>) -> Result<Value<'a>> {
        match e {
            Expr::Literal(l) => Ok(Value::from(l)),
            Expr::Unary(u) => match self.evaluate(&u.right) {
                Ok(v) => {
                    if let Value::Dep(_) = v {
                        return Ok(v);
                    }
                    match u.typ {
                        UnaryExprType::ArithmNegation() => match u.right {
                            Expr::Literal(LiteralExpr::Int(i)) => Ok(Value::from(-i)),
                            Expr::Literal(LiteralExpr::Flo(f)) => Ok(Value::from(-f)),
                            _ => bail!("Arithmetic negation operator - can only be applied on number types"),
                        },
                        UnaryExprType::LogicalNegation() => match u.right {
                            Expr::Literal(LiteralExpr::Bool(b)) => Ok(Value::from(b)),
                            _ => bail!("Logical negation ! operator can only be applied to a boolean"),
                        }
                    }
                }
                Err(e) => Err(e),
            },
            Expr::Binary(b) => self.eval_binary(b),
            _ => unimplemented!(),
        }
    }
    fn eval_binary(&'a self, b: &'a BinaryExpr) -> Result<Value<'a>> {
        match self.evaluate(&b.left) {
            Ok(left) => {
                if let Value::Dep(_) = left {
                    if let Ok(Value::Dep(_)) = self.evaluate(&b.right) {
                        unimplemented!();
                    }
                    return Ok(left);
                }
                match b.typ {
                    BinaryExprType::And() | BinaryExprType::Or() | BinaryExprType::Arrow() => {
                        if let Value::Bool(lb) = left {
                            if lb {
                                if let BinaryExprType::Or() = b.typ {
                                    return Ok(Value::Bool(true));
                                }
                            } else {
                                if let BinaryExprType::And() = b.typ {
                                    return Ok(Value::Bool(false));
                                }
                                if let BinaryExprType::Arrow() = b.typ {
                                    return Ok(Value::Bool(true));
                                }
                            }

                            if let Ok(Value::Bool(rb)) = self.evaluate(&b.right) {
                                return Ok(Value::Bool(eval_logical(b.typ, lb, rb)));
                            }
                            bail!(
                                "Expecting right operand to be a boolean for opearator {:?}",
                                b.right
                            )
                        }
                        bail!("Expecting left operand to be a boolean for operator {:?}, but got {:?}", b.typ, left);
                    }
                    BinaryExprType::Equals() => {
                        let right = self.evaluate(&b.right)?;
                        Ok(Value::Bool(eval_equal(&left, &right)))
                    }
                    BinaryExprType::NotEquals() => {
                        let right = self.evaluate(&b.right)?;
                        Ok(Value::Bool(!eval_equal(&left, &right)))
                    }
                    _ => unimplemented!(),
                }
            }
            Err(e) => Err(e),
        }
    }
    pub fn interpret(&'a mut self) -> Result<Value<'a>> {
        self.evaluate(self.ast)
    }
    pub fn new(ast: &'a Ast) -> Self {
        let env = Env::new(None, false);
        Self { ast, env }
    }
}

fn eval_logical(e: BinaryExprType, l: bool, r: bool) -> bool {
    match e {
        BinaryExprType::And() => l && r,
        BinaryExprType::Or() => l || r,
        BinaryExprType::Arrow() => !l || r,
        _ => unimplemented!(),
    }
}

// FIX: impl deep equality
fn eval_equal(l: &Value, r: &Value) -> bool {
    match (l, r) {
        (Value::Func(), _) | (Value::PFunc(), _) | (_, Value::Func()) | (_, Value::PFunc()) => {
            false
        }
        _ => l == r,
    }
}
