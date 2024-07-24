use std::fmt;

use crate::expr::Expr;
use crate::token::token::Token;

#[derive(Debug, Clone)]
pub enum Statement {
    Print(Expr),
    Expression(Expr),
    Var(Token, Option<Expr>),
    Block(Vec<Statement>),
    If(Expr, Box<Statement>, Option<Box<Statement>>),
    While(Expr, Box<Statement>),
}

impl fmt::Display for Statement {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "STATEMENT")
    }
}
