use core::fmt;

use crate::expr::Expr;
use crate::interpreter::evaluate;
use crate::symbol::Symbol;

#[derive(Debug)]
pub enum Statement<'a> {
    Var(Symbol, Option<Expr<'a>>),
    Print(Expr<'a>),
    Expression(Expr<'a>),
}

impl<'a> fmt::Display for Statement<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Statement::Print(expression) => {
                write!(f, "{}", evaluate(expression).unwrap())
            }
            Statement::Expression(expression) => write!(f, "{}", evaluate(expression).unwrap()),
            Statement::Var(token, maybe_expression) => match maybe_expression {
                Some(expression) => write!(f, "{} {}", token, evaluate(expression).unwrap()),
                None => write!(f, "{}", token),
            },
        }
    }
}
