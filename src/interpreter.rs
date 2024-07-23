use core::fmt;
use std::collections::HashMap;

use crate::environment::Environment;
use crate::expr::{BinaryOp, Expr, Literal, UnaryOp};
use crate::statement::Statement;
use crate::value::Value;

#[derive(Debug)]
pub enum Error {
    RuntimeError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::RuntimeError(message) => write!(f, "{}", message),
        }
    }
}

static ENVIRONMENT: Environment = Environment {
    values: HashMap::new(),
};

pub fn interpret<'a>(statements: Vec<Statement>) {
    for statement in statements {
        execute(statement)
    }
}

fn execute<'a>(statement: Statement) {
    match statement {
        Statement::Print(expression) => {
            let value = evaluate(&expression).unwrap();
            println!("{}", value);
        }
        Statement::Expression(expression) => {
            let _ = evaluate(&expression);
        }
        Statement::Var(token, maybe_expression) => {
            todo!()
        }
    }
}
fn expression_literal_to_value<'a>(literal: &Literal) -> Value {
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

fn interpret_unary<'a>(operator: UnaryOp, expression: &Expr) -> Result<Value, Error> {
    let value = evaluate(expression)?;
    match (operator, &value) {
        (UnaryOp::Minus, Value::Number(n)) => Ok(Value::Number(-n)),
        (UnaryOp::Bang, _) => Ok(Value::Bool(!is_truthy(&value))),
        _ => Err(Error::RuntimeError("Unexpected runtime error!".to_string())),
    }
}

fn interpret_binary<'a>(
    left_expression: &Expr,
    operator: BinaryOp,
    right_expression: &Expr,
) -> Result<Value, Error> {
    let left = evaluate(left_expression)?;
    let right = evaluate(right_expression)?;

    return match (left, operator, right) {
        (Value::Number(n1), BinaryOp::Minus, Value::Number(n2)) => Ok(Value::Number(n1 - n2)),
        (Value::Number(n1), BinaryOp::Slash, Value::Number(n2)) => Ok(Value::Number(n1 / n2)),
        (Value::Number(n1), BinaryOp::Plus, Value::Number(n2)) => Ok(Value::Number(n1 + n2)),
        (Value::Number(n1), BinaryOp::Star, Value::Number(n2)) => Ok(Value::Number(n1 * n2)),
        _ => {
            return Err(Error::RuntimeError(
                "Runtime error in binary expression!".to_string(),
            ));
        }
    };
}

pub fn evaluate<'a>(expr: &Expr) -> Result<Value, Error> {
    return match expr {
        Expr::Literal(literal) => Ok(expression_literal_to_value(literal)),
        Expr::Grouping(expr) => evaluate(expr),
        Expr::Unary(operator, expression_right) => interpret_unary(*operator, expression_right),
        Expr::Variable(token) => match ENVIRONMENT.get(token) {
            Ok(x) => Ok(x.clone()),
            Err(err) => Err(err),
        },
        Expr::Binary(left_expression, operator, right_expression) => {
            interpret_binary(left_expression, *operator, right_expression)
        }
    };
}
