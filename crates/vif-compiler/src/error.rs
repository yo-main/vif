#[derive(Debug)]
pub enum CompilerError {
    EOF,
    Break,
    Continue,
    ConstantNotFound(String),
    ScanningError(String),
    SyntaxError(String),
    Unknown(String),
}

impl std::fmt::Display for CompilerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::EOF => write!(f, "EOF"),
            Self::Break => write!(f, "Break"),
            Self::Continue => write!(f, "Continue"),
            Self::ConstantNotFound(e) => write!(f, "ConstantNotFound: {e}"),
            Self::ScanningError(e) => write!(f, "ScanningError: {e}"),
            Self::SyntaxError(e) => write!(f, "SyntaxError: {e}"),
            Self::Unknown(e) => write!(f, "Unknown: {e}"),
        }
    }
}
