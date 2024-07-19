use crate::token::token_type;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal<'a> {
    Identifier(String),
    Str(&'a str),
    Number(f64),
}

impl<'a> fmt::Display for Literal<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Literal::Identifier(id) => id.to_string(),
            Literal::Str(str) => str.to_string(),
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}.0", n)
                } else {
                    format!("{:.}", n)
                }
            }
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug, Clone)]
pub struct Token<'a> {
    pub token_type: token_type::TokenType,
    pub lexeme: &'a str,
    pub literal: Option<Literal<'a>>,
    pub line: usize,
}

impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match &self.literal {
            Some(lit) => write!(f, "{} {} {}", self.token_type, self.lexeme, lit),
            None => write!(f, "{} {} null", self.token_type, self.lexeme),
        };
    }
}
