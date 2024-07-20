use core::fmt;

use crate::expr::{BinaryOp, Expr, UnaryOp};

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

pub fn interpret<'a>(expression: Expr) -> Result<String, Error<'a>> {
    return match evaluate(expression) {
        Ok(value) => Ok(value),
        Err(err) => Err(err),
    };
}

fn evaluate<'a>(expr: Expr) -> Result<String, Error<'a>> {
    return match expr {
        Expr::Literal(literal) => Ok(literal.to_string()),
        Expr::Grouping(expr) => evaluate(*expr),
        Expr::Unary(operator, expression_right) => {
            let right = evaluate(*expression_right)?;
            match operator {
                UnaryOp::Minus => Ok((-1.0 * right.parse::<f32>().unwrap()).to_string()),
                UnaryOp::Bang => Err(Error::RuntimeError(
                    "Expected MINUS token in Unary Expression",
                )),
            }
        }
        Expr::Binary(left_expression, operator, right_expression) => {
            let left = evaluate(*left_expression)?;
            let right = evaluate(*right_expression)?;

            let left_value = left.parse::<f32>().unwrap();
            let right_value = right.parse::<f32>().unwrap();

            return match operator {
                BinaryOp::Minus => Ok((left_value - right_value).to_string()),
                BinaryOp::Slash => Ok((left_value / right_value).to_string()),
                BinaryOp::Plus => Ok((left_value + right_value).to_string()),
                BinaryOp::Star => Ok((left_value * right_value).to_string()),
                other => {
                    return Err(Error::RuntimeError(
                        "Expected MINUS, SLASH or STAR in binary expression.",
                    ));
                }
            };
        }
    };
}
