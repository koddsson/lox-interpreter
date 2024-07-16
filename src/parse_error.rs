use crate::token::token::Token;
use std::fmt;

pub struct ParseError {
    pub peek: &Token,
    pub message: String,
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.peek, self.message)
    }
}
