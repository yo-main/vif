pub use vif_ast::{LogicalOperator, LoopKeyword, Operator, TypeAnnotation, UnaryOperator, Value};
use vif_objects::span::Span;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Float,
    Bool,
    String,
    None,
    Unknown,
    KeyWord,
    Callable(Callable),
}

impl Type {
    pub fn from_annotation(annotation: &TypeAnnotation) -> Self {
        match annotation {
            TypeAnnotation::Int => Self::Int,
            TypeAnnotation::Float => Self::Float,
            TypeAnnotation::String => Self::String,
            TypeAnnotation::Bool => Self::Bool,
            TypeAnnotation::None => Self::None,
        }
    }

    pub fn if_unknown_set_to(&mut self, new_type: Type) {
        match self {
            Type::Unknown => *self = new_type,
            _ => (),
        }
    }

    pub fn is_unknown(&self) -> bool {
        match self {
            Type::Unknown => true,
            _ => false,
        }
    }

    // pub fn return_as_pointer(&self) -> Option<bool> {
    //     match &self {
    //         Type::Callable(c) => Some(c.return_pointer),
    //         _ => None,
    //     }
    // }

    pub fn as_string(&self) -> String {
        format!("{self}")
    }

    pub fn printf_formatter(&self) -> &str {
        match self {
            Self::Int => "%d ",
            Self::Float => "%f ",
            Self::None => "None ",
            Self::Bool => "%b ",
            Self::String => "%s ",
            Self::Callable(f) => f.output.printf_formatter(),
            _ => " %s",
        }
    }

    pub fn get_concrete_type(&self) -> Self {
        match self {
            Self::Callable(c) => c.output.get_concrete_type().clone(),
            v => v.clone(),
        }
    }
}

