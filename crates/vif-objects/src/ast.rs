use crate::span::Span;

#[derive(Debug, PartialEq)]
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
    BangEqual,
    Modulo,
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Not,
}

#[derive(Debug, PartialEq)]
pub enum Group {
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftAccolade,
    RightAccolade,
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub expr: Box<Expr>,
    pub then: Box<Stmt>,
    pub r#else: Option<Box<Stmt>>,
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Operator,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Unary {
    pub operator: UnaryOperator,
    pub right: Box<Expr>,
}

impl Unary {
    pub fn new(operator: UnaryOperator, right: Box<Expr>) -> Self {
        Unary { operator, right }
    }
}

#[derive(Debug, PartialEq)]
pub struct Grouping {
    pub left: Group,
    pub expr: Box<Expr>,
    pub right: Group,
}

#[derive(Debug, PartialEq)]
pub struct Variable {
    pub name: String,
    pub value: Box<Expr>,
    pub typing: Typing,
}

impl Variable {
    pub fn new(name: String, value: Box<Expr>, mutable: bool) -> Self {
        Variable {
            name,
            value,
            typing: Typing::new(mutable),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Assign {
    pub name: String,
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub callee: Box<Expr>,
    pub arguments: Vec<Box<Expr>>,
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Assert {
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct Signature {
    pub parameters: Vec<bool>,
}

impl Signature {
    pub fn new(parameters: Vec<bool>) -> Self {
        Signature { parameters }
    }
}

impl std::default::Default for Signature {
    fn default() -> Self {
        Signature {
            parameters: Vec::new(),
        }
    }
}

impl PartialEq for Signature {
    fn eq(&self, other: &Self) -> bool {
        self.parameters == other.parameters
    }
}

impl std::fmt::Display for Signature {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}]",
            self.parameters
                .iter()
                .map(|b| match b {
                    true => "true",
                    false => "false",
                })
                .collect::<Vec<&str>>()
                .join(", ")
        )
    }
}

#[derive(Debug, Clone)]
pub struct Callable {
    pub signature: Signature,
    pub output: Typing,
}

impl PartialEq for Callable {
    fn eq(&self, other: &Self) -> bool {
        self.signature == other.signature && self.output == other.output
    }
}

impl Callable {
    pub fn new(signature: Signature, output: Typing) -> Self {
        Self { signature, output }
    }
}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.signature)
    }
}

#[derive(Debug, Clone)]
pub struct Typing {
    pub mutable: bool,
    pub callable: Option<Box<Callable>>,
}

impl PartialEq for Typing {
    fn eq(&self, other: &Self) -> bool {
        self.mutable == other.mutable && self.callable == other.callable
    }
}

impl std::fmt::Display for Typing {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "mut[{}] callable[{:?}]", self.mutable, self.callable)
    }
}

impl Typing {
    pub fn new(mutable: bool) -> Self {
        Self {
            mutable,
            callable: None,
        }
    }

    pub fn callable_eq(&self, other: &Option<Box<Callable>>) -> bool {
        match &self.callable {
            None => match other {
                None => true,
                _ => false,
            },
            Some(callable1) => match other {
                None => false,
                Some(callable2) => callable1.signature == callable2.signature,
            },
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub typing: Typing,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<FunctionParameter>,
    pub body: Vec<Stmt>,
    pub typing: Typing,
}

impl Function {
    pub fn new(name: String, params: Vec<FunctionParameter>, body: Vec<Stmt>) -> Self {
        Function {
            typing: Typing::new(false),
            name,
            params,
            body,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialEq)]
pub enum LoopKeyword {
    Continue,
    Break,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    // Operator(Operator),
    String(String),
    Integer(i64),
    Float(f64),
    Variable(String),
    // NewLine,
    True,
    False,
    None,
    // Ignore,
}

#[derive(Debug, PartialEq)]
pub enum LogicalOperator {
    And,
    Or,
}

#[derive(Debug, PartialEq)]
pub struct Logical {
    pub left: Box<Expr>,
    pub operator: LogicalOperator,
    pub right: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub span: Span,
    pub body: ExprBody,
    pub typing: Typing,
}

#[derive(Debug, PartialEq)]
pub enum ExprBody {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Value(Value),
    LoopKeyword(LoopKeyword),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Box<Expr>),
    Var(Variable),
    Function(Function),
    Block(Vec<Stmt>),
    Condition(Condition),
    While(While),
    Return(Return),
    Assert(Assert),
}

impl Expr {
    pub fn new(body: ExprBody, typing: Typing, span: Span) -> Self {
        Expr { body, typing, span }
    }
}

impl std::fmt::Display for Return {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "return {}", self.value)
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
            Self::Return(v) => write!(f, "{}", v),
            Self::Assert(v) => write!(f, "{}", v),
        }
    }
}

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "function[{}]", self.name)
    }
}

impl std::fmt::Display for Assert {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "assert[{}]", self.value)
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

// impl std::fmt::Display for VariableReference {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         if self.typing.mutable {
//             write!(f, "mut {}", self.name)
//         } else {
//             write!(f, "{}", self.name)
//         }
//     }
// }

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            // Self::Operator(v) => write!(f, "{}", v),
            Self::String(v) => write!(f, "{}", v),
            Self::Variable(v) => write!(f, "var[{}]", v),
            Self::Integer(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::None => write!(f, "None"),
            // Self::NewLine => write!(f, "\\n"),
            // Self::Ignore => write!(f, ""),
        }
    }
}

impl std::fmt::Display for LoopKeyword {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Break => write!(f, "break"),
            Self::Continue => write!(f, "continue"),
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
                Self::BangEqual => "!=",
                Self::Greater => ">",
                Self::GreaterEqual => ">=",
                Self::Less => "<",
                Self::LessEqual => "<=",
                Self::Modulo => "%",
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
                Self::Not => "!",
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
        write!(f, "{}", self.body)
    }
}

impl std::fmt::Display for ExprBody {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Binary(e) => write!(f, "{}", e),
            Self::Unary(e) => write!(f, "{}", e),
            Self::Grouping(e) => write!(f, "{}", e),
            Self::Value(e) => write!(f, "Value[{}]", e),
            Self::Assign(e) => write!(f, "Assign[{}]", e),
            Self::Logical(e) => write!(f, "{}", e),
            Self::Call(e) => write!(f, "Call[{}]", e),
            Self::LoopKeyword(e) => write!(f, "{}", e),
        }
    }
}
