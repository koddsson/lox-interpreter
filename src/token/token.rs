use crate::token::token_type;
use std::fmt;

#[derive(Debug, Clone)]
pub enum Literal {
    Identifier(String),
    Str(String),
    Number(f64),
}

impl<'a> fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Literal::Identifier(id) => id,
            Literal::Str(literal_string) => literal_string,
            Literal::Number(n) => &n.to_string(),
        };
        write!(f, "{}", message)
    }
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
        return match &self.literal {
            Some(literal) => write!(f, "{}", literal),
            None => write!(f, "{}", self.token_type),
        };
    }
}
