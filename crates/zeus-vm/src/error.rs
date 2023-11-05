pub enum InterpreterError {
    Ok,
    EmptyStack,
    ConstantNotFound,
    CompileError(String),
    RuntimeError(RuntimeErrorType),
}

pub enum RuntimeErrorType {
    ValueError(String),
    KeyError(String),
}

impl InterpreterError {
    pub fn value_error(msg: String) -> Self {
        InterpreterError::RuntimeError(RuntimeErrorType::ValueError(msg))
    }
}

impl std::fmt::Display for RuntimeErrorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValueError(s) => write!(f, "ValueError: {s}"),
            Self::KeyError(s) => write!(f, "KeyError: {s}"),
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
        }
    }
}
