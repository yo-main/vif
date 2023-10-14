use crate::tokens::Token;
use crate::Visitor;

pub struct Operator {
    pub value: Token,
}

pub struct Literal {
    pub value: Token,
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Unary {
    pub operator: Token,
    pub right: Box<Expr>,
}

pub struct Grouping {
    pub left: Token,
    pub expr: Box<Expr>,
    pub right: Token,
}

pub enum Expr {
    Operator(Operator),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

Visitor!(AstVisitor[Operator, Literal, Unary, Binary, Grouping, Expr]);

impl Literal {
    pub fn new(token: Token) -> Self {
        Literal { value: token }
    }
}

impl Operator {
    pub fn new(token: Token) -> Self {
        Operator { value: token }
    }
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Self {
        Unary { operator, right }
    }
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl Grouping {
    pub fn new(left: Token, expr: Box<Expr>, right: Token) -> Self {
        Grouping { left, expr, right }
    }
}
