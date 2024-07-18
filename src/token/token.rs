use crate::token::token_type;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: token_type::TokenType,
    pub lexeme: &'a str,
    pub literal: Option<Literal>,
    pub line: usize,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.literal.is_none() {
            write!(f, "{:?} {} null", self.token_type, self.lexeme)
        } else {
            write!(
                f,
                "{:?} {} {:?}",
                self.token_type, self.lexeme, self.literal
            )
        }
    }
}
