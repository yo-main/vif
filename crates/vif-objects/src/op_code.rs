use crate::span::Span;
use crate::variable::InheritedLocalPos;

#[derive(PartialEq, Debug)]
pub struct Return {
    span: Option<Span>,
}

impl Return {
    pub fn new(span: Option<Span>) -> Self {
        Self { span }
    }
}

#[derive(PartialEq, Debug)]
pub enum OpCode {
    Return(Return),
    Global(usize),
    GlobalVariable(usize),
    GetGlobal(usize),
    SetGlobal(usize),
    GetLocal(usize),
    CreateLocal(usize),
    SetLocal(usize),
    GetInheritedLocal(InheritedLocalPos),
    SetInheritedLocal(InheritedLocalPos),
    Negate,
    Add,
    Substract,
    Multiply,
    Divide,
    Modulo,
    True,
    False,
    None,
    Not,
    Equal,
    NotEqual,
    Greater,
    Less,
    GreaterOrEqual,
    LessOrEqual,
    Pop,
    AssertTrue,
    JumpIfFalse(usize),
    Jump(usize),
    Goto(usize),
    Call(usize),
    NotImplemented,
}

impl std::fmt::Display for OpCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Return(_) => write!(f, "OP_RETURN"),
            Self::Global(c) => write!(f, "OP_GLOBAL({c})"),
            Self::GlobalVariable(c) => write!(f, "OP_GLOBAL_VARIABLE({c})"),
            Self::GetGlobal(c) => write!(f, "OP_GET_GLOBAL({c})"),
            Self::SetGlobal(c) => write!(f, "OP_SET_GLOBAL({c})"),
            Self::GetLocal(c) => write!(f, "OP_GET_LOCAL({c})"),
            Self::CreateLocal(c) => write!(f, "OP_CREATE_LOCAL({c})"),
            Self::SetLocal(c) => write!(f, "OP_SET_LOCAL({c})"),
            Self::GetInheritedLocal(v) => write!(f, "OP_GET_INH_LOCAL({v})"),
            Self::SetInheritedLocal(v) => write!(f, "OP_SET_INH_LOCAL({v})"),
            Self::Negate => write!(f, "OP_NEGATE"),
            Self::Add => write!(f, "OP_ADD"),
            Self::Substract => write!(f, "OP_SUBSTRACT"),
            Self::Multiply => write!(f, "OP_MULTIPLY"),
            Self::Divide => write!(f, "OP_DIVIDE"),
            Self::Modulo => write!(f, "OP_MODULO"),
            Self::True => write!(f, "OP_TRUE"),
            Self::False => write!(f, "OP_FALSE"),
            Self::None => write!(f, "OP_NONE"),
            Self::Not => write!(f, "OP_NOT"),
            Self::Equal => write!(f, "OP_EQUAL"),
            Self::NotEqual => write!(f, "OP_NOT_EQUAL"),
            Self::Greater => write!(f, "OP_GREATER"),
            Self::Less => write!(f, "OP_LESS"),
            Self::GreaterOrEqual => write!(f, "OP_GREATER_OR_EQUAL"),
            Self::LessOrEqual => write!(f, "OP_LESS_OR_EQUAL"),
            Self::Pop => write!(f, "OP_POP"),
            Self::JumpIfFalse(i) => write!(f, "OP_JUMP_IF_FALSE {i}"),
            Self::Jump(i) => write!(f, "OP_JUMP {i}"),
            Self::Goto(i) => write!(f, "OP_GOTO {i}"),
            Self::Call(i) => write!(f, "OP_CALL {i}"),
            Self::AssertTrue => write!(f, "OP_ASSERT_TRUE"),
            Self::NotImplemented => write!(f, "NOT_IMPLEMENTED"),
        }
    }
}
