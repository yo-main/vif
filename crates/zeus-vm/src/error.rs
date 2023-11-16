pub enum InterpreterError {
    Ok,
    EmptyStack,
    ConstantNotFound,
    CompileError(String),
    RuntimeError(RuntimeErrorType),
    Impossible,
}

pub enum RuntimeErrorType {
    ValueError(String),
    KeyError(String),
    DivideByZero(String),
    UndeclaredVariable(String),
}

#[macro_export]
macro_rules! value_error {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        Err($crate::error::InterpreterError::RuntimeError($crate::error::RuntimeErrorType::ValueError(res)))
    }}
}

#[macro_export]
macro_rules! divide_by_zero_error {
    ($($arg:tt)*) => {{
        let res = format!($($arg)*);
        Err($crate::error::InterpreterError::RuntimeError($crate::error::RuntimeErrorType::DivideByZero(res)))
    }}
}

impl std::fmt::Display for RuntimeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueError(s) => write!(f, "ValueError: {s}"),
            Self::KeyError(s) => write!(f, "KeyError: {s}"),
            Self::DivideByZero(s) => write!(f, "Divide by zero: {s}"),
            Self::UndeclaredVariable(s) => write!(f, "Undeclared variable: {s}"),
        }
    }
}

impl std::fmt::Display for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK error"),
            Self::CompileError(e) => write!(f, "Compiling error: {e}"),
            Self::RuntimeError(e) => write!(f, "Interpreter error: {e}"),
            Self::EmptyStack => write!(f, "Empty Stack"),
            Self::ConstantNotFound => write!(f, "Constant not found"),
            Self::Impossible => write!(f, "Impossible"),
        }
    }
}

impl std::fmt::Debug for InterpreterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ok => write!(f, "OK error"),
            Self::CompileError(e) => write!(f, "Compiling error: {e}"),
            Self::RuntimeError(e) => write!(f, "Interpreter error: {e}"),
            Self::EmptyStack => write!(f, "Empty Stack"),
            Self::ConstantNotFound => write!(f, "Constant not found"),
            Self::Impossible => write!(f, "Impossible"),
        }
    }
}
