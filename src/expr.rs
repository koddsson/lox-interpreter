#[derive(Debug, Clone)]
pub enum Expr {
    Unary(UnaryOp, Box<Expr>),
    Literal(Literal),
}

#[derive(Debug, Copy, Clone)]
pub enum UnaryOp {
    Minus,
    Bang,
}

pub struct UnaryOperator {
    pub token: UnaryOp,
}

#[derive(Debug, Clone)]
pub enum Literal {
    Number(f64),
    Nil,
    False,
    True,
}
