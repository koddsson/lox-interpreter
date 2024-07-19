use std::fmt;

#[derive(Debug, Clone)]
pub enum Expr {
    Unary(UnaryOp, Box<Expr>),
    Literal(Literal),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Grouping(Box<Expr>),
}

impl<'a> fmt::Display for Expr {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Expr::Unary(operator, expression) => format!("({} {})", operator, expression),
            Expr::Literal(lit) => format!("{}", lit),
            Expr::Binary(left, operator, right) => format!("{} {} {}", left, operator, right),
            Expr::Grouping(expression) => format!("(group {})", expression),
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

impl<'a> fmt::Display for BinaryOp {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            BinaryOp::Plus => "PLUS",
            BinaryOp::Minus => "MINUS",
            BinaryOp::Star => "STAR",
            BinaryOp::Slash => "SLASH",
            BinaryOp::EqualEqual => "EQUAL_EQUAL",
            BinaryOp::BangEqual => "BANG_EQUAL",
            BinaryOp::Less => "LESS",
            BinaryOp::LessEqual => "LESS_EQUAL",
            BinaryOp::Greater => "GREATER",
            BinaryOp::GreaterEqual => "GREATER_EQUAL",
        };

        write!(f, "{}", message)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Minus,
    Bang,
}

impl<'a> fmt::Display for UnaryOp {
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

impl<'a> fmt::Display for Literal {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let message = match self {
            Literal::Str(str) => format!("{}", str),
            Literal::Number(n) => {
                if n.fract() == 0.0 {
                    format!("{}.0", n)
                } else {
                    format!("{:.}", n)
                }
            }
            Literal::Nil => format!("nil"),
            Literal::False => format!("false"),
            Literal::True => format!("true"),
        };
        write!(f, "{}", message)
    }
}
