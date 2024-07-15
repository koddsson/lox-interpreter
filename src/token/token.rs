use crate::token::token_type;
use std::fmt;

pub struct Token {
    pub token_type: token_type::TokenType,
    // TODO: Make lexeme a string? Just easier to work with?
    pub lexeme: String,
    pub literal: Option<String>,
    pub line: usize,
}

impl fmt::Display for Token {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.literal.is_none() {
            write!(f, "{:?} {} null", self.token_type, self.lexeme)
        } else {
            write!(
                f,
                "{:?} {} {}",
                self.token_type,
                self.lexeme,
                self.literal.as_ref().unwrap()
            )
        }
    }
}
