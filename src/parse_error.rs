use std::fmt;

pub struct ParseError {}

impl<'a> fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "A parse error occured!")
    }
}
