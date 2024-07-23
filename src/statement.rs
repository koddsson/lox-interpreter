use std::fmt;

use crate::expr::Expr;
use crate::symbol::Symbol;

#[derive(Debug, Clone)]
pub enum Statement {
    Var(Symbol, Option<Expr>),
    Print(Expr),
    Expression(Expr),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}
