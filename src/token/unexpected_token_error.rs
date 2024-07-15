use std::fmt;

pub struct UnexpectedTokenError {
    pub line: usize,
    pub token: char,
}

impl fmt::Display for UnexpectedTokenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "[line {}] Error: Unexpected character: {}",
            self.line, self.token
        )
    }
}
