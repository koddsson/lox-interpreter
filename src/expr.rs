#[derive(Debug, Clone)]
pub enum Expr {
    Unary(UnaryOp, Box<Expr>),
    Literal(Literal),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
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

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Minus,
    Bang,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Nil,
    False,
    True,
}
