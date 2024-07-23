use std::collections::HashMap;

use crate::interpreter::Error;
use crate::token::token::Token;
use crate::value::Value;

pub struct Environment {
    pub values: HashMap<String, Value>,
}

impl Environment {
    pub fn get(&self, name: &Token) -> Result<&Value, Error> {
        if self.values.contains_key(name.lexeme) {
            return Ok(self.values.get(name.lexeme).unwrap());
        }

        Err(Error::RuntimeError(format!(
            "Undefined variable '{}'.",
            name.lexeme
        )))
    }

    fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
}
