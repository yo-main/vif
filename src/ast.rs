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

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Literal[{}]", self.value)
    }
}

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Binary[{}, {}, {}]",
            self.left, self.operator, self.right
        )
    }
}

impl std::fmt::Display for Grouping {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Grouping[{}, {}, {}]", self.left, self.expr, self.right)
    }
}

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Operator[{}]", self.value)
    }
}

impl std::fmt::Display for Unary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Unary[{} {}]", self.operator, self.right)
    }
}

impl std::fmt::Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Operator(e) => write!(f, "{}", e),
            Expr::Binary(e) => write!(f, "{}", e),
            Expr::Unary(e) => write!(f, "{}", e),
            Expr::Grouping(e) => write!(f, "{}", e),
            Expr::Literal(e) => write!(f, "{}", e),
        }
    }
}
