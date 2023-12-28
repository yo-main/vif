#[derive(Debug, PartialEq)]
pub struct UserFunction {
    pub declaration: Function,
}

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
    Greater,
    Less,
    GreaterEqual,
    LessEqual,
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Bang,
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
pub enum Literal {
    String(String),
    Identifier(String),
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
pub enum Number {
    Integer(i64),
    Float(f64),
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<String>,
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

#[derive(Debug, PartialEq)]
pub enum BuiltIn {
    Print,
    GetTime,
}

#[derive(Debug, PartialEq)]
pub enum Value {
    Operator(Operator),
    String(String),
    Integer(i64),
    Float(f64),
    Variable(String),
    NewLine,
    True,
    False,
    Break,
    Continue,
    None,
    Ignore,
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
pub enum Expr {
    Binary(Binary),
    Unary(Unary),
    Grouping(Grouping),
    Literal(Literal),
    Value(Value),
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
                Self::Identifier(v) => v,
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
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::None => write!(f, "None"),
            Self::NewLine => write!(f, "\\n"),
            Self::Ignore => write!(f, ""),
            Self::Break => write!(f, "break"),
            Self::Continue => write!(f, "continue"),
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
