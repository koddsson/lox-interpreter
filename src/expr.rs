use crate::token::token::Token;

pub trait Expression {}

pub struct Expr {
    pub left: Option<Box<Expr>>,
    pub operator: Option<Token>,
    pub right: Option<Box<Expr>>,
}

pub struct Unary {
    pub operator: Option<&'static Token>,
    pub right: Option<Box<Expr>>,
}
