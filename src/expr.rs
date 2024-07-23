use std::fmt;

use crate::token::token::Token;

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(UnaryOp, Box<Expr>),
    Literal(Literal),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
    Variable(Token),
}

impl fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Expr::Unary(operator, expression) => format!("({} {})", operator, expression),
            Expr::Literal(lit) => format!("{}", lit),
            Expr::Binary(left, operator, right) => format!("({} {} {})", operator, left, right),
            Expr::Grouping(expression) => format!("(group {})", expression),
            Expr::Variable(token) => format!("{}", token),
        };
        write!(f, "{}", message)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum BinaryOp {
    Plus,
    Minus,
    Star,
    Slash,
    EqualEqual,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
}

impl fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            BinaryOp::Plus => "+",
            BinaryOp::Minus => "-",
            BinaryOp::Star => "*",
            BinaryOp::Slash => "/",
            BinaryOp::EqualEqual => "==",
            BinaryOp::BangEqual => "!=",
            BinaryOp::Less => "<",
            BinaryOp::LessEqual => "<=",
            BinaryOp::Greater => ">",
            BinaryOp::GreaterEqual => ">=",
        };

        write!(f, "{}", message)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Minus,
    Bang,
}

impl fmt::Display for UnaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        return match self {
            UnaryOp::Minus => write!(f, "-"),
            UnaryOp::Bang => write!(f, "!"),
        };
    }
}

#[derive(Debug, Clone)]
pub enum Literal {
    Str(String),
    Number(f64),
    Nil,
    False,
    True,
}

impl fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Literal::Str(str) => str.to_string(),
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}.0", n)
                } else {
                    format!("{:.}", n)
                }
            }
            Literal::Nil => "nil".to_string(),
            Literal::False => "false".to_string(),
            Literal::True => "true".to_string(),
        };
        write!(f, "{}", message)
    }
}
