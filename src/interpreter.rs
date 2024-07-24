use core::fmt;
use std::collections::HashMap;

use crate::expr::{BinaryOp, Expr, Literal, LogicalOp, UnaryOp};
use crate::statement::Statement;

pub enum Error {
    RuntimeError(String),
    DivisionByZeroError,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RuntimeError(message) => write!(f, "{}", message),
            Error::DivisionByZeroError => write!(f, "Tried dividing by zero!"),
        }
    }
}

pub struct Environment {
    pub map: HashMap<String, Value>,
    pub enclosing: HashMap<String, Value>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            map: HashMap::new(),
            enclosing: HashMap::new(),
        }
    }
}

impl Environment {
    fn define(&mut self, key: String, value: Value) {
        self.map.insert(key, value);
    }

    fn get(&self, key: &String) -> Result<Value, Error> {
        match (self.map.get(key), self.enclosing.get(key)) {
            (Some(value), _) => Ok(value.clone()),
            (None, Some(value)) => Ok(value.clone()),
            (None, None) => Err(Error::RuntimeError(format!(
                "Undefined variable '{}'.",
                key
            ))),
        }
    }

    fn assign(&mut self, name: String, value: Value) -> Result<(), Error> {
        if self.map.contains_key(&name) {
            self.map.insert(name, value);
            return Ok(());
        } else {
            self.enclosing.insert(name, value);
            return Ok(());
        }
    }
}

pub struct Interpreter {
    pub environment: Environment,
}

impl Default for Interpreter {
    fn default() -> Self {
        Interpreter {
            environment: Environment {
                ..Default::default()
            },
        }
    }
}

impl Interpreter {
    pub fn interpret(&mut self, statements: Vec<Statement>) -> Result<Vec<Value>, Error> {
        let mut values = Vec::new();

        for statement in statements {
            values.push(self.execute(statement)?);
        }
        Ok(values)
    }

    fn evaluate(&mut self, expr: &Expr) -> Result<Value, Error> {
        return match expr {
            Expr::Literal(literal) => Ok(expression_literal_to_value(literal)),
            Expr::Grouping(expr) => self.evaluate(expr),
            Expr::Unary(operator, expression_right) => {
                self.interpret_unary(*operator, expression_right)
            }
            Expr::Binary(left_expression, operator, right_expression) => {
                self.interpret_binary(left_expression, *operator, right_expression)
            }
            Expr::Variable(token) => self.environment.get(&token.lexeme),
            Expr::Assign(token, expression) => {
                let value = self.evaluate(&expression)?;
                self.environment
                    .assign(token.lexeme.clone(), value.clone())?;
                return Ok(value);
            }
            Expr::Logical(left, operator, right) => self.interpret_logical(left, *operator, right),
        };
    }

    fn interpret_logical(
        &mut self,
        left: &Expr,
        operator: LogicalOp,
        right: &Expr,
    ) -> Result<Value, Error> {
        let left = self.evaluate(left)?;

        match operator {
            LogicalOp::Or => {
                if is_truthy(&left) {
                    return Ok(left);
                }
            }
            _ => {
                if !is_truthy(&left) {
                    return Ok(left);
                }
            }
        }

        return self.evaluate(right);
    }