impl std::fmt::Display for Type {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "Int"),
            Self::Float => write!(f, "Float"),
            Self::Bool => write!(f, "Bool"),
            Self::String => write!(f, "String"),
            Self::None => write!(f, "None"),
            Self::Unknown => write!(f, "Unknown"),
            Self::KeyWord => write!(f, "KeyWord"),
            Self::Callable(c) => write!(f, "Callable[{}]", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum CallableParameter {
    Infinite,
    Parameters(Vec<FunctionParameter>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Callable {
    pub parameters: CallableParameter,
    pub output: Box<Type>,
}

impl Callable {
    pub fn new(parameters: Vec<FunctionParameter>, output: Box<Type>) -> Self {
        Self {
            parameters: CallableParameter::Parameters(parameters),
            output,
        }
    }

    pub fn new_infinite(output: Box<Type>) -> Self {
        Self {
            parameters: CallableParameter::Infinite,
            output,
        }
    }
}

impl std::fmt::Display for Callable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.parameters {
            CallableParameter::Parameters(params) => {
                write!(
                    f,
                    "callable [{}] -> {}",
                    params
                        .iter()
                        .map(|p| p.name.to_owned())
                        .collect::<Vec<String>>()
                        .join(","),
                    self.output
                )
            }
            CallableParameter::Infinite => {
                write!(f, "callable [*] -> {}", self.output)
            }
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Condition {
    pub expr: Expr,
    pub then: Box<Stmt>,
    pub r#else: Option<Box<Stmt>>,
}

impl Condition {
    pub fn new(expr: Expr, then: Box<Stmt>, r#else: Option<Box<Stmt>>) -> Self {
        Self { expr, then, r#else }
    }
}

#[derive(Debug, PartialEq)]
pub struct Binary {
    pub left: Box<Expr>,
    pub operator: Operator,
    pub right: Box<Expr>,
}

impl Binary {
    pub fn new(left: Box<Expr>, operator: Operator, right: Box<Expr>) -> Self {
        Self {
            left,
            operator,
            right,
        }
    }
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
    pub value: Expr,
    pub mutable: bool,
    pub typing: Type,
}

impl Variable {
    pub fn new(name: String, value: Expr, mutable: bool, typing: Type) -> Self {
        Variable {
            name,
            value,
            mutable,
            typing,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Assign {
    pub name: String,
    pub value: Box<Expr>,
    pub typing: Type,
}

impl Assign {
    pub fn new(name: String, value: Box<Expr>, typing: Type) -> Self {
        Self {
            name,
            value,
            typing,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Call {
    pub callee: Box<Expr>,
    pub arguments: Vec<Box<Expr>>,
}

impl Call {
    pub fn new(callee: Box<Expr>, arguments: Vec<Box<Expr>>) -> Self {
        Self { callee, arguments }
    }
}

#[derive(Debug, PartialEq)]
pub struct Return {
    pub value: Expr,
    pub typing: Type,
}

impl Return {
    pub fn new(value: Expr, typing: Type) -> Self {
        Self { value, typing }
    }
}

#[derive(Debug, PartialEq)]
pub struct Assert {
    pub value: Expr,
}

impl Assert {
    pub fn new(value: Expr) -> Self {
        Self { value }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub mutable: bool,
    pub typing: Type,
}

impl FunctionParameter {
    pub fn new(name: String, mutable: bool, typing: Type) -> Self {
        Self {
            name,
            mutable,
            typing,
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
    pub output: Type,
}

impl Function {
    pub fn new(
        name: String,
        params: Vec<FunctionParameter>,
        body: Vec<Stmt>,
        output: Type,
    ) -> Self {
        Function {
            name,
            params,
            body,
            output,
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct While {
    pub condition: Expr,
    pub body: Box<Stmt>,
}

impl While {
    pub fn new(condition: Expr, body: Box<Stmt>) -> Self {
        Self { condition, body }
    }
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
    pub typing: Type,
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
    pub fn new(body: ExprBody, span: Span, typing: Type) -> Self {
        Expr { body, span, typing }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt {
    Expression(Expr),
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

impl std::fmt::Display for Binary {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Binary[{}, {}, {}]",
            self.left, self.operator, self.right
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
    use super::{Condition, Expr, ExprBody, Return, Span, Stmt, Type, Value, Variable, While};

    #[test]
    fn test_get_all_returns_no_returns() {
        let ast = Stmt::Var(Variable {
            name: "variable".to_owned(),
            value: Expr::new(ExprBody::Value(Value::True), Span::new(1, 1), Type::Unknown),
            mutable: true,
            typing: Type::Unknown,
        });

        assert_eq!(ast.get_all_returns().len(), 0)
    }

    #[test]
    fn test_get_all_returns_simple() {
        let ast = Stmt::Return(Return::new(
            Expr::new(
                ExprBody::Value(Value::False),
                Span::new(1, 1),
                Type::Unknown,
            ),
            Type::Unknown,
        ));

        assert_eq!(ast.get_all_returns().len(), 1)
    }

    #[test]
    fn test_get_all_returns_several() {
        let condition = Stmt::Condition(Condition::new(
            Expr::new(
                ExprBody::Value(Value::Integer(1)),
                Span::new(1, 1),
                Type::Unknown,
            ),
            Box::new(Stmt::Return(Return::new(
                Expr::new(
                    ExprBody::Value(Value::False),
                    Span::new(1, 1),
                    Type::Unknown,
                ),
                Type::Unknown,
            ))),
            Some(Box::new(Stmt::Return(Return::new(
                Expr::new(
                    ExprBody::Value(Value::False),
                    Span::new(1, 1),
                    Type::Unknown,
                ),
                Type::Unknown,
            )))),
        ));

        assert_eq!(condition.get_all_returns().len(), 2)
    }

    #[test]
    fn test_get_all_returns_condition_without_else() {
        let condition = Stmt::Condition(Condition::new(
            Expr::new(
                ExprBody::Value(Value::Integer(1)),
                Span::new(1, 1),
                Type::Unknown,
            ),
            Box::new(Stmt::Return(Return::new(
                Expr::new(
                    ExprBody::Value(Value::False),
                    Span::new(1, 1),
                    Type::Unknown,
                ),
                Type::Unknown,
            ))),
            None,
        ));

        assert_eq!(condition.get_all_returns().len(), 1)
    }

    #[test]
    fn test_get_all_returns_block() {
        let block = Stmt::Block(vec![
            Stmt::While(While::new(
                Expr::new(
                    ExprBody::Value(Value::False),
                    Span::new(1, 1),
                    Type::Unknown,
                ),
                Box::new(Stmt::Condition(Condition::new(
                    Expr::new(
                        ExprBody::Value(Value::Integer(1)),
                        Span::new(1, 1),
                        Type::Unknown,
                    ),
                    Box::new(Stmt::Return(Return::new(
                        Expr::new(
                            ExprBody::Value(Value::False),
                            Span::new(1, 1),
                            Type::Unknown,
                        ),
                        Type::Unknown,
                    ))),
                    Some(Box::new(Stmt::Return(Return::new(
                        Expr::new(
                            ExprBody::Value(Value::False),
                            Span::new(1, 1),
                            Type::Unknown,
                        ),
                        Type::Unknown,
                    )))),
                ))),
            )),
            Stmt::Return(Return::new(
                Expr::new(
                    ExprBody::Value(Value::False),
                    Span::new(1, 1),
                    Type::Unknown,
                ),
                Type::Unknown,
            )),
        ]);

        assert_eq!(block.get_all_returns().len(), 3)
    }
}
