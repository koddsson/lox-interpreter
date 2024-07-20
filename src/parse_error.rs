use std::fmt;

use crate::token::token_type::TokenType;

pub enum ParseError {
    UnexpectedTokenError(TokenType),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            Self::UnexpectedTokenError(token_type) => {
                write!(f, "Parse Error: Unexpected token: {}", token_type)
            }
        };
    }
}
