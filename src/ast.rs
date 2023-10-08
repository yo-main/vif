use crate::tokens::Token;
use crate::Visitor;

pub enum Operator {
    value(Token),
}

pub enum Literal {
    value(Token),
}

pub enum Binary {
    Left(Box<Expr>),
    Operator(Token),
    Right(Box<Expr>),
}

pub enum Unary {
    Operator(Token),
    Right(Box<Expr>),
}

pub enum Grouping {
    Left(Token),
    Expr(Box<Expr>),
    Right(Token),
}

pub enum Expr {
    Operator(Operator),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
}

Visitor!(AstVisitor[Operator, Literal, Unary, Binary, Grouping, Expr]);
