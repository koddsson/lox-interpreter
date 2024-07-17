use crate::token::token::Token;

pub trait Expression {}

pub struct Expr<'a> {
    pub left: Option<Box<Expr<'a>>>,
    pub operator: Option<&'a Token>,
    pub right: Option<Box<Expr<'a>>>,
}
