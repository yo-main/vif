use crate::environment::Environment;
use crate::errors::ZeusErrorType;
use crate::interpreter::Interpreter;
use crate::tokens::{Token, TokenType};
use crate::Visitor;

#[derive(Clone)]
pub struct UserFunction {
    pub declaration: Function,
}

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

#[derive(Clone)]
pub enum UnaryOperator {
    Minus,
    Bang,
}

#[derive(Clone)]
pub enum Group {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftAccolade,
    RightAccolade,
}

#[derive(Clone)]
pub enum Literal {
    String(String),
    Indentifier(String),
}

#[derive(Clone)]
pub struct Condition {
    pub expr: Box<Expr>,
    pub then: Box<Stmt>,
    pub r#else: Option<Box<Stmt>>,
}

#[derive(Clone)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Operator,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub struct Grouping {
    pub left: Group,
    pub expr: Box<Expr>,
    pub right: Group,
}

#[derive(Clone)]
pub struct Variable {
    pub name: String,
    pub value: Box<Expr>,
}

#[derive(Clone)]
pub struct Assign {
    pub name: String,
    pub value: Box<Expr>,
}

#[derive(Clone)]
pub struct Call {
    pub callee: Box<Expr>,
    pub arguments: Vec<Box<Expr>>,
}

#[derive(Clone)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

// pub struct Parameter {
//     pub name: String,
//     pub expr: Box<Expr>,
// }

#[derive(Clone)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Clone)]
pub enum BuiltIn {
    Print,
    GetTime,
}

#[derive(Clone)] // TODO: remove that clone
pub enum Value {
    Operator(Operator),
    String(String),
    Integer(i64),
    Float(f64),
    Variable(String),
    BuiltIn(BuiltIn),
    UserFunction(UserFunction),
    NewLine,
    True,
    False,
    Break,
    Continue,
    None,
    Ignore,
}

#[derive(Clone)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Clone)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: LogicalOperator,
    pub right: Box<Expr>,
}

#[derive(Clone)]
pub enum Expr {
    Operator(Operator),
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
    Value(Value),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

#[derive(Clone)]
pub enum Stmt {
    Expression(Box<Expr>),
    Var(Variable),
    Function(Function),
    Block(Vec<Stmt>),
    Condition(Condition),
    While(While),
}

Visitor!(AstVisitor[Operator, Literal, Unary, Binary, Grouping, Expr, Value, Stmt, Variable, Assign, Condition, Logical, While, Call, Function]);

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

// impl Parameter {
//     pub fn new(name: String, expr: Box<Expr>) -> Self {
//         Parameter { name, expr }
//     }
// }

impl Function {
    pub fn new(name: String, params: Vec<String>, body: Vec<Stmt>) -> Self {
        Function { name, params, body }
    }
}

impl Call {
    pub fn new(callee: Box<Expr>, arguments: Vec<Box<Expr>>) -> Self {
        Call { callee, arguments }
    }
}

impl While {
    pub fn new(condition: Box<Expr>, body: Box<Stmt>) -> Self {
        While { condition, body }
    }
}

impl Assign {
    pub fn new(name: String, value: Box<Expr>) -> Result<Self, ZeusErrorType> {
        Ok(Assign { name, value })
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

impl Condition {
    pub fn new(expr: Box<Expr>, then: Box<Stmt>, r#else: Option<Box<Stmt>>) -> Self {
        Condition { expr, then, r#else }
    }
}

impl Variable {
    pub fn new(name: Token, value: Box<Expr>) -> Result<Self, ZeusErrorType> {
        let name = match name.r#type {
            TokenType::Identifier(s) => s,
            e => {
                return Err(ZeusErrorType::ParsingError(format!(
                    "Not an identifier: {}",
                    e
                )))
            }
        };

        Ok(Variable { name, value })
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

impl LogicalOperator {
    pub fn new(operator: Token) -> Result<Self, ZeusErrorType> {
        match operator {
            t if t.r#type == TokenType::And => Ok(LogicalOperator::And),
            t if t.r#type == TokenType::Or => Ok(LogicalOperator::Or),
            e => Err(ZeusErrorType::ParsingError(format!(
                "Not a logical operator: {}",
                e
            ))),
        }
    }
}

impl Logical {
    pub fn new(left: Box<Expr>, operator: Token, right: Box<Expr>) -> Result<Self, ZeusErrorType> {
        Ok(Logical {
            left,
            operator: LogicalOperator::new(operator)?,
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

impl UserFunction {
    pub fn new(declaration: Function) -> Self {
        UserFunction { declaration }
    }

    pub fn call(
        &mut self,
        interpreter: &mut Interpreter,
        arguments: Vec<Value>,
    ) -> Result<Value, ZeusErrorType> {
        let mut env = Environment::new();

        for (i, argument) in arguments.into_iter().enumerate() {
            env.define(self.declaration.params[i].clone(), argument);
        }

        interpreter.execute_block(&self.declaration.body, Some(env))
    }
}

impl std::fmt::Display for Stmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Expression(e) => write!(f, "{}", e),
            Self::Var(v) => write!(f, "{}", v),
            Self::Block(stmts) => {
                let texts: Vec<String> = stmts.iter().map(|s| format!("{}", s)).collect();
                return write!(f, "{}", texts.join(">"));
            }
            Self::Condition(c) => write!(f, "{}", c),
            Self::While(w) => write!(f, "{}", w),
            Self::Function(v) => write!(f, "{}", v),
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function[{}]", self.name)
    }
}

impl std::fmt::Display for While {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "while {} [{}]", self.condition, self.body)
    }
}

impl std::fmt::Display for Condition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} todo", self.expr, self.then)
    }
}

impl std::fmt::Display for LogicalOperator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                LogicalOperator::And => "and",
                LogicalOperator::Or => "or",
            }
        )
    }
}

impl std::fmt::Display for Logical {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.left, self.operator, self.right)
    }
}

impl std::fmt::Display for Variable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}={}", self.name, self.value)
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

impl std::fmt::Display for BuiltIn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

// impl std::fmt::Display for Parameter {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}={}", self.name, self.expr)
//     }
// }

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Operator(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Variable(v) => write!(f, "{}", v),
            Self::Integer(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::BuiltIn(v) => write!(f, "{}", v),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::None => write!(f, "None"),
            Self::NewLine => write!(f, "\\n"),
            Self::Ignore => write!(f, ""),
            Self::Break => write!(f, "break"),
            Self::Continue => write!(f, "continue"),
            Self::UserFunction(v) => write!(f, "{}", v),
        }
    }
}

impl std::fmt::Display for UserFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function {}", self.declaration.name)
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

impl std::fmt::Display for Assign {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Assign[{}={}]", self.name, self.value)
    }
}

impl std::fmt::Display for Call {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Function[{}]", self.callee)
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
            Expr::Assign(e) => write!(f, "{}", e),
            Expr::Logical(e) => write!(f, "{}", e),
            Expr::Call(e) => write!(f, "{}", e),
        }
    }
}