    fn interpret_unary<'a>(
        &mut self,
        operator: UnaryOp,
        expression: &Expr,
    ) -> Result<Value, Error> {
        let value = self.evaluate(expression)?;
        match (operator, &value) {
            (UnaryOp::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
            (UnaryOp::Minus, _) => {
                Err(Error::RuntimeError("Operand must be a number.".to_string()))
            }
            (UnaryOp::Bang, _) => Ok(Value::Bool(!is_truthy(&value))),
        }
    }

    fn execute(&mut self, statement: Statement) -> Result<Value, Error> {
        match statement {
            Statement::Print(expr) => {
                let value = self.evaluate(&expr)?;
                println!("{}", value);
                Ok(value)
            }
            Statement::Expression(expr) => self.evaluate(&expr),
            Statement::Var(token, expr) => {
                match expr {
                    Some(expression) => match self.evaluate(&expression) {
                        Ok(value) => Some(self.environment.define(token.lexeme, value)),
                        Err(_) => None,
                    },
                    None => None,
                };

                Ok(Value::Nil)
            }
            Statement::Block(statements) => {
                for statement in statements {
                    self.execute(statement)?;
                }
                Ok(Value::Nil)
            }
            Statement::If(condition, then_branch, else_branch) => {
                let x = self.evaluate(&condition)?;

                if is_truthy(&x) {
                    self.execute(*then_branch)?;
                } else if else_branch.is_some() {
                    let y = else_branch.unwrap();
                    self.execute(*y)?;
                }
                Ok(Value::Nil)
            }
            Statement::While(expr, statement) => {
                while is_truthy(&self.evaluate(&expr)?) {
                    self.execute(*statement.clone())?;
                }
                Ok(Value::Nil)
            }
        }
    }

    fn interpret_binary<'a>(
        &mut self,
        left_expression: &Expr,
        operator: BinaryOp,
        right_expression: &Expr,
    ) -> Result<Value, Error> {
        let left = self.evaluate(left_expression)?;
        let right = self.evaluate(right_expression)?;

        return match (left, operator, right) {
            (Value::Number(n1), BinaryOp::Minus, Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
            (_, BinaryOp::Minus, _) => {
                Err(Error::RuntimeError("Operands must be numbers.".to_string()))
            }

            (Value::Number(n1), BinaryOp::Slash, Value::Number(n2)) => {
                if n2 == 0.0 {
                    Err(Error::DivisionByZeroError)
                } else {
                    Ok(Value::Number(n1 / n2))
                }
            }
            (_, BinaryOp::Slash, _) => {
                Err(Error::RuntimeError("Operands must be numbers.".to_string()))
            }

            (Value::Number(n1), BinaryOp::Plus, Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
            (Value::String(n1), BinaryOp::Plus, Value::String(n2)) => {
                Ok(Value::String(format!("{}{}", n1, n2)))
            }

            (Value::Number(n1), BinaryOp::Star, Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
            (_, BinaryOp::Star, _) => {
                Err(Error::RuntimeError("Operands must be numbers.".to_string()))
            }

            (Value::Number(n1), BinaryOp::Greater, Value::Number(n2)) => Ok(Value::Bool(n1 > n2)),
            (_, BinaryOp::Greater, _) => {
                Err(Error::RuntimeError("Operands must be numbers.".to_string()))
            }

            (Value::Number(n1), BinaryOp::GreaterEqual, Value::Number(n2)) => {
                Ok(Value::Bool(n1 >= n2))
            }
            (_, BinaryOp::GreaterEqual, _) => {
                Err(Error::RuntimeError("Operands must be numbers.".to_string()))
            }

            (Value::Number(n1), BinaryOp::Less, Value::Number(n2)) => Ok(Value::Bool(n1 < n2)),
            (_, BinaryOp::Less, _) => {
                Err(Error::RuntimeError("Operands must be numbers.".to_string()))
            }

            (Value::Number(n1), BinaryOp::LessEqual, Value::Number(n2)) => {
                Ok(Value::Bool(n1 <= n2))
            }
            (_, BinaryOp::LessEqual, _) => {
                Err(Error::RuntimeError("Operands must be numbers.".to_string()))
            }

            (Value::Number(n1), BinaryOp::BangEqual, Value::Number(n2)) => {
                Ok(Value::Bool(n1 != n2))
            }
            (Value::Number(n1), BinaryOp::EqualEqual, Value::Number(n2)) => {
                Ok(Value::Bool(n1 != n2))
            }
            (Value::Nil, BinaryOp::EqualEqual, Value::Nil) => Ok(Value::Bool(true)),
            (Value::Nil, _, _) => Ok(Value::Bool(false)),
            _ => {
                return Err(Error::RuntimeError(
                    "Runtime error in binary expression!".to_string(),
                ));
            }
        };
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(str) => write!(f, "\"{}\"", str),
            Value::Number(n) => write!(f, "{}", n),
            Value::Bool(bool) => write!(f, "{}", bool),
            Value::Nil => write!(f, "nil"),
        }
    }
}

fn expression_literal_to_value(literal: &Literal) -> Value {
    match literal {
        Literal::Number(n) => Value::Number(*n),
        Literal::Str(string) => Value::String(string.clone()),
        Literal::False => Value::Bool(false),
        Literal::True => Value::Bool(true),
        Literal::Nil => Value::Nil,
    }
}

fn is_truthy(value: &Value) -> bool {
    match value {
        Value::Number(n) => *n > 0.0,
        Value::String(str) => !str.is_empty(),
        Value::Nil => false,
        Value::Bool(bool) => *bool,
    }
}
