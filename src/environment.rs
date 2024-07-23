use std::collections::HashMap;

use crate::interpreter::Error;
use crate::token::token::Token;
use crate::value::Value;

pub struct Environment {
    pub values: HashMap<String, Value>,
}

impl Environment {
    pub fn get(&self, name: &Token) -> Result<&Value, Error> {
        let key = String::from_utf8(name.lexeme.clone()).unwrap();
        if self.values.contains_key(&key) {
            return Ok(self.values.get(&key).unwrap());
        }

        Err(Error::RuntimeError(format!(
            "Undefined variable '{}'.",
            key
        )))
    }

    fn define(&mut self, name: String, value: Value) {
        self.values.insert(name, value);
    }
}
