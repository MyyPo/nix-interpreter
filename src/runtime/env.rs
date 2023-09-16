use std::collections::BTreeSet;

use crate::parser::{Ast, Expr, IdentExpr, LiteralExpr};
use anyhow::{bail, Error, Result};

#[derive(Debug, PartialEq)]
pub enum Value<'a> {
    Dep(BTreeSet<&'a str>),
    Str(&'a str),
    Path(&'a str),
    NixPath(&'a str),
    Int(i32),
    Flo(f32),
    Bool(bool),
    Null(),
    List(),
    Set(),
    Func(),
    PFunc(),
}

impl<'a> From<&'a LiteralExpr<'a>> for Value<'a> {
    fn from(l: &'a LiteralExpr<'a>) -> Self {
        match l {
            LiteralExpr::Str(s) => Value::Str(s),
            LiteralExpr::Int(i) => Value::Int(*i),
            LiteralExpr::Flo(f) => Value::Flo(*f),
            LiteralExpr::Path(p) => Value::Path(p),
            LiteralExpr::NixPath(p) => Value::NixPath(p),
            LiteralExpr::Bool(b) => Value::Bool(*b),
            LiteralExpr::Null() => Value::Null(),
        }
    }
}

impl<'a> From<i32> for Value<'a> {
    fn from(i: i32) -> Self {
        Value::Int(i)
    }
}
impl<'a> From<f32> for Value<'a> {
    fn from(f: f32) -> Self {
        Value::Flo(f)
    }
}
impl<'a> From<bool> for Value<'a> {
    fn from(b: bool) -> Self {
        Value::Bool(b)
    }
}

impl<'a> TryFrom<IdentExpr<'a>> for Value<'a> {
    type Error = Error;
    fn try_from(v: IdentExpr<'a>) -> Result<Self> {
        unimplemented!();
    }
}

type Attributes<'a> = std::collections::BTreeMap<&'a str, Value<'a>>;

#[derive(Debug)]
pub struct Env<'a> {
    parent: Option<Box<Env<'a>>>,
    children: Vec<Attributes<'a>>,
    attrs: Attributes<'a>,
    allow_dep: bool,
}
impl<'a> Env<'a> {
    pub fn new(maybe_parent: Option<Env<'a>>, allow_dep: bool) -> Self {
        let children: Vec<Attributes<'a>> = vec![];
        let attrs: Attributes<'a> = Attributes::new();
        Self {
            parent: maybe_parent.map(Box::new),
            children,
            attrs,
            allow_dep,
        }
    }
    pub fn set(&'a mut self, key: &'a str, val: Value<'a>) -> Result<&'a mut Self> {
        if self.attrs.insert(key, val).is_some() {
            bail!("duplicate attribute key in the environment")
        }
        Ok(self)
    }
    pub fn has_indep(&self, key: &'a str) -> bool {
        if let Some(v) = self.attrs.get(key) {
            if let Value::Dep(_) = v {
                return false;
            }
            return true;
        }
        false
    }
    pub fn get(&'a self, id: &IdentExpr) -> Option<&'a Value> {
        self.attrs.get(id.name)
    }
    pub fn resolve(&'a self, id: &IdentExpr) -> Option<&'a Value> {
        if let Some(v) = self.get(id) {
            return Some(v);
        }

        while let Some(p) = &self.parent {
            if let Some(v) = p.get(id) {
                return Some(v);
            }
        }

        if let Some(v) = self.children.iter().rev().find_map(|t| t.get(id.name)) {
            return Some(v);
        }

        unimplemented!();
        // let deps = BTreeSet::new();
        // Some(Value::Dep(deps))
    }

    pub fn attach(&'a mut self, attrs: Attributes<'a>) {
        self.children.push(attrs);
    }
    pub fn dettach(&'a mut self) -> Option<Attributes> {
        self.children.pop()
    }
}
