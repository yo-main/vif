use vif_objects::{errors::ValueError, op_code::ItemReference, span::Span};

pub enum InterpreterError {
    ValueError(String),
    WrongValue(String),
    KeyError(String),
    DivideByZero(String),
    UndeclaredVariable(String),
    FunctionCall(String),
    FunctionFailed(String),
    AssertFail(String),
}

impl InterpreterError {
    pub fn add_span(&mut self, content: &str, reference: &ItemReference) {
        match self {
            Self::ValueError(ref mut e) => *e = reference.format(content, e.as_str()),
            Self::WrongValue(ref mut e) => *e = reference.format(content, e.as_str()),
            Self::KeyError(ref mut e) => *e = reference.format(content, e.as_str()),
            Self::DivideByZero(ref mut e) => *e = reference.format(content, e.as_str()),
            Self::UndeclaredVariable(ref mut e) => *e = reference.format(content, e.as_str()),
            Self::FunctionCall(ref mut e) => *e = reference.format(content, e.as_str()),
            Self::FunctionFailed(ref mut e) => *e = reference.format(content, e.as_str()),
            Self::AssertFail(ref mut e) => *e = reference.format(content, e.as_str()),
        };
    }
}

#[macro_export]
macro_rules! value_error {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        Err($crate::error::InterpreterError::ValueError(res))
    }}
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueError(s) => write!(f, "{s} (ValueError)"),
            Self::WrongValue(s) => write!(f, "{s} (WrongValue)"),
            Self::KeyError(s) => write!(f, "{s} (KeyError)"),
            Self::DivideByZero(s) => write!(f, "{s} (DivideByZero)"),
            Self::UndeclaredVariable(s) => write!(f, "{s} (Undeclared Variable)"),
            Self::FunctionCall(s) => write!(f, "{s} (FunctionCall)"),
            Self::FunctionFailed(s) => write!(f, "{s} (FunctionFailed)"),
            Self::AssertFail(s) => write!(f, "{s}"),
        }
    }
}

impl From<ValueError> for InterpreterError {
    fn from(value: ValueError) -> Self {
        InterpreterError::ValueError(format!("{value}"))
    }
}
