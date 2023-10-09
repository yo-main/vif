use crate::tokens::Token;
use crate::Visitor;

pub struct Operator<'a> {
    pub value: &'a Token,
}

pub struct Literal<'a> {
    pub value: &'a Token,
}

pub struct Binary<'a> {
    pub left: &'a Expr<'a>,
    pub operator: &'a Token,
    pub right: &'a Expr<'a>,
}

pub struct Unary<'a> {
    pub operator: &'a Token,
    pub right: &'a Expr<'a>,
}

pub struct Grouping<'a> {
    pub left: &'a Token,
    pub expr: &'a Expr<'a>,
    pub right: &'a Token,
}

pub enum Expr<'a> {
    Operator(Operator<'a>),
    Binary(Binary<'a>),
    Unary(Unary<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal<'a>),
}

Visitor!(AstVisitor[Operator<'a>, Literal<'a>, Unary<'a>, Binary<'a>, Grouping<'a>, Expr<'a>]);

impl<'a> Literal<'a> {
    pub fn new(token: &'a Token) -> Self {
        Literal { value: token }
    }
}

impl<'a> Operator<'a> {
    pub fn new(token: &'a Token) -> Self {
        Operator { value: token }
    }
}

impl<'a> Unary<'a> {
    pub fn new(operator: &'a Token, right: &'a Expr) -> Self {
        Unary { operator, right }
    }
}

impl<'a> Grouping<'a> {
    pub fn new(left: &'a Token, expr: &'a Expr, right: &'a Token) -> Self {
        Grouping { left, expr, right }
    }
}
