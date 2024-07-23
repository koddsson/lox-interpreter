use core::fmt;

use crate::expr::{BinaryOp, Expr, Literal, UnaryOp};

pub enum Error<'a> {
    RuntimeError(&'a str),
}

impl<'a> fmt::Display for Error<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RuntimeError(message) => write!(f, "{}", message),
        }
    }
}

pub fn interpret<'a>(expression: &Expr) -> Result<String, Error<'a>> {
    return match evaluate(expression) {
        Ok(value) => Ok(value.to_string()),
        Err(err) => Err(err),
    };
}

#[derive(Debug, Clone)]
enum Value {
    Number(f64),
    String(String),
    Bool(bool),
    Nil,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::String(str) => write!(f, "{}", str),
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
        Value::String(str) => str.len() > 0,
        Value::Nil => false,
        Value::Bool(bool) => *bool,
    }
}

fn interpret_unary<'a>(operator: UnaryOp, expression: &Expr) -> Result<Value, Error<'a>> {
    let value = evaluate(expression)?;
    match (operator, &value) {
        (UnaryOp::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
        (UnaryOp::Bang, _) => Ok(Value::Bool(!is_truthy(&value))),
        _ => Err(Error::RuntimeError("Unexpected runtime error!")),
    }
}

fn interpret_binary<'a>(
    left_expression: &Expr,
    operator: BinaryOp,
    right_expression: &Expr,
) -> Result<Value, Error<'a>> {
    let left = evaluate(left_expression)?;
    let right = evaluate(right_expression)?;

    return match (left, operator, right) {
        (Value::Number(n1), BinaryOp::Minus, Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
        (Value::Number(n1), BinaryOp::Slash, Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
        (Value::Number(n1), BinaryOp::Plus, Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
        (Value::Number(n1), BinaryOp::Star, Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
        _ => {
            return Err(Error::RuntimeError("Runtime error in binary expression!"));
        }
    };
}

fn evaluate<'a>(expr: &Expr) -> Result<Value, Error<'a>> {
    return match expr {
        Expr::Literal(literal) => Ok(expression_literal_to_value(literal)),
        Expr::Grouping(expr) => evaluate(expr),
        Expr::Unary(operator, expression_right) => interpret_unary(*operator, expression_right),
        Expr::Binary(left_expression, operator, right_expression) => {
            interpret_binary(left_expression, *operator, right_expression)
        }
    };
}
