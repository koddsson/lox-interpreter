use crate::token::token::Token;
use std::fmt;

pub struct ParseError<'a> {
    pub peek: &'a Token,
    pub message: String,
}

impl<'a> fmt::Display for ParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {}", self.peek, self.message)
    }
}
