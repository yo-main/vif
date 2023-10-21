use crate::errors::ZeusErrorType;
use crate::tokens::{Token, TokenType};
use crate::Visitor;

#[derive(Clone)]
pub enum Operator {
    Comma,
    Plus,
    Minus,
    Equal,
    Divide,
    Multiply,
    MinusEqual,
    PlusEqual,
    DevideEqual,
    MultiplyEqual,
    EqualEqual,
    BangEqual,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

pub enum UnaryOperator {
    Minus,
    Bang,
}

pub enum Group {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftAccolade,
    RightAccolade,
}

pub enum Literal {
    String(String),
    Indentifier(String),
}

pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Operator,
    pub right: Box<Expr>,
}

pub struct Unary {
    pub operator: UnaryOperator,
    pub right: Box<Expr>,
}

pub struct Grouping {
    pub left: Group,
    pub expr: Box<Expr>,
    pub right: Group,
}

pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Clone)]
pub enum Value {
    Operator(Operator),
    String(String),
    Identifier(String),
    Integer(i64),
    Float(f64),
    NewLine,
    True,
    False,
    None,
    Ignore,
}

pub enum Expr {
    Operator(Operator),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
    Value(Value),
}

pub enum Stmt {
    Expression(Box<Expr>),
    Test(Box<Expr>),
}

Visitor!(AstVisitor[Operator, Literal, Unary, Binary, Grouping, Expr, Value, Stmt]);

impl Literal {
    pub fn new(token: Token) -> Result<Self, ZeusErrorType> {
        match token.r#type {
            TokenType::String(s) => Ok(Literal::String(s)),
            e => Err(ZeusErrorType::ParsingError(format!(
                "This is not a literal: {}",
                e
            ))),
        }
    }
}

impl Group {
    fn new(token: Token) -> Result<Self, ZeusErrorType> {
        match token.r#type {
            TokenType::LeftParen => Ok(Group::LeftParen),
            TokenType::RightParen => Ok(Group::RightParen),
            TokenType::LeftBrace => Ok(Group::LeftBrace),
            TokenType::RightBrace => Ok(Group::RightBrace),
            TokenType::LeftAccolade => Ok(Group::LeftAccolade),
            TokenType::RightAccolade => Ok(Group::RightAccolade),
            e => Err(ZeusErrorType::ParsingError(format!(
                "Cannot build a pair with: {}",
                e
            ))),
        }
    }
}

impl UnaryOperator {
    fn new(token: Token) -> Result<Self, ZeusErrorType> {
        match token.r#type {
            TokenType::Bang => Ok(UnaryOperator::Bang),
            TokenType::Minus => Ok(UnaryOperator::Minus),
            e => Err(ZeusErrorType::ParsingError(format!(
                "Not an unary operator: {}",
                e
            ))),
        }
    }
}

impl Operator {
    pub fn new(token: Token) -> Result<Self, ZeusErrorType> {
        match token.r#type {
            TokenType::Minus => Ok(Operator::Minus),
            TokenType::Plus => Ok(Operator::Plus),
            TokenType::Star => Ok(Operator::Multiply),
            TokenType::Slash => Ok(Operator::Divide),
            TokenType::Equal => Ok(Operator::Equal),
            TokenType::EqualEqual => Ok(Operator::EqualEqual),
            TokenType::MinusEqual => Ok(Operator::MinusEqual),
            TokenType::PlusEqual => Ok(Operator::PlusEqual),
            TokenType::SlashEqual => Ok(Operator::DevideEqual),
            TokenType::StarEqual => Ok(Operator::MultiplyEqual),
            TokenType::Greater => Ok(Operator::Greater),
            TokenType::GreaterEqual => Ok(Operator::GreaterEqual),
            TokenType::Less => Ok(Operator::Less),
            TokenType::LessEqual => Ok(Operator::LessEqual),
            TokenType::BangEqual => Ok(Operator::BangEqual),
            TokenType::Comma => Ok(Operator::Comma),
            e => Err(ZeusErrorType::ParsingError(format!(
                "Not an operator: {}",
                e
            ))),
        }
    }
}

impl Unary {
    pub fn new(operator: Token, right: Box<Expr>) -> Result<Self, ZeusErrorType> {
        Ok(Unary {
            operator: UnaryOperator::new(operator)?,
            right,
        })
    }
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<Self, ZeusErrorType> {
        Ok(Binary {
            left,
            operator: Operator::new(operator)?,
            right,
        })
    }
}

impl Grouping {
    pub fn new(left: Token, expr: Box<Expr>, right: Token) -> Result<Self, ZeusErrorType> {
        Ok(Grouping {
            left: Group::new(left)?,
            expr,
            right: Group::new(right)?,
        })
    }
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Expression(e) => e,
                Self::Test(t) => t,
            }
        )
    }
}

impl std::fmt::Display for Literal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Literal[{}]",
            match self {
                Self::String(v) => v,
                Self::Indentifier(v) => v,
            }
        )
    }
}

impl std::fmt::Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Integer(i) => i.to_string(),
                Self::Float(f) => f.to_string(),
            }
        )
    }
}

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operator(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Identifier(v) => write!(f, "{}", v),
            Self::Integer(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::None => write!(f, "None"),
            Self::NewLine => write!(f, "\\n"),
            Self::Ignore => write!(f, ""),
        }
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

impl std::fmt::Display for Group {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::LeftParen => "(",
                Self::RightParen => ")",
                Self::LeftBrace => "[",
                Self::RightBrace => "]",
                Self::LeftAccolade => "{",
                Self::RightAccolade => "}",
            }
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
        write!(
            f,
            "{}",
            match self {
                Self::Comma => ",",
                Self::Plus => "+",
                Self::Minus => "-",
                Self::Equal => "=",
                Self::Divide => "/",
                Self::Multiply => "*",
                Self::MinusEqual => "-=",
                Self::PlusEqual => "+=",
                Self::DevideEqual => "/=",
                Self::MultiplyEqual => "*=",
                Self::EqualEqual => "==",
                Self::BangEqual => "!=",
                Self::Greater => ">",
                Self::GreaterEqual => ">=",
                Self::Less => "<",
                Self::LessEqual => "<=",
            }
        )
    }
}

impl std::fmt::Display for UnaryOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Minus => "-",
                Self::Bang => "!",
            }
        )
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
            Expr::Value(e) => write!(f, "{}", e),
        }
    }
}
