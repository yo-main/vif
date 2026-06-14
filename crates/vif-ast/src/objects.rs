use vif_objects::span::Span;
#[derive(Debug, PartialEq)]
pub enum Operator {
    Plus,
    Minus,
    EqualEqual,
    Divide,
    Multiply,
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
pub struct Condition {
    pub expr: Box<Expr>,
    pub then: Box<Stmt>,
    pub r#else: Option<Box<Stmt>>,
}

impl Condition {
    pub fn new(expr: Box<Expr>, then: Box<Stmt>, r#else: Option<Box<Stmt>>) -> Self {
        Self { expr, then, r#else }
    }
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
pub struct Variable {
    pub name: String,
    pub value: Box<Expr>,
    pub mutable: bool,
    pub annotation: Option<TypeAnnotation>,
}

impl Variable {
    pub fn new(
        name: String,
        value: Box<Expr>,
        mutable: bool,
        annotation: Option<TypeAnnotation>,
    ) -> Self {
        Variable {
            name,
            value,
            mutable,
            annotation,
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

impl Return {
    pub fn new(value: Box<Expr>) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq)]
pub struct Assert {
    pub value: Box<Expr>,
}

#[derive(Debug, PartialEq)]
pub enum TypeAnnotation {
    Int,
    Float,
    String,
    Bool,
}

#[derive(Debug, PartialEq)]
pub struct FunctionParameter {
    pub name: String,
    pub mutable: bool,
    pub annotation: Option<TypeAnnotation>,
}

impl FunctionParameter {
    pub fn new(name: String, mutable: bool, annotation: Option<TypeAnnotation>) -> Self {
        Self {
            name,
            mutable,
            annotation,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Entrypoint {
    pub body: Vec<Stmt>,
}

#[derive(Debug, PartialEq)]
pub struct Function {
    pub name: String,
    pub params: Vec<FunctionParameter>,
    pub body: Vec<Stmt>,
}

impl Function {
    pub fn new(name: String, params: Vec<FunctionParameter>, body: Vec<Stmt>) -> Self {
        Function { name, params, body }
    }
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: Box<Expr>,
    pub body: Box<Stmt>,
}

impl While {
    pub fn new(condition: Box<Expr>, body: Box<Stmt>) -> Self {
        Self { condition, body }
    }
}

#[derive(Debug, PartialEq)]
pub enum LoopKeyword {
    Continue,
    Break,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Value {
    String(String),
    Integer(i64),
    Float(f64),
    Variable(String),
    True,
    False,
    None,
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

impl Logical {
    pub fn new(left: Box<Expr>, operator: LogicalOperator, right: Box<Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Expr {
    pub span: Span,
    pub body: ExprBody,
}

#[derive(Debug, PartialEq)]
pub enum ExprBody {
    Binary(Binary),
    Unary(Unary),
    Value(Value),
    LoopKeyword(LoopKeyword),
    Assign(Assign),
    Logical(Logical),
    Call(Call),
}

impl Expr {
    pub fn new(body: ExprBody, span: Span) -> Self {
        Expr { body, span }
    }
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

impl Stmt {
    pub fn get_all_returns(&self) -> Vec<&Return> {
        match self {
            Self::Function(f) => f
                .body
                .iter()
                .map(|b| b.get_all_returns())
                .flatten()
                .collect(),
            Self::Block(b) => b.iter().map(|b| b.get_all_returns()).flatten().collect(),
            Self::Condition(c) => c
                .then
                .get_all_returns()
                .into_iter()
                .chain(
                    c.r#else
                        .as_ref()
                        .map(|f| f.get_all_returns())
                        .unwrap_or_else(|| vec![])
                        .into_iter(),
                )
                .collect(),
            Self::While(w) => w.body.get_all_returns(),
            Self::Return(r) => vec![r],
            Self::Assert(_) => Vec::new(),
            Self::Expression(_) => Vec::new(),
            Self::Var(_) => Vec::new(),
        }
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

impl std::fmt::Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(v) => write!(f, "{}", v),
            Self::Variable(v) => write!(f, "var[{}]", v),
            Self::Integer(v) => write!(f, "{}", v),
            Self::Float(v) => write!(f, "{}", v),
            Self::True => write!(f, "True"),
            Self::False => write!(f, "False"),
            Self::None => write!(f, "None"),
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

impl std::fmt::Display for Operator {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Plus => "+",
                Self::Minus => "-",
                Self::EqualEqual => "==",
                Self::Divide => "/",
                Self::Multiply => "*",
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
            Self::Value(e) => write!(f, "Value[{}]", e),
            Self::Assign(e) => write!(f, "Assign[{}]", e),
            Self::Logical(e) => write!(f, "{}", e),
            Self::Call(e) => write!(f, "Call[{}]", e),
            Self::LoopKeyword(e) => write!(f, "{}", e),
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Condition, Expr, ExprBody, Return, Span, Stmt, Value, Variable, While};

    #[test]
    fn test_get_all_returns_no_returns() {
        let ast = Stmt::Var(Variable {
            name: "variable".to_owned(),
            value: Box::new(Expr::new(ExprBody::Value(Value::True), Span::new(1, 1))),
            mutable: true,
            annotation: None,
        });

        assert_eq!(ast.get_all_returns().len(), 0)
    }

    #[test]
    fn test_get_all_returns_simple() {
        let ast = Stmt::Return(Return::new(Box::new(Expr::new(
            ExprBody::Value(Value::False),
            Span::new(1, 1),
        ))));

        assert_eq!(ast.get_all_returns().len(), 1)
    }

    #[test]
    fn test_get_all_returns_several() {
        let condition = Stmt::Condition(Condition::new(
            Box::new(Expr::new(
                ExprBody::Value(Value::Integer(1)),
                Span::new(1, 1),
            )),
            Box::new(Stmt::Return(Return::new(Box::new(Expr::new(
                ExprBody::Value(Value::False),
                Span::new(1, 1),
            ))))),
            Some(Box::new(Stmt::Return(Return::new(Box::new(Expr::new(
                ExprBody::Value(Value::False),
                Span::new(1, 1),
            )))))),
        ));

        assert_eq!(condition.get_all_returns().len(), 2)
    }

    #[test]
    fn test_get_all_returns_condition_without_else() {
        let condition = Stmt::Condition(Condition::new(
            Box::new(Expr::new(
                ExprBody::Value(Value::Integer(1)),
                Span::new(1, 1),
            )),
            Box::new(Stmt::Return(Return::new(Box::new(Expr::new(
                ExprBody::Value(Value::False),
                Span::new(1, 1),
            ))))),
            None,
        ));

        assert_eq!(condition.get_all_returns().len(), 1)
    }

    #[test]
    fn test_get_all_returns_block() {
        let block = Stmt::Block(vec![
            Stmt::While(While::new(
                Box::new(Expr::new(ExprBody::Value(Value::False), Span::new(1, 1))),
                Box::new(Stmt::Condition(Condition::new(
                    Box::new(Expr::new(
                        ExprBody::Value(Value::Integer(1)),
                        Span::new(1, 1),
                    )),
                    Box::new(Stmt::Return(Return::new(Box::new(Expr::new(
                        ExprBody::Value(Value::False),
                        Span::new(1, 1),
                    ))))),
                    Some(Box::new(Stmt::Return(Return::new(Box::new(Expr::new(
                        ExprBody::Value(Value::False),
                        Span::new(1, 1),
                    )))))),
                ))),
            )),
            Stmt::Return(Return::new(Box::new(Expr::new(
                ExprBody::Value(Value::False),
                Span::new(1, 1),
            )))),
        ]);

        assert_eq!(block.get_all_returns().len(), 3)
    }
}
